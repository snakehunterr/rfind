use std::{io, path};

/// Prints an error message with the provided I/O error and file path.
///
/// This function is used to handle and report I/O errors that occur during file operations.
/// It prints the error message to the standard error stream, including the error details and the file path where the error occurred.
///
/// # Arguments
/// * `err`: The `io::Error` instance containing the error details.
/// * `path`: The `path::PathBuf` representing the file path where the error occurred.
pub fn error_func(err: io::Error, path: &path::PathBuf) {
    std::eprintln!("{}: {:?}", err, path);
}
