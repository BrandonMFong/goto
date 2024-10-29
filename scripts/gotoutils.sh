#!/bin/bash


GOTO_UTILS_DATA_DIR=~/.gotoutils
GOTO_UTILS_TOOL=$GOTO_UTILS_DATA_DIR/gototool

GOTO_UTILS_TOOL_ACCEPTED_ARGS=("$($GOTO_UTILS_TOOL --accepted-args)");

function goto() {
	old=$(emulate);
	emulate bash;
	cont=true;
	p=$($GOTO_UTILS_TOOL $@);
	error=$?;
	if [ $error -ne 0 ]; then
		cont=false;
	fi

	if [ $cont == true ]; then
		len=$#;
		if [ $len -eq 0 ]; then 
			cont=false;
		fi
	fi

	if [ $cont == true ]; then
		for arg in ${GOTO_UTILS_TOOL_ACCEPTED_ARGS[@]};
		do
			if [ "$arg" == "$1" ]; then
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
	
	emulate $old;
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

