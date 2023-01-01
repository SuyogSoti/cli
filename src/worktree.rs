use crate::errors::Error;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum WortreeCommands {
    ADD { branch: String },
    DELETE { branch: String },
}

pub fn worktree(wt_cmd: WortreeCommands) -> Result<(), Error> {
    match wt_cmd {
        WortreeCommands::ADD { branch } => worktree_add_branch(branch),
        WortreeCommands::DELETE { branch } => worktree_delete_branch(branch),
    }
}

fn worktree_add_branch(_branch: String) -> Result<(), Error> {
    let result = git2::Repository::open(".");
    match result {
        Ok(repo) => {
            let path = repo.path();
            repo.workdir().map(|idx| println!("{}", idx.display()));
            println!("{}", path.display());
            return Ok(());
        }
        Err(err) => Err(Error::new(err.message())),
    }
}

fn worktree_delete_branch(branch: String) -> Result<(), Error> {
    let result = git2::Repository::open(".").and_then(|repo| repo.find_worktree(branch.as_str()));
    match result {
        Ok(_path) => {
            return Ok(());
        }
        Err(err) => Err(Error::new(err.message())),
    }
}
