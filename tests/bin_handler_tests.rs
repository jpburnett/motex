// tests/bin_handler_tests.rs

#[cfg(test)]
#[path = "../src/bin_handler.rs"]
mod bin_handler;

mod tests {
    use super::bin_handler::BinFile;
    use std::path::Path; // Import BinFile inside the mod tests

    #[test]
    fn test_read_file_bytes() {
        let path = Path::new("tests/test_files/hello.txt");
        let expected = b"Hello there!";
        let result = BinFile::from_path(path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bin_file_from_path() {
        let file_path = Path::new("tests/test_files/hello.txt");
        let expected_path = file_path.to_path_buf();

        let expected_data = b"Hello there!";
        let bin_file = BinFile::from_path(&file_path).unwrap();
        assert_eq!(bin_file.path, expected_path);
        assert_eq!(bin_file.data, expected_data);
    }
}
