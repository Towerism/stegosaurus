use super::chunker::Chunker;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EncodeResult {
    Encoded(u8),
    NotEncoded(u8)
}

pub struct Encoder {
    chunker: Chunker
}

impl Encoder {
    pub fn new(data: Vec<u8>) -> Encoder {
        Encoder {
            chunker: Chunker::new(data)
        }
    }

    pub fn encode_next(&mut self, data: u8) -> EncodeResult {
        match self.chunker.next() {
            Some(bit) => EncodeResult::Encoded(data & 0b1111_1110 | bit),
            None => EncodeResult::NotEncoded(data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding_zero_bit_clears_lsb() {
        let data = vec![0b0000_0000];
        let mut encoder = Encoder::new(data);

        let encoded = encoder.encode_next(0b1111_1111);

        assert_eq!(EncodeResult::Encoded(0b1111_1110), encoded);
    }

    #[test]
    fn encoding_one_bit_sets_lsb() {
        let data = vec![0b0000_0001];
        let mut encoder = Encoder::new(data);

        let encoded = encoder.encode_next(0b1111_1110);

        assert_eq!(EncodeResult::Encoded(0b1111_1111), encoded);
    }
}
