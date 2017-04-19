#[derive(Debug,PartialEq)]
pub enum RleComponent {
    Run(usize, u8),
    Literal(Vec<u8>)
}

pub fn encode(input: &str) -> Vec<RleComponent> {
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

pub fn decode(input: Vec<RleComponent>) -> String {
    let mut decoded = vec!();

    for component in input {
        let tv = match component {
            RleComponent::Run(length, byte) => vec![byte;length],
            RleComponent::Literal(bytes) => bytes,
        };
        decoded.extend(tv);
    }

    String::from_utf8(decoded).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_encode() {
        assert_eq!(
            encode("hello"),
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
            encode("helloo"),
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
            encode("aaaaaab"),
            [
                RleComponent::Run(6, 97),
                RleComponent::Literal(vec![98]),
            ]
        );
    }


    #[test]
    fn basic_decode() {
        assert_eq!(
            decode(encode("aabb")),
            "aabb"
        );
    }

    #[test]
    fn basic_decode_only_literal() {
        assert_eq!(
            decode(encode("ab")),
            "ab"
        );
    }
}
