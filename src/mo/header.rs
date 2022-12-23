//! Mobile Originated Header
//!
//! A mobile originated header is one of the information element types that
//! compose a mobile originated message. It is defined by an information
//! element identifier (IEI) with value 0x01.

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::{DateTime, TimeZone, Utc};
use derive_builder::Builder;

use crate::error::Error;
use crate::InformationElement;

#[derive(Debug, PartialEq)]
/// Session Status
///
/// Status:
/// * 0: Success
///
pub(crate) enum SessionStatus {
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
pub(crate) struct Header {
    cdr_uid: u32,
    imei: [u8; 15],
    session_status: SessionStatus,
    momsn: u16,
    mtmsn: u16,
    time_of_session: DateTime<Utc>,
}

impl Header {
    // Import a Header from a Read trait
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Header, Error> {
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

//#[allow(dead_code)]
impl InformationElement for Header {
    /// MT-Header identifier
    fn identifier(&self) -> u8 {
        0x01
    }

    // Header length field
    //
    // This is a fixed value for the Header, but used to keep consistency with
    // the other IEI.
    fn len(&self) -> u16 {
        28
    }

    /// Export a Header using a Write trait
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u8(0x01)?;
        wtr.write_u16::<BigEndian>(28)?;
        wtr.write_u32::<BigEndian>(self.cdr_uid)?;
        wtr.write_all(&self.imei)?;
        let n = self.session_status.write(wtr)?;
        debug_assert_eq!(n, 1);
        wtr.write_u16::<BigEndian>(self.momsn)?;
        wtr.write_u16::<BigEndian>(self.mtmsn)?;
        wtr.write_u32::<BigEndian>(
            self.time_of_session
                .timestamp()
                .try_into()
                .expect("Can't handle time before 1970"),
        )?;
        Ok(28)
    }
}

#[cfg(test)]
mod test_mt_header {
    use super::{DateTime, Header, InformationElement, SessionStatus, Utc};

    #[test]
    fn header_write() {
        let header = Header {
            cdr_uid: 9999,
            imei: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            session_status: SessionStatus::Success,
            momsn: 999,
            mtmsn: 111,
            time_of_session: "2000-03-14T12:12:12Z".parse::<DateTime<Utc>>().unwrap(),
        };
        let msg = header.to_vec();
        assert_eq!(
            msg,
            [
                0x01, 0x00, 0x1c, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x00, 0x03, 0xe7, 0x00, 0x6f, 0x38,
                0xce, 0x2c, 0x9c
            ]
        );
    }

    #[test]
    fn roundtrip_to_vec_n_read() {
        let header = Header {
            cdr_uid: 9999,
            imei: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            session_status: SessionStatus::Success,
            momsn: 999,
            mtmsn: 111,
            time_of_session: "2000-03-14T12:12:12Z".parse::<DateTime<Utc>>().unwrap(),
        };
        assert_eq!(
            header,
            Header::from_reader(header.to_vec().as_slice()).unwrap()
        );
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
