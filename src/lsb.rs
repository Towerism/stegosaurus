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

pub struct Decoder;

impl Decoder {
    pub fn new() -> Decoder {
        Decoder
    }

    pub fn decode_next(&self, chunk: &[u8]) -> u8 {
        assert_eq!(8, chunk.len());

        let mut result = 0u8;
        for i in 0..8 {
            result = result | ((chunk[i] & 0x1) << i);
        }
        result
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

    #[test]
    fn decoding_sets_one_lsb() {
        let data = vec![0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let decoder = Decoder::new();
        let byte = decoder.decode_next(&data);

        assert_eq!(0b0000_0001, byte);
    }

    #[test]
    fn decoding_combines_8_lsb_into_one_byte() {
        let data = vec![0x51, 0x40, 0xa1, 0x0, 0xc1, 0xa0, 0xc1, 0x31];
        let decoder = Decoder::new();
        let byte = decoder.decode_next(&data);

        assert_eq!(0b1101_0101, byte);
    }
}
