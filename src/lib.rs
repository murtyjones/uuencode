extern crate binascii;

use binascii::*;

// encodes up to 3 bytes
fn uuencode(input: &[u8]) -> [u8;4] {
    // padding is hard
    let i = [ input[0],
        *input.get(1).unwrap_or(&0),
        *input.get(2).unwrap_or(&0) ];

    [ 32 + (i[0]>>2),
        32 + ((i[0]<<6 | i[1]>>2) >> 2),
        32 + ((i[1]<<4 | i[2]>>4) >> 2),
        32 + ((i[2]<<2) >> 2) ]
}

fn uuencode_all(input: &[u8]) -> String {
    let mut output : Vec<u8> = Vec::new();
    // in rust, char != u8, so we need to prefix with a b
//    output.extend(b"begin 644 file.txt\n");
    for line in input.chunks(45) {
        let line_length = line.len() as u8 + 32;
        output.push(line_length);
        for c in line.chunks(3) {
            output.extend(uuencode(c).into_iter());
        }
        output.push(b'\n');
    }
//    output.extend(b"`\nend");

    String::from_utf8(output).unwrap()
}

mod test {
    use crate::*;

    #[test]
    fn test_cat() {
        const INPUT: &'static str = "Cat";
        const OUTPUT: &'static str = "#0V%T";
        let r = uuencode_all(INPUT.as_bytes());
        assert_eq!(OUTPUT, r);
    }

    #[test]
    fn decode_truncatd() {
        let input = include!("../logo_raw_truncated");
        let output = "M_]C_X  02D9)1@ ! @$!+ $L  #_[0 L4&AO=&]S:&]P(#,N,  X0DE- ^T \n";
        let r = uuencode_all(input);
        assert_eq!(output, r);
    }

    #[test]
    fn wow() {
        let input = include!("../logo_raw");
        let expected_encoded = include_str!("../logo_encoded_no_header");
        let r = uuencode_all(input);
        assert_eq!(expected_encoded, r);
    }

    #[test]
    fn hm() {
        let my_version = r#"M</9=\DGK\)Z58U/"JOJ]GA;4W*L?.B*R<?FA\5_P+U^FF)RV<R==>.V*O641"#;
        let sec_version_bytes = b"p\xf6]\xf2I\xeb\xf0\x9e\x95cS\xc2\xaa\xfa\xbd\x9e\x16\xd4\xdc\xab\x1f:\"#\xb2q\xf9\xa1\xf1_\xf0/_\xa6\x98\x9c\xb6s\']x\xed\x8a\xbde";
        let sec_version = r#"M</9=\DGK\)Z58U/"JOJ]GA;4W*L?.B(CLG'YH?%?\"]?IIB<MG,G77CMBKUE
"#;
        let r = uuencode_all(sec_version_bytes);
        assert_eq!(sec_version, r);
    }
}
