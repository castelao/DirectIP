//! Mobile Terminated Header
//!
//! A mobile terminated header is one of the information element types that
//! compose a mobile terminated message. It is defined by an information
//! element identifier (IEI) with value 0x41.

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use derive_builder::Builder;

use crate::error::Error;
use crate::InformationElement;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Builder, Clone, Debug, PartialEq, Eq)]
/// Disposition Flags
///
/// Flags:
/// * Flush MT Queue: Delete all MT payloads in the SSD’s MT queue
/// * Send Ring Alert - Mo MTM: Send ring alert with no associated MT payload
///   (normal ring alert rules apply)
/// * Update SSD Location: Update SSD location with given lat/lon values
/// * High Priority Message: Place the associated MT payload in front of queue
/// * Assign MTMSN: Use the value in the Unique ID field as the MTMSN
///
/// # Notes
///
/// * The bit 3 was not defined at this point, skipping from 2nd to 4th.
///   Therefore, all flags on would be 0b0000_0000_0011_1011.
pub struct DispositionFlags {
    #[builder(default = "false")]
    flush_queue: bool,
    #[builder(default = "false")]
    send_ring_alert: bool,
    #[builder(default = "false")]
    update_location: bool,
    #[builder(default = "false")]
    high_priority: bool,
    #[builder(default = "false")]
    assign_mtmsn: bool,
}

impl DispositionFlags {
    /// Decode a u16 into a DispositionFlags
    ///
    /// Each flag is encoded by a bit in a specific position, which is on
    /// (true) or off (false). Parsing the that sequence of bits, assuming
    /// a big endian unsigned integer, results in the following values when
    /// activated:
    ///
    /// * Flush MT Queue: 1
    /// * Send Ring Alert - Mo MTM: 2
    /// * Update SSD Location: 8
    /// * High Priority Message: 16
    /// * Assign MTMSN: 32
    ///
    /// # Notes:
    ///
    /// - All non used bits are ignored. It might be useful to consider a more
    ///   strict approach, where this would fail if a non-expected bit is
    ///   activated.
    fn decode(code: u16) -> Self {
        let flush_queue = matches!(code & 0b0000_0000_0000_0001, 1);
        let send_ring_alert = matches!(code & 0b0000_0000_0000_0010, 2);
        let update_location = matches!(code & 0b0000_0000_0000_1000, 8);
        let high_priority = matches!(code & 0b0000_0000_0001_0000, 16);
        let assign_mtmsn = matches!(code & 0b0000_0000_0010_0000, 32);

        DispositionFlags {
            flush_queue,
            send_ring_alert,
            update_location,
            high_priority,
            assign_mtmsn,
        }
    }

    /// Parse a DispositionFlags from a Read trait
    fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self, Error> {
        let code = rdr.read_u16::<BigEndian>()?;
        Ok(DispositionFlags::decode(code))
    }

    /// Encode a DispositionFlags into an u16
    fn encode(&self) -> u16 {
        (u16::from(self.assign_mtmsn) << 5)
            + (u16::from(self.high_priority) << 4)
            + (u16::from(self.update_location) << 3)
            + (u16::from(self.send_ring_alert) << 1)
            + u16::from(self.flush_queue)
    }

    /// Save a DispositionFlags using a Write trait
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u16::<BigEndian>(self.encode())?;
        Ok(2)
    }
}

#[cfg(test)]
mod test_disposition_flags {
    use super::DispositionFlags;

