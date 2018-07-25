use clap::ArgMatches;
use std::process;

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
            result.push(alphabets::base64(short_intval).to_string());
        }
        if (idx + 6) <= bit_str_len {
            let slice = &vec[idx..idx+6].join("");
            //println!("The slice is --> {}", slice);
            let intval = isize::from_str_radix(slice, 2).unwrap();
            //println!("The intval is --> {}", intval);
            result.push(alphabets::base64(intval).to_string());
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

pub fn handle_encoding(matches: &ArgMatches) {
    match matches.value_of("input") {
        Some(x) => println!("{}", encode_to_base64(x)),
        None    => {
            println!("");
            process::exit(0);
        }
    }
}

pub fn handle_decoding(_matches: &ArgMatches) {
    println!("handle_decoding");  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc4648_test_vector_empty_string() {
        assert_eq!(encode_to_base64(""), "");
    }

    #[test]
    fn rfc4648_test_vector_f_string() {
        assert_eq!(encode_to_base64("f"), "Zg==");
    }

    #[test]
    fn rfc4648_test_vector_fo_string() {
        assert_eq!(encode_to_base64("fo"), "Zm8=");
    }

    #[test]
    fn rfc4648_test_vector_foo_string() {
        assert_eq!(encode_to_base64("foo"), "Zm9v");
    }

    #[test]
    fn rfc4648_test_vector_foob_string() {
        assert_eq!(encode_to_base64("foob"), "Zm9vYg==");
    }

    #[test]
    fn rfc4648_test_vector_fooba_string() {
        assert_eq!(encode_to_base64("fooba"), "Zm9vYmE=");
    }

    #[test]
    fn rfc4648_test_vector_foobar_string() {
        assert_eq!(encode_to_base64("foobar"), "Zm9vYmFy");
    }
}
