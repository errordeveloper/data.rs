#![crate_id = "data"]
#![crate_type = "lib"]
#![feature(globs, macro_rules, default_type_params)]

//! ## Introduction
//!
//! data.rs is targeted as a data encoding/decoding library that's language agnostic. This implementation
//! strives to be compatible with fressian.
//!
//! data.rs has the same goals and requirements as fressian:
//!
//! * Language agnostic (This is simply a Rust implementation)
//! * Rich Types -> The data should be encoded back into native Rust equivalent types. Writers should
//!                 be able to also encode Rust specific types.
//! * Extensibility -> A writer and reader don't have to be compatible. i.e., a writer can
//!                    extend the system with more types or encoding schemas and the reader is unaffected.
//! * Self-describing -> There is no IDL. The writer and reader don't have to be compatible.
//! * Binary -> Binary is self-describing and super efficient.
//! * Schema-free -> No IDL. This makes it painless to change the data type, order, etc...
//!
//! This library specifically has no knowledge of networking or I/O. That's not the job of data.rs. There
//! are only few concepts that fundamentally deal with Readers and Writers. Higher-level libraries can
//! implement RPC or networking around data.rs that deals with broadcasting messages across the network.
//!
//! ## Caching
//!
//! data.rs can also do domain-specific compression/caching. A writer can cache a certain value and tag it with
//! a short bytecode sequence. Future writes will include the cached value (often a single byte!).
//!
//! ## Usage
//!
//! FIXME

extern crate serialize;

/// All the default bytecode definitions.
pub mod bytecode;
pub mod result;
pub mod encoder;
pub mod tagged;
pub mod util;
