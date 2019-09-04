extern crate binascii;

use binascii::*;

// encodes up to 3 bytes
fn uuencode_chuck(input: &[u8]) -> [u8;4] {
    // padding is hard
    let i = [ input[0],
        *input.get(1).unwrap_or(&0),
        *input.get(2).unwrap_or(&0) ];

    [ 32 + (i[0]>>2),
        32 + ((i[0]<<6 | i[1]>>2) >> 2),
        32 + ((i[1]<<4 | i[2]>>4) >> 2),
        32 + ((i[2]<<2) >> 2) ]
}

// decodes up to 3 bytes
fn uudecode_chuck(input: &[u8]) -> [u8;4] {
    // padding is hard
    let i = [ input[0],
        *input.get(1).unwrap_or(&0),
        *input.get(2).unwrap_or(&0) ];

    [ 32 + (i[0]>>2),
        32 + ((i[0]<<6 | i[1]>>2) >> 2),
        32 + ((i[1]<<4 | i[2]>>4) >> 2),
        32 + ((i[2]<<2) >> 2) ]
}

fn uuencode(filename: &str, input: &[u8]) -> String {
    let mut output : Vec<u8> = Vec::new();
    // in rust, char != u8, so we need to prefix with a b

    output.extend(b"begin 644 ");
    output.extend(filename.as_bytes());
    output.extend(b"\n");
    for line in input.chunks(45) {
        let line_length = line.len() as u8 + 32;
        output.push(line_length);
        for c in line.chunks(3) {
            output.extend(uuencode_chuck(c).into_iter());
        }
        output.push(b'\n');
    }
    output.extend(b"`\nend");

    String::from_utf8(output).unwrap()
}

fn uudecode_chunk(bytes: &[u8]) -> impl Iterator<Item=u8> {
    let combined: u32 = bytes.iter().enumerate()
        .fold(0, | acc, (index, &val) | {
            acc + (((val as u32) - 32) << 6 * (3 - index))
        });

    (0..3).rev().map(move |val| {
        let val = (combined >> (8 * val)) & 255;
        val as u8
    })
}

fn uudecode(encoded: &str) -> Option<(Vec<u8>, String)> {
    let mut lines = encoded.lines();

    let name = lines.next().expect("No next lines!").split(" ").collect::<Vec<_>>()[2].to_string(); //eugh

    let mut output: Vec<u8> = Vec::new();
    for line in lines {
        if let Some(chr) = line.chars().nth(0) {
            match chr {
                '`' => break,
                ' '...'_' => {
                    for dc in line[1..].as_bytes().chunks(4) {
                        output.extend( uudecode_chunk(dc) );
                    }
                },
                _ => break
            }
        }
    }
    Some((output, name))
}


mod test {
    use crate::*;

    #[test]
    fn test_cat() {
        let input = "Cat\nCat";
        let filename = "wow.jpg";
        let expected_output = "begin 644 wow.jpg\n#0V%T\n#0V%T\n`\nend";
        let encoded = uuencode(filename, input.as_bytes());
        let decoded = uudecode(&*encoded).unwrap();
        assert_eq!(expected_output, encoded);
//        assert_eq!(input, decoded.0);
        assert_eq!(filename, decoded.1);
    }

    #[test]
    fn test_truncated_logo() {
        let input = include!("../logo_raw_truncated");
        let filename = "amglogoa09.jpg";
        let expected_output = "begin 644 amglogoa09.jpg\nM_]C_X  02D9)1@ ! @$!+ $L  #_[0 L4&AO=&]S:&]P(#,N,  X0DE- ^T \n`\nend";
        let encoded = uuencode(filename, input);
        let decoded = uudecode(&*encoded).unwrap();
        assert_eq!(expected_output, encoded);
        assert_eq!(input.len(), decoded.0.len());
        assert_eq!(input.to_vec(), decoded.0);
        assert_eq!(filename, decoded.1);
    }

    #[test]
    fn test_logo() {
        let input = include!("../logo_raw");
        let filename = "amglogoa09.jpg";
        let expected_output = include_str!("../logo_encoded");
        let encoded = uuencode(filename, input);
        let decoded = uudecode(expected_output);
//        assert_eq!(expected_output, encoded);
//        assert_eq!(expected_output, decoded.0);
//        assert_eq!(filename, decoded.1);
    }
}
