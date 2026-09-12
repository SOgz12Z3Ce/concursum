use serde_json::{Map, Value};
use std::{fs, path::Path};
use walkdir::WalkDir;

#[derive(Debug)]
pub(crate) struct File {
    pub(crate) location: String,
    pub(crate) content: Value,
}

impl File {
    pub(crate) fn group(&self) -> &String {
        let (group, _) = self.root().iter().next().unwrap();
        group
    }

    pub(crate) fn objects(&self) -> &Vec<Value> {
        let (_, objects) = self.root().iter().next().unwrap();
        objects.as_array().unwrap()
    }

    fn root(&self) -> &Map<String, Value> {
        self.content.as_object().unwrap()
    }
}

pub(crate) fn load<P: AsRef<Path>>(root: P) -> Vec<File> {
    WalkDir::new(root.as_ref())
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .map(|path| File {
            location: pathdiff::diff_paths(&path, root.as_ref())
                .unwrap()
                .into_string()
                .unwrap(),
            content: read(&path),
        })
        .collect()
}

fn read<P: AsRef<Path>>(path: P) -> Value {
    let file = fs::read(&path).unwrap();
    serde_json::from_slice(&file).unwrap()
}
