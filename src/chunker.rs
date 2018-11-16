/// An iterable object that can chunk its
/// data into n bit items.
struct Chunker {
    data: Vec<u8>
}

impl Chunker {
    fn new(data: Vec<u8>) -> Chunker {
        Chunker {
            data
        }
    }
}

impl Iterator for Chunker {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_returns_none_for_empty_data() {
        let mut chunker = Chunker::new(Vec::new());

        assert_eq!(None, chunker.next());
    }
}
