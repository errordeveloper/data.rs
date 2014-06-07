# Thrust

A thrift implementation for Rust utilizing Rust's syntax extension. This allows one to **never** having to run a separate code generation tool. The Thrift IDL is built-into your code.

(This should convince you that Rust is pretty epic.)

# Installation

Sigh. This is quite a manual process for Rust libraries... But, only for now.

1. Clone the repo: (Most likely in a submodule or something)

```bash
git clone git@github.com:TheHydroImpulse/thrust.git
```

2. Build the library:

```bash
cd thrust/
make
```

3. Build your library and point to the Thrust crate:

```bash
rustc -Lthrust/target --out-dir target lib.rs
```

4. Enable the syntax phase for the crate:

```rust
#![crate_id = "foobar"]
#![crate_type = "lib"]
#![features(phase, macro_rules)]

#[phase(syntax)]
extern crate thrustmacro;
extern crate thrust

fn main() {
  let thrust = thrust!(
    namespace foobar Thrift

    enum Numberz
    {
      ONE = 1,
      TWO,
      THREE,
      FIVE = 5,
      SIX,
      EIGHT = 8
    }
  );

  let server = thrust.server("0.0.0.0", 4555);
  server.listen();
}
```

## Testing

```bash
make test
```

## License

The MIT License (MIT)

Copyright (c) 2014 Daniel Fagnan <dnfagnan@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

