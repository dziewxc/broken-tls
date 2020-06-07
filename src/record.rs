pub struct Record {
    record_header: RecordHeader,
    protocol_data: Box<Vec<u8>>
}

pub enum RecordType {
    Handshake,
    Alert,
    ChangeCipherSpec,
    ApplicationData
}

impl Record {
    pub fn create(r_type: RecordType, data: Box<Vec<u8>>) -> Record {
        let header = RecordHeader {
            record_type: r_type,
            version: vec![3, 1],
            length: vec![0, data.len() as u8] //todo?
        };

        Record {
            record_header: header,
            protocol_data: data
        }
    }
}

struct RecordHeader {
    record_type: RecordType,
    version: Vec<u8>, //todo: check how to reserve only 2 bytes
    length: Vec<u8>
}

impl RecordType {
    fn code(&self) -> u8 {
        match &self {
            RecordType::Handshake => 22, //decimal
            RecordType::Alert => 1, //todo
            RecordType::ChangeCipherSpec => 1, //todo
            RecordType::ApplicationData => 1, //todo
        }
    }
}
