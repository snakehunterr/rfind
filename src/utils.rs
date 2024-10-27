use crate::cli;
use std::{fs, io, os::unix::fs::FileTypeExt, path};

pub fn print_func(regex: regex::Regex) -> impl Fn(&path::PathBuf, &String) {
    let func = move |path: &path::PathBuf, name: &String| {
        let name = String::from(regex.replacen(&name, 1, |n: &regex::Captures| {
            String::from("\x1b[31m") + &n[0] + "\x1b[0m"
        }));
        std::println!("{}", path.join(name).to_string_lossy());
    };
    return func;
}

pub fn error_func(err: io::Error, path: &path::PathBuf) {
    std::eprintln!("{}: {:?}", err, path);
}

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
