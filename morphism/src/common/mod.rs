//! # Common Utilities for use throughout Morphism
//!
//! This module can essentially be thought of as an extension to the Rust core library. It provides
//! standard library like functionality for the Morphism kernel. This  module should not duplicate
//! functionality that is already provided by core or any other high quality no_std library unless
//! there is a compelling reason to do so.

pub mod bitwise;
pub mod cell;
pub mod io;
pub mod raw_mutex;
pub mod traits;
pub mod vector;
