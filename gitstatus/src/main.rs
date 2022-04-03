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
    let ahead_behind;
    let branch = match exist_git() {
        Ok(Some(str)) => {
            ahead_behind = "0 0".to_string();
            str
        }
        Ok(None) => {
            ahead_behind = no_branch();
            "none".to_string()
        }
        Err(_) => {
            print_none();
            return;
        }
    };

    let change = match change_git() {
        Ok(strs) => strs,
        Err(_) => {
            print_none();
            return;
        }
    };
    let (staged, conflicts) = match staged_git() {
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
    if branch == "none" {
        print_none();
    } else {
        print!(
            "{} {} {} {} {} {} TRUE",
            branch, ahead_behind, staged, conflicts, change, untracked_git
        );
    }
}

fn print_none() {
    print!("none 0 0 0 0 0 0 FALSE");
}

fn exist_git() -> Result<Option<String>, InvalidGitError> {
    let exist_git = Command::new("git")
        .args(&["symbolic-ref", "HEAD"])
        .output()
        .unwrap();
    let (err, exist_git) = (exist_git.stderr, exist_git.stdout);
    let exist_git = String::from_utf8(exist_git).unwrap();
    let err = String::from_utf8(err).unwrap();
    // println!("{}", exist_git);

    if Regex::new(r"fatal: Not a git repository")
        .unwrap()
        .is_match(&err)
    {
        return Err(InvalidGitError {
            message: "No".to_string(),
        });
    }
    if exist_git.len() >= 11 {
        let branch_git = &exist_git[11..(exist_git.len() - 1)];
        Ok(Some(branch_git.to_string()))
    } else {
        Ok(None)
    }
}

fn no_branch() -> String {
    //#Todo
    "0 0".to_string()
}

fn change_git() -> Result<String, InvalidGitError> {
    // #Todo
    let change_file = Command::new("git")
        .args(&["diff", "--name-status"])
        .output()
        .expect("failed to start `git status`");
    let (err, change_file) = (change_file.stderr, change_file.stdout);
    let change_file = String::from_utf8(change_file).unwrap();
    let _ = String::from_utf8(err).unwrap();

    let mut cnt_line = 0;
    let mut cnt_u = 0;
    for file in change_file.lines() {
        cnt_line += 1;
        if file.chars().nth(0).unwrap() == 'U' {
            cnt_u += 1;
        }
    }

    Ok((cnt_line - cnt_u).to_string())
}
fn staged_git() -> Result<(String, String), InvalidGitError> {
    // #Todo
    let staged_file = Command::new("git")
        .args(&["diff", "--staged", "--name-status"])
        .output()
        .expect("failed");
    let (err, staged_file) = (staged_file.stderr, staged_file.stdout);
    let staged_file = String::from_utf8(staged_file).unwrap();
    let _ = String::from_utf8(err).unwrap();

    let mut cnt_line = 0;
    let mut cnt_u = 0;
    for file in staged_file.lines() {
        cnt_line += 1;
        if file.chars().nth(0).unwrap() == 'U' {
            cnt_u += 1;
        }
    }

    Ok(((cnt_line - cnt_u).to_string(), cnt_u.to_string()))
}
fn untracked_git() -> Result<String, InvalidGitError> {
    // #Todo
    let untracked_file = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .expect("failed");
    let (err, untracked_file) = (untracked_file.stderr, untracked_file.stdout);
    let untracked_file = String::from_utf8(untracked_file).unwrap();
    let _ = String::from_utf8(err).unwrap();

    let mut count: usize = 0;
    for file in untracked_file.lines() {
        if file.chars().nth(0).unwrap() == '?' && file.chars().nth(1).unwrap() == '?' {
            count += 1;
        }
    }
    return Ok(count.to_string());
}
