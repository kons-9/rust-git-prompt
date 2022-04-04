#!/bin/zsh
setopt PROMPT_SUBST
autoload -Uz colors: colors

#hello
# git prompt for zsh
export __GIT_DIR=${0:A:h}

function git_PS() {
    echo "`$__GIT_DIR/gitstatus/target/release/gitstatus $1`"
}
