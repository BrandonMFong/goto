#!/bin/bash

GOTO_UTILS_DATA_DIR=~/.gotoutils
GOTO_UTILS_TOOL=$GOTO_UTILS_DATA_DIR/gototool

# gets path from keypaths file using key pair then cds to it
function goto() {
	p=$($GOTO_UTILS_TOOL getpath $1);
	error=$?;
	if [ $error -eq 0 ]; then 
		cd $p;
	fi
}

# cds into the last previous path used in goto()
function goto-prev() {
	p=$($GOTO_UTILS_TOOL getpath-prev);
	error=$?;
	if [ $error -eq 0 ]; then 
		cd $p;
	else
		__goto-print-error "history empty";
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
	if [ $# -ne 2 ]; then
		__goto-print-error "goto-add <key> <path>";
	else
		$GOTO_UTILS_TOOL add $1 $2;
	fi
}

function goto-remove() {
	$GOTO_UTILS_TOOL rm $1;
}

function goto-showkeys() {
	$GOTO_UTILS_TOOL getkeys $1;
}

function __goto-print-error() {
	echo "goto: $@" 1>&2;
}

function goto-showall() {
	$GOTO_UTILS_TOOL getallpairs;
}

function goto-version() {
	$GOTO_UTILS_TOOL version;
}
