use crate::data::cs::{group::Group, localization::Locale, object::OwnedKey};
use axum::{http::StatusCode, response::IntoResponse};
use serde_json::{Map, Value};
use std::{ffi::OsString, fmt::Display, io, path::PathBuf};
use tantivy::{TantivyError, query::QueryParserError};
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

    #[error(transparent)]
    JsonSchema(#[from] JsonSchemaError),

    #[error("bad group: '{0}'")]
    Group(String),

    #[error("duplicated '{locale}' localization objects are found: '{key}'")]
    DuplicatedLocalizationObject { locale: Locale, key: OwnedKey },

    #[error(transparent)]
    Tantivy(#[from] TantivyError),

    #[error("search with empty keywords parameter")]
    EmptySearch,

    #[error(transparent)]
    QueryParser(#[from] QueryParserError),

    #[error("bad query grammar: '{0}'")]
    QueryGrammar(String),

    #[error("field '{0}' is not supported to search")]
    NotsupportedQuery(String),

    #[error("there is no object with group '{group}' and ID '{id}'")]
    ObjectNotFound { group: Group, id: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "text/plain; charset=utf-8")
            .body(self.to_string().into())
            .unwrap()
    }
}

#[derive(Debug, Error)]
#[error("bad JSON schema: '{value}' ({kind})")]
pub(crate) struct JsonSchemaError {
    value: Value,
    kind: JsonSchemaErrorKind,
}

impl JsonSchemaError {
    pub(crate) fn new(value: Value, kind: JsonSchemaErrorKind) -> Self {
        Self { value, kind }
    }

    pub(crate) fn missing(value: Map<String, Value>, key: String) -> Self {
        Self {
            value: Value::Object(value),
            kind: JsonSchemaErrorKind::MissingMember(key),
        }
    }
}

#[derive(Debug)]
pub(crate) enum JsonSchemaErrorKind {
    UnexpectedKind(JsonKind),
    MissingMember(String),
    Other(String),
}

impl Display for JsonSchemaErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonSchemaErrorKind::UnexpectedKind(json_kind) => {
                write!(f, "expected a(n) '{json_kind}'")
            }
            JsonSchemaErrorKind::MissingMember(key) => write!(f, "expected member '{key}'"),
            JsonSchemaErrorKind::Other(message) => write!(f, "{message}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum JsonKind {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl Display for JsonKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool => write!(f, "bool"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Array => write!(f, "array"),
            Self::Object => write!(f, "object"),
        }
    }
}
