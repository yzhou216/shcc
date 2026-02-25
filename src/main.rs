#![feature(slice_split_once)]
use clap::Parser;
use directories::BaseDirs;
use log::info;
use std::fs;
use std::io::Write;
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    file_path: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    //match cli.debug {
    //    0 => println!("Debug mode is off"),
    //    1 => println!("Debug mode is kind of on"),
    //    2 => println!("Debug mode is on"),
    //    _ => println!("Don't be crazy"),
    //}

    if let Some(name) = cli.file_path.as_deref() {
        info!("C source path: {name}");
        let source = fs::read(name).expect("failed to read file");

        let stem = Path::new(name).file_stem().unwrap().to_str().unwrap();
        let cache_dir = BaseDirs::new()
            .unwrap()
            .cache_dir()
            .join(env!("CARGO_PKG_NAME"));
        fs::create_dir_all(&cache_dir).unwrap(); // Create directory if it doesn't exist
        let exe_path = cache_dir.join(stem);
        let exe_path = exe_path.to_str().unwrap();

        let mut child = Command::new("cc")
            .args(["-x", "c", "-o", exe_path, "-"])
            .stdin(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to execute the C compiler process");

        child
            .stdin
            .take()
            .unwrap()
            .write_all(if source.starts_with(b"#!") {
                // Skip shebang line if it exists
                source
                    .iter()
                    .position(|&b| b == b'\n')
                    .map(|pos| &source[pos + 1..])
                    .unwrap_or(&source)
            } else {
                &source
            })
            .expect("failed to send source to compiler");

        if !child.wait().expect("wait failed").success() {
            eprintln!("Compilation failed");
            std::process::exit(1);
        }

        info!("exec path: {exe_path}");
        let output = Command::new(exe_path)
            .output()
            .expect("failed to execute program");
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
