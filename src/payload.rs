use std::error::Error;

pub struct Payload {
    bytes: Vec<u8>
}

impl Payload {
    fn calculate_size(bytes: &Vec<u8>) -> usize {
        return DataHeader::size() + bytes.len();
    }

    pub fn new(mut data: Vec<u8>) -> Result<Payload, Box<dyn Error>> {
        let mut header = DataHeader {
            byte_count: 0
        };
        header.byte_count = Payload::calculate_size(&data);

        let mut header_data = bincode::serialize(&header)?;
        data.append(&mut header_data);

        Ok(Payload {
            bytes: data
        })
    }

    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataHeader {
    pub byte_count: usize
}

impl DataHeader {
    fn new() -> DataHeader {
        DataHeader {
            byte_count: 0
        }
    }

    fn size() -> usize {
        let header = DataHeader::new();
        bincode::serialized_size(&header).unwrap() as usize
    }
}
