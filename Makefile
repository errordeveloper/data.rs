RUSTC ?= rustc
RUSTC_FLAGS ?=
CARGO ?= cargo

SRC = $(shell find src -name '*.rs')

all: lib

lib: $(SRC)
	mkdir -p target
	$(RUSTC) --out-dir target src/data/lib.rs

test: $(SRC)
	$(CARGO) test

clean:
	@rm -rf target


.PHONY: clean
