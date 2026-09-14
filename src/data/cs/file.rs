use crate::error::Error;
use serde_json::{Map, Value};
use std::{fmt::Debug, fs, path::Path};
use walkdir::WalkDir;

#[derive(Debug)]
pub(crate) struct File {
    location: String,
    content: Value,
}

impl File {
    pub(crate) fn load<P: AsRef<Path>>(root: P) -> Result<Vec<File>, Error> {
        let root = root.as_ref();
        let mut files = Vec::new();
        for entry in WalkDir::new(root) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let entry_path = entry.into_path();
            let location = locate(&entry_path, root)?;
            let content = read(&entry_path)?;

            let file = File { location, content };
            files.push(file);
        }
        Ok(files)
    }

    pub(crate) fn group(&self) -> Result<&String, Error> {
        let (group, _) = self.root_member()?;
        Ok(group)
    }

    pub(crate) fn objects(&self) -> Result<&Vec<Value>, Error> {
        let (_, objects) = self.root_member()?;
        objects.as_array().ok_or_else(|| Error::JsonSchema {
            value: objects.to_owned(),
            message: "expected value of file root object is an array".to_owned(),
        })
    }

    fn root_member(&self) -> Result<(&String, &Value), Error> {
        let root = self.root()?;
        let Some((group, objects)) = root.iter().next() else {
            return Err(Error::JsonSchema {
                value: Value::Object(root.to_owned()),
                message: "expected file root object has one name/value pair".to_owned(),
            });
        };
        Ok((group, objects))
    }

    fn root(&self) -> Result<&Map<String, Value>, Error> {
        self.content.as_object().ok_or_else(|| Error::JsonSchema {
            value: self.content.to_owned(),
            message: "expected file root is an object".to_owned(),
        })
    }
}

fn locate<P, B>(path: P, base: B) -> Result<String, Error>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let base = base.as_ref();

    let Some(diff) = pathdiff::diff_paths(path, base) else {
        return Err(Error::UnrelatedPath {
            path: path.to_owned(),
            base: base.to_owned(),
        });
    };
    diff.into_os_string()
        .into_string()
        .map_err(Error::NonUtf8Path)
}

fn read<P: AsRef<Path>>(path: P) -> Result<Value, Error> {
    let path = path.as_ref();
    let content = fs::read(path)?;
    serde_json::from_slice(&content).map_err(|error| Error::Deserialization {
        path: path.to_owned(),
        source: error,
    })
}
