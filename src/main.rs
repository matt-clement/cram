use std::io;
use std::io::Write;
use std::io::stdout;

mod run_length_encoding;

fn main() {
    let mut input = String::new();

    print!("Enter text to encode: ");
    stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let compressed = run_length_encoding::encode(&input.trim());
    println!("{:?}", compressed);
    let decompressed = run_length_encoding::decode(compressed);
    println!("{:?}", decompressed);
}
