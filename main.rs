use ferris_says::say;
use std::io::{stdout, BufWriter, Bytes, Error};
use std::io;
use std::str::from_utf8;
use std::ops::{BitXor, BitXorAssign};

fn main() {

    let mut message  = String::new();

    println!("Hi, enter the message");
    match io::stdin().read_line(&mut message){
       Ok(_) => {
           println!("You message is: {}", message)
       }
        _ => {}
    }

    let mut key = String::new();

    println!("Now, enter the key");
    match io::stdin().read_line(&mut key){
        Ok(_) => {
            println!("You key is: {}", key)
        }
        _ => {}
    }

    if (message.len() == key.len()){
        let ciphertext =  encrypt(String::from(message), String::from(key));
        println!("This is the ciphertext: {}", ciphertext);

        //let message_decrypt = decrypt(String::from(ciphertext), String::from(key));
        //println!("This is the mesagge: {}", message_decrypt);

    }else{
       println!("sorry, the length of the key and the message must be the same length")
    }

    println!("Thank you for use the program!")
}

fn encrypt(message: String, key: String) -> String {

    let message_bytes= message.as_bytes();
    let key_bytes= key.as_bytes();
    let mut ciphertext = ;

    return String::from(std::str::from_utf8(&ciphertext).unwrap());
}

//fn decrypt(ciphertext: String, key: String) -> String {

//      let key_bytes = key.as_bytes();
//      let message = key_bytes ^ ciphertext;
//
  //  return  String::from(std::str::from_utf8(&message).unwrap());
//}
