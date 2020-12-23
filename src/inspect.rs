use std::ffi::OsStr;
use std::path::PathBuf;

use crate::config::{Common, Dependency};
use crate::prelude::*;

#[derive(Debug)]
pub struct InspectResult {
    pub dep: Dependency,
    pub state: InspectState,
}

#[derive(Debug)]
pub enum InspectState {
    Installed(PathBuf),
    NotFound,
    // MismatchVersion,
}

pub fn common(cfg: &Common) -> Result<Vec<InspectResult>> {
    let results = cfg
        .dependencies
        .iter()
        .map(|dep| {
            let state = match lookup_binary_path(dep.binary_name()) {
                Some(path) => InspectState::Installed(path),
                None => InspectState::NotFound,
            };

            InspectResult {
                dep: dep.clone(),
                state,
            }
        })
        .collect();

    Ok(results)
}

pub fn lookup_binary_path(binary_name: impl AsRef<OsStr>) -> Option<PathBuf> {
    match which::which(binary_name) {
        Ok(path) => Some(path),
        Err(err) => {
            warn!("which: {}", err);
            None
        }
    }
}
