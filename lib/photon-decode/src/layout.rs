//! Albion example packets layout
//! 
//! Single reliable command
//! | PhotonHeader |    ReliableCommand    | Message |
//! |--------------|-----------------------|---------|
//! |      12      | 11 + 1 (command type) |    ?    |
//! |--------------|-----------------------|---------|
//! 
//! Single unreliable command
//! | PhotonHeader |   UnreliableCommand   | Message |
//! |--------------|-----------------------|---------|
//! |      12      | 15 + 1 (command type) |    ?    |
//! |--------------|-----------------------|---------|
//! 

use std::collections::HashMap;

pub type Parameters = HashMap<u8, Value>;

/// PhotonHeader frame (12 bytes)
/// | peer_id | crc_enabled | command_count | timestamp | challenge |
/// |---------|-------------|---------------|-----------|-----------|
/// |    2    |      1      |       1       |     4     |      4    |
/// |---------|-------------|---------------|-----------|-----------|
#[derive(Debug)]
pub struct PhotonHeader {
    pub peer_id: i16,
    pub crc_enabled: bool,
    pub command_count: u8,
    pub timestamp: u32,
    pub challenge: u32,
}

/// ReliableCommand header frame (11 bytes)
/// | channel_id | flags | reserved_byte | msg_len | reliable_sequence_number |
/// |------------|-------|---------------|---------|--------------------------|
/// |    1       |   1   |       1       |    4    |            4             |
/// |------------|-------|---------------|---------|--------------------------|
#[derive(Debug, Clone)]
pub struct ReliableCommand {
    pub channel_id: u8,
    pub flags: u8,
    pub reserved_byte: u8,
    pub msg_len: u32,
    pub reliable_sequence_number: u32,
}

/// UnreliableCommand header frame (15 bytes)
/// | ReliableCommand | unknown |
/// |-----------------|---------|
/// |        11       |    4    |
/// |-----------------|---------|
pub struct UnreliableCommand {
    pub reliable_command: ReliableCommand,
    pub unknown: u32,
}

/// ReliableFragment header frame (31 bytes + payload)
/// | ReliableCommand | sequence_number | fragment_count | fragment_number | total_length | operation_length | payload |
/// |-----------------|-----------------|----------------|-----------------|--------------|------------------|---------|
/// |        11       |       4         |        4       |       4         |      4       |        4         |    ?    |
/// |-----------------|-----------------|----------------|-----------------|--------------|------------------|---------|
#[derive(Debug, Clone)]
pub struct ReliableFragment {
    pub reliable_command: ReliableCommand,
    pub sequence_number: u32,
    pub fragment_count: u32,
    pub fragment_number: u32,
    pub total_length: u32,
    pub operation_length: u32,
    pub payload: Vec<u8>,
}

/// EventData frame (1 byte + parameters)
/// | code | parameters |
/// |------|------------|
/// |   1  |      ?     |
/// |------|------------|
#[derive(Clone, Debug, PartialEq)]
pub struct EventData {
    pub code: u8,
    pub parameters: Parameters,
}

/// OperationResponse frame (3 bytes + debug_message + parameters)
/// | code | return_code | debug_message | parameters |
/// |------|-------------|---------------|------------|
/// |   1  |      2      |       ?       |     ?      |
/// |------|-------------|---------------|------------|
#[derive(Clone, Debug, PartialEq)]
pub struct OperationResponse {
    pub code: u8,
    pub return_code: i16,
    pub debug_message: String,
    pub parameters: Parameters,
}

/// OperationRequest frame (1 byte + parameters)
/// | code | parameters |
/// |------|------------|
/// |   1  |      ?     |
/// |------|------------|
#[derive(Clone, Debug, PartialEq)]
pub struct OperationRequest {
    pub code: u8,
    pub parameters: Parameters,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None,
    Dictionary(HashMap<String, Value>),
    StringArray(Vec<String>),
    Byte(u8),
    Double(f64),
    EventData(EventData),
    Float(f32),
    Integer(u32),
    Short(i16),
    Long(i64),
    BooleanArray(Vec<bool>),
    Boolean(bool),
    OperationResponse(OperationResponse),
    OperationRequest(OperationRequest),
    String(String),
    ByteArray(Vec<u8>),
    Array(Vec<Value>),
    Object(Box<Value>),
    ObjectArray(Vec<Box<Value>>),
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Request(OperationRequest),
    Response(OperationResponse),
    Event(EventData),
}

pub enum Command {
    LogOut,
    SendUnreliable(ReliableCommand),
    SendReliable(ReliableCommand),
    SendReliableFragment(ReliableFragment),
}
