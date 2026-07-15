BINARY = shsh
RELEASE_DIR = target/release
UNAME := $(shell uname -s)

.PHONY: build install clean uninstall check-os

check-os:
ifneq ($(UNAME),Linux)
	$(error This project only supports Linux. Detected: $(UNAME))
endif

build: check-os
	cargo build --release

install: build
	mkdir -p ~/.local/bin
	cp $(RELEASE_DIR)/$(BINARY) ~/.local/bin/
	@echo "Installed to ~/.local/bin — make sure it's in your PATH"

uninstall:
	rm -rf ~/.local/bin/$(TARGET)
	cargo clean
