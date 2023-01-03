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

fn worktree_add_branch(worktree: String) -> Result<(), Error> {
    let cur_dir = std::env::current_dir().map_err(map_io_err)?;
    let repo = get_repo_root(&cur_dir)?;
    let worktree_exists = repo.find_worktree(&worktree).is_ok();
    if worktree_exists {
        return Ok(());
    }
    let wt_path = repo.path().join(std::path::Path::new(&worktree));
    // TODO(suyogsoti): figure out how to set wt add options like track the existing branch and
    // upstream origin if possible
    repo.worktree(&worktree, &wt_path, None)
        .map_err(map_git2_err)?;
    Ok(())
}

fn worktree_delete_branch(branch: String) -> Result<(), Error> {
    let repo = git2::Repository::open(".").map_err(map_git2_err)?;
    let wt = repo.find_worktree(branch.as_str()).map_err(map_git2_err)?;
    std::fs::remove_dir_all(wt.path()).map_err(map_io_err)?;
    wt.prune(None).map_err(map_git2_err)?;
    let mut branch = repo
        .find_branch(&branch, git2::BranchType::Local)
        .map_err(map_git2_err)?;
    branch.delete().map_err(map_git2_err)
}
