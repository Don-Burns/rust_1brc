pub mod process_data;
pub mod read;
pub mod types;

use crate::process_data::process_data;
use crate::read::read_file;
use crate::types::CliArgs;

use std::env::args;

fn main() {
    let cli_args = CliArgs {
        file_path: args().nth(1).expect("No file path provided"),
    };

    println!("Reading file: {}", cli_args.file_path);
    let data = read_file(cli_args.file_path.as_str());
    let result = process_data(data);
    println!("{}", result)
}
