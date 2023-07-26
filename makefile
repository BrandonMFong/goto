# 
# author: Brando
# date: 6/15/23
#

GOTO_UTILS_DATA_DIR = ~/.gotoutils
GOTO_TOOL_NAME = gototool
GOTO_TOOL_BUILD_PATH_RELEASE = ./target/release/$(GOTO_TOOL_NAME)
GOTO_TOOL_BUILD_PATH_DEBUG = ./target/debug/$(GOTO_TOOL_NAME)
GOTO_SCRIPT_NAME = gotoutils.sh
GOTO_SCRIPT_PATH = ./scripts/$(GOTO_SCRIPT_NAME)
BIN_DIR = ./bin
BIN_DIR_RELEASE = $(BIN_DIR)/release
BIN_DIR_DEBUG = $(BIN_DIR)/debug

release: setup
	cargo build --release
	cp -afv $(GOTO_TOOL_BUILD_PATH_RELEASE) $(BIN_DIR_RELEASE)
	cp -afv $(GOTO_SCRIPT_PATH) $(BIN_DIR_RELEASE)

debug: setup
	cargo build
	cp -afv $(GOTO_TOOL_BUILD_PATH_DEBUG) $(BIN_DIR_DEBUG)
	cp -afv $(GOTO_SCRIPT_PATH) $(BIN_DIR_DEBUG)

clean:
	rm -rfv $(BIN_DIR)
	cargo clean --verbose --color always

setup:
	@mkdir -p $(BIN_DIR)
	@mkdir -p $(BIN_DIR_RELEASE)
	@mkdir -p $(BIN_DIR_DEBUG)

install:
	@mkdir -p $(GOTO_UTILS_DATA_DIR)
	cp -afv $(BIN_DIR_RELEASE)/* $(GOTO_UTILS_DATA_DIR)

uninstall:
	rm -rfv $(GOTO_UTILS_DATA_DIR)

test:
	cargo test

