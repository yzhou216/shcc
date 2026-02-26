#![feature(slice_split_once)]

use std::path::PathBuf;

use clap::Parser;

use shcc::program::ProgramBuilder;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        print!(
            "{}",
            ProgramBuilder::default()
                .path(name.into())
                .build()?
                .run()
                .unwrap()
        );
    };

    Ok(())
}
