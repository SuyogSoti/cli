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

fn get_repo_root(path: &std::path::PathBuf) -> Option<git2::Repository> {
    let repo = git2::Repository::open(path);
    match repo.as_ref().map(|r| r.is_bare()) {
        Ok(true) => match repo {
            Ok(repo) => Some(repo),
            _ => None,
        },
        _ => match path.parent() {
            Some(parent) => get_repo_root(&parent.to_path_buf()),
            None => None,
        },
    }
}

fn worktree_add_branch(branch: String) -> Result<(), Error> {
    let repo = std::env::current_dir().map(|pathbuf| get_repo_root(&pathbuf));
    match repo {
        Ok(Some(repo)) => Ok(println!(
            "{} -  need to add {}",
            repo.path().display(),
            branch
        )),
        _ => Ok(()),
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
