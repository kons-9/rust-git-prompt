#!/bin/zsh
# git prompt
add-zsh-hook precmd git_prompt

RIGHT="("
LEFT=")"
SEPARATOR="|"

function git_prompt() {
    unset CURRENT
    __STATUS=".${0:A:h}/bin/gitstatus"
    __CURRENT=(`$__STATUS`)

	GIT_BRANCH=$__CURRENT[1]
	GIT_AHEAD=$__CURRENT[2]
	GIT_BEHIND=$__CURRENT[3]
	GIT_STAGED=$__CURRENT[4]
	GIT_CONFLICTS=$__CURRENT[5]
	GIT_CHANGED=$__CURRENT[6]
	GIT_UNTRACKED=$__CURRENT[7]
    DISPLAY=$__CURRENT[8]
}

function git_PS() {
    git_prompt
    if [DISPLAY -eq "TRUE"]; then
        echo $RIGHT$GIT_BRANCH$LEFT
    else
        # not git project
        echo ""
    fi
}
