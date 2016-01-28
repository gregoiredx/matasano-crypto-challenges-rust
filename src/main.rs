extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex, ToHex};

fn main() {
    let a_hex = "1c0111001f010100061a024b53535009181c";
    let b_hex = "686974207468652062756c6c277320657965";
    let a = a_hex.from_hex().unwrap();
    let b = b_hex.from_hex().unwrap();
    let mut a_xor_b = Vec::new();
    for (i, x) in a.iter().enumerate() {
        let y = b[i];
        a_xor_b.push(x ^ y);
    }
    let a_xor_b2: Vec<u8> = a.iter().enumerate().map(|(i,x)| x ^ b[i]).collect();
    let a_xor_b_hex = a_xor_b.to_hex();
    println!("{}", a_xor_b_hex);
    println!("{}", a_xor_b2.to_hex());
}
