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

pub fn worktree_tmux(wt_cmd: WortreeCommands) -> Result<(), Error> {
    match wt_cmd {
        WortreeCommands::ADD { branch } => worktree_add_branch_attach_tmux(branch),
        WortreeCommands::DELETE { branch } => worktree_delete_branch_kill_tmux_sess(branch),
    }
}

fn get_repo_root(path: &std::path::PathBuf) -> Result<git2::Repository, Error> {
    let repo = git2::Repository::open(path)?;
    if repo.is_bare() {
        return Ok(repo);
    }
    let parent = path.parent().ok_or_else(|| Error::new("not git repo"))?;
    get_repo_root(&parent.to_path_buf())
}

fn create_wt_base_folders(repo: &git2::Repository, worktree: &str) -> Result<(), Error> {
    let wt_config_base = repo.path().join("worktrees/");
    let wt_path = std::path::Path::new(&worktree);
    wt_path
        .parent()
        .map(|p| std::fs::create_dir_all(wt_config_base.join(p)))
        .unwrap_or(Ok(()))?;
    wt_path
        .parent()
        .map(|p| std::fs::create_dir_all(repo.path().join(p)))
        .unwrap_or(Ok(()))?;
    Ok(())
}

fn add_worktree(worktree: &str) -> Result<(git2::Repository, git2::Worktree), Error> {
    let cur_dir = std::env::current_dir()?;
    let repo = get_repo_root(&cur_dir)?;
    let existing_wt = repo.find_worktree(&worktree);
    if existing_wt.is_ok() {
        return Ok((repo, existing_wt?));
    }
    create_wt_base_folders(&repo, &worktree)?;
    // TODO(suyogsoti): figure out how to set wt add options like track the existing branch and
    // upstream origin if possible
    let wt_path = repo.path().join(std::path::Path::new(&worktree));
    let wt = repo.worktree(&worktree, &wt_path, None)?;
    Ok((repo, wt))
}

fn worktree_add_branch(worktree: String) -> Result<(), Error> {
    add_worktree(&worktree)?;
    Ok(())
}

pub fn worktree_add_branch_attach_tmux(worktree: String) -> Result<(), Error> {
    let (repo, wt) = add_worktree(&worktree)?;
    let proj = repo
        .path()
        .file_name()
        .map(|p| p.to_str())
        .flatten()
        .ok_or(Error::new("worktree creation unsuccessful"))?;
    let session_name = format!("{}_{}", String::from(proj), worktree.replace("/", "_"));
    tmux_interface::NewSession::new()
        .session_name(&session_name)
        .start_directory(wt.path().display().to_string())
        .detached()
        .output()?;
    if std::env::var("TMUX").is_ok() {
        tmux_interface::SwitchClient::new()
            .target_session(&session_name)
            .output()?;
    } else {
        tmux_interface::AttachSession::new()
            .target_session(&session_name)
            .output()?;
    }
    Ok(())
}

fn worktree_delete_branch_kill_tmux_sess(worktree: String) -> Result<(), Error> {
    let repo = cleanup_branch(&worktree)?;
    let proj = repo
        .path()
        .file_name()
        .map(|p| p.to_str())
        .flatten()
        .ok_or(Error::new("worktree creation unsuccessful"))?;
    let session_name = format!("{}_{}", String::from(proj), worktree.replace("/", "_"));
    tmux_interface::KillSession::new()
        .target_session(&session_name)
        .output()?;
    Ok(())
}

fn cleanup_branch(worktree: &str) -> Result<git2::Repository, Error> {
    let cur_dir = std::env::current_dir()?;
    let repo = get_repo_root(&cur_dir)?;
    let wt = repo.find_worktree(&worktree)?;
    std::fs::remove_dir_all(wt.path())?;
    wt.prune(None)?;
    {
        // this is to scope repo's borrow
        let mut branch = repo.find_branch(&worktree, git2::BranchType::Local)?;
        branch.delete()?;
    }
    Ok(repo)
}

fn worktree_delete_branch(branch: String) -> Result<(), Error> {
    cleanup_branch(&branch)?;
    Ok(())
}
