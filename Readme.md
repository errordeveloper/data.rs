# Thrust

A thrift implementation for Rust utilizing Rust's syntax extension. This allows one to **never** having to run a separate code generation tool. The Thrift IDL is built-into your code.

(This should convince you that Rust is pretty epic.)

# Installation

Sigh. This is quite a manual process for Rust libraries... But, only for now.

1. Clone the repo: (Most likely in a submodule or something)

```bash
git clone x
```

2. Build the library:

```bash
cd thrust/
make lib
```

3. Build your library and point to the Thrust crate:

```bash
rustc -Lthrust/target --out-dir target lib.rs
```

4. Enable the syntax phase for the crate:

```rust
#![crate_id = "foobar"]
#![crate_type = "lib"]
#![features(
```


