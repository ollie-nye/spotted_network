pub struct SpottedHeader {
  id: [u8; 8],
  opcode: u16,
  protocol_version: u16
}

impl SpottedHeader {
  pub fn new(opcode: u16) -> SpottedHeader {

    let mut id: [u8; 8] = id();

    let protocol_version: u16 = 0x0001;

    let header = SpottedHeader { id, opcode, protocol_version };

    header
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend(self.id.iter().cloned());
    out.extend(self.opcode.to_be_bytes().iter().cloned());
    out.extend(self.protocol_version.to_be_bytes().iter().cloned());

    out
  }

  pub fn opcode(&self) -> u16 {
    self.opcode
  }
}

pub fn id() -> [u8; 8] {
  let mut id: [u8; 8] = [0x00; 8];

  id.copy_from_slice("Spotted\0".as_bytes());

  id
}
