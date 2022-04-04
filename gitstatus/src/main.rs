use regex::Regex;
use std::env;
use std::fmt;
use std::process::Command;

const GIT_PROMPT_PREFIX: &str = "(";
const GIT_PROMPT_SUFFIX: &str = ")";
const GIT_PROMPT_SEPARATOR: &str = "|";
const GIT_PROMPT_BRANCH: &str = r"%{$fg[magenta]%}";
const GIT_PROMPT_STAGED: &str = r"%{$fg[red]%}%{●%G%}";
const GIT_PROMPT_CONFLICTS: &str = r"%{$fg_bold[red]%}%{✖%G%}";
const GIT_PROMPT_CHANGED: &str = r"%{$fg[blue]%}%{✚%G%}";
const GIT_PROMPT_BEHIND: &str = r"%{↓%G%}";
const GIT_PROMPT_AHEAD: &str = r"%{↑%G%}";
const GIT_PROMPT_UNTRACKED: &str = r"%{?%G%}";
const RESET: &str = r"${reset_color}";
const DEFAULT_OUTPUT: &str = r"%s ";

// OP:color:symbol:bold
// const DEFAULT_STATUS: &str = "PR:white:(: BR:magenta:: BE::: AH::: SE:white:| ";

// if you want to use escape color
// const GIT_PROMPT_PREFIX: &str = "(";
// const GIT_PROMPT_SUFFIX: &str = ")";
// const GIT_PROMPT_SEPARATOR: &str = "|";
// const GIT_PROMPT_BRANCH: &str = r"\e[35m";
// const GIT_PROMPT_STAGED: &str = r"\e[31m%{●%G%}";
// const GIT_PROMPT_CONFLICTS: &str = r"\e[31m{✖%G%}";
// const GIT_PROMPT_CHANGED: &str = r"\e[34m%{✚%G%}";
// const GIT_PROMPT_BEHIND: &str = r"%{↓%G%}";
// const GIT_PROMPT_AHEAD: &str = r"%{↑%G%}";
// const GIT_PROMPT_UNTRACKED: &str = r"%{?%G%}";
// const RESET: &str = r"\e[m";
// const DEFAULT_OUTPUT: &str = r"%s ";

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

type FileSize = u64;

fn main() {
    let branch = match exist_git() {
        Ok(str) => (str),
        Err(_) => {
            print!("");
            return;
        }
    };
    let (ahead, behind) = (0, 0);

    let change = match change_git() {
        Ok(strs) => strs,
        Err(_) => {
            print!("");
            return;
        }
    };
    let (staged, conflicts) = match staged_git() {
        Ok(strs) => strs,
        Err(_) => {
            print!("");
            return;
        }
    };
    let untracked_git = match untracked_git() {
        Ok(str) => str,
        Err(_) => {
            print!("");
            return;
        }
    };

    print_all(
        branch,
        ahead,
        behind,
        staged,
        conflicts,
        change,
        untracked_git,
    );
}

#[inline(always)]
fn print_all(
    branch: String,
    ahead: FileSize,
    behind: FileSize,
    staged: FileSize,
    conflicts: FileSize,
    change: FileSize,
    untracked_git: FileSize,
) {
    if branch == "none"
        && ahead == 0
        && behind == 0
        && staged == 0
        && conflicts == 0
        && change == 0
        && untracked_git == 0
    {
        return;
    }

    let mut status = format!(
        "{}{}{}{}",
        GIT_PROMPT_PREFIX, GIT_PROMPT_BRANCH, branch, RESET
    );
    if behind != 0 {
        status = format!("{}{}{}{}", status, GIT_PROMPT_BEHIND, behind, RESET);
    }
    if ahead != 0 {
        status = format!("{}{}{}{}", status, GIT_PROMPT_AHEAD, ahead, RESET);
    }
    status = if staged == 0 && conflicts == 0 && change == 0 && untracked_git == 0 {
        format!("{}{}", status, GIT_PROMPT_SUFFIX)
    } else {
        status = status + GIT_PROMPT_SEPARATOR;
        if staged != 0 {
            status = format!("{}{}{}{}", status, GIT_PROMPT_STAGED, staged, RESET);
        }
        if conflicts != 0 {
            status = format!("{}{}{}{}", status, GIT_PROMPT_CONFLICTS, conflicts, RESET);
        }
        if change != 0 {
            status = format!("{}{}{}{}", status, GIT_PROMPT_CHANGED, change, RESET);
        }
        if untracked_git != 0 {
            status = format!(
                "{}{}{}{}",
                status, GIT_PROMPT_UNTRACKED, untracked_git, RESET
            );
        }
        format!("{}{}", status, GIT_PROMPT_SUFFIX)
    };

    let args: Vec<String> = env::args().collect();
    let output_format = if args.len() >= 2 {
        &args[1]
    } else {
        DEFAULT_OUTPUT
    };
    print!(r"{}", output_format.replace("%s", &status));
}

#[inline(always)]
fn exist_git() -> Result<String, InvalidGitError> {
    let exist_git = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "@"])
        .output()
        .unwrap();
    let (err, exist_git) = (exist_git.stderr, exist_git.stdout);
    let exist_git = String::from_utf8(exist_git).unwrap();
    let err = String::from_utf8(err).unwrap();

    if Regex::new(r"fatal: Not a git repository")
        .unwrap()
        .is_match(&err)
    {
        return Err(InvalidGitError {
            message: "No".to_string(),
        });
    }
    if exist_git.len() == 0 {
        let exist_git = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .unwrap()
            .stdout;
        let exist_git = String::from_utf8(exist_git).unwrap();
        if exist_git.len() == 0 {
            return Err(InvalidGitError {
                message: "No".to_string(),
            });
        }
        Ok(exist_git)
    } else {
        Ok(exist_git)
    }
}

#[inline(always)]
fn change_git() -> Result<FileSize, InvalidGitError> {
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

    Ok(cnt_line - cnt_u)
}

#[inline(always)]
fn staged_git() -> Result<(FileSize, FileSize), InvalidGitError> {
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

    Ok(((cnt_line - cnt_u), cnt_u))
}

#[inline(always)]
fn untracked_git() -> Result<FileSize, InvalidGitError> {
    let untracked_file = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .expect("failed");
    let (err, untracked_file) = (untracked_file.stderr, untracked_file.stdout);
    let untracked_file = String::from_utf8(untracked_file).unwrap();
    let _ = String::from_utf8(err).unwrap();

    let mut count: FileSize = 0;
    for file in untracked_file.lines() {
        if file.chars().nth(0).unwrap() == '?' && file.chars().nth(1).unwrap() == '?' {
            count += 1;
        }
    }
    return Ok(count);
}
