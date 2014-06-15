RUSTC ?= rustc
RUSTC_FLAGS ?=

SRC = $(shell find src -name '*.rs')

all: lib

lib: $(SRC)
	mkdir -p target
	$(RUSTC) --out-dir target src/data/lib.rs

test: $(SRC)
	mkdir -p target
	RUST_TEST_NOCAPTURE=1 $(RUSTC) --test -Ltarget --out-dir target src/data/lib.rs
	./target/data

clean:
	@rm -rf target


.PHONY: clean
