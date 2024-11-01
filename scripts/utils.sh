#!/bin/bash

GOTO_UTILS_DATA_DIR_NAME=.goto
GOTO_UTILS_DATA_DIR=~/$GOTO_UTILS_DATA_DIR_NAME
GOTO_UTILS_TOOL_NAME=gototool
GOTO_UTILS_TOOL=$GOTO_UTILS_DATA_DIR/$GOTO_UTILS_TOOL_NAME
GOTO_COMPLETIONS_ZSH_FILE_NAME="_goto";

function __goto_emulate() {
	type emulate > /dev/null 2>&1;
	if [ $? -eq 0 ]; then
		emulate $@;
	fi
}

function goto() {
	old=$(__goto_emulate);
	__goto_emulate bash;
	cont=true;
	p=$($GOTO_UTILS_TOOL $@);
	error=$?;
	if [ $error -ne 0 ]; then
		cont=false;
	fi

	# should default to show help
	if [ $cont == true ]; then
		len=$#;
		if [ $len -eq 0 ]; then 
			cont=false;
		fi
	fi

	# seeing if the user passed an argument for the tool as opposed
	# to a key for a path
	#
	# we are also looking out for `--prev`, as the tool will
	# return the previous directory we were in. Then we will cd
	# into that directory
	if [ $cont == true ]; then
		for arg in ${GOTO_UTILS_TOOL_ACCEPTED_ARGS[@]};
		do
			if [ "$arg" == "$1" ] && [ "$arg" != "--prev" ]; then
				cont=false;
			fi
		done
	fi

	if [ $cont == true ]; then
		cd $p;
	else
		if [ ${#p} -gt 0 ]; then
			printf "$p\n";
		fi
	fi
	
	__goto_emulate $old;
	return $error;
}

# https://keyholesoftware.com/2022/07/18/adding-autocompletion-to-bash-scripts/
function __goto_completion() {
	if [ $COMP_CWORD -eq 1 ]; then 
		cur=${COMP_WORDS[COMP_CWORD]}
		COMPREPLY=( $($GOTO_UTILS_TOOL --show-suggested-keys $cur) )
	fi
}

function goto-init-bash() {
	complete -F __goto_completion goto
}

function goto-init-zsh() {
	fpath+="$GOTO_UTILS_DATA_DIR";
}

function goto-completion-zsh-reload() {
	$GOTO_UTILS_TOOL --completion-zsh > $GOTO_UTILS_DATA_DIR/$GOTO_COMPLETIONS_ZSH_FILE_NAME;
}

function goto-init() {
	GOTO_UTILS_TOOL_ACCEPTED_ARGS=("$($GOTO_UTILS_TOOL --accepted-args)");

	if [ "$ZSH_VERSION" != "" ]; then
		goto-init-zsh;
	else
		goto-init-bash;
	fi
}

