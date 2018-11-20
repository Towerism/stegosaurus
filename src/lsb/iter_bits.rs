pub trait BitIterable {
    fn into_iter_bits(self) -> BitIterator;
}

impl BitIterable for Vec<u8> {
    fn into_iter_bits(self) -> BitIterator {
        BitIterator::new(self)
    }
}

/// An iterable object that can chunk its
/// data into n bit items.
pub struct BitIterator {
    data: Vec<u8>,
    chunk_masker: MaskGenerator,
    position: usize
}

impl BitIterator {
    pub fn new(data: Vec<u8>) -> BitIterator {
        let chunk_masker = MaskGenerator::new(&data);
        let chunker = BitIterator {
            data,
            chunk_masker,
            position: 0
        };
        return chunker;
    }
}

impl Iterator for BitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.data.len() {
            return None;
        }
        if let Some(mask) = self.chunk_masker.next() {
            let masked_byte = self.data[self.position] & mask;
            if masked_byte == 0 {
                Some(0)
            } else {
                Some(1)
            }
        } else {
            self.position += 1;
            self.next()
        }
    }
}

/// Generates masks for each bit in a chunk
struct MaskGenerator {
    bits_remaining: u8,
    mask: u8
}

impl MaskGenerator {
    fn new(data: &Vec<u8>) -> MaskGenerator {
        MaskGenerator {
            bits_remaining: if data.is_empty() {
                0
            } else {
                8
            },
            mask: 0b0000_0001
        }
    }
}

impl Iterator for MaskGenerator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits_remaining == 0 {
            self.bits_remaining = 8;
            return None;
        }
        let shift = 8 - self.bits_remaining;
        self.bits_remaining -= 1;
        Some(self.mask << shift)
    }
}

#[cfg(test)]
mod tests {
    mod one_bit_at_a_time {
        use super::super::*;

        #[test]
        fn next_returns_none_for_empty_data() {
            let mut chunker = BitIterator::new(Vec::new());

            assert_eq!(None, chunker.next());
        }

        #[test]
        fn next_returns_the_next_bit() {
            let mut chunker = BitIterator::new(vec![0b0000_0011]);

            assert_eq!(Some(0x01), chunker.next());
        }

        #[test]
        fn next_returns_the_next_bit_for_whole_byte() {
            let chunker = BitIterator::new(vec![0b1001_1011]);
            let expected = vec![
                0x1, 0x1, 0x0, 0x1, 0x1, 0x0, 0x0, 0x1];

            for (i, bit) in chunker.enumerate() {
                assert_eq!(expected[i], bit);
            }
        }

        #[test]
        fn next_returns_the_next_bit_for_all_bytes() {
            let chunker = BitIterator::new(vec![0b1001_1011, 0b0010_0011, 0b0000_0000, 0b1111_1111, 0b1010_1010, 0b1111_0000]);
            let expected = vec![
                1, 1, 0, 1, 1, 0, 0, 1,
                1, 1, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 1, 1, 1, 1, 1, 1,
                0, 1, 0, 1, 0, 1, 0, 1,
                0, 0, 0, 0, 1, 1, 1, 1
            ];

            for (i, bit) in chunker.enumerate() {
                assert_eq!(expected[i], bit);
            }
        }
    }
}
