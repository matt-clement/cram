use std::io;
use std::io::Write;
use std::io::stdout;

#[derive(Debug,PartialEq)]
enum RleComponent {
    Run(usize, u8),
    Literal(Vec<u8>)
}

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

fn rle(input: &str) -> Vec<RleComponent> {
    let mut last : u8 = input.bytes().nth(0).unwrap();
    let mut counter : usize = 0;
    let mut encoded = vec!();

    for bv in input.bytes() {
        if bv == last {
            counter += 1;
        } else {
            if counter == 1 {
                // TODO: Combine adjacent literals.
                encoded.push(RleComponent::Literal(vec![last]));
            } else {
                encoded.push(RleComponent::Run(counter, last));
            }

            counter = 1;
            last = bv;
        }
    }
    // TODO: This repeated code isn't ideal.
    if counter == 1 {
        encoded.push(RleComponent::Literal(vec![last]));
    } else {
        encoded.push(RleComponent::Run(counter, last));
    }


    encoded//.to_vec()
}

fn rld(input: Vec<RleComponent>) -> String {
    let mut decoded = vec!();

    for component in input {
        match component {
            RleComponent::Run(length, byte) => {
                // TODO: Is there a better way to add
                // repeated values to the end of a vec?
                for _ in 0..length {
                    decoded.push(byte);
                }
            },
            RleComponent::Literal(bytes) => {
                decoded.extend(bytes);
            }
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
            [
                RleComponent::Literal(vec![104]),
                RleComponent::Literal(vec![101]),
                RleComponent::Run(2, 108),
                RleComponent::Literal(vec![111])
            ]
        );
    }

    #[test]
    fn with_trailing_run() {
        assert_eq!(
            rle("helloo"),
            [
                RleComponent::Literal(vec![104]),
                RleComponent::Literal(vec![101]),
                RleComponent::Run(2, 108),
                RleComponent::Run(2, 111),
            ]
        );
    }

    #[test]
    fn moretests() {
        assert_eq!(
            rle("aaaaaab"),
            [
                RleComponent::Run(6, 97),
                RleComponent::Literal(vec![98]),
            ]
        );
    }


    #[test]
    fn basic_rld() {
        assert_eq!(
            rld(rle("aabb")),
            "aabb"
        );
    }

    #[test]
    fn basic_rld_only_literal() {
        assert_eq!(
            rld(rle("ab")),
            "ab"
        );
    }
}
