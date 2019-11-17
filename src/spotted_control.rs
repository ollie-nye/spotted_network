use super::header::opcode;
use super::header::spotted_header::SpottedHeader;

pub struct SpottedControl {
  header: SpottedHeader,
  key: u8,
  value: u8
}

impl SpottedControl {
  pub fn new(key:u8, value: u8) -> SpottedControl {
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
