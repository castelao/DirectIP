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
        let output: String = msg
            .imei()
            .unwrap()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<_>>()
            .join(":");
        println!("{}", output);
    } else if direction {
        println!("{}", msg.message_type());
    } else {
        dbg!(msg);
    }
}
