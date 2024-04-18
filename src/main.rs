use std::io;
use sha256::digest;
use num::BigInt;


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn entropy_to_mnemonic12(entropy: &[u8]) -> Vec<String> {
    // Ensure the entropy has exactly 16 bytes
    assert!(entropy.len() == 16);
    //println!("entropy {:?}", entropy);
    let other_entropy: [u8; 16] = entropy.try_into().unwrap();
    let v = u128::from_be_bytes(other_entropy);
    let v = BigInt::from(v);
    
    //equivalent of left bitshift of 4
    //v = v * 16;
    let mut v = v << 4;     
    
    let mut indexes = Vec::new();
    
    for i in 0..12 {
        let m:  BigInt = &v % BigInt::from(2048);
        let m = m.to_u32_digits().1[0] as u16;
        indexes.insert(0, m);
        v = v / 2048;
    }

    assert_eq!(v, BigInt::from(0));

    // Add the checksum to the last index
    let a = digest(entropy);
    let offset = &mut indexes[11];
    let checksum= (hex::decode(&a).expect("invalid hex")[0] >> 4) as u16;
    *offset = checksum + *offset;
    
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
