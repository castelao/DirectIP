//! Client used to build and send MT-Messages
//!

use clap::{Arg, ArgAction, Command};
use directip::mt::MTMessage;
use log::LevelFilter;
//use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

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

    //let term_loglevel = match matches.occurrences_of("verbose") {
    let term_loglevel = match matches
        .get_one::<u8>("verbose")
        .expect("Count always defaulted")
    {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    dbg!(term_loglevel);

    let server = matches.get_one::<String>("server").unwrap();
    let msg_id = *matches.get_one::<u32>("msg_id").unwrap();
    let imei = matches.get_one::<String>("imei").unwrap();
    let payload = matches.get_one::<String>("payload").unwrap();

    println!(
        "verbose: {:?}",
        matches
            .get_one::<u8>("verbose")
            .expect("Count always defaulted")
    );

    let msg = MTMessage::builder()
        .client_msg_id(msg_id)
        // .imei(imei.clone().into_bytes().try_into().unwrap())
        .imei(imei.as_bytes().try_into().unwrap())
        .payload(payload.as_bytes().try_into().unwrap())
        .build();

    /*
        disposition_flags: DispositionFlags::decode(0x0000),
    */
    dbg!(&msg);
    dbg!(msg.to_vec());

    //let mut stream = TcpStream::connect(server);
    /*
    stream.write(msg.to_vec()).unrwap();
    stream.read(&mut)
    */
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
