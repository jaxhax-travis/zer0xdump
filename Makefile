# Requirements
ifeq ($(shell which cc),)
$(error "cc not found in PATH, consider running: apt update && apt install -y build-essential")
endif
ifeq ($(shell which rustup),)
$(error "rustup not found in PATH, consider running: wget -q --https-only --secure-protocol=TLSv1_2 https://sh.rustup.rs -O - | sh /dev/stdin -y && source $$HOME/.cargo/env")
endif
ifeq ($(shell rustup show|grep stable),)
$(error "No stable Rust found in toolchain, consider running: rustup toolchain install stable")
endif
ifneq ($(shell which sudo),)
sudo_pfx = sudo
endif

binary:
	@echo "Building binary"
	cargo build --release
	@strip $(shell pwd)/target/release/0xdump
	@echo "Done!"

install:
	@echo "Installing..."
	@$(sudo_pfx) cp $(shell pwd)/target/release/0xdump /usr/bin/0xdump
	@echo "Done!"

clean:
	@echo "Cleaning up"
	@cargo clean
	@rm Cargo.lock
	@echo "Done!"
