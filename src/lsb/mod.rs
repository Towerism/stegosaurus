mod bits_view;
use self::bits_view::ViewBits;

#[derive(Debug, PartialEq)]
pub enum EncodeResult {
    Encoded(u8),
    NotEncoded(u8),
}

pub struct Encoder {
    bits: bits_view::BitsView
}

impl Encoder {
    pub fn new(data: Vec<u8>) -> Encoder {
        Encoder {
            bits: data.view_bits(),
        }
    }

    pub fn encode_using_bit_at(&self, data: u8, index: usize) -> EncodeResult {
        match self.bits.get_bit(index) {
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
        let encoder = Encoder::new(data);

        let encoded = encoder.encode_using_bit_at(0b1010_0011, 0);

        assert_eq!(EncodeResult::Encoded(0b1010_0010), encoded);
    }

    #[test]
    fn encoding_one_bit_sets_lsb() {
        let data = vec![0b0000_0001];
        let encoder = Encoder::new(data);

        let encoded = encoder.encode_using_bit_at(0b0000_0000, 0);

        assert_eq!(EncodeResult::Encoded(0b0000_0001), encoded);
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
        let data = vec![0x51, 0x41, 0xa0, 0x0, 0xc1, 0xa1, 0xc0, 0x31];
        let decoder = Decoder::new();
        let byte = decoder.decode_next(&data);

        assert_eq!(0b1011_0011, byte);
    }
}
