use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::os::unix::process::CommandExt;

use clap::Parser;

/// Args structure for argument parsing using clap
#[derive(Parser, Debug)]
#[clap(author)]
#[clap(version = "0.1.0")]
#[clap(about = "Simple application to reconfigure CMake project by removing CMakeCache.txt from build directory.")]
#[clap(long_about = None)]
#[clap(trailing_var_arg = true)]
#[clap(allow_hyphen_values = true)]
struct Args {
    /// Build directory of cmake, defaults to current directory
    #[clap(short = 'B')]
    #[clap(help = "Provide target build directory. If not provided, defaults to current working directory.")]
    #[clap(default_value = ".")]
    build_directory: String,

    /// Other cmake arguments go here
    #[clap(help = "All other arguments are passed to CMake executable togheter with -B option.")]
    args: Vec<String>,
}

/// Simple method for deletion of single file, if existing
fn delete_file(path: &Path) {
    if path.exists() {
        if let Err(e) = std::fs::remove_file(path) {
            eprintln!("{:?}", e);
        }
    }
}

///Run CMake command with custom arguments from command line
fn run_cmake_command(build_dir: &str, args: &[String]) -> io::Error {
    Command::new("cmake")
        .arg("-B")
        .arg(build_dir)
        .args(args)
        .exec()
}

fn main() -> io::Result<()> {
    let Args { mut build_directory, args } = Args::parse();

    if let Some(index) = args.iter().position(|item| item == "-B") {
        if args.len() >= index + 2 {
            build_directory = args[index + 1].clone();
        }
    }

    let mut path = PathBuf::from(build_directory.clone());

    if path.is_dir() {
        path.push("CMakeCache.txt");
        delete_file(&path);
    }

    Err(run_cmake_command(&build_directory, &args))
}