#[derive(Debug)]
pub enum ProtocolCode {
    RegisterPublisher = 1,
    RegisterSubscriber,
    CreateBoxRequest,
    CreateBoxResponse,
    RemoveBoxRequest,
    RemoveBoxResponse,
    ListBoxesRequest,
    ListBoxesResponse,
    PublisherMessage,
    SubscriberMessage,
}

impl ProtocolCode {
    fn from_u8(value: u8) -> Result<ProtocolCode, String> {
        match value {
            1 => Ok(ProtocolCode::RegisterPublisher),
            2 => Ok(ProtocolCode::RegisterSubscriber),
            3 => Ok(ProtocolCode::CreateBoxRequest),
            4 => Ok(ProtocolCode::CreateBoxResponse),
            5 => Ok(ProtocolCode::RemoveBoxRequest),
            6 => Ok(ProtocolCode::RemoveBoxResponse),
            7 => Ok(ProtocolCode::ListBoxesRequest),
            8 => Ok(ProtocolCode::ListBoxesResponse),
            9 => Ok(ProtocolCode::PublisherMessage),
            10 => Ok(ProtocolCode::SubscriberMessage),
            _ => Err("Invalid protocol code".to_string()),
        }
    }
}

pub trait IsProtocol {
    fn from_bytes(bytes: Vec<u8>) -> Self;
    fn to_bytes(self) -> Vec<u8>;
}

struct RequestProtocol {
    code: ProtocolCode,
    client_named_pipe: String,
    box_name: String,
}

impl IsProtocol for RequestProtocol {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        let code = ProtocolCode::from_u8(bytes[0]).unwrap();
        let client_named_pipe = String::from(bytes[1].to_string().split('\0').nth(0).unwrap());
        let box_name = String::from(bytes[1].to_string().split('\0').nth(1).unwrap());

        Self {
            code,
            client_named_pipe,
            box_name,
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.code as u8);
        bytes.extend(self.client_named_pipe.as_bytes());
        bytes.extend(self.box_name.as_bytes());

        return bytes;
    }
}

struct ResponseProtocol {
    code: ProtocolCode,
    return_code: i32,
    error_msg: String,
}

impl IsProtocol for ResponseProtocol {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        let code = ProtocolCode::from_u8(bytes[0]).unwrap();
        let return_code = i32::from_be_bytes(bytes[1..5].try_into().unwrap());
        let error_msg = String::from(bytes[5].to_string().split('\0').nth(0).unwrap());

        Self {
            code,
            return_code,
            error_msg,
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.code as u8);
        bytes.extend(self.return_code.to_be_bytes().iter());
        bytes.extend(self.error_msg.as_bytes());

        return bytes;
    }
}

struct MessageProtocol {
    code: ProtocolCode,
    message: String,
}

impl IsProtocol for MessageProtocol {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        let code = ProtocolCode::from_u8(bytes[0]).unwrap();
        let message = String::from(bytes[1].to_string().split('\0').nth(0).unwrap());

        Self { code, message }
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.code as u8);
        bytes.extend(self.message.as_bytes());

        return bytes;
    }
}
