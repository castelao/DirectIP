use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::error::{Error, Result};
use crate::InformationElement;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Orientation {
    NE,
    NW,
    SE,
    SW,
}

impl Orientation {
    #[allow(dead_code)]
    /// Decode an Orientation struct from a byte
    ///
    /// Only the two least significant bytes are used. The rest was supposed
    /// to be zero, but to be safe, all other bits are masked, thus ignored.
    fn decode(orientation: &u8) -> Result<Orientation> {
        let mask: u8 = 0b0000_0011;
        match orientation & mask {
            0 => Ok(Orientation::NE),
            1 => Ok(Orientation::NW),
            2 => Ok(Orientation::SE),
            3 => Ok(Orientation::SW),
            _ => Err(Error::Undefined),
        }
    }

    #[allow(dead_code)]
    fn encode(&self) -> u8 {
        match self {
            Orientation::NE => 0,
            Orientation::NW => 1,
            Orientation::SE => 2,
            Orientation::SW => 3,
        }
    }
}

#[cfg(test)]
mod test_orientation {
    use super::Orientation;

    #[test]
    fn roundtrip_decode_encode() {
        for o in [0, 1, 2, 3] {
            assert_eq!(o, Orientation::decode(&o).unwrap().encode())
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq)]
/// A geolocation coordinate
///
/// A coordinate structure keep tight the encode and decode so avoiding bugs
/// due to the wrong order as well as possible sizes.
///
/// Keep in mind the valid range [-90:90] for latitude and [-180:180] for
/// longitude.
struct Coordinate {
    latitude: f64,
    longitude: f64,
}

impl Coordinate {
    fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Coordinate> {
        let mut buffer = [0u8; 7];
        rdr.read_exact(&mut buffer)?;
        let coordinate = Coordinate::decode(&buffer);
        Ok(coordinate)
    }

    #[allow(dead_code)]
    fn encode(&self) -> [u8; 7] {
        let mut buf = [0u8; 7];

        let orientation = if (self.latitude < 0.0) & (self.longitude > 0.0) {
            Orientation::SE
        } else if (self.latitude < 0.0) & (self.longitude < 0.0) {
            Orientation::SW
        } else if (self.latitude > 0.0) & (self.longitude < 0.0) {
            Orientation::NW
        } else {
            Orientation::NE
        };
        buf[0] = orientation.encode();

        // It can't be larger than 90, so it is safe convert to u8
        buf[1] = self.latitude.abs().trunc() as u8;
        let tmp = ((self.latitude.abs().fract() * 60e3) as u16).to_be_bytes();
        buf[2] = tmp[0];
        buf[3] = tmp[1];

        // It can't be larger than 180, so it is safe convert to u8
        buf[4] = self.longitude.abs().trunc() as u8;
        let tmp = ((self.longitude.abs().fract() * 60e3) as u16).to_be_bytes();
        buf[5] = tmp[0];
        buf[6] = tmp[1];
        buf
    }

    #[allow(dead_code)]
    fn decode(buffer: &[u8]) -> Coordinate {
        if buffer.len() < 7 {
            todo!()
        }
        let orientation = Orientation::decode(&buffer[0]).unwrap();
        let lat_sign = match orientation {
            Orientation::SE => -1.,
            Orientation::SW => -1.,
            _ => 1.,
        };
        let lon_sign = match orientation {
            Orientation::NW => -1.,
            Orientation::SW => -1.,
            _ => 1.,
        };

        let lat_decimal = ((buffer[2] as u16) << 8 | buffer[3] as u16) as f64 / 60e3;
        let latitude = lat_sign * (buffer[1] as f64 + lat_decimal);
        let lon_decimal = ((buffer[5] as u16) << 8 | buffer[6] as u16) as f64 / 60e3;
        let longitude = lon_sign * (buffer[4] as f64 + lon_decimal);

        Coordinate {
            latitude,
            longitude,
        }
    }
}

#[cfg(test)]
mod test_coordinate {
    use super::Coordinate;

    #[test]
    fn encode() {
        let c = Coordinate {
            latitude: 15.0,
            longitude: -38.0,
        };
        let buf = [0x01, 0x0f, 0x00, 0x00, 0x26, 0x00, 0x00];
        assert_eq!(buf, c.encode())
    }
    #[test]
    fn roundtrip_decode_encode() {
        let buf = [0x01, 0x0f, 0x00, 0x00, 0x26, 0x00, 0x00];
        assert_eq!(buf, Coordinate::decode(&buf).encode())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq)]
pub(super) struct Location {
    coordinate: Coordinate,
    cep_radius: u32,
}

impl Location {
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Location> {
        let iei = rdr.read_u8()?;
        assert_eq!(iei, 0x03);
        let len = rdr.read_u16::<BigEndian>()?;
        assert_eq!(len, 11);

        let coordinate = Coordinate::from_reader(&mut rdr)?;
        let cep_radius = rdr.read_u32::<BigEndian>()?;

        Ok(Location {
            coordinate,
            cep_radius,
        })
    }

    #[allow(dead_code)]
    fn decode(buffer: &[u8]) -> Result<Self> {
        if buffer.len() < 3 {
            todo!()
        }

        if buffer[0] != 0x03 {
            todo!()
        }
        let length: u16 = (buffer[1] as u16) << 8 | buffer[2] as u16;
        if buffer.len() < length as usize + 3 {
            todo!()
        }
        let coordinate = Coordinate::decode(&buffer[3..]);
        let cep_radius: u32 = (buffer[10] as u32) << 24
            | (buffer[11] as u32) << 16
            | (buffer[12] as u32) << 8
            | buffer[13] as u32;

        Ok(Location {
            coordinate,
            cep_radius,
        })
    }

    fn encode(&self) -> [u8; 7] {
        todo!();
        let buffer = [0u8; 7];
        buffer
    }
}

impl InformationElement for Location {
    /// MO-Location Identifier
    fn identifier(&self) -> u8 {
        0x03
    }

    /// Location element length
    fn len(&self) -> u16 {
        11
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize> {
        todo!();
        wtr.write_u8(self.identifier())?;
        wtr.write_u16::<BigEndian>(self.len())?;
        wtr.write_all(&self.encode())?;
        wtr.write_u32::<BigEndian>(self.cep_radius)?;
        Ok(14)
    }
}

#[cfg(test)]
mod test_location {
    use super::{InformationElement, Location};

    #[test]
    fn decode() {
        let buffer = [
            0x03, 0x00, 0x0b, 0x01, 0x21, 0x28, 0x47, 0x76, 0x7f, 0x06, 0x00, 0x01, 0x00, 0x00,
        ];
        let location = Location::decode(&buffer).unwrap();
        assert_eq!(location.len(), 11);
        assert_eq!(location.cep_radius, 65536);
    }
}
