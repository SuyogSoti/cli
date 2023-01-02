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

fn map_git2_err(err: git2::Error) -> Error {
    Error::new(err.message())
}

fn map_io_err(err: std::io::Error) -> Error {
    Error::new(err.to_string().as_str())
}

fn get_repo_root(path: &std::path::PathBuf) -> Result<git2::Repository, Error> {
    let repo = git2::Repository::open(path).map_err(map_git2_err)?;
    if repo.is_bare() {
        return Ok(repo);
    }
    let parent = path.parent().ok_or_else(|| Error::new("not git repo"))?;
    get_repo_root(&parent.to_path_buf())
}

fn worktree_add_branch(branch: String) -> Result<(), Error> {
    let cur_dir = std::env::current_dir().map_err(map_io_err)?;
    let repo = get_repo_root(&cur_dir)?;
    let branch_exists = repo.find_worktree(&branch).is_ok();
    if branch_exists {
        return Ok(());
    }
    let wt_path = repo.path().join(std::path::Path::new(&branch));
    repo.worktree(&branch, &wt_path, None)
        .map_err(map_git2_err)?;
    Ok(())
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
