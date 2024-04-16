pub fn read(file_path: &str) -> std::fs::File {
    std::fs::File::open(file_path).expect("File not found")
}
