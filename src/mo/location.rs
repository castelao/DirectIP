use crate::error::{Error, Result};

enum Orientation {
    NE,
    NW,
    SE,
    SW,
}

impl Orientation {
    fn decode(orientation: &u8) -> Result<Orientation> {
        match orientation {
            0 => Ok(Orientation::NE),
            1 => Ok(Orientation::NW),
            2 => Ok(Orientation::SE),
            3 => Ok(Orientation::SW),
            _ => Err(Error::Undefined),
        }
    }
}

struct LocationData {
    latitude: f64,
    longitude: f64,
}

impl LocationData {
    fn encode() {
        let mut buf: [u8; 7];
        let  buf[0] = if (self.latitude < 0) & (self.longitude < 0) {
            3
        } else if (self.latitude < 0) & (self.longitude > 0) {
            2
        } else if (self.latitude > 0) & (self.longitude < 0) {
            1
        } else {
            0
        };

        buf[1] = self.latitude.abs().trunc();
        buf[2..=3] = u16::try_from(self.latitude.abs().fract() * 60e3).unwrap().to_be_bytes();
        buf[4] = self.longitude.abs().trunc();
        buf[5..=6] = u16::try_from(self.longitude.abs().fract() * 60e3).unwrap().to_be_bytes();

        buf
    }
}

struct Location {
    latitude: f64,
    longitude: f64,
    cep_radius: u32,
}
