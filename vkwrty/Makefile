.PHONY: install uninstall clean compile

compile:
	cargo build --release --bin vkwrty

clear: 
	cargo clean

install:
	mkdir -p /usr/local/bin
	mkdir -p /usr/local/lib/systemd/system
	install --mode=755 --owner=root --group=root target/release/vkwrty /usr/local/bin/vkwrty
	install --mode=644 --owner=root --group=root package/vkwrty.service /usr/local/lib/systemd/system/vkwrty.service

uninstall: 
	rm /usr/local/bin/vkwrty
	rm /usr/local/lib/systemd/system/vkwrty.service
