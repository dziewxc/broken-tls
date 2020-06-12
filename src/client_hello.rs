use crate::handshake_message::HandshakeMessage;

pub struct ClientHello {
    pub data: Vec<u8>
}

impl HandshakeMessage for ClientHello {}

impl ClientHello {
    pub fn create(data: Vec<u8>) -> ClientHello {
        ClientHello {
            data
        }
    }
}