use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum, Copy)]
pub enum FileType {
    All,
    File,
    Dir,
    Link,
    Socket,
    CharDevice,
}

/// The command-line arguments for the application.
///
/// This struct defines the various command-line arguments that the application
/// accepts, including the path to search, a regular expression pattern, whether
/// to search recursively, the file type to filter by, and whether to print
/// errors.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct CliArgs {
    /// vals
    #[arg(default_value = "")]
    pub path: String,

    /// RegExp pattern
    #[arg(short('e'), default_value = "")]
    pub regexp: String,

    /// Recursive
    #[arg(long("recursive"), short('r'))]
    pub recursive: bool,

    /// FileType
    #[arg(long("filetype"), short('T'), default_value = "all")]
    pub filetype: FileType,

    #[arg(long("print-errors"), short('E'))]
    pub print_errors: bool,
}
