#[derive(Debug,PartialEq)]
pub enum RleComponent {
    Run(usize, u8),
    Literal(Vec<u8>)
}

struct Encoder {
    working_values: Vec<u8>,
    components: Vec<RleComponent>,
}

impl Encoder {
    fn new() -> Encoder {
        Encoder { working_values: vec![], components: vec![] }
    }

    fn commit(&mut self, component: RleComponent) {
        self.components.push(component);
        self.working_values = vec![];
    }
}

pub fn encode(input: &str) -> Vec<RleComponent> {
    let mut encoder = Encoder::new();
    let mut input_iter = input.bytes().peekable();
    let mut last: u8 = input_iter.nth(0).unwrap();
    let mut counter: usize = 1;

    for bv in input_iter {
        if bv == last {
            counter += 1;
        } else {
            if counter == 1 {
                // TODO: Combine adjacent literals.
                encoder.commit(RleComponent::Literal(vec![last]));
            } else {
                encoder.commit(RleComponent::Run(counter, last));
            }

            counter = 1;
            last = bv;
        }
    }
    // TODO: This repeated code isn't ideal.
    if counter == 1 {
        encoder.commit(RleComponent::Literal(vec![last]));
    } else {
        encoder.commit(RleComponent::Run(counter, last));
    }

    encoder.components
}

pub fn decode(input: Vec<RleComponent>) -> String {
    let mut decoded = vec!();

    for component in input {
        let next_sequence = match component {
            RleComponent::Run(length, byte) => vec![byte;length],
            RleComponent::Literal(bytes) => bytes,
        };
        decoded.extend(next_sequence);
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
    #[ignore]
    fn empty_encode() {
        assert_eq!(
            encode(""),
            []
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
