# data.rs [![Build Status](https://travis-ci.org/TheHydroImpulse/data.rs.svg?branch=master)](https://travis-ci.org/TheHydroImpulse/data.rs) [![Stories in Ready](https://badge.waffle.io/thehydroimpulse/data.rs.png?label=ready&title=Ready)](https://waffle.io/thehydroimpulse/data.rs) 
WIP!

A [fressian](https://github.com/Datomic/fressian) implementation in Rust.

What's fressian? We'll, it's a data encoding specification that has very high goals.

* self-describing: Fressian is a byte-code driven format.
* schema-free: There's no IDL like Thrift or ProtoBuf. Readers don't need any extra 
               information before reading.
* extensible: Readers and Writers are completely independent. A writer can implement new named
              types. Readers are not required to understand what's being read. If a reader
              doesn't know of a custom type, it'll be tagged for later use.
* batteries-included: Lots of built-in data types.
* binary: Need I say more?
* efficient

The philosophy of data.rs is that of values. Data is driving the encoding, nothing else. Thus,
there is no such thing as references or objects. Just plain values.

It also supports many different things like domain-specific compression, packed encoding, and
chunked encoding.

# Installation

Sigh. This is quite a manual process for Rust libraries... But, only for now.

1. Clone the repo: (Most likely in a submodule or something)

```bash
git clone git@github.com:TheHydroImpulse/data.rs.git
```

2. Build the library:

```bash
cd data.rs/
make
```

3. Build your library and point to the data crate:

```bash
rustc -Ldata.rs/target --out-dir target lib.rs
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

