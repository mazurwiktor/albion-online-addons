pub enum MessageCode {
    Request = 0x02,
    Response = 0x03,
    Event = 0x04,
    Unknown
}

impl From<u8> for MessageCode {
    fn from(v: u8) -> Self {
        match v {
            0x02 => MessageCode::Request,
            0x03 => MessageCode::Response,
            0x04 => MessageCode::Event,
            _ => MessageCode::Unknown,
        }
    }
}

pub enum CommandCode {
    LogOut = 0x04,
    SendUnreliable = 0x07,
    SendReliableFragment = 0x08,
    Unknown
}

impl From<u8> for CommandCode {
    fn from(v: u8) -> Self {
        match v {
            0x04 => CommandCode::LogOut,
            0x07 => CommandCode::SendUnreliable,
            0x08 => CommandCode::SendReliableFragment,
            _ => CommandCode::Unknown,
        }
    }
}

pub enum TypeCode {
    None = 0x00,
    Null = 0x2A,
    Dictionary = 0x44,
    StringArray = 0x61,
    Byte = 0x62,
    Double = 0x64,
    EventData = 0x65,
    Float = 0x66,
    Integer = 0x69,
    Short = 0x6B,
    Long = 0x6C,
    BooleanArray = 0x6E,
    Boolean = 0x6F,
    OperationResponse = 0x70,
    OperationRequest = 0x71,
    String = 0x73,
    ByteArray = 0x78,
    Array = 0x79,
    ObjectArray = 0x7A,
    Unknown,
}

impl From<u8> for TypeCode {
    fn from(v: u8) -> Self {
        match v {
            0x00 => TypeCode::None,
            0x2A => TypeCode::Null,
            0x44 => TypeCode::Dictionary,
            0x61 => TypeCode::StringArray,
            0x62 => TypeCode::Byte,
            0x64 => TypeCode::Double,
            0x65 => TypeCode::EventData,
            0x66 => TypeCode::Float,
            0x69 => TypeCode::Integer,
            0x6B => TypeCode::Short,
            0x6C => TypeCode::Long,
            0x6E => TypeCode::BooleanArray,
            0x6F => TypeCode::Boolean,
            0x70 => TypeCode::OperationResponse,
            0x71 => TypeCode::OperationRequest,
            0x73 => TypeCode::String,
            0x78 => TypeCode::ByteArray,
            0x79 => TypeCode::Array,
            0x7A => TypeCode::ObjectArray,
            _ => TypeCode::Unknown,
        }
    }
}