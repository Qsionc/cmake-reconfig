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
        if let Err(e) = std::fs::remove_file(&path) {
            eprintln!("{:?}", e);
        }
    }
}

///Check is path exists and is a directory
fn is_existing_directory(path: &Path) -> bool {
    path.exists() && path.is_dir()
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
    let mut args = Args::parse();

    if args.args.contains(&"-B".to_string()) {
        let index = args.args.iter().position(|item| item == "-B").unwrap();
        if args.args.len() >= index + 2 {
            args.build_directory = args.args[index + 1].clone();
        }
    }

    let mut path = PathBuf::from(args.build_directory.clone());

    if is_existing_directory(&path) {
        path.push("CMakeCache.txt");
        delete_file(&path);
    }

    Err(run_cmake_command(&args.build_directory, &args.args))
}