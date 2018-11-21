use std::iter::Iterator;

pub trait ViewBits {
    fn view_bits(self) -> BitsView;
}

impl ViewBits for Vec<u8> {
    fn view_bits(self) -> BitsView {
        BitsView::new(self)
    }
}

pub struct BitsView {
    data: Vec<u8>,
    position: usize,
    bit_count: usize
}

impl BitsView {
    pub fn new(data: Vec<u8>) -> BitsView {
        let bits = BitsView {
            bit_count: data.len() * 8,
            data,
            position: 0
        };
        return bits;
    }

    pub fn get_bit(&self, index: usize) -> Option<u8> {
        if index >= self.bit_count {
            return None;
        }
        let rem = index % 8;
        if (self.data[(index / 8)] & (0x1 << rem)) == 0 {
            Some(0)
        } else {
            Some(1)
        }
    }
}

impl Iterator for BitsView {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = self.get_bit(self.position);
        self.position += 1;
        bit
    }
}

#[cfg(test)]
mod tests {
    mod one_bit_at_a_time {
        use super::super::*;

        #[test]
        fn next_returns_none_for_empty_data() {
            let mut bits = BitsView::new(Vec::new());

            assert_eq!(None, bits.next());
        }

        #[test]
        fn next_returns_the_next_bit() {
            let mut bits = BitsView::new(vec![0b0000_0011]);

            assert_eq!(Some(0x01), bits.next());
        }

        #[test]
        fn next_returns_the_next_bit_for_whole_byte() {
            let bits = BitsView::new(vec![0b1001_1011]);
            let expected = vec![0x1, 0x1, 0x0, 0x1, 0x1, 0x0, 0x0, 0x1];

            for (i, bit) in bits.enumerate() {
                assert_eq!(expected[i], bit, "failed index {}", i);
            }
        }

        #[test]
        fn next_returns_the_next_bit_for_all_bytes() {
            let bits = BitsView::new(vec![
                0b1001_1011,
                0b0010_0011,
                0b0000_0000,
                0b1111_1111,
                0b1010_1010,
                0b1111_0000,
            ]);
            let expected = vec![
                1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
                1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1,
            ];

            for (i, bit) in bits.enumerate() {
                assert_eq!(expected[i], bit);
            }
        }
    }

    mod get_bit {
        use super::super::*;

        #[test]
        fn first_bit() {
            let bits = BitsView::new(vec![0b0000_0001]);
            assert_eq!(0x1, bits.get_bit(0).unwrap());
        }

        #[test]
        fn last_bit() {
            let bits = BitsView::new(vec![0b1000_0000]);
            assert_eq!(0x1, bits.get_bit(7).unwrap());
        }

        #[test]
        fn several_bytes() {
            let bits = BitsView::new(vec![0b0000_0000, 0b0000_0000, 0b0010_0000]);
            assert_eq!(0x1, bits.get_bit(21).unwrap());
        }

        #[test]
        fn no_data() {
            let bits = BitsView::new(Vec::new());
            assert_eq!(None, bits.get_bit(0));
        }
    }
}
