//! # Mobile Originated Message
//!
//! ## MO - Information Elements
//!
//! * 0x01 Header IEI
//! * 0x02 Payload IEI
//! * 0x03 Lat/Lon Location Information IEI
//! * 0x05 Confirmation Message IEI
//!
//! ## Example of an MT Message
//!
//! Protocol Revision Number        1   1
//! Overall Message Length          2   31
//! MO Header IEI                   1   0x01
//! MO Header Length                2   28
//! CDR Reference (Auto ID)         4   123456
//! IMEI (User ID)                  15  314159265358979
//! Session Status                  1   0
//! MOMSN                           2   12345
//! MTMSN                           2   54321
//! Time of Session                 4   xxxxxxxx
//! MO Payload IEI                  1   0x02
//! MO Payload Length               2   12
//! MO Payload                      12  "Hello World!"

mod header;
mod payload;
