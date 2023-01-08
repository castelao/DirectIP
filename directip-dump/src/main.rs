use camino::Utf8PathBuf as PathBuf;
use clap::Parser;

use directip::Message;

/* Possible uses/combinations. Think about it.
 *
 * Use ncdump as a reference for API behavior
 *
 * - default: show everything
 * --direction: MT or MO
 * --imei: 012345678901234
 * --imei-hex: 00:01:02:03:04:05:06:07:08:09:0a:0b:0c:0d:0e
 * --payload=hex
 * --payload=bin
 * --payload=int
 *
 */

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
