use clap::Parser;
use clap::arg;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short='t', long="time", default_value="now")]
    time: String,
    query: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello, world! {:?}", args);
}
