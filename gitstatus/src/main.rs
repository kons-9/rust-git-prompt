use regex::Regex;
use std::fmt;
use std::process::Command;

#[derive(Debug, Clone)]
struct InvalidGitError {
    message: String,
}
impl fmt::Display for InvalidGitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for InvalidGitError {}

fn main() {
    let mut ahead_behind = "0 0".to_string();
    let branch = match exist_git() {
        Ok(Some(str)) => str,
        Ok(None) => {
            ahead_behind = no_branch();
            "none".to_string()
        }
        Err(_) => {
            print_none();
            return;
        }
    };

    let (staged, conflicts, change) = match change_git() {
        Ok(strs) => strs,
        Err(_) => {
            print_none();
            return;
        }
    };
    let untracked_git = match untracked_git() {
        Ok(str) => str,
        Err(_) => {
            print_none();
            return;
        }
    };
    print!(
        "{} {} {} {} {} {} TRUE",
        branch, ahead_behind, staged, conflicts, change, untracked_git
    );
}

fn print_none() {
    print!("none 0 0 0 0 0 0 FALSE");
}

fn exist_git() -> Result<Option<String>, InvalidGitError> {
    let exist_git = Command::new("git")
        .args(&["git", "symbolic-ref", "HEAD"])
        .output()
        .unwrap();
    let (err, exist_git) = (exist_git.stderr, exist_git.stdout);
    let exist_git = String::from_utf8(exist_git).unwrap();
    let err = String::from_utf8(err).unwrap();

    if Regex::new(r"*fatal: Not a git repository*")
        .unwrap()
        .is_match(&err)
    {
        return Err(InvalidGitError {
            message: "No".to_string(),
        });
    }
    let branch_git = &exist_git[11..];
    Ok(Some(branch_git.to_string()))
}
fn no_branch() -> String {
    //#Todo
    "0 0".to_string()
}

fn change_git() -> Result<(String, String, String), InvalidGitError> {
    let change_file = Command::new("git")
        .args(&["diff", "--name-status"])
        .output()
        .expect("failed to start `git status`");
    let (err, change_file) = (change_file.stderr, change_file.stdout);
    let change_file = String::from_utf8(change_file).unwrap();
    let err = String::from_utf8(err).unwrap();

    Ok(("0".to_string(), "0".to_string(), "0".to_string()))
}
fn untracked_git() -> Result<String, InvalidGitError> {
    let untracked_file = Command::new("git")
        .args(&["git", "diff", "--staged", "--name-status"])
        .output()
        .expect("failed");
    Ok("".to_string())
}
