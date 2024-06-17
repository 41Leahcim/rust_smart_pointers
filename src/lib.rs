#![no_std]
#![warn(clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::pub_use,
    clippy::single_call_fn,
    clippy::absolute_paths,
    clippy::implicit_return,
    clippy::expect_used,
    clippy::missing_trait_methods,
    clippy::unwrap_used,
    clippy::multiple_unsafe_ops_per_block,
    clippy::float_cmp,
    clippy::semicolon_outside_block,
    clippy::use_debug,
    clippy::redundant_clone,
    clippy::min_ident_chars
)]
mod shared_pointer;
mod unique_pointer;

pub use shared_pointer::SharedPointer;
pub use unique_pointer::UniquePointer;
