# Makefile to aid with local development and testing
# This is not required for automated builds

# Detect architecture
ARCH := $(shell uname -m)
# Detect OS
ifeq ($(OS),Windows_NT)
	PLATFORM := win
else
	UNAME := $(shell uname)
	ifeq ($(UNAME),Linux)
		PLATFORM := linux
	endif
	ifeq ($(UNAME),Darwin)
		PLATFORM := mac
	endif
endif

TARGET_DIR := ../target

CARGO_BUILD_FLAGS = --release -p c_api --no-default-features --features "rust_native_crypto, file_io"

# Helper function to create a zip file
define make_zip
	set -e; \
	echo "Reading version from $(1)/c2pa.h"; \
	if [ "$(PLATFORM)" = "win" ]; then \
		VERSION=$$(findstr "^// Version:" $(1)/c2pa.h | awk -F": " "{print \$$2}"); \
	else \
		VERSION=$$(grep '^// Version:' $(1)/c2pa.h | sed 's/^\/\/ Version: //'); \
	fi; \
	mkdir -p $(TARGET_DIR)/artifacts $(1)/include $(1)/lib; \
	cp $(1)/c2pa.h $(1)/include/; \
	cp $(1)/libc2pa_c.* $(1)/lib/; \
	(cd $(1) && zip -9 -r ../../artifacts/c2pa-v$${VERSION}-$(2).zip include lib); \
	echo "Zip file created: $(TARGET_DIR)/artifacts/c2pa-v$${VERSION}-$(2).zip"
endef

check-format:
	cargo +nightly fmt -- --check

clippy:
	cargo clippy --features "rust_native_crypto, file_io" -- -D warnings

test-local:
	cargo test --features "rust_native_crypto, file_io"

# Full local validation, build and test all features including wasm
# Run this before pushing a PR to pre-validate
test: check-format clippy test-local

fmt:
	cargo +nightly fmt

# These are for building the c2patool release bin on various platforms
release-win-x86_64:
	rustup update stable-msvc
	rustup target add x86_64-pc-windows-msvc
	cargo build --target=x86_64-pc-windows-msvc $(CARGO_BUILD_FLAGS)


release-win-aarch64:
	rustup update stable-msvc
	rustup target add aarch64-pc-windows-msvc
	cargo build --target=aarch64-pc-windows-msvc $(CARGO_BUILD_FLAGS)
#  powershell -ExecutionPolicy Bypass -File ../../setup-rust-openssl.ps1

release-mac-arm:
	rustup target add aarch64-apple-darwin
	MACOSX_DEPLOYMENT_TARGET=11.1 cargo build --target=aarch64-apple-darwin $(CARGO_BUILD_FLAGS)
	install_name_tool -id @rpath/libc2pa_c.dylib $(TARGET_DIR)/aarch64-apple-darwin/release/libc2pa_c.dylib

release-mac-x86:
	rustup target add x86_64-apple-darwin
	MACOSX_DEPLOYMENT_TARGET=10.15 cargo build --target=x86_64-apple-darwin $(CARGO_BUILD_FLAGS)
	install_name_tool -id @rpath/libc2pa_c.dylib $(TARGET_DIR)/x86_64-apple-darwin/release/libc2pa_c.dylib

release-mac-universal: release-mac-arm release-mac-x86
	rm -rf $(TARGET_DIR)/universal-apple-darwin
	mkdir -p $(TARGET_DIR)/universal-apple-darwin/release
	lipo -create -output $(TARGET_DIR)/universal-apple-darwin/release/libc2pa_c.dylib $(TARGET_DIR)/aarch64-apple-darwin/release/libc2pa_c.dylib ../target/x86_64-apple-darwin/release/libc2pa_c.dylib
	cp $(TARGET_DIR)/aarch64-apple-darwin/release/c2pa.h $(TARGET_DIR)/universal-apple-darwin/release/c2pa.h
	install_name_tool -id @rpath/libc2pa_c.dylib $(TARGET_DIR)/universal-apple-darwin/release/libc2pa_c.dylib
	dsymutil $(TARGET_DIR)/universal-apple-darwin/release/libc2pa_c.dylib

	cd $(TARGET_DIR)/universal-apple-darwin/release && \
		zip -r9 libc2pa_c.dylib.dSYM.zip libc2pa_c.dylib.dSYM && \
		rm -rf libc2pa_c.dylib.dSYM && \
		ls -l

	strip -x $(TARGET_DIR)/universal-apple-darwin/release/libc2pa_c.dylib
	@$(call make_zip,$(TARGET_DIR)/universal-apple-darwin/release,universal-apple-darwin)

	strip -x $(TARGET_DIR)/aarch64-apple-darwin/release/libc2pa_c.dylib
	@$(call make_zip,$(TARGET_DIR)/aarch64-apple-darwin/release,aarch64-apple-darwin)
	
	strip -x $(TARGET_DIR)/x86_64-apple-darwin/release/libc2pa_c.dylib
	@$(call make_zip,$(TARGET_DIR)/x86_64-apple-darwin/release,x86_64-apple-darwin)

release-linux-gnu-x86:
	rustup target add x86_64-unknown-linux-gnu
	cargo build --target=x86_64-unknown-linux-gnu  $(CARGO_BUILD_FLAGS)
	@$(call make_zip,$(TARGET_DIR)/x86_64-unknown-linux-gnu/release,x86_64-unknown-linux-gnu)

release-linux-gnu-arm:
	rustup target add aarch64-unknown-linux-gnu
	cargo build --target=aarch64-unknown-linux-gnu  $(CARGO_BUILD_FLAGS)
	@$(call make_zip,$(TARGET_DIR)/aarch64-unknown-linux-gnu/release,aarch64-unknown-linux-gnu)

# make release
# Builds and packages a zip for c_api for each platform
ifeq ($(PLATFORM), mac)
release: release-mac-universal
endif
ifeq ($(PLATFORM), win)
ifeq ($(ARCH), x86_64)
release: release-win
endif
ifeq ($(ARCH), aarch64)
echo "No release build for aarch64 on Windows"
endif
endif
ifeq ($(PLATFORM), linux)
ifeq ($(ARCH), x86_64)
release: release-linux-gnu-x86
endif
ifeq ($(ARCH), aarch64)
release: release-linux-gnu-arm
endif
endif
