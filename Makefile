.PHONY: vkwrty
vkwrty:
	cargo build --release --bin vkwrty

.PHONY:
clean: 
	cargo clean