    #[test]
    fn decode_all_false() {
        let ans = DispositionFlags {
            flush_queue: false,
            send_ring_alert: false,
            update_location: false,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(ans, DispositionFlags::decode(0));
    }

    #[test]
    fn decode_flush_queue() {
        let ans = DispositionFlags {
            flush_queue: true,
            send_ring_alert: false,
            update_location: false,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(ans, DispositionFlags::decode(1));
    }

    #[test]
    fn decode_send_ring_alert() {
        let ans = DispositionFlags {
            flush_queue: false,
            send_ring_alert: true,
            update_location: false,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(ans, DispositionFlags::decode(2));
    }

    #[test]
    fn decode_update_location() {
        let ans = DispositionFlags {
            flush_queue: false,
            send_ring_alert: false,
            update_location: true,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(ans, DispositionFlags::decode(8));
    }

    #[test]
    fn decode_all_true() {
        let ans = DispositionFlags {
            flush_queue: true,
            send_ring_alert: true,
            update_location: true,
            high_priority: true,
            assign_mtmsn: true,
        };
        assert_eq!(ans, DispositionFlags::decode(59));
    }

    #[test]
    fn encode_all_false() {
        let flags = DispositionFlags {
            flush_queue: false,
            send_ring_alert: false,
            update_location: false,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(flags.encode(), 0);
    }

    #[test]
    fn encode_flush_queue() {
        let flags = DispositionFlags {
            flush_queue: true,
            send_ring_alert: false,
            update_location: false,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(flags.encode(), 1);
    }

    #[test]
    fn encode_send_ring_alert() {
        let flags = DispositionFlags {
            flush_queue: false,
            send_ring_alert: true,
            update_location: false,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(flags.encode(), 2);
    }

    #[test]
    fn encode_update_location() {
        let flags = DispositionFlags {
            flush_queue: false,
            send_ring_alert: false,
            update_location: true,
            high_priority: false,
            assign_mtmsn: false,
        };
        assert_eq!(flags.encode(), 8);
    }

    #[test]
    fn encode_high_priority() {
        let flags = DispositionFlags {
            flush_queue: false,
            send_ring_alert: false,
            update_location: false,
            high_priority: true,
            assign_mtmsn: false,
        };
        assert_eq!(flags.encode(), 16);
    }

    #[test]
    fn encode_assign_mtmsn() {
        let flags = DispositionFlags {
            flush_queue: false,
            send_ring_alert: false,
            update_location: false,
            high_priority: false,
            assign_mtmsn: true,
        };
        assert_eq!(flags.encode(), 32);
    }

    #[test]
    fn encode_all_true() {
        let flags = DispositionFlags {
            flush_queue: true,
            send_ring_alert: true,
            update_location: true,
            high_priority: true,
            assign_mtmsn: true,
        };
        assert_eq!(flags.encode(), 59);
    }

    #[test]
    fn roundtrip_decode_encode() {
        let combinations = vec![
            1, 2, 3, 8, 9, 10, 11, 16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35, 40, 41, 42, 43,
            48, 49, 50, 51, 56, 57, 58, 59,
        ];
        for i in combinations {
            assert_eq!(i, DispositionFlags::decode(i).encode())
        }
    }

    #[test]
    fn read() {
        let buffer = [0x00, 0x01].as_slice();
        let flags = DispositionFlags::from_reader(buffer).unwrap();
        assert!(flags.flush_queue);
    }
}

#[cfg(all(test, feature = "serde"))]
mod test_disposition_flags_serde {
    use super::DispositionFlags;

    #[test]
    fn roundtrip() {
        let disposition_flags = DispositionFlags {
            flush_queue: true,
            send_ring_alert: true,
            update_location: true,
            high_priority: true,
            assign_mtmsn: true,
        };
        let json = serde_json::to_string(&disposition_flags).unwrap();

        let roundtrip: DispositionFlags = serde_json::from_str(&json).unwrap();

        assert_eq!(disposition_flags, roundtrip);
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Builder, Debug, PartialEq)]
#[builder(pattern = "owned", build_fn(error = "crate::error::Error"))]
/// Mobile Terminated Header
///
/// IEI: 0x41
///
/// Fixed total size of 24 bytes.
///
/// # Components
///
/// * Client message ID: A 4-byte ID defined by the client which is used in
///   the confirmation message sent back to the client. If the Assign MTMSN
///   flag of the DispositionFlags is activated, this id value is assumed to
///   be the MTMSN for the associated MT message payload.
/// * IMEI: Equipment identifier of the MT message destination. This is a
///   unique 15-digit number in ASCII format.
/// * DispositionFlags: A set of flags available to the client trigger
///   specific actions on the Iridium Gateway. See [DispositionFlags] for
///   more details.
pub(crate) struct Header {
    client_msg_id: u32, // or 4 u8?
    imei: [u8; 15],
    #[builder(default = "DispositionFlagsBuilder::default().build().unwrap()")]
    disposition_flags: DispositionFlags,
}

impl Header {
    // Import a Header from a Read trait
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Header, Error> {
        let iei = rdr.read_u8()?;
        assert_eq!(iei, 0x41);
        let len = rdr.read_u16::<BigEndian>()?;
        assert_eq!(len, 21);

        let client_msg_id = rdr.read_u32::<BigEndian>()?;
        let mut imei = [0; 15];
        rdr.read_exact(&mut imei)?;
        let disposition_flags = DispositionFlags::from_reader(rdr)?;

        Ok(Header {
            client_msg_id,
            imei,
            disposition_flags,
        })
    }

    /// client_msg_id field
    #[allow(dead_code)]
    fn client_msg_id(self) -> u32 {
        self.client_msg_id
    }

    /// imei field
    pub(crate) fn imei(&self) -> [u8; 15] {
        self.imei
    }

    /// DispositionFlags field
    #[allow(dead_code)]
    fn disposition_flags(self) -> DispositionFlags {
        self.disposition_flags
    }

    #[allow(dead_code)]
    pub(crate) fn builder() -> HeaderBuilder {
        HeaderBuilder::default()
    }
}

// Let's allow dead while still WIP
#[allow(dead_code)]
impl InformationElement for Header {
    /// MT-Header identifier
    fn identifier(&self) -> u8 {
        0x41
    }

    // Header length field
    //
    // This is a fixed value for the Header, but used to keep consistency with
    // the other IEI.
    fn len(&self) -> u16 {
        21
    }

    // Export a Header using a Write trait
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u8(0x41)?;
        wtr.write_u16::<BigEndian>(21)?;
        wtr.write_u32::<BigEndian>(self.client_msg_id)?;
        wtr.write_all(&self.imei)?;
        self.disposition_flags.write(wtr)?;
        Ok(24)
    }
}

#[cfg(test)]
mod test_mt_header {
    use super::{DispositionFlags, Header, InformationElement};

    #[test]
    fn header_write() {
        let header = Header {
            client_msg_id: 9999,
            imei: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            disposition_flags: DispositionFlags {
                flush_queue: true,
                send_ring_alert: true,
                update_location: true,
                high_priority: true,
                assign_mtmsn: true,
            },
        };
        let mut msg = vec![];
        let n = header.write(&mut msg);
        // Total size is always 24
        assert_eq!(n.unwrap(), 24);
        assert_eq!(
            msg,
            [
                0x41, 0x00, 0x15, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x00, 0x3b
            ]
        );
    }

    #[test]
    fn header_to_vec() {
        let header = Header {
            client_msg_id: 9999,
            imei: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            disposition_flags: DispositionFlags {
                flush_queue: true,
                send_ring_alert: true,
                update_location: true,
                high_priority: true,
                assign_mtmsn: true,
            },
        };
        let output = header.to_vec();

        assert_eq!(
            output,
            [
                0x41, 0x00, 0x15, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x00, 0x3b
            ]
        );
    }

    #[test]
    fn roundtrip_to_vec_n_read() {
        let header = Header {
            client_msg_id: 9999,
            imei: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            disposition_flags: DispositionFlags {
                flush_queue: true,
                send_ring_alert: true,
                update_location: true,
                high_priority: true,
                assign_mtmsn: true,
            },
        };
        assert_eq!(
            header,
            Header::from_reader(header.to_vec().as_slice()).unwrap()
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod test_header_serde {
    use super::{DispositionFlags, Header};

    #[test]
    fn roundtrip() {
        let header = Header {
            client_msg_id: 9999,
            imei: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            disposition_flags: DispositionFlags {
                flush_queue: true,
                send_ring_alert: true,
                update_location: true,
                high_priority: true,
                assign_mtmsn: true,
            },
        };
        let json = serde_json::to_string(&header).unwrap();
        //assert_eq!(json, "");

        let roundtrip: Header = serde_json::from_str(&json).unwrap();

        assert_eq!(header, roundtrip);
    }
}

#[cfg(test)]
mod test_mt_header_builder {
    use super::{Error, HeaderBuilder};

    #[test]
    fn build_missing_required() {
        let header = HeaderBuilder::default().build();
        match header {
            Err(Error::UninitializedFieldError(_)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn build() {
        let header = HeaderBuilder::default()
            .client_msg_id(9999)
            .imei([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4])
            .build()
            .unwrap();
        assert_eq!(9999, header.client_msg_id());
    }
}
