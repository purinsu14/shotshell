TARGET = shsh
BINARY = shotshell
RELEASE_DIR = target/release

.PHONY: build install clean uninstall

build:
	cargo build --release
	cp $(RELEASE_DIR)/$(BINARY) $(RELEASE_DIR)/$(TARGET)

install: build
	mkdir -p ~/.local/bin
	cp $(RELEASE_DIR)/$(TARGET) ~/.local/bin/

clean:
	cargo clean

uninstall:
	rm -rf ~/.local/bin/$(TARGET)
