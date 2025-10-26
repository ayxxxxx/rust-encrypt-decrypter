use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

fn rotate_left(byte: u8, shift: u8) -> u8 {
    byte.rotate_left((shift % 8) as u32)
}

fn rotate_right(byte: u8, shift: u8) -> u8 {
    byte.rotate_right((shift % 8) as u32)
}

fn xor_encrypt_decrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| {
            let xored = byte ^ key;
            if i % 2 == 0 {
                rotate_left(xored, key % 8)
            } else {
                rotate_right(xored, (key + i as u8) % 8)
            }
        })
        .collect()
}

fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| {
            let rotated = if i % 2 == 0 {
                rotate_right(byte, key % 8)
            } else {
                rotate_left(byte, (key + i as u8) % 8)
            };
            rotated ^ key
        })
        .collect()
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
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let decrypted_data = xor_decrypt(&buffer, key);

    let mut output_file = File::create(output_path)?;
    output_file.write_all(&decrypted_data)?;

    Ok(())
}

fn print_usage() {
    eprintln!("Usage: locker (encrypt|decrypt) <input_file> <output_file> <key>\n");
    eprintln!("E: locker encrypt secret.txt secret.lock 42\n");
    eprintln!("Use --help to get help.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--help" {
        print_usage();
        return;
    }

    if args.len() != 5 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let input = &args[2];
    let output = &args[3];

    let key: u8 = match args[4].parse() {
        Ok(k) if k > 0 && k < 255 => k,
        _ => {
            eprintln!("E: invalid key, must be number between 1 and 255");
            process::exit(1);
        }
    };

    let result = match command.as_str() {
        "encrypt" => encrypt_file(input, output, key),
        "decrypt" => decrypt_file(input, output, key),
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("E: {}", e);
        process::exit(1);
    } else {
        println!("success: {} -> {}", input, output);
    }
}
