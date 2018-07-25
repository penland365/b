use clap::ArgMatches;
use std::process;
use std::str;

use alphabets;

fn encode_to_base64(input_str: &str) -> String {
    let bs = input_str.as_bytes(); 
    let mut vec: Vec<String> = Vec::new();
    for elem in bs.iter() {
        let bit_str = format!("{:b}", elem);
        let bit_str_len = bit_str.len();
        let num_leading_0s = 8 - bit_str_len; 
        let mut idx = 0;
        while idx < num_leading_0s {
            vec.push("0".to_string());
            idx += 1;
        }
        for bit in bit_str.chars() {
            vec.push(bit.to_string());
        }
    }

    let mut idx = 0;
    let bit_str_len = vec.len();
    let mut result: Vec<String> = Vec::new();
    //println!("{:?}", vec);
    while idx < bit_str_len {
        //println!("Idx[{}], bit_str_len[{}]", idx, bit_str_len);
        if bit_str_len - idx < 6 {
            let mut short_slice = vec[idx..].join("");
            let mut remainder = bit_str_len - idx;
            while remainder < 6 {
                short_slice.push_str("0");
                remainder += 1;
            }
            //println!("The short slice is --> {}", short_slice);
            let short_intval = isize::from_str_radix(&short_slice, 2).unwrap();
            //println!("The intval is --> {}", short_intval);
            result.push(alphabets::base64_encode(short_intval).to_string());
        }
        if (idx + 6) <= bit_str_len {
            let slice = &vec[idx..idx+6].join("");
            //println!("The slice is --> {}", slice);
            let intval = isize::from_str_radix(slice, 2).unwrap();
            //println!("The intval is --> {}", intval);
            result.push(alphabets::base64_encode(intval).to_string());
        }
        idx += 6;
    }
    
    let num_quantum = bit_str_len / 8;
    if (num_quantum % 3) == 2 {
        result.push("=".to_string());
    } else if (num_quantum % 3) == 1 {
        result.push("=".to_string());
        result.push("=".to_string());
    }

    result.join("")
}

fn decode_from_base64(input_str: &str) -> String {
    let mut vec: Vec<String> = Vec::new();
    for elem in input_str.to_string().chars() {
        if elem != '=' {
            let bit_str = format!("{:b}", alphabets::base64_decode(&elem.to_string()));

            // ensure that each decimal decoded has a full 6 bits,
            // adding all leading 0s as necessary
            let mut num_remainder_bits: isize = 6 - (bit_str.len() as isize);
            while num_remainder_bits > 0 {
                vec.push("0".to_string());
                num_remainder_bits -= 1;
            }

            for bit in bit_str.to_string().chars() {
                vec.push(bit.to_string());
            }
        } else {
            vec.push("0".to_string());
            vec.push("0".to_string());
            vec.push("0".to_string());
            vec.push("0".to_string());
            vec.push("0".to_string());
            vec.push("0".to_string());
        }
    }

    let mut utf_bytes: Vec<u8> = Vec::new();
    let mut idx = 0;
    while idx < vec.len() {
        let slice = vec[idx..idx+8].join("");
        utf_bytes.push(isize::from_str_radix(&slice, 2).unwrap() as u8);

        idx += 8;
    }

    let null: u8 = 0;
    let foo_bar : Vec<u8> = utf_bytes.into_iter()
        .filter(|x| x != &null)
        .collect();

    str::from_utf8(&foo_bar).unwrap().to_string()
}

pub fn handle_encoding(matches: &ArgMatches) {
    match matches.value_of("input") {
        Some(x) => println!("{}", encode_to_base64(x)),
        None    => {
            println!("");
            process::exit(0);
        }
    }
}

pub fn handle_decoding(matches: &ArgMatches) {
    match matches.value_of("input") {
        Some(x) => println!("{}", decode_from_base64(x)),
        None    => {
            println!("");
            process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc4648_test_vector_empty_string() {
        let decoded = "";
        let encoded = "";
        assert_eq!(encode_to_base64(decoded), encoded);
        assert_eq!(decode_from_base64(encoded), decoded);
    }

    #[test]
    fn rfc4648_test_vector_f_string() {
        let decoded = "f";
        let encoded = "Zg==";
        assert_eq!(encode_to_base64(decoded), encoded);
        assert_eq!(decode_from_base64(encoded), decoded);
    }

    #[test]
    fn rfc4648_test_vector_fo_string() {
        let decoded = "fo";
        let encoded = "Zm8=";
        assert_eq!(encode_to_base64(decoded), encoded);
        assert_eq!(decode_from_base64(encoded), decoded);
    }

    #[test]
    fn rfc4648_test_vector_foo_string() {
        let decoded = "foo";
        let encoded = "Zm9v";
        assert_eq!(encode_to_base64(decoded), encoded);
        assert_eq!(decode_from_base64(encoded), decoded);
    }

    #[test]
    fn rfc4648_test_vector_foob_string() {
        let decoded = "foob";
        let encoded = "Zm9vYg==";
        assert_eq!(encode_to_base64(decoded), encoded);
        assert_eq!(decode_from_base64(encoded), decoded);
    }

    #[test]
    fn rfc4648_test_vector_fooba_string() {
        let decoded = "fooba";
        let encoded = "Zm9vYmE=";
        assert_eq!(encode_to_base64(decoded), encoded);
        assert_eq!(decode_from_base64(encoded), decoded);
    }

    #[test]
    fn rfc4648_test_vector_foobar_string() {
        let decoded = "foobar";
        let encoded = "Zm9vYmFy";
        assert_eq!(encode_to_base64(decoded), encoded);
        assert_eq!(decode_from_base64(encoded), decoded);
    }

    quickcheck! {
        fn base64_roundtrip(s: String) -> bool {
            let mut non_nulls: String = s;
            non_nulls.retain(|c| c != '\0');
            non_nulls == decode_from_base64(&encode_to_base64(&non_nulls)) 
        }
    }

}
