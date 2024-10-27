use std::{fs, io, path};

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
