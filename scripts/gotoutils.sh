#!/bin/bash

GOTO_UTILS_DATA_DIR=~/.gotoutils
GOTO_UTILS_TOOL=$GOTO_UTILS_DATA_DIR/gototool

function goto() {
	path=$($GOTO_UTILS_TOOL getpath $1);
	error=$?;
	if [ $error -eq 0 ]; then 
		cd $path;
	fi
}

# https://keyholesoftware.com/2022/07/18/adding-autocompletion-to-bash-scripts/
function __goto_completion() {
	if [ $COMP_CWORD -eq 1 ]; then 
		cur=${COMP_WORDS[COMP_CWORD]}
		COMPREPLY=( $(./bin/release/gototool getsugkeys $cur) )
	fi
}
complete -F __goto_completion goto

function goto-add() {
	$GOTO_UTILS_TOOL add $1 $2;
}

function goto-remove() {
	$GOTO_UTILS_TOOL rm $1;
}

function goto-showkeys() {
	$GOTO_UTILS_TOOL getkeys $1;
}

function goto-print-error() {
	echo "$@" 1>&2;
}

