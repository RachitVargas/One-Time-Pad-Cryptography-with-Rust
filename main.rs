use std::io::{stdout, BufWriter, Bytes, Error, Read};
use std::io;

fn main() {

    let mut message  = String::new();
    println!("Hi, enter the message");
    match io::stdin().read_line(&mut message){
        Ok(_) => {
            println!("Your message is: {}", message)
        }
        _ => {}
    }

    let mut key = String::new();
    println!("Now, enter the key");
    match io::stdin().read_line(&mut key){
        Ok(_) => {
            println!("Your key is: {}", key)
        }
        _ => {}
    }

    if (message.len() == key.len()){
        let ciphertext =  encrypt(String::from(message), String::from(key));
        println!("This is the ciphertext: {}", ciphertext.to_string());

        let mut key_two = String::new();
        println!("Again, enter the key");
        match io::stdin().read_line(&mut key_two){
            Ok(_) => {
                println!("Your key is: {}", key_two)
            }
            _ => {}
        }

        if (key_two.len() == ciphertext.len()) {
            let message_decrypt = decrypt(String::from(ciphertext),
                                          String::from(key_two));
            println!("This is the mesagge: {}", message_decrypt);
        }else{
            println!("sorry, the length of the key and the previous must be same")
        }

    }else{
        println!("sorry, the length of the key and the message must be the same length")
    }

    println!("Thank you for use the One Time Pad!")
}

fn encrypt(message: String, key: String) -> String {

    let mut ciphertext = String::new();
    let message_char: Vec<char> = message.chars().collect();
    let key_char: Vec<char> = key.chars().collect();

    let mut xor_aux: u32;

    for i in 0 .. key.chars().count() {
        xor_aux = u32::from(message_char[i]) ^ u32::from(key_char[i]);
        let help_to_push = std::char::from_u32(xor_aux);
        ciphertext.push(*help_to_push.as_ref().unwrap());
    }
    return String::from(ciphertext.as_str());
}

fn decrypt (ciphertext:String, key_two: String) -> String{

    let mut message = String::new();
    let ciphertext_char: Vec<char> = ciphertext.chars().collect();
    let key_char: Vec<char> = key_two.chars().collect();
    let mut xor_aux: u32;

    for i in 0 .. key_two.chars().count() {
        xor_aux = u32::from(ciphertext_char[i]) ^ u32::from(key_char[i]);
        let help_to_push = std::char::from_u32(xor_aux);
        message.push(*help_to_push.as_ref().unwrap());
    }
    return String::from(message.as_str());
}
