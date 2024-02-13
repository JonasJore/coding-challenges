pub mod input_utils {
    pub fn read_file_to_string(file_path: &str) -> String {
        let file = std::fs::read_to_string(file_path).expect("error");
        return file;
    }
}
