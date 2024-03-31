mod process_data;
mod read;

use crate::buffer_lines::process_data::process_data;
use crate::buffer_lines::read::read;

pub fn run(file_path: &str) -> String {
    let data = read(file_path);
    process_data(data)
}
