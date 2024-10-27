use std::{io, path, process};

use regex::Regex;

use clap::Parser;
mod cli;
mod utils;
mod walk;

/// The main entry point for the application.
///
/// This function parses the command-line arguments, creates a regular expression
/// from the provided pattern, and then walks the specified directory (or the
/// current directory if none is provided) to find files that match the pattern.
/// If any errors occur during the file walking process, they are printed to
/// stderr, and the program exits with a non-zero exit code.
fn main() {
    let cli_args = cli::CliArgs::parse();

    let regex = match Regex::new(&cli_args.regexp) {
        Ok(re) => re,
        Err(why) => {
            std::eprintln!("{}", why);
            process::exit(1);
        }
    };

    let path = match cli_args.path.as_str() {
        "" => path::PathBuf::from("."),
        other => path::PathBuf::from(other),
    };

    let error_func: Option<&dyn Fn(io::Error, &path::PathBuf)> = if cli_args.print_errors {
        Some(&utils::error_func)
    } else {
        None
    };

    match walk::walk(
        path.clone(),
        &utils::print_func(regex.clone()),
        &utils::compare_func(regex.clone(), cli_args.filetype),
        error_func,
        cli_args.recursive,
    ) {
        Ok(_) => {}
        Err(why) => {
            std::eprintln!("{}: {:?}", why, path);
            process::exit(1);
        }
    }
}
