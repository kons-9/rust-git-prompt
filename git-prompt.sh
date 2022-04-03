#!/bin/zsh

# git prompt for zsh
# add-zsh-hook precmd git_prompt

RIGHT="("
LEFT=")"
SEPARATOR="|"

export __GIT_DIR=${0:A:h}

function _git_prompt() {
    unset __CURRENT
    __STATUS="$__GIT_DIR/gitstatus/target/release/gitstatus"
                #    /.zsh/rust-git-prompt/gitstatus/target/release/gitstatus
    # __STATUS="./gitstatus/target/release/gitstatus"
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
    _git_prompt
    if [ $DISPLAY = "TRUE" ]; then
        # printf $1 $RIGHT$GIT_BRANCH$SEPARATOR$GIT_CHANGED$LEFT
        STATUS="$ZSH_THEME_GIT_PROMPT_PREFIX$ZSH_THEME_GIT_PROMPT_BRANCH$GIT_BRANCH%{${reset_color}%}"
        if [ "$GIT_BEHIND" -ne "0" ]; then
            STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_BEHIND$GIT_BEHIND%{${reset_color}%}"
        fi
        if [ "$GIT_AHEAD" -ne "0" ]; then
            STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_AHEAD$GIT_AHEAD%{${reset_color}%}"
        fi
        if [ "$GIT_CHANGED" -eq "0" ] && [ "$GIT_CONFLICTS" -eq "0" ] && [ "$GIT_STAGED" -eq "0" ] && [ "$GIT_UNTRACKED" -eq "0" ]; then
            STATUS="$ZSH_THEME_GIT_PROMPT_PREFIX$ZSH_THEME_GIT_PROMPT_BRANCH$GIT_BRANCH%{${reset_color}%}"
            # STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_CLEAN"
        else
            STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_SEPARATOR"
            if [ "$GIT_STAGED" -ne "0" ]; then
                STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_STAGED$GIT_STAGED%{${reset_color}%}"
            fi
            if [ "$GIT_CONFLICTS" -ne "0" ]; then
                STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_CONFLICTS$GIT_CONFLICTS%{${reset_color}%}"
            fi
            if [ "$GIT_CHANGED" -ne "0" ]; then
                STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_CHANGED$GIT_CHANGED%{${reset_color}%}"
            fi
            if [ "$GIT_UNTRACKED" -ne "0" ]; then
                STATUS="$STATUS$ZSH_THEME_GIT_PROMPT_UNTRACKED$GIT_UNTRACKED%{${reset_color}%}"
            fi
        fi
        STATUS="$STATUS%{${reset_color}%}$ZSH_THEME_GIT_PROMPT_SUFFIX"
        echo "$STATUS"
    else
        # not git project
        echo ""
    fi

# Default values for the appearance of the prompt. Configure at will.
}
ZSH_THEME_GIT_PROMPT_PREFIX="("
ZSH_THEME_GIT_PROMPT_SUFFIX=")"
ZSH_THEME_GIT_PROMPT_SEPARATOR="|"
ZSH_THEME_GIT_PROMPT_BRANCH="%{$fg[magenta]%}"
ZSH_THEME_GIT_PROMPT_STAGED="%{$fg[red]%}%{●%G%}"
ZSH_THEME_GIT_PROMPT_CONFLICTS="%{$fg_bold[red]%}%{✖%G%}"
ZSH_THEME_GIT_PROMPT_CHANGED="%{$fg[blue]%}%{✚%G%}"
ZSH_THEME_GIT_PROMPT_BEHIND="%{↓%G%}"
ZSH_THEME_GIT_PROMPT_AHEAD="%{↑%G%}"
ZSH_THEME_GIT_PROMPT_UNTRACKED="%{?%G%}"
ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[green]%}%{ok%G%}"
