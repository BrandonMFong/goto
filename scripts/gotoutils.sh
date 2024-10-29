#!/bin/bash


GOTO_UTILS_DATA_DIR=~/.gotoutils
GOTO_UTILS_TOOL=$GOTO_UTILS_DATA_DIR/gototool

GOTO_UTILS_TOOL_ACCEPTED_ARGS=("$($GOTO_UTILS_TOOL --accepted-args)");

function emulate_goto() {
	type emulate > /dev/null 2>&1;
	if [ $? -eq 0 ]; then
		emulate $@;
	fi
}

function goto() {
	old=$(emulate_goto);
	emulate_goto bash;
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
	
	emulate_goto $old;
	return $error;
}

# https://keyholesoftware.com/2022/07/18/adding-autocompletion-to-bash-scripts/
function __goto_completion() {
	if [ $COMP_CWORD -eq 1 ]; then 
		cur=${COMP_WORDS[COMP_CWORD]}
		COMPREPLY=( $($GOTO_UTILS_TOOL --show-suggested-keys $cur) )
	fi
}
complete -F __goto_completion goto

