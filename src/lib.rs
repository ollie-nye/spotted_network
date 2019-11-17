pub mod header;

pub mod spotted_image;
pub mod spotted_control;
pub mod spotted_coordinate;

use spotted_image::SpottedImage;
use spotted_control::SpottedControl;
use spotted_coordinate::SpottedCoordinate;

pub enum SpottedPacket {
  SpottedImage(SpottedImage),
  SpottedControl(SpottedControl),
  SpottedCoordinate(SpottedCoordinate),
  None
}

pub fn deserialize(data: &[u8]) -> SpottedPacket {
  let id = &data[0..8];
  if id == header::spotted_header::id() {
    let opcode: u16 = ((data[8] as u16) << 8) | data[9] as u16;
    let version: u16 = ((data[10] as u16) << 8) | data[11] as u16;

    let header = header::spotted_header::SpottedHeader::new(opcode);

    let content = &data[12..];

    match opcode {
      header::opcode::OP_SPOT_CONTROL => return SpottedPacket::SpottedControl(SpottedControl::deserialize(content)),
      header::opcode::OP_SPOT_COORDINATE => return SpottedPacket::SpottedCoordinate(SpottedCoordinate::deserialize(content)),
      header::opcode::OP_SPOT_IMAGE => return SpottedPacket::SpottedImage(SpottedImage::deserialize(content)),
      _ => return SpottedPacket::None
    }
  } else {
    return SpottedPacket::None;
  }
}
