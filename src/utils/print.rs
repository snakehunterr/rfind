use std::path;

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
