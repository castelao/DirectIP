use camino::Utf8PathBuf as PathBuf;
use clap::Parser;

use directip::Message;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    direction: bool,

    #[arg(long)]
    imei: bool,

    path: PathBuf,
}

fn main() {
    let Cli {
        path,
        direction,
        imei,
    } = Cli::parse();

    let rdr = std::fs::File::open(path).expect("Error opening file");

    let msg = Message::from_reader(rdr).unwrap();

    /*
    let msg = match MTMessage::from_reader(rdr) {
        Ok(v) => v,
        Err(_) => todo!("try MOMessage"),
    };
    */

    if imei {
        dbg!(msg.imei());
    } else if direction {
        dbg!(msg.message_type());
    } else {
        dbg!(msg);
    }
}
