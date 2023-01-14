mod errors;
mod worktree;
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
    WTTA {
        branch: String,
    },
    WTTD {
        branch: String,
    },
}

fn main() {
    let cli: Cli = Cli::parse();
    let result = match cli.cmd {
        TopLevelCmds::Wt { wt_cmd } => worktree::worktree(wt_cmd),
        TopLevelCmds::WtTmux { wt_cmd } => worktree::worktree_tmux(wt_cmd),
        TopLevelCmds::WTTA { branch } => worktree::worktree_add_branch_attach_tmux(branch),
        TopLevelCmds::WTTD { branch } => worktree::worktree_delete_branch_kill_tmux_sess(branch),
    };
    match result {
        Err(err) => println!("Error executing command: {}", err.to_string()),
        _ => (),
    }
}
