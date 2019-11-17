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

            data.copy_from_slice(&content[12..]);
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
