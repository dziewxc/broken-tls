use crate::handshake_message::HandshakeMessage;
use crate::client_hello::ClientHello;
use crate::handshake::Handshake;

pub struct Record {
    record_header: RecordHeader,
    pub protocol_data: Handshake,
}

pub enum RecordType {
    Handshake,
    //Alert,
    //ChangeCipherSpec,
    //ApplicationData
}

impl Record {
    pub fn create(r_type: RecordType, data: Handshake) -> Record {
        let header = RecordHeader {
            record_type: r_type,
            version: [3, 1],
            length: [(data.full_len() / 256) as u8, data.full_len() as u8],
        };

        Record {
            record_header: header,
            protocol_data: data,
        }
    }

    pub fn header_to_message(&self) -> [u8; 5] {
        let mut data: [u8; 5] = [0; 5];
        data[0] = self.record_header.record_type.code();
        data[1] = self.record_header.version[0]; //todo: slice array
        data[2] = self.record_header.version[1];
        data[3] = self.record_header.length[0];
        data[4] = self.record_header.length[1];
        //data[1..2] = [&self.record_header.version[0], &self.record_header.version[1];
        //data[3..4] = self.record_header.length[&0 as usize..&1 as usize];
        println!("sent record header data: {:?}", &data);
        data
    }
}

struct RecordHeader {
    record_type: RecordType,
    version: [u8; 2],
    length: [u8; 2],
}

impl RecordType {
    fn code(&self) -> u8 {
        match &self {
            RecordType::Handshake => 22, //decimal
            //RecordType::Alert => 1, //todo
            //RecordType::ChangeCipherSpec => 1, //todo
            //RecordType::ApplicationData => 1, //todo
        }
    }
}
