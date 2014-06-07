RUSTC ?= rustc
RUSTC_FLAGS ?=

SRC = $(shell find src -name '*.rs')

all: lib

lib: $(SRC)
	mkdir -p target
	$(RUSTC) --out-dir target src/thrust/lib.rs

macro: $(SRC) lib
	mkdir -p target
	$(RUSTC) --out-dir target src/thrustmacro/lib.rs

test: $(SRC) lib macro
	mkdir -p target
	$(RUSTC) --test -Ltarget --out-dir target src/thrusttest/lib.rs
	./target/thrusttest

clean:
	@rm -rf target


.PHONY: clean