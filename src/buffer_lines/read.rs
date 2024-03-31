pub fn read(file_path: &str) -> std::io::BufReader<std::fs::File> {
    let file = std::fs::File::open(file_path).expect("File not found");
    std::io::BufReader::new(file)
}
