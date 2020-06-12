use crate::handshake::HandshakeType::ClientHello;
use crate::client_hello;
use crate::handshake_message;

pub struct Handshake {
    header: HandshakeHeader,
    pub message: client_hello::ClientHello,
}

impl Handshake {
    pub fn create(header_type: HandshakeType, body: Vec<u8>) -> Handshake {
        let header = HandshakeHeader {
            header_type,
            length: [(body.len() / 256 * 256) as u8, (body.len() / 256) as u8, body.len() as u8],
        };

        let handshake_body: client_hello::ClientHello; //todo: this should be trait HandshakeMessage
        //only ClientHello implemented
        match header.header_type {
            HandshakeType::ClientHello =>
                handshake_body = client_hello::ClientHello::create(body)
        }

        Handshake {
            header,
            message: handshake_body,
        }
    }

    pub fn header_to_message(&self) -> [u8; 4] {
        let mut data: [u8; 4] = [0; 4];
        data[0] = self.header.header_type.code();
        data[1] = self.header.length[0]; //todo:refactor to slice array(?)
        data[2] = self.header.length[1];
        data[3] = self.header.length[2];

        println!("sent handshake header data: {:?}", &data);
        data
    }

    pub fn full_len(&self) -> u16 {
        4 + self.message.data.len() as u16
    }
}

struct HandshakeHeader {
    header_type: HandshakeType,
    length: [u8; 3],
}

pub enum HandshakeType {
    ClientHello,
    //ServerHello,
    //...
}

impl HandshakeType {
    fn code(&self) -> u8 {
        match *self {
            HandshakeType::ClientHello => 1,
            //HandshakeType::ServerHello => 2,
        }
    }
}