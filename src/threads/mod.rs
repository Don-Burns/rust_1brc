mod process_data;
mod read;

use crate::threads::process_data::process_data;
use crate::threads::read::read;

pub fn run(file_path: &str) -> String {
    let data = read(file_path);
    process_data(data)
}
