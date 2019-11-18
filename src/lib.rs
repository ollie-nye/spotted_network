pub mod header;

pub mod spotted_control;
pub mod spotted_coordinate;
pub mod spotted_image;

use header::opcode;
use header::spotted_header::SpottedHeader;

use spotted_control::SpottedControl;
use spotted_coordinate::SpottedCoordinate;
use spotted_image::SpottedImage;

pub struct SpottedPacket {
    header: header::spotted_header::SpottedHeader,
    data: [u8; 65507 - 12], // UDP max size - header bytes
}

impl SpottedPacket {
    pub fn deserialize(content: &[u8]) -> SpottedPacket {
        let id = &content[0..8];

        let mut header = SpottedHeader::new(0x0000);

        let mut data = [0x00; 65507 - 12];

        if id == header::spotted_header::id() {
            let opcode: u16 = ((content[8] as u16) << 8) | content[9] as u16;

            header = SpottedHeader::new(opcode);

            let trim_content = &content[12..];

            let mut i = 0;
            for cell in trim_content {
                data[i] = *cell;
                i += 1;
            }
        }

        SpottedPacket { header, data }
    }

    pub fn is_spotted_packet(&self) -> bool {
        self.header.opcode() != 0x0000
    }

    pub fn is_control(&self) -> bool {
        self.header.opcode() == opcode::OP_CONTROL
    }

    pub fn is_image(&self) -> bool {
        self.header.opcode() == opcode::OP_IMAGE
    }

    pub fn is_coordinate(&self) -> bool {
        self.header.opcode() == opcode::OP_COORDINATE
    }

    pub fn into_control(self) -> SpottedControl {
        SpottedControl::deserialize(&self.data)
    }

    pub fn into_image(self) -> SpottedImage {
        SpottedImage::deserialize(&self.data)
    }

    pub fn into_coordinate(self) -> SpottedCoordinate {
        SpottedCoordinate::deserialize(&self.data)
    }
}

#[cfg(test)]
#[test]
fn test_new_is_spotted_packet() {
    let header = header::spotted_header::SpottedHeader::new(0x0000);
    let bytes = header.serialize();
    let packet = SpottedPacket::deserialize(&bytes);

    assert_eq!(packet.is_spotted_packet(), false);
}

#[test]
fn test_new_is_control() {
    let header = header::spotted_header::SpottedHeader::new(0x0002);
    let bytes = header.serialize();
    let packet = SpottedPacket::deserialize(&bytes);

    assert_eq!(packet.is_control(), true);
}

#[test]
fn test_new_is_image() {
    let header = header::spotted_header::SpottedHeader::new(0x0001);
    let bytes = header.serialize();
    let packet = SpottedPacket::deserialize(&bytes);

    assert_eq!(packet.is_image(), true);
}

#[test]
fn test_new_is_coordinate() {
    let header = header::spotted_header::SpottedHeader::new(0x0003);
    let bytes = header.serialize();
    let packet = SpottedPacket::deserialize(&bytes);

    assert_eq!(packet.is_coordinate(), true);
}
