mod commands;
mod errors;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    cmd: String,
}

fn main() {
    let args: Args = Args::parse();
    let result = commands::top_level_commands(args.cmd)
        .map(|cmd| cmd.run());

    match result {
        Err(err) => println!("Error executing command: {}", err.to_string()),
        _ => (),
    }
}
