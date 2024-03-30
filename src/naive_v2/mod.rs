mod process_data;
mod read;

use crate::naive_v2::process_data::process_data;
use crate::naive_v2::read::read_file;

pub fn run(file_path: &str) -> String {
    let data = read_file(file_path);
    process_data(data)
}
