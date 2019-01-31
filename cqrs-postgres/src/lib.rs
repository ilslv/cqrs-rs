//! # cqrs-postgres
//!
//! `cqrs-postgres` is an implementation of the CQRS system with persistence to a PostgreSQL backend.

#![warn(
    unused_import_braces,
    unused_imports,
    unused_qualifications,
    missing_docs
)]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_must_use
)]

#[cfg(test)]
extern crate cqrs;
#[cfg(test)]
extern crate cqrs_todo_core;
#[cfg(test)]
extern crate static_assertions;

mod error;
mod store;
mod util;

pub mod raw;

#[doc(inline)]
pub use crate::error::{LoadError, PersistError};
#[doc(inline)]
pub use crate::store::PostgresStore;

#[cfg(test)]
mod tests {
    use super::*;
    use cqrs_todo_core::{TodoAggregate, TodoMetadata};
    use static_assertions::assert_impl;

    #[test]
    fn postgres_store_is_an_entity_store() {
        assert_impl!(PostgresStore<TodoAggregate, TodoMetadata>, cqrs::EntityStore<TodoAggregate, TodoMetadata>);
    }

    #[test]
    fn postgres_store_is_an_entity_source() {
        assert_impl!(PostgresStore<TodoAggregate, TodoMetadata>, cqrs::EntitySource<TodoAggregate>);
    }

    #[test]
    fn postgres_store_is_an_entity_sink() {
        assert_impl!(PostgresStore<TodoAggregate, TodoMetadata>, cqrs::EntitySink<TodoAggregate, TodoMetadata>);
    }
}
