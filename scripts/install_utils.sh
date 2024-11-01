#!/bin/bash
#
# author: brando
# date: 4/2/24
#
# used by install and uninstall script

SCRIPT_PATH=$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P );

## CONSTANTS

ARG_HELP="help";
ARG_HELP_V2="--help";
ARG_HELP_V3="-h";

GOTO_UTILS_FILE_NAME="utils.sh";
GOTO_ENV_FILE_NAME="env.sh";

source $SCRIPT_PATH/utils.sh

# update if there are more things we need to copy over
GOTO_COMPONENTS=( "$GOTO_UTILS_TOOL_NAME" "$GOTO_UTILS_FILE_NAME" "$GOTO_ENV_FILE_NAME" )

## GLOBALS
gShowHelp=false;

# 
# arguments_read()
#
# returns error code
#
# install and uninstall don't take any args
function arguments_read() 
{
	for arg in "${V_ARGS[@]}"
	do
		if [ "$ARG_HELP" == "$arg" ]; then
			gShowHelp=true;
		elif [ "$ARG_HELP_V2" == "$arg" ]; then
			gShowHelp=true;
		elif [ "$ARG_HELP_V3" == "$arg" ]; then
			gShowHelp=true;
		fi
	done

	return 0;
}

