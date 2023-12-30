SHELL=/bin/bash

# Or just CARGO=cargo
CARGO=OPENSSL_INCLUDE_DIR=/usr/local/musl/lib/ \
      OPENSSL_INCLUDE_DIR=/usr/local/musl/include \
      OPENSSL_STATIC=true \
      PKG_CONFIG_ALLOW_CROOSS=1 \
      cargo


.PHONY: all
all: prebuild build

.PHONY: debug
debug:
	@$(CARGO) build --target=x86_64-unknown-linux-musl --package ${PKG}

.PHONY: build
build:
	@$(CARGO) build --target=x86_64-unknown-linux-musl --release


.PHONY: prebuild
prebuild:
	@$(CARGO) fmt
	@$(CARGO) clippy


.PHONY: clean
clean:
	@$(CARGO) clean


.PHONY: test
test:
	@$(CARGO) test


.PHONY: version
version:
	@$(CARGO) --version


.PHONY: ldd
ldd:
	@echo ${PKG} | xargs -I@ find ./target -type f -name @ -exec ldd {} \;


.PHONY: run
run:
	@echo ${PKG} | xargs -I@ find ./target -type f -name @ -exec bash -c {} \;
