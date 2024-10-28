use std::{fs, os::unix::fs::FileTypeExt};

use crate::cli;

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
