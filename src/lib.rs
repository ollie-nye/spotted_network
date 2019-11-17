pub mod header;

pub mod spotted_image;
pub mod spotted_control;
pub mod spotted_coordinate;

use spotted_image::SpottedImage;
use spotted_control::SpottedControl;
use spotted_coordinate::SpottedCoordinate;

#[derive(Clone)]
pub struct SpottedPacket {
  header: header::spotted_header::SpottedHeader,
  data: [u8; 65,507 - 12] // UDP max size - header bytes
}

impl SpottedPacket {
  pub fn deserialize(content: &[u8]) -> SpottedPacket {
    let id = &content[0..8];

    let mut header = header::spotted_header::SpottedHeader::new(0x0000);

    let mut data = [0x00; 65,507 - 12];

    if id == header::spotted_header::id() {
      let opcode: u16 = ((content[8] as u16) << 8) | content[9] as u16;
      let version: u16 = ((content[10] as u16) << 8) | content[11] as u16;

      header = header::spotted_header::SpottedHeader::new(opcode);

      data.copy_from_slice(&content[12..]);
    }

    return SpottedPacket { header, data};
  }

  pub fn is_spotted_packet(&self) -> bool {
    if self.header.opcode() != 0x0000 {
      true
    } else {
      false
    }
  }

  pub fn is_control(&self) -> bool {
    if self.header.opcode() == header::opcode::OP_CONTROL {
      true
    } else {
      false
    }
  }

  pub fn is_image(&self) -> bool {
    if self.header.opcode() == header::opcode::OP_IMAGE {
      true
    } else {
      false
    }
  }

  pub fn is_coordinate(&self) -> bool {
    if self.header.opcode() == header::opcode::OP_COORDINATE {
      true
    } else {
      false
    }
  }

  pub fn into_control(&self) -> SpottedControl {
    SpottedControl::deserialize(self.data)
  }

  pub fn into_image(&self) -> SpottedImage {
    SpottedImage::deserialize(self.data)
  }

  pub fn into_coordinate(&self) -> SpottedCoordinate {
    SpottedCoordinate::deserialize(self.data)
  }
}
