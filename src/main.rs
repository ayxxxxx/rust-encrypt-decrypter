use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

fn xor_encrypt_decrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|byte| byte ^ key).collect()
}

fn encrypt_file(input_path: &str, output_path: &str, key: u8) -> std::io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let encrypted_data = xor_encrypt_decrypt(&buffer, key);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&encrypted_data)?;

    Ok(())
}

fn decrypt_file(input_path: &str, output_path: &str, key: u8) -> std::io::Result<()> {
    encrypt_file(input_path, output_path, key) // XOR operation
}

fn print_usage() {
    eprintln!("usage: locker (encrypt/decrypt) (input) (output) (key)");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        print_usage();
        process::exit(1);
    }
    let command = &args[1];
    let input = &args[2];
    let output = &args[3];
    let key: u8 = match args[4].parse() {
        Ok(k) => k,
        Err(_) => {
            eprintln!("error: invalid key, number must be between 1 and 255");
            process::exit(1);
        }
    };

    let result = match command.as_str() {
        "encrypt" => encrypt_file(input, output, key), "decrypt" => decrypt_file(input, output, key), _ => {
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        process::exit(1);
    } else {
        println!("operation succes: {} -> {}", input, output);
    }
}