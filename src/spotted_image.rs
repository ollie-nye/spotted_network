use super::header;
use super::header::opcode;
use super::header::spotted_header::SpottedHeader;

pub struct SpottedImage {
    header: SpottedHeader,
    data: [u8; header::IMAGE_SIZE],
}

impl SpottedImage {
    pub fn new(data: [u8; header::IMAGE_SIZE]) -> SpottedImage {
        let header = SpottedHeader::new(opcode::OP_IMAGE);

        SpottedImage { header, data }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.extend(self.data.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> SpottedImage {
        let mut image: [u8; header::IMAGE_SIZE] = [0x00; header::IMAGE_SIZE];

        image.copy_from_slice(data);

        SpottedImage::new(image)
    }
}

#[cfg(test)]
#[test]
fn test_new_opcode() {
    let packet = SpottedImage::new([0x00; header::IMAGE_SIZE]);
    assert_eq!(packet.header.opcode(), 0x0001);
}
