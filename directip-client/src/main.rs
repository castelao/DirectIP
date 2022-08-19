use clap::{Arg, ArgAction, Command};
use log::LevelFilter;

// Need:
// - Disposition Flags
// - Payload
fn main() {
    println!("Hello, world!");

    let cmd = Command::new("DIPCommand")
        .author(clap::crate_authors!("\n"))
        .version(clap::crate_version!())
        .about("Send MT messges")
        .arg(Arg::new("payload"))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("server")
                .long("dest")
                .required(true)
                .help("Destination server host"),
        )
        .arg(
            Arg::new("imei")
                .long("imei")
                .required(true)
                .help("Target IMEI"),
        )
        .arg(
            Arg::new("msgid")
                .long("msg-id")
                .help("Unique client message id"),
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

    println!(
        "verbose: {:?}",
        matches
            .get_one::<u8>("verbose")
            .expect("Count always defaulted")
    );
}
