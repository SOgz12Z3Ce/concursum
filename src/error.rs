use serde_json::Value;
use std::{ffi::OsString, io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("failed to walk the directory")]
    WalkDirectory(#[from] walkdir::Error),

    #[error("'{path}' is not descendant path of '{base}'")]
    UnrelatedPath { path: PathBuf, base: PathBuf },

    #[error("'{0:?}' ({path}) is not a UTF-8 string. consider moving it to an ASCII only location", path = .0.display())]
    NonUtf8Path(OsString),

    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("failed to deserialize file '{path}'")]
    Deserialization {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("bad JSON schema: '{value}' ({message})")]
    JsonSchema { value: Value, message: String },
}
