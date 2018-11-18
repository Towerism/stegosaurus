/// An iterable object that can chunk its
/// data into n bit items.
pub struct Chunker {
    data: Vec<u8>,
    chunk_masker: ChunkMasker,
    position: usize
}

// I think maybe reference counting for chunk is more appropriate???
impl Chunker {
    pub fn new(data: Vec<u8>) -> Chunker {
        let chunk_masker = ChunkMasker::new(&data);
        let chunker = Chunker {
            data,
            chunk_masker,
            position: 0
        };
        return chunker;
    }
}

impl Iterator for Chunker {
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
struct ChunkMasker {
    bits_remaining: u8,
    mask: u8
}

impl ChunkMasker {
    fn new(data: &Vec<u8>) -> ChunkMasker {
        ChunkMasker {
            bits_remaining: if data.is_empty() {
                0
            } else {
                8
            },
            mask: 0b0000_0001
        }
    }
}

impl Iterator for ChunkMasker {
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
            let mut chunker = Chunker::new(Vec::new());

            assert_eq!(None, chunker.next());
        }

        #[test]
        fn next_returns_the_next_bit() {
            let mut chunker = Chunker::new(vec![0b0000_0011]);

            assert_eq!(Some(0x01), chunker.next());
        }

        #[test]
        fn next_returns_the_next_bit_for_whole_byte() {
            let mut chunker = Chunker::new(vec![0b1001_1011]);
            let expected = vec![
                0x1, 0x1, 0x0, 0x1, 0x1, 0x0, 0x0, 0x1];

            for (i, bit) in chunker.enumerate() {
                assert_eq!(expected[i], bit);
            }
        }

        #[test]
        fn next_returns_the_next_bit_for_all_bytes() {
            let mut chunker = Chunker::new(vec![0b1001_1011, 0b0010_0011]);
            let expected = vec![
                0x1, 0x1, 0x0, 0x1, 0x1, 0x0, 0x0, 0x1,
                0x1, 0x1, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0];

            for (i, bit) in chunker.enumerate() {
                assert_eq!(expected[i], bit);
            }
        }
    }
}
