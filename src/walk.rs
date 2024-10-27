use std::{fs, io, path};

/// Recursively walks a directory and its subdirectories, calling the provided
/// `out_func` callback for each file that passes the `filter_func` check.
///
/// # Arguments
/// * `path` - The path to the directory to walk.
/// * `out_func` - A callback function that will be called for each file that passes the filter.
/// * `filter_func` - A callback function that determines whether a file should be included.
/// * `error_func` - An optional callback function that will be called if an error occurs during the walk.
/// * `recursive` - Whether to recursively walk subdirectories.
///
/// # Errors
/// This function will return an error if there is a problem reading the directory or its contents.
pub fn walk(
    path: path::PathBuf,
    out_func: &dyn Fn(&path::PathBuf, &String),
    filter_func: &dyn Fn(&fs::DirEntry, &String) -> bool,
    error_func: Option<&dyn Fn(io::Error, &path::PathBuf)>,
    recursive: bool,
) -> Result<(), io::Error> {
    let files = fs::read_dir(&path)?;

    for file in files.flatten() {
        let filepath = file.path();
        let filename = match filepath.clone().file_name() {
            Some(name) => String::from(name.to_string_lossy()),
            None => {
                continue;
            }
        };

        if filter_func(&file, &filename) {
            out_func(&path, &filename);
        }
        if let Ok(filetype) = file.file_type() {
            if recursive && filetype.is_dir() {
                match walk(filepath, out_func, filter_func, error_func, recursive) {
                    Ok(()) => (),
                    Err(why) => {
                        if let Some(func) = error_func {
                            func(why, &path);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
