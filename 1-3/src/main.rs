use std::env;

extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex};

fn main() {
    let encoded_hex_string = env::args().nth(1).unwrap();
    let encoded = encoded_hex_string.from_hex().unwrap();
    let best_key = find_best_key(&encoded);
    println!("{} -> {}", best_key as char, decode(&encoded, best_key));
}

fn find_best_key(encoded: &Vec<u8>)  -> u8 {
    let mut best_score = 0;
    let mut best_key = 33;
    for key_char in 33..126 {
        let decoded: Vec<u8> = encoded.iter().map(|x| x ^ key_char).collect();
        let score = count_alphanumeric_chars(&decoded);
        if score > best_score {
            best_key = key_char;
            best_score = score;
        }
    }
    best_key
}

fn decode(encoded: &Vec<u8>, key_char: u8) -> String {
    let decoded: Vec<u8> = encoded.iter().map(|x| x ^ key_char).collect();
    String::from_utf8(decoded).unwrap()
}

fn is_alphanumeric_char(byte: u8) -> bool {
    byte > 64 && byte < 123
}

fn count_alphanumeric_chars(bytes: &Vec<u8>) -> usize {
    bytes.iter().filter(|&x| is_alphanumeric_char(*x)).count()
}

#[test]
fn can_find_best_key() {
    let encoded_hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encoded = encoded_hex_string.from_hex().unwrap();
    
    let best_key = find_best_key(&encoded);
    
    assert_eq!(88, best_key);
}

#[test]
fn can_decrypt() {
    let encoded_hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encoded = encoded_hex_string.from_hex().unwrap();
    
    let decoded_text = decode(&encoded, 88);
    
    assert_eq!("Cooking MC's like a pound of bacon", decoded_text);
}

#[test]
fn can_tell_if_byte_is_an_alphanumeric_char() {
    let ascii_printable_char = "61".from_hex().unwrap()[0];
    
    assert!(is_alphanumeric_char(ascii_printable_char));
}

#[test]
fn can_tell_if_byte_is_not_an_alphanumeric_char() {
    let not_an_ascii_printable_char = "00".from_hex().unwrap()[0];
    
    assert!(! is_alphanumeric_char(not_an_ascii_printable_char));
}

#[test]
fn can_count_how_many_bytes_are_alphanumeric_chars() {
    let data_with_two_printable_chars = "00660065".from_hex().unwrap();
    
    assert_eq!(2, count_alphanumeric_chars(&data_with_two_printable_chars));
}
