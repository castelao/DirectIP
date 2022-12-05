//! Mobile Originated Header
//!
//! A mobile originated header is one of the information element types that
//! compose a mobile originated message. It is defined by an information
//! element identifier (IEI) with value 0x01.

use crate::error::Error;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, PartialEq)]
/// Session Status
///
/// Status:
/// * 0: Success
///
enum SessionStatus {
    Success,
    MTTooLarge,
    BadLocation,
    Timeout,
    MOTooLarge,
    RFLoss,
    SSDAnomaly,
    SSDProhibited,
}

impl SessionStatus {
    /// Decode a SessionStatus from an u8
    ///
    /// * Success: 0
    /// * MTTooLarge: 1
    ///
    fn decode(status: &u8) -> Result<SessionStatus, Error> {
        match status {
            0 => Ok(SessionStatus::Success),
            1 => Ok(SessionStatus::MTTooLarge),
            2 => Ok(SessionStatus::BadLocation),
            10 => Ok(SessionStatus::Timeout),
            12 => Ok(SessionStatus::MOTooLarge),
            13 => Ok(SessionStatus::RFLoss),
            14 => Ok(SessionStatus::SSDAnomaly),
            15 => Ok(SessionStatus::SSDProhibited),
            s => Err(Error::InvalidSessionStatus(*s)),
        }
    }

    #[allow(dead_code)]
    /// Parse a DispositionFlags from a Read trait
    fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self, Error> {
        let status = rdr.read_u8()?;
        SessionStatus::decode(&status)
    }

    /// Encode a SessionStatus into an u8
    fn encode(&self) -> u8 {
        match self {
            SessionStatus::Success => 0,
            SessionStatus::MTTooLarge => 1,
            SessionStatus::BadLocation => 2,
            SessionStatus::Timeout => 10,
            SessionStatus::MOTooLarge => 12,
            SessionStatus::RFLoss => 13,
            SessionStatus::SSDAnomaly => 14,
            SessionStatus::SSDProhibited => 15,
        }
    }

    /// Save a SessionStatus using a Write trait
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u8(self.encode())?;
        Ok(1)
    }
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SessionStatus::Success => write!(f, "Session completed successfully"),
            SessionStatus::MTTooLarge => {
                write!(f, "MO transfer success, but MT message is too large")
            }
            SessionStatus::BadLocation => write!(f, "MO transfer success, but bad locaton"),
            SessionStatus::Timeout => write!(f, "Session timed out before completion"),
            SessionStatus::MOTooLarge => write!(f, "MO message too large"),
            SessionStatus::RFLoss => write!(f, "Lost connection during session"),
            SessionStatus::SSDAnomaly => write!(f, "Device protocol anomaly"),
            SessionStatus::SSDProhibited => {
                write!(f, "Device prohibited from acessing the Gateway")
            }
        }
    }
}

#[cfg(test)]
mod test_session_status {
    use super::Error;
    use super::SessionStatus;

    #[test]
    fn decode() {
        match SessionStatus::decode(&0x00) {
            Ok(SessionStatus::Success) => SessionStatus::Success,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x01) {
            Ok(SessionStatus::MTTooLarge) => SessionStatus::MTTooLarge,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x02) {
            Ok(SessionStatus::BadLocation) => SessionStatus::BadLocation,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x0a) {
            Ok(SessionStatus::Timeout) => SessionStatus::Timeout,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x0b) {
            Err(Error::InvalidSessionStatus(0x0b)) => (),
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x0c) {
            Ok(SessionStatus::MOTooLarge) => SessionStatus::MOTooLarge,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x0d) {
            Ok(SessionStatus::RFLoss) => SessionStatus::RFLoss,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x0e) {
            Ok(SessionStatus::SSDAnomaly) => SessionStatus::SSDAnomaly,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x0f) {
            Ok(SessionStatus::SSDProhibited) => SessionStatus::SSDProhibited,
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
        match SessionStatus::decode(&0x10) {
            Err(Error::InvalidSessionStatus(0x10)) => (),
            Err(error) => panic!("Error: {:?}", error),
            _ => panic!("Decoded wrong session status"),
        };
    }

    #[test]
    fn roundtrip_decode_encode() {
        let combinations = vec![0, 1, 2, 10, 12, 13, 14, 15];
        for i in combinations {
            assert_eq!(i, SessionStatus::decode(&i).unwrap().encode())
        }
    }

    #[test]
    // Expand this. Maybe replace with a read/write roundtrip
    fn read() {
        let buffer = [0x00].as_slice();
        let status = SessionStatus::from_reader(buffer).unwrap();
        assert_eq!(status, SessionStatus::Success);
    }
}

#[derive(Builder, Debug, PartialEq)]
#[builder(pattern = "owned", build_fn(error = "crate::error::Error"))]
/// Mobile Originated Header
///
/// IEI: 0x01
///
/// Fixed total size of 28 bytes.
///
/// # Components
///
/// * CDR Reference (Auto ID): A 4-byte unique ID for each call data
///   record (CDR).
/// * IMEI: Equipment identifier of the MT message destination. This is a
///   unique 15-digit number in ASCII format.
/// * Session Status:
/// * MOMSN
/// * MTMSN
/// * Time of Session
struct Header {
    cdr_uid: u32,
    imei: [u8; 15],
    session_status: SessionStatus,
    momsn: u16,
    mtmsn: u16,
    time_of_session: DateTime<Utc>,
}

impl Header {
    #[allow(dead_code)]
    // Import a Header from a Read trait
    fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Header, Error> {
        let iei = rdr.read_u8()?;
        assert_eq!(iei, 0x01);
        let len = rdr.read_u16::<BigEndian>()?;
        assert_eq!(len, 28);

        let cdr_uid = rdr.read_u32::<BigEndian>()?;

        let mut imei = [0; 15];
        rdr.read_exact(&mut imei)?;

        let session_status = SessionStatus::from_reader(&mut rdr)?;
        let momsn = rdr.read_u16::<BigEndian>()?;
        let mtmsn = rdr.read_u16::<BigEndian>()?;

        let dt = rdr.read_u32::<BigEndian>()?;
        let time_of_session = Utc.timestamp_opt(dt.into(), 0).single().unwrap();

        Ok(Header {
            cdr_uid,
            imei,
            session_status,
            momsn,
            mtmsn,
            time_of_session,
        })
    }

    #[allow(dead_code)]
    fn imei(self) -> [u8; 15] {
        self.imei
    }

    #[allow(dead_code)]
    fn builder() -> HeaderBuilder {
        HeaderBuilder::default()
    }
}

#[cfg(test)]
mod test_header_builder {
    use super::{Error, HeaderBuilder, SessionStatus, Utc};

    #[test]
    fn build_missing_required() {
        let header = HeaderBuilder::default().build();
        assert!(matches!(header, Err(Error::UninitializedFieldError(_))))
    }

    #[test]
    fn build() {
        let header = HeaderBuilder::default()
            .cdr_uid(9999)
            .imei([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4])
            .session_status(SessionStatus::Success)
            .momsn(999)
            .mtmsn(9999)
            .time_of_session(Utc::now())
            .build()
            .unwrap();
        assert_eq!(9999, header.cdr_uid);
    }
}
