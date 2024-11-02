# 
# author: Brando
# date: 6/15/23
#

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
	BUILD_TYPE=linux
else 
ifeq ($(UNAME_S),Darwin)
	BUILD_TYPE=macos
endif
endif
 
GOTO_TOOL_NAME = gototool
SCRIPTS_PATH = ./scripts/*
BIN_DIR = ./bin
PACKAGE_NAME = goto

CONFIG=release
ifeq ($(CONFIG), release)
BIN_DIR_OUTPUT = $(BIN_DIR)/release
GOTO_TOOL_BUILD_PATH = ./target/release/$(GOTO_TOOL_NAME)
BUILD_TYPE_FLAG=--release
else ifeq ($(CONFIG), debug)
BIN_DIR_OUTPUT = $(BIN_DIR)/debug
GOTO_TOOL_BUILD_PATH = ./target/debug/$(GOTO_TOOL_NAME)

# default is debug
BUILD_TYPE_FLAG=
endif

SCRIPT_NAMES = install uninstall install_utils.sh utils.sh env.sh
SCRIPT_DEST = $(patsubst %, $(BIN_DIR_OUTPUT)/%, $(SCRIPT_NAMES))

COMPONENT_NAMES = $(SCRIPT_NAMES) $(GOTO_TOOL_NAME)
PACKAGE_COMPONENTS = $(patsubst %, $(PACKAGE_NAME)/%, $(COMPONENT_NAMES))

.PHONY: package-setup

build: setup $(SCRIPT_DEST)
	cargo build $(BUILD_TYPE_FLAG) --target-dir $(BIN_DIR)

$(BIN_DIR_OUTPUT)/%: scripts/%
	@cp -afv $< $(BIN_DIR_OUTPUT)

setup: $(BIN_DIR_OUTPUT)
	git submodule update --init --recursive external/libs

$(BIN_DIR)/%:
	mkdir -p $@

clean:
	rm -rfv $(BIN_DIR)
	rm -rfv $(PACKAGE_NAME)
	cargo clean --verbose --color always

test:
	RUST_BACKTRACE=1 cargo test -- --test-threads=1

package: $(PACKAGE_NAME) build $(PACKAGE_COMPONENTS)
	zip -r $(BIN_DIR)/$(PACKAGE_NAME)-$(BUILD_TYPE).zip $(PACKAGE_NAME)
	tar vczf $(BIN_DIR)/$(PACKAGE_NAME)-$(BUILD_TYPE).tar.gz $(PACKAGE_NAME)

$(PACKAGE_NAME):
	mkdir -p $@

$(PACKAGE_NAME)/%: $(BIN_DIR_OUTPUT)/%
	@cp -afv $< $(PACKAGE_NAME)

