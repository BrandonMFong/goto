# 
# author: Brando
# date: 6/15/23
#

GOTO_TOOL_NAME = gototool
GOTO_TOOL_BUILD_PATH_RELEASE = ./target/release/$(GOTO_TOOL_NAME)
GOTO_TOOL_BUILD_PATH_DEBUG = ./target/debug/$(GOTO_TOOL_NAME)
SCRIPTS_PATH = ./scripts/*
BIN_DIR = ./bin
BIN_DIR_RELEASE = $(BIN_DIR)/release
BIN_DIR_DEBUG = $(BIN_DIR)/debug
PACKAGE_NAME = goto

release: setup
	cargo build --release
	cp -afv $(GOTO_TOOL_BUILD_PATH_RELEASE) $(BIN_DIR_RELEASE)
	cp -afv $(SCRIPTS_PATH) $(BIN_DIR_RELEASE)

debug: setup
	cargo build
	cp -afv $(GOTO_TOOL_BUILD_PATH_DEBUG) $(BIN_DIR_DEBUG)
	cp -afv $(SCRIPTS_PATH) $(BIN_DIR_DEBUG)

clean:
	rm -rfv $(BIN_DIR)
	cargo clean --verbose --color always

setup:
	@mkdir -p $(BIN_DIR)
	@mkdir -p $(BIN_DIR_RELEASE)
	@mkdir -p $(BIN_DIR_DEBUG)

test:
	RUST_BACKTRACE=1 cargo test -- --test-threads=1

package:
	rm -rfv $(PACKAGE_NAME)
	mkdir $(PACKAGE_NAME)
	cp -afv $(BIN_DIR_RELEASE)/* $(PACKAGE_NAME)
	zip -r $(BIN_DIR)/$(PACKAGE_NAME).zip $(PACKAGE_NAME)

