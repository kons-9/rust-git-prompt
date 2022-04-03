# rust-git-prompt for zsh
![]()
A script that can make zsh display information about the current git status.
it is written in Rust and zsh script.

# install
1. Download git-prompt.sh and gitstatus 
2. Source git-prompt.sh from your .zshrc config file
```
source {path/to/git-prompt.sh}
function precmd {
PROMPT="%F{green}kons@%m: %F{yellow}%~%f
$(git_PS "%s ")%F{white}$%f "
}
```

I refer to https://github.com/olivierverdier/zsh-git-prompt , it is written in python.