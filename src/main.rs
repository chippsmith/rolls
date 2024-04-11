use std::{io, ptr::read};
use sha256::digest;
extern crate sha256;


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn entropy_to_mnemonic12(entropy: &[u8]) -> Vec<String> {
    // Ensure the entropy has exactly 16 bytes
    assert!(entropy.len() == 16);
    println!("entropy {:?}", entropy);
    let other_entropy: [u8; 16] = entropy.try_into().unwrap();
    println!("other entropy {:?}", other_entropy);
    let mut v = u128::from_be_bytes(other_entropy);
    println!("v before byte shift: {}", v);

    //equivalent of left bitshift of 4
    //v = v * 16;
    
    let mut v = v << 16;

    println!("v after byte shift: {}", v);
     
    

    //let mut v: u128 = 0x00;
    //let mut v: u128 = 0xffffffffffffffffffffffffffffffff;
    let mut indexes = Vec::new();
    for i in 0..12 {
        let m = v % 2048;
        indexes.insert(i, m as u64);
        v = v / 2048;

    }

    println!("indexes {:?}", indexes);
  
    // Add the checksum to the last index
    
  
    let wl = include_str!("english_wordlist.txt");
    let mut mnemonics = Vec::new();
    for i in indexes {
        let word = wl.lines().skip(i as usize).next().unwrap().to_string();
        mnemonics.push(word);
    }
  
    mnemonics
  }


fn main() {
    
    println!("Enter 99 dice rolls or more");
    let num = read_input();

    let h = digest(&num);

    if h == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(){
        println!("WARNING: Input is empty. This is a known wallet\n")
    }

    if num.len() < 99 {
        let bits_of_entropy = 2.585 * num.len() as f64;
        println!("'WARNING: Input is only %d bits of entropy {}\n", bits_of_entropy)
    }

    let byte_slice = hex::decode(&h).expect("invalid hex");
    //let aaa: [u8; 8] = byte_slice.try_into().unwrap();
    let a: &[u8] = &byte_slice[0..16];

    let mnemonic = entropy_to_mnemonic12(a);

    println!("mnemonic: {:?}", mnemonic)

}
