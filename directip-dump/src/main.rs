use camino::Utf8PathBuf as PathBuf;
use clap::Parser;

use directip::mt::MTMessage;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() {
    let Cli { path } = Cli::parse();

    let rdr = std::fs::File::open(path).expect("Error opening file");

    let msg = match MTMessage::from_reader(rdr) {
        Ok(v) => v,
        Err(_) => todo!("try MOMessage"),
    };

    dbg!(msg);
}
