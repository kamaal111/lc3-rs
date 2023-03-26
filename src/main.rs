mod virtual_machine;

use clap::Parser;
use std::fs;
use virtual_machine::VirtualMachine;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();
    let file_path = args.file_path;

    let program = fs::read_to_string(file_path).expect("Should have been able to read the file");

    VirtualMachine::run(program);
}
