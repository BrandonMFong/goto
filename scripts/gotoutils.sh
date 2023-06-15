#!/bin/bash

GOTO_UTILS_DATA_DIR=~/.gotoutils
GOTO_UTILS_DATA_DB=~/.gotoutils/keypaths
GOTO_UTILS_TOOL=$GOTO_UTILS_DATA_DIR/gototool

function goto() {
	$GOTO_UTILS_TOOL;
}

function goto-add() {
	echo "TODO: add a path";
	$GOTO_UTILS_TOOL;
}

function goto-remove() {
	echo "TODO: remove a path";
}

function goto-print-error() {
	echo "$@" 1>&2;
}

