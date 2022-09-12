//! Client used to compose and send MT-Messages
//!
//! # Example
//! ```text
//! directip-client --msg-id=987 --server 127.0.0.1:10800 --imei 012345678901234 "Hello World"
//! ```
//!
//! # API reference
//!
//! - from-file: Optional argument. When used, it is expected a path to a file
//!   instead of the payload itself, such as:
//!   ```text
//!   echo "Hello world" > ./my_command.txt
//!   directip-client ... --from-file ./my_command.txt
//!   ```
//!
//! # Future plans (not in priority order):
//!
//! * Allow use of disposition flags. Currently assumes all flags off;
//! * Allow binary payload. Currently only takes ASCII;
//! * Logging on terminal as well as file based to keep history;
//! * Default value for client message id, so that the user doesn't need to
//!   define it explicitly;
//! * A catalog of destinations. It is not always convenient to memorize
//!   IMEIs, thus an internal catalog with aliases can be quite convenient;
//! * A dry-run option;
//! * Handle the confirmation acknowledgment. Inform success with queue
//!   position or an informative error message;

#[macro_use]
extern crate log;

use clap::{Arg, ArgAction, Command};
use directip::mt::MTMessage;
// use log::LevelFilter;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    let cmd = Command::new("DIPCommand")
        .author(clap::crate_authors!("\n"))
        .version(clap::crate_version!())
        .about("Send MT messges")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("server")
                .long("server")
                .required(true)
                .takes_value(true)
                .help("Destination server host"),
        )
        .arg(
            Arg::new("imei")
                .long("imei")
                .required(true)
                .takes_value(true)
                .help("Target IMEI"),
        )
        .arg(
            Arg::new("msg_id")
                .long("msg-id")
                .value_parser(clap::value_parser!(u32))
                .takes_value(true)
                .help("Unique client message id"),
        )
        .arg(
            Arg::new("encoding")
                .long("encoding")
                .value_parser(["ascii", "hex", "binary"])
                .default_value("ascii")
                .help("Payload encoding"),
        )
        .arg(
            Arg::new("from_file")
                .long("from-file")
                .action(ArgAction::SetTrue)
                .help("Reads payload from a file"),
        )
        .arg(
            Arg::new("payload")
                .required(true)
                .takes_value(true)
                .help("Payload"),
        )
        // .arg(Arg::new("disposition-flags").multiple_values(true))
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        );
    let matches = cmd.get_matches();

    /*
    let term_loglevel = match matches
        .get_one::<u8>("verbose")
        .expect("Count always defaulted")
    {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    */

    let server = matches.get_one::<String>("server").unwrap();
    let msg_id = *matches.get_one::<u32>("msg_id").unwrap();
    let imei = matches.get_one::<String>("imei").unwrap();
    let encoding: &String = matches.get_one("encoding").expect("default");
    let payload = matches.get_one::<String>("payload").unwrap();
    let from_file = matches.get_one::<bool>("from_file").unwrap_or(&false);

    let payload: Vec<u8> = if *from_file {
        let mut reader = BufReader::new(File::open(payload)?);

        if matches!(encoding.as_ref(), "binary") {
            let mut s = vec![];
            reader.read_to_end(&mut s)?;
            s
        // Handle binary and hex
        } else {
            let mut s = String::new();
            reader.read_to_string(&mut s)?;
            s.trim_end().into()
        }
    } else {
        payload.as_bytes().into()
    };

    debug!("Composing MT-Message");
    let msg = MTMessage::builder()
        .client_msg_id(msg_id)
        .imei(imei.as_bytes().try_into().unwrap())
        .payload(payload)
        .build();

    /*
        disposition_flags: DispositionFlags::decode(0x0000),
    */
    debug!("Composed message: {:?}", msg);
    debug!("MTMessage stream: {:02x?}", msg);

    debug!("Connecting");
    let mut stream = TcpStream::connect(server).unwrap();
    debug!("Transmitting");
    let n = stream.write(msg.to_vec().as_slice()).unwrap();
    info!("Transmitted {} bytes", n);
    let mut buffer = [0u8; 56];
    let n = stream.read(&mut buffer).unwrap();
    info!("Confirmation: {:02x?}", &buffer[..n]);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::MTMessage;

    #[test]
    // Dummy test to confirm that I can create an MTMessage here. Once I test
    // the command, this will be implicit. Right now it confirms that I have
    // access to what I need.
    fn tmp() {
        let msg = MTMessage::builder()
            .client_msg_id(999)
            .imei([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])
            .payload("Hello".into())
            .build()
            .to_vec();

        dbg!(msg);
    }
}
