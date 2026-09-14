use crate::{
    data::{JsonValueExt, cs::group::Group},
    error::{Error, JsonSchemaError, JsonSchemaErrorKind},
};
use serde_json::{Map, Value};
use std::{
    fmt::{Debug, Display},
    fs,
    ops::Deref,
    path::Path,
};
use walkdir::WalkDir;

#[derive(Debug)]
pub(crate) struct File {
    location: Location,
    content: Value,
}

impl File {
    pub(crate) fn load<P: AsRef<Path>>(root: P) -> Result<Vec<Self>, Error> {
        let root = root.as_ref();
        let mut files = Vec::new();
        for entry in WalkDir::new(root) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let entry_path = entry.into_path();
            let location = Location::new(&entry_path, root)?;
            let content = Self::read(&entry_path)?;

            let file = Self { location, content };
            files.push(file);
        }
        Ok(files)
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }

    pub(crate) fn group(&self) -> Result<&String, JsonSchemaError> {
        let (group, _) = self.root_member()?;
        Ok(group)
    }

    pub(crate) fn objects(&self) -> Result<&Vec<Value>, JsonSchemaError> {
        let (_, objects) = self.root_member()?;
        objects.try_as_array()
    }

    fn root_member(&self) -> Result<(&String, &Value), JsonSchemaError> {
        let root = self.root()?;
        if root.len() != 1 {
            return Err(JsonSchemaError::new(
                Value::Object(root.to_owned()),
                JsonSchemaErrorKind::Other(String::from(
                    "expected root object has exactly one member",
                )),
            ));
        }
        Ok(root
            .iter()
            .next()
            .expect("root object has been checked to have exactly one member"))
    }

    fn root(&self) -> Result<&Map<String, Value>, JsonSchemaError> {
        self.content.try_as_object()
    }

    fn read<P: AsRef<Path>>(path: P) -> Result<Value, Error> {
        let path = path.as_ref();
        let content = fs::read(path)?;
        serde_json::from_slice(&content).map_err(|error| Error::Deserialization {
            path: path.to_owned(),
            source: error,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Location(String);

impl Location {
    fn new<P, B>(path: P, base: B) -> Result<Self, Error>
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
            .map(Self)
            .map_err(Error::NonUtf8Path)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub(crate) struct SortedFiles {
    files: Vec<File>,
}

impl Deref for SortedFiles {
    type Target = [File];

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl SortedFiles {
    pub(crate) fn new<T: IntoIterator<Item = File>>(files: T) -> Result<Self, Error> {
        let mut files = files
            .into_iter()
            .map(|file| Ok((file.group()?.parse()?, file)))
            .collect::<Result<Vec<(Group, File)>, Error>>()?;
        files.sort_by(|(a_group, a_file), (b_group, b_file)| {
            a_group
                .cmp(b_group)
                .then(a_file.location.cmp(&b_file.location))
        });
        let files = files.into_iter().map(|(_, file)| file).collect();
        Ok(Self { files })
    }
}
