// tests/bin_handler_tests.rs

#[cfg(test)]
use motex::bin_handler;

mod tests {
    use super::bin_handler::BinFile;
    use std::path::Path;

    /// Tests the behavior of `BinFile::from_path` when reading a file.
    ///
    /// This test reads a file containing the text "Hello there!".
    /// It expects the `BinFile` instance to contain the correct path and data.
    #[test]
    fn test_bin_file_from_path_full() {
        let file_path = Path::new("tests/test_files/hello.txt");
        let expected_path = file_path.to_path_buf();
        let expected_data = b"Hello there!";

        let bin_file = BinFile::from_path(&file_path).unwrap();

        // Check both the path and data fields in one test
        assert_eq!(bin_file.path, expected_path);
        assert_eq!(bin_file.data, expected_data);
    }

    /// Tests the behavior when attempting to read a non-existent file.
    ///
    /// This test expects `BinFile::from_path` to return an error
    /// when the specified file does not exist.
    #[test]
    fn test_non_existent_file() {
        let path = Path::new("tests/test_files/non_existent.txt");
        let result = BinFile::from_path(path);

        // Check that an error is returned, instead of panicking
        assert!(
            result.is_err(),
            "Expected an error when file does not exist"
        );
    }

    /// Tests the behavior when attempting to read a non-existent file.
    ///
    /// This test expects `BinFile::from_path` to return an error
    /// when the specified file does not exist.
    #[test]
    fn test_empty_file() {
        let path = Path::new("tests/test_files/empty.txt");
        let result = BinFile::from_path(path).expect("Failed to read the file at the given path");

        // Check that an error is returned, instead of panicking
        assert!(result.data.is_empty(), "Expected file to be empty");
    }
}
