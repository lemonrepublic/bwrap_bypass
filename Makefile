# Makefile for bwrap_bypass

.PHONY: all build release test clean install uninstall

all: release

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

clean:
	cargo clean

install: release
	install -Dm755 target/release/fake_bwrap /usr/local/bin/fake_bwrap
	install -Dm755 target/release/libbwrap_bypass.so /usr/local/lib/libbwrap_bypass.so
	install -Dm755 scripts/flatpak-wrapper /usr/local/bin/flatpak-wrapper

uninstall:
	rm -f /usr/local/bin/fake_bwrap
	rm -f /usr/local/lib/libbwrap_bypass.so
	rm -f /usr/local/bin/flatpak-wrapper
