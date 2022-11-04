use super::InformationElementTemplate;
use crate::error::{Error, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone, Debug)]
pub(super) enum MessageStatus {
    // Successful, order of message in the MT message queue starting on 0
    // Currently, the maximum value is 50
    SuccessfulQueueOrder(u8),
    // Invalid IMEI – too few characters, non-numeric characters
    InvalidIMEI,
    // Unknown IMEI – not provisioned on the GSS
    UnkownIMEI,
    // Payload size exceeded maximum allowed
    PayloadOversized,
    // Payload expected, but none received
    PayloadMissing,
    // MT message queue full (max of 50)
    MTQueueFull,
    // MT resources unavailable
    MTResourcesUnavailable,
    // Violation of MT DirectIP protocol
    ProtocolViolation,
    // Ring alerts to the given SSD are disabled
    RingAlertsDisabled,
    // The given SSD is not attached (not set to receive ring alerts)
    SSDNotAttached,
    // Source address rejected by MT filter
    SourceAddressRejected,
    // MTMSN value is out of range (valid range is 1 – 65,535)
    MTMSNOutOfRange,
    // Client SSL/TLS certificate rejected by MT filter
    CertificateRejected,
}

impl MessageStatus {
    fn decode(status: i16) -> Result<MessageStatus> {
        if (0..=50).contains(&status) {
            return Ok(MessageStatus::SuccessfulQueueOrder(
                status.try_into().unwrap(),
            ));
        }
        match status {
            -1 => Ok(MessageStatus::InvalidIMEI),
            -2 => Ok(MessageStatus::UnkownIMEI),
            -3 => Ok(MessageStatus::PayloadOversized),
            -4 => Ok(MessageStatus::PayloadMissing),
            -5 => Ok(MessageStatus::MTQueueFull),
            -6 => Ok(MessageStatus::MTResourcesUnavailable),
            -7 => Ok(MessageStatus::ProtocolViolation),
            -8 => Ok(MessageStatus::RingAlertsDisabled),
            -9 => Ok(MessageStatus::SSDNotAttached),
            -10 => Ok(MessageStatus::SourceAddressRejected),
            -11 => Ok(MessageStatus::MTMSNOutOfRange),
            -12 => Ok(MessageStatus::CertificateRejected),
            s => Err(Error::InvalidMessageStatus(s)),
        }
    }

    fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<MessageStatus> {
        MessageStatus::decode(rdr.read_i16::<BigEndian>()?)
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize> {
        let status = match self {
            MessageStatus::SuccessfulQueueOrder(n) => i16::from(*n),
            MessageStatus::InvalidIMEI => -1,
            MessageStatus::UnkownIMEI => -2,
            MessageStatus::PayloadOversized => -3,
            MessageStatus::PayloadMissing => -4,
            MessageStatus::MTQueueFull => -5,
            MessageStatus::MTResourcesUnavailable => -6,
            MessageStatus::ProtocolViolation => -7,
            MessageStatus::RingAlertsDisabled => -8,
            MessageStatus::SSDNotAttached => -9,
            MessageStatus::SourceAddressRejected => -10,
            MessageStatus::MTMSNOutOfRange => -11,
            MessageStatus::CertificateRejected => -12,
        };
        wtr.write_i16::<BigEndian>(status)?;
        Ok(2)
    }

    fn is_successful(&self) -> bool {}
}

impl std::fmt::Display for MessageStatus {}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", build_fn(error = "crate::error::Error"))]
pub(super) struct Confirmation {
    // From Client (not MTMSN)
    client_msg_id: u32,
    // ASCII Numeric Characters
    imei: [u8; 15],
    // 0 – 4294967295
    // It will be zero when there is an error in processing the message
    id_reference: u32,
    // Order of message in SSD's queue or error reference
    message_status: MessageStatus,
}

impl InformationElementTemplate for Confirmation {
    /// Information Element Identifier
    fn identifier(&self) -> u8 {
        0x44
    }

    // Length field of the Confirmation element
    //
    // The length is the second field, just after the Information Element
    // Identified, and defines how many bytes more after itself composes the
    // Information Element. Therefore it is the total size minus 3 bytes
    // (IEI + length).
    fn len(&self) -> u16 {
        25
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize> {
        wtr.write_u8(0x44)?;
        wtr.write_u16::<BigEndian>(self.len())?;
        wtr.write_u32::<BigEndian>(self.client_msg_id)?;
        wtr.write_all(&self.imei)?;
        wtr.write_u32::<BigEndian>(self.id_reference)?;
        // Shall we recover n written bytes and confirm that it was 2?
        let n = self.message_status.write(wtr)?;
        debug_assert_eq!(n, 2);
        Ok(28)
    }
}

impl Confirmation {
    #[allow(dead_code)]
    /// Parse a DispositionFlags from a Read trait
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Confirmation> {
        let iei = rdr.read_u8()?;
        assert_eq!(iei, 0x44);
        let len = rdr.read_u16::<BigEndian>()?;
        assert_eq!(len, 25);

        let client_msg_id = rdr.read_u32::<BigEndian>()?;
        let mut imei = [0; 15];
        rdr.read_exact(&mut imei)?;
        let id_reference = rdr.read_u32::<BigEndian>()?;
        let message_status = MessageStatus::from_reader(rdr)?;
        Ok(Confirmation {
            client_msg_id,
            imei,
            id_reference,
            message_status,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn builder() -> ConfirmationBuilder {
        ConfirmationBuilder::default()
    }
}

#[cfg(test)]
mod test_mt_confirmation {
    use super::{Confirmation, InformationElementTemplate, MessageStatus};

    #[test]
    fn confirmation_write() {
        let confirmation = Confirmation {
            client_msg_id: 9999,
            imei: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
            id_reference: 4294967295,
            message_status: MessageStatus::MTMSNOutOfRange,
        };
        let mut msg = vec![];
        let n = confirmation.write(&mut msg);
        // Total size is always 28
        assert_eq!(n.unwrap(), 28);
        assert_eq!(
            msg,
            [
                0x44, 0x00, 0x19, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf5
            ]
        );
    }
}

#[cfg(test)]
mod test_mt_confirmation_builder {
    use super::{ConfirmationBuilder, Error};

    #[test]
    fn build_missing_required() {
        let confirmation = ConfirmationBuilder::default().build();
        match confirmation {
            Err(Error::UninitializedFieldError(_)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn build() {
        let confirmation = ConfirmationBuilder::default()
            .client_msg_id(9999)
            .imei([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4])
            .id_reference(0)
            .message_status(super::MessageStatus::SuccessfulQueueOrder(0))
            .build()
            .unwrap();

        // assert_eq!(0x44, confirmation.identifier());
    }
}
