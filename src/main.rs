use clap::{Parser, Subcommand};
use std::{env, fs::OpenOptions, io::Write, path::PathBuf, process::Command};

#[derive(Subcommand)]
enum SubCommand {
    #[clap(name = "new")]
    New { name: String },
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Location {
    location: PathBuf,
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: SubCommand,
}

fn create_shortcut(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Command::new("pwsh");

    input.arg("-c").arg("$PROFILE");
    let a = String::from_utf8(input.output().expect("Error!").stdout).unwrap();
    println!("{}", a);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(a.trim())
        .unwrap();

    let to_write = format!(
        "\nfunction cd{} {{\n\tcd {}\n}}\nSet-Alias {} cd{}",
        name,
        env::current_dir().unwrap().display(),
        name,
        name,
    );

    if let Err(e) = writeln!(file, "{}", to_write) {
        eprintln!("Couldn't write to file: {}", e);
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.command {
        SubCommand::New { name } => {
            println!("Creating new shortcut with command: {}", name);
            match create_shortcut(name) {
                Ok(_) => println!("Successfully created shortcut, please restart your shell for it to make effect."),
                Err(e) => println!("Failed to create shortcut: {}", e),
            };
        }
    };
}
