use super::header::opcode;
use super::header::spotted_header::SpottedHeader;

pub struct SpottedCoordinate {
  header: SpottedHeader,
  camera: u8,
  hotspot_count: u8,
  hotspots: Vec<(u16, u16)>
}

impl SpottedCoordinate {
  pub fn new(camera: u8) -> SpottedCoordinate {
    let header = SpottedHeader::new(opcode::OP_COORDINATE);

    let hotspot_count:u8 = 0;
    let hotspots: Vec<(u16, u16)> = Vec::new();

    SpottedCoordinate { header, camera, hotspot_count, hotspots }
  }

  pub fn add_hotspot(&mut self, hotspot: (u16, u16)) {
    self.hotspots.push(hotspot);
    self.hotspot_count += 1;
  }

  pub fn hotspots(&self) -> Vec<(u16, u16)> {
    self.hotspots.clone()
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.push(self.camera);
    out.push(self.hotspot_count);

    for hotspot in self.hotspots.iter() {
      let (x, y) = hotspot;
      out.extend(x.to_be_bytes().iter().cloned());
      out.extend(y.to_be_bytes().iter().cloned());
    }

    println!("sending {:?}", out);

    out
  }

  pub fn deserialize(data: &[u8]) -> SpottedCoordinate {
    let camera = data[0];
    let hotspot_count = data[1];

    let mut out = SpottedCoordinate::new(camera);

    for i in 0..hotspot_count {
      let offset: usize = 2 + (i as usize * 4);
      let x: u16 = ((data[offset] as u16) << 8) | data[offset + 1] as u16;
      let y: u16 = ((data[offset + 2] as u16) << 8) | data[offset + 3] as u16;
      out.add_hotspot((x, y));
    }

    out
  }
}
