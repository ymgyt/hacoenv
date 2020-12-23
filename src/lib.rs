#![allow(clippy::module_inception)]
// #![deny(missing_docs)]

//! hacobu local development environment setup tool.

/// cli command module.
pub mod cmd;
pub mod inspect;

mod config;

mod prelude {
    pub use anyhow::{Context, Result};

    pub use tracing::{debug, error, info, trace, warn};
}
