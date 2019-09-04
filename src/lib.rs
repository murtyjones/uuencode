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

pub fn uuencode(filename: &str, input: &[u8]) -> String {
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

pub fn uudecode(encoded: &str) -> Option<(Vec<u8>, String)> {
    let mut lines = encoded.lines();

    let name = lines.next().expect("No next lines!").split(" ").collect::<Vec<_>>()[2].to_string(); //eugh

    let mut output: Vec<u8> = Vec::new();
    for line in lines {
        let padded_line = maybe_pad_line(line);
        if let Some(chr) = padded_line.chars().nth(0) {
            match chr {
                '`' => break,
                ' '...'_' => {
                    for dc in padded_line[1..].as_bytes().chunks(4) {
                        output.extend( uudecode_chunk(dc) );
                    }
                },
                _ => break
            }
        }
    }
    Some((output, name))
}

/// Ensure that a line has sufficient padding
fn maybe_pad_line(line: &str) -> String {
    const REQUIRED_LENGTH: usize = 61;
    let actual_length = line.len();
    let diff = REQUIRED_LENGTH - actual_length;
    match diff {
        d if d <= 0 => String::from(line),
        _ => {
            let mut padded = String::from(line);
            for i in 1..=diff {
                padded.push(' ');
            }
            return padded;
        },
    }
}


mod test {
    use crate::*;
    use std::io::prelude::*;
    use std::fs::File;

    fn write_to_file(filename: String, data: &[u8]) -> std::io::Result<()> {
        let mut pos = 0;
        let mut buffer = std::fs::File::create(format!("/Users/murtyjones/Desktop/{}", filename)).expect("Couldn't make file!");
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..]).expect("Couldn't write to file!");
            pos += bytes_written;
        }
        Ok(())
    }

    #[test]
    fn test_cat() {
        let filename = "wow.jpg";
        let original_encoded = "begin 644 wow.jpg\nM0V%T                                                        \n`\nend";
        let decoded = uudecode(original_encoded).unwrap();
        let encoded = uuencode(filename, decoded.0.as_slice());
        assert_eq!(original_encoded, encoded);
    }

    #[test]
    fn test_logo() {
        let filename = "amglogoa09.jpg";
        let original_encoded = include_str!("../images/logo_encoded_padded").trim();
        let decoded = uudecode(original_encoded).unwrap();
        write_to_file(decoded.1, decoded.0.as_slice());
        let encoded = uuencode(filename, decoded.0.as_slice());
        assert_eq!(original_encoded, encoded);
    }

    #[test]
    fn test_piechart() {
        let filename = "aumpiechartscombinded5217v4.jpg";
        let original_encoded = include_str!("../images/piechart_encoded_padded").trim();
        let decoded = uudecode(original_encoded).unwrap();
        write_to_file(decoded.1, decoded.0.as_slice());
        let encoded = uuencode(filename, decoded.0.as_slice());
        assert_eq!(original_encoded, encoded);
    }
    
    #[test]
    fn test_pad_line() {
        let unpadded = r#"=HHH **** "BBB@ HHHH **** "BBB@ HHHH _]D!"#;
        let padded = r#"=HHH **** "BBB@ HHHH **** "BBB@ HHHH _]D!                    "#;
        let r = maybe_pad_line(unpadded);
        assert_eq!(padded, r);
    }
}
