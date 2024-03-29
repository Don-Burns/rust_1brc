mod process_data;
mod read;

use crate::naive::process_data::process_data;
use crate::naive::read::read_file;

pub fn run(file_path: &str) -> String {
    let data = read_file(file_path);
    process_data(data)
}
