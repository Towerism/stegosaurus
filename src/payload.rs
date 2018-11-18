use std::error::Error;
use std::fmt;

pub struct Payload {
    bytes: Vec<u8>
}

impl Payload {
    fn calculate_size(bytes: &Vec<u8>) -> usize {
        return DataHeader::size() + bytes.len();
    }

    pub fn new(mut data: Vec<u8>) -> Result<Payload, Box<dyn Error>> {
        //let mut header = DataHeader::new();
        //header.byte_count = Payload::calculate_size(&data);

        //let mut header_data = bincode::serialize(&header)?;
        //header_data.append(&mut data);

        Ok(Payload {
            bytes: data
        })
    }

    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Payload {
        Payload {
            bytes
        }
    }

    pub fn data(&self) -> Result<&[u8], Box<dyn Error>> {
        let (_, data) = DataHeader::extract_from(&self.bytes)?;
        Ok(data)
    }
}

static MAGIC_CONSTANT: u64 = 0x0;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataHeader {
    magic_constant: u64,
    pub byte_count: usize
}

impl DataHeader {
    fn new() -> DataHeader {
        DataHeader {
            magic_constant: MAGIC_CONSTANT,
            byte_count: 0
        }
    }

    fn size() -> usize {
        let header = DataHeader::new();
        bincode::serialized_size(&header).unwrap() as usize
    }

    fn extract_from(bytes: &[u8]) -> Result<(DataHeader, &[u8]), Box<dyn Error>> {
        let header_size = DataHeader::size();
        let reader = bincode::SliceReader::new(&bytes[0..header_size]);
        let header: DataHeader = bincode::deserialize_from(reader)?;
        if header.magic_constant != MAGIC_CONSTANT {
            return Err(Box::new(DataHeaderError::new(&format!("magic constant was {}, expected {}", header.magic_constant, MAGIC_CONSTANT))));
        }
        let data_len = header.byte_count;
        Ok((header, &bytes[header_size..data_len]))
    }
}

#[derive(Debug)]
struct DataHeaderError {
    message: String
}
impl DataHeaderError {
    fn new(message: &str) -> DataHeaderError {
        DataHeaderError {
            message: String::from(message)
        }
    }
}
impl Error for DataHeaderError {}

impl fmt::Display for DataHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "a valid payload was not found ({})", self.message)
    }
}
