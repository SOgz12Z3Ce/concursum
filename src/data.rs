pub(crate) mod cs;

use crate::error::{JsonKind, JsonSchemaError, JsonSchemaErrorKind};
use serde_json::{Map, Value};

pub(crate) trait JsonValueExt {
    #[allow(unused)] // No need to read such type now.
    fn try_as_null(&self) -> Result<(), JsonSchemaError>;

    fn try_as_bool(&self) -> Result<bool, JsonSchemaError>;

    #[allow(unused)] // No need to read such type now.
    fn try_as_i64(&self) -> Result<i64, JsonSchemaError>;

    #[allow(unused)] // No need to read such type now.
    fn try_as_u64(&self) -> Result<u64, JsonSchemaError>;

    #[allow(unused)] // No need to read such type now.
    fn try_as_f64(&self) -> Result<f64, JsonSchemaError>;

    fn try_as_str(&self) -> Result<&str, JsonSchemaError>;

    fn try_as_array(&self) -> Result<&Vec<Value>, JsonSchemaError>;

    fn try_as_object(&self) -> Result<&Map<String, Value>, JsonSchemaError>;
}

impl JsonValueExt for Value {
    fn try_as_null(&self) -> Result<(), JsonSchemaError> {
        self.as_null().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::Null),
            )
        })
    }

    fn try_as_bool(&self) -> Result<bool, JsonSchemaError> {
        self.as_bool().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::Bool),
            )
        })
    }

    fn try_as_i64(&self) -> Result<i64, JsonSchemaError> {
        self.as_i64().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::Number),
            )
        })
    }

    fn try_as_u64(&self) -> Result<u64, JsonSchemaError> {
        self.as_u64().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::Number),
            )
        })
    }

    fn try_as_f64(&self) -> Result<f64, JsonSchemaError> {
        self.as_f64().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::Number),
            )
        })
    }

    fn try_as_str(&self) -> Result<&str, JsonSchemaError> {
        self.as_str().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::String),
            )
        })
    }

    fn try_as_array(&self) -> Result<&Vec<Value>, JsonSchemaError> {
        self.as_array().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::Array),
            )
        })
    }

    fn try_as_object(&self) -> Result<&Map<String, Value>, JsonSchemaError> {
        self.as_object().ok_or_else(|| {
            JsonSchemaError::new(
                self.to_owned(),
                JsonSchemaErrorKind::UnexpectedKind(JsonKind::Object),
            )
        })
    }
}

pub(crate) trait JsonObjectExt {
    fn try_get(&self, key: &str) -> Result<&Value, JsonSchemaError>;
}

impl JsonObjectExt for Map<String, Value> {
    fn try_get(&self, key: &str) -> Result<&Value, JsonSchemaError> {
        self.get(key)
            .ok_or(JsonSchemaError::missing(self.to_owned(), key.to_owned()))
    }
}
