use ferris_says::say;
use std::io::{stdout,  BufWriter};

fn main() {

    let message = "s";
    let key = "s";

    let ciphertext =  encrypt(String::from(message), String::from(key));
    println!("This is the ciphertext: {}",ciphertext);

  // let message_decrypt = decrypt(String::from(ciphertext), String::from(key));
    //println!("{}", std::str::from_utf8((&message_decrypt).as_ref()).unwrap());
}

fn encrypt(message: String, key: String) -> String {

    let message_bytes = message.as_bytes();
    let key_bytes = key.as_bytes();

    let ciphertext = key_bytes ^ message_bytes;

    return  String::from("d");
}

//fn decrypt(ciphertext: String, key: String) -> String {

//      let key_bytes = key.as_bytes();
//      let message = key_bytes ^ ciphertext;
//
  //  return  String::from(message);
//}

//println!("{}", std::str::from_utf8(&key_bytes).unwrap());
