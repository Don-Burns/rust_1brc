pub mod buffer_lines;
pub mod naive;
pub mod naive_v2;

use std::env::args;

pub struct CliArgs {
    pub file_path: String,
    pub strategy: String,
}

fn main() {
    let cli_args = CliArgs {
        file_path: args().nth(1).expect("No file path provided"),
        strategy: args().nth(2).unwrap_or("buffer_lines".to_string()),
    };

    println!("Using file: {}", cli_args.file_path);
    let result = match cli_args.strategy.as_str() {
        "naive" => {
            println!("Using naive strategy");
            naive::run(cli_args.file_path.as_str())
        }
        "naive_v2" => {
            println!("Using naive_v2 strategy");
            naive_v2::run(cli_args.file_path.as_str())
        }
        "buffer_lines" => {
            println!("Using buffer_lines strategy");
            buffer_lines::run(cli_args.file_path.as_str())
        }
        _ => panic!("Invalid strategy, valid options are: naive, naive_v2, buffer_lines"),
    };
    println!("{}", result)
}
