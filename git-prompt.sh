#!/bin/zsh
# git prompt for zsh

export __GIT_DIR=${0:A:h}

function _git_prompt() {
    unset __CURRENT
    __CURRENT=(`$__GIT_DIR/gitstatus/target/release/gitstatus`)

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
    _git_prompt
    if [ $DISPLAY = "TRUE" ]; then
        # printf $1 $RIGHT$GIT_BRANCH$SEPARATOR$GIT_CHANGED$LEFT
        STATUS="$GIT_PROMPT_PREFIX$GIT_PROMPT_BRANCH$GIT_BRANCH%{${reset_color}%}"
        if [ "$GIT_BEHIND" -ne "0" ]; then
            STATUS="$STATUS$GIT_PROMPT_BEHIND$GIT_BEHIND%{${reset_color}%}"
        fi
        if [ "$GIT_AHEAD" -ne "0" ]; then
            STATUS="$STATUS$GIT_PROMPT_AHEAD$GIT_AHEAD%{${reset_color}%}"
        fi
        if [ "$GIT_CHANGED" -eq "0" ] && [ "$GIT_CONFLICTS" -eq "0" ] && [ "$GIT_STAGED" -eq "0" ] && [ "$GIT_UNTRACKED" -eq "0" ]; then
            STATUS="$GIT_PROMPT_PREFIX$GIT_PROMPT_BRANCH$GIT_BRANCH%{${reset_color}%}"
            # STATUS="$STATUS$GIT_PROMPT_CLEAN"
        else
            STATUS="$STATUS$GIT_PROMPT_SEPARATOR"
            if [ "$GIT_STAGED" -ne "0" ]; then
                STATUS="$STATUS$GIT_PROMPT_STAGED$GIT_STAGED%{${reset_color}%}"
            fi
            if [ "$GIT_CONFLICTS" -ne "0" ]; then
                STATUS="$STATUS$GIT_PROMPT_CONFLICTS$GIT_CONFLICTS%{${reset_color}%}"
            fi
            if [ "$GIT_CHANGED" -ne "0" ]; then
                STATUS="$STATUS$GIT_PROMPT_CHANGED$GIT_CHANGED%{${reset_color}%}"
            fi
            if [ "$GIT_UNTRACKED" -ne "0" ]; then
                STATUS="$STATUS$GIT_PROMPT_UNTRACKED$GIT_UNTRACKED%{${reset_color}%}"
            fi
        fi
        STATUS="$STATUS%{${reset_color}%}$GIT_PROMPT_SUFFIX"
        echo "$STATUS"
    else
        # if not exist git project
        echo ""
    fi
}

GIT_PROMPT_PREFIX="("
GIT_PROMPT_SUFFIX=")"
GIT_PROMPT_SEPARATOR="|"
GIT_PROMPT_BRANCH="%{$fg[magenta]%}"
GIT_PROMPT_STAGED="%{$fg[red]%}%{●%G%}"
GIT_PROMPT_CONFLICTS="%{$fg_bold[red]%}%{✖%G%}"
GIT_PROMPT_CHANGED="%{$fg[blue]%}%{✚%G%}"
GIT_PROMPT_BEHIND="%{↓%G%}"
GIT_PROMPT_AHEAD="%{↑%G%}"
GIT_PROMPT_UNTRACKED="%{?%G%}"
