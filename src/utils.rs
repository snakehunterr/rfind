use crate::cli;
use std::{fs, io, os::unix::fs::FileTypeExt, path};

/// Generates a function that prints file paths with matched regex patterns highlighted in red.
///
/// The returned function takes a `path::PathBuf` and a `String` representing the file name,
/// and prints the full file path with any matched regex patterns highlighted in ANSI red.
///
/// # Arguments
/// * `regex`: The regex pattern to match against file names.
///
/// # Returns
/// - A function that takes a `path::PathBuf` and a `String`, and prints the file path with
/// matched regex patterns highlighted.
pub fn print_func(regex: regex::Regex) -> impl Fn(&path::PathBuf, &String) {
    let func = move |path: &path::PathBuf, name: &String| {
        let name = String::from(regex.replacen(&name, 1, |n: &regex::Captures| {
            String::from("\x1b[31m") + &n[0] + "\x1b[0m"
        }));
        std::println!("{}", path.join(name).to_string_lossy());
    };
    return func;
}

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

/// Generates a function that filters file entries based on a regex pattern and file type.
///
/// The returned function takes a `fs::DirEntry` and a `String` representing the file name,
/// and returns a boolean indicating whether the file should be included based on the provided
/// regex pattern and file type.
///
/// # Arguments
/// * `regex`: The regex pattern to match against file names.
/// * `ftype`: The file type to filter by (e.g. directory, file, symlink, etc.).
///
/// # Returns
/// - A function that takes a `fs::DirEntry` and a `String`, and returns a boolean indicating
/// whether the file should be included.
pub fn compare_func(
    regex: regex::Regex,
    ftype: cli::FileType,
) -> impl Fn(&fs::DirEntry, &String) -> bool {
    let func = move |file: &fs::DirEntry, name: &String| -> bool {
        let filetype = match file.file_type() {
            Ok(filetype) => filetype,
            Err(_) => {
                // ! WARNING !
                // TODO What to do?
                return false;
            }
        };

        match ftype.clone() {
            cli::FileType::All => (),
            cli::FileType::Dir => {
                if !filetype.is_dir() {
                    return false;
                }
            }
            cli::FileType::File => {
                if !filetype.is_file() {
                    return false;
                }
            }
            cli::FileType::Link => {
                if !filetype.is_symlink() {
                    return false;
                }
            }
            cli::FileType::CharDevice => {
                if !filetype.is_char_device() {
                    return false;
                }
            }
            cli::FileType::Socket => {
                if !filetype.is_socket() {
                    return false;
                }
            }
        }
        if !regex.is_match(name) {
            return false;
        }
        return true;
    };

    return func;
}
