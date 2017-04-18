use std::io;
use std::io::Write;
use std::io::stdout;

fn main() {
    let mut input = String::new();

    print!("Enter text to encode: ");
    stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let compressed = rle(&input.trim());
    println!("{:?}", compressed);
    let decompressed = rld(compressed);
    println!("{:?}", decompressed);
}

fn rle(input: &str) -> Vec<(usize, u8)> {
    // The funny thing about this function, is that it will probably have a
    // negative compression ratio. But it's a decent starting point.
    let mut last : u8 = input.bytes().nth(0).unwrap();
    let mut counter : usize = 0;
    let mut encoded = vec!();


    for bv in input.bytes() {
        if bv == last {
            counter += 1;
        } else {
            encoded.push((counter, last));
            counter = 1;
            last = bv;
        }
    }
    encoded.push((counter, last));


    encoded.to_vec()
}

fn rld(input: Vec<(usize, u8)>) -> String {
    let mut decoded = vec!();
    for &(length, byte) in input.iter() {
        for _ in 0..length {
            decoded.push(byte);
        }
    }
    String::from_utf8(decoded).unwrap()
}

mod tests {
    #[allow(unused_imports)] // Shouldn't warn when we're not running tests
    use super::*;
    #[test]
    fn basic_rle() {
        assert_eq!(
            rle("hello"),
            [(1, 104), (1, 101), (2, 108), (1, 111)]
        );
    }

    #[test]
    fn with_trailing_run() {
        assert_eq!(
            rle("helloo"),
            [(1, 104), (1, 101), (2, 108), (2, 111)]
        );
    }

    #[test]
    fn moretests() {
        assert_eq!(
            rle("aaaaaab"),
            [(6, 97), (1, 98)]
        );
    }

    #[test]
    fn basic_rld() {
        assert_eq!(
            rld(rle("aabb")),
            "aabb"
        );
    }
}
