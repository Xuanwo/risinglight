#![deny(unused_must_use)]

pub mod array;
pub mod binder;
pub mod catalog;
mod db;
pub mod executor;
pub mod logical_plan;
pub mod parser;
pub mod physical_plan;
pub mod storage;
pub mod types;

pub use self::db::{Database, Error};