
#![feature(lazy_cell)]

// linting directives (see https://doc.rust-lang.org/rustc/lints/index.html)
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call, unreachable_pub)]
#![warn(variant_size_differences)]
#![deny(deprecated, legacy_derive_helpers)]
#![deny(non_ascii_idents)]

//! SAP Robotic Process Automation bots

/// name of the folder where images are cached
pub const CACHE_DIR: &str = "cache";

pub mod api;
pub mod input;
pub mod processes;
