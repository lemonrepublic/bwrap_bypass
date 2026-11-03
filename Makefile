all:
	cargo build --release

install: all
	install -d /usr/local/lib /usr/local/bin
	install -m 755 target/release/libbwrap_bypass.so /usr/local/lib/
	install -m 755 target/release/fake_bwrap /usr/local/bin/
	install -m 755 target/release/wrapper /usr/local/bin/flatpak-wrapper
	@echo "Ecosystem successfully installed!"

clean:
	cargo clean

.PHONY: all install clean
