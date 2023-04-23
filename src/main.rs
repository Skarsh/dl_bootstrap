use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    driver: bool,
}

fn main() {
    let args = Args::parse();

    if args.driver {
        let mut driver_version = Command::new("nvidia-smi");
        driver_version.status().expect("process failed to execute");
    }


    println!();
}
