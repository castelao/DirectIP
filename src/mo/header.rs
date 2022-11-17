//! Mobile Originated Header
//!
//! A mobile originated header is one of the information element types that
//! compose a mobile originated message. It is defined by an information
//! element identifier (IEI) with value 0x01.

use crate::error::Error;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::{DateTime, Utc};

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
        Ok(SessionStatus::decode(&status)?)
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
    cdr_id: u32,
    imei: [u8; 15],
    session_status: SessionStatus,
    momsn: u16,
    mtmsn: u16,
    time_of_session: DateTime<Utc>,
}
