use std::env;

extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex, ToHex};

fn main() {
    let a_hex = env::args().nth(1).unwrap();
    let b_hex = env::args().nth(2).unwrap();
    println!("{}", hex_xor(a_hex, b_hex));
}

fn hex_xor(a_hex: String, b_hex:String) -> String {
    let a = a_hex.from_hex().unwrap();
    let b = b_hex.from_hex().unwrap();
    xor(a, b).to_hex()
}

fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.iter().enumerate().map(|(i,x)| x ^ b[i]).collect()
}

#[test]
fn can_xor() {
    let a = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
    let b = "686974207468652062756c6c277320657965".from_hex().unwrap();
    
    let a_xor_b = xor(a, b);
    
    assert_eq!("746865206b696420646f6e277420706c6179", a_xor_b.to_hex());
}

#[test]
fn can_xor_on_hex_str() {
    let a_hex = "1c0111001f010100061a024b53535009181c".to_string();
    let b_hex = "686974207468652062756c6c277320657965".to_string();
    
    let a_xor_b_hex = hex_xor(a_hex, b_hex);
    
    assert_eq!("746865206b696420646f6e277420706c6179", a_xor_b_hex);
}
