use super::header::opcode;
use super::header::spotted_header::SpottedHeader;

pub struct SpottedControl {
    header: SpottedHeader,
    key: u8,
    value: u8,
}

impl SpottedControl {
    pub fn new(key: u8, value: u8) -> SpottedControl {
        let header = SpottedHeader::new(opcode::OP_CONTROL);

        SpottedControl { header, key, value }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.push(self.key);
        out.push(self.value);

        out
    }

    pub fn deserialize(data: &[u8]) -> SpottedControl {
        let key = data[0];
        let value = data[1];

        SpottedControl::new(key, value)
    }
}

#[cfg(test)]
#[test]
fn test_new_opcode() {
    let packet = SpottedControl::new(0x11, 0x22);
    assert_eq!(packet.header.opcode(), 0x0002);
}

#[test]
fn test_new_key() {
    let packet = SpottedControl::new(0x11, 0x22);
    assert_eq!(packet.key, 0x11);
}

#[test]
fn test_new_value() {
    let packet = SpottedControl::new(0x11, 0x22);
    assert_eq!(packet.value, 0x22);
}

#[test]
fn test_serialize_key() {
    let packet = SpottedControl::new(0x11, 0x22);
    let bytes = packet.serialize();

    assert_eq!(bytes[12], 0x11);
}

#[test]
fn test_serialize_value() {
    let packet = SpottedControl::new(0x11, 0x22);
    let bytes = packet.serialize();

    assert_eq!(bytes[13], 0x22);
}

#[test]
fn test_deserialize_key() {
    let data = [0x11, 0x22];
    let packet = SpottedControl::deserialize(&data);

    assert_eq!(packet.key, 0x11);
}

#[test]
fn test_deserialize_value() {
    let data = [0x11, 0x22];
    let packet = SpottedControl::deserialize(&data);

    assert_eq!(packet.value, 0x22);
}
