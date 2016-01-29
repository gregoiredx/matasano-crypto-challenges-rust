extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex};

fn main() {
    let encoded_hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encoded = encoded_hex_string.from_hex().unwrap();
    for key_char in 33..126 {
        let decoded: Vec<u8> = encoded.iter().map(|x| x ^ key_char).collect();
        if count_printable_chars(&decoded) == decoded.len() {
            println!("{} {}", key_char, decode(&encoded, key_char));
        }
    }
}

fn decode(encoded: &Vec<u8>, key_char: u8) -> String {
    let decoded: Vec<u8> = encoded.iter().map(|x| x ^ key_char).collect();
    String::from_utf8(decoded).unwrap()
}

fn is_printable(byte: u8) -> bool {
    byte > 31 && byte < 127
}

fn count_printable_chars(bytes: &Vec<u8>) -> usize {
    bytes.iter().filter(|&x| is_printable(*x)).count()
}

#[test]
fn can_decrypt() {
    let encoded_hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encoded = encoded_hex_string.from_hex().unwrap();
    
    let decoded_text = decode(&encoded, 88);
    
    assert_eq!("Cooking MC's like a pound of bacon", decoded_text);
}

#[test]
fn can_tell_if_byte_is_an_ascii_printable_char() {
    let ascii_printable_char = "61".from_hex().unwrap()[0];
    
    assert!(is_printable(ascii_printable_char));
}

#[test]
fn can_tell_if_byte_is_not_an_ascii_printable_char() {
    let not_an_ascii_printable_char = "00".from_hex().unwrap()[0];
    
    assert!(! is_printable(not_an_ascii_printable_char));
}

#[test]
fn can_count_how_many_bytes_are_ascii_printable_chars() {
    let data_with_two_printable_chars = "00320065".from_hex().unwrap();
    
    assert_eq!(2, count_printable_chars(data_with_two_printable_chars));
}
