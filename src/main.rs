mod errors;
mod worktree;
use tmux_interface;
use clap::{Parser, Subcommand};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: TopLevelCmds,
}

#[derive(Subcommand)]
enum TopLevelCmds {
    Wt {
        #[command(subcommand)]
        wt_cmd: worktree::WortreeCommands,
    },
    WtTmux {
        #[command(subcommand)]
        wt_cmd: worktree::WortreeCommands,
    },
}

fn main() {
    let cli: Cli = Cli::parse();
    let result = match cli.cmd {
        TopLevelCmds::Wt { wt_cmd } => worktree::worktree(wt_cmd),
        TopLevelCmds::WtTmux { wt_cmd } => worktree::worktree_tmux(wt_cmd),
    };
    match result {
        Err(err) => println!("Error executing command: {}", err.to_string()),
        _ => (),
    }
}
