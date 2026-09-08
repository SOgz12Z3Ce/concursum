use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::BufReader,
    iter,
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub(crate) static DATA: LazyLock<Data> = LazyLock::new(load);

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) files: Vec<String>,
    pub(crate) groups: Vec<String>,
    pub(crate) objects: Vec<Object>,

    pub(crate) group_files: HashMap<String, Vec<usize>>,
    pub(crate) file_objects: HashMap<String, Vec<usize>>,
}

#[derive(Debug)]
pub(crate) struct Object {
    pub(crate) group: String, // TODO: Cow this.
    pub(crate) file: String,  // TODO: Cow this.
    pub(crate) id: String,
    pub(crate) content: Value, // TODO: Fill this type.
}

fn load() -> Data {
    // TODO: carefully process these JSONs.
    // There is no guarantee that different localization use same file tree
    // and one folder just contains exact one group of objects.
    let base = Path::new("content/loc_zh-hans");
    let dirs = {
        let mut buffer: Vec<PathBuf> = fs::read_dir(base)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        buffer.sort();
        buffer
    };
    let files = {
        let mut buffer = vec![];
        for dir in dirs {
            let dir = fs::read_dir(dir).unwrap();
            let mut files: Vec<PathBuf> = dir.into_iter().map(|e| e.unwrap().path()).collect();
            files.sort();
            buffer.extend(files);
        }
        buffer
    };
    let file_names: Vec<String> = files
        .iter()
        .map(|f| f.file_name().unwrap().to_string_lossy().into_owned())
        .collect();

    let files: Vec<(String, Value)> = iter::zip(file_names.clone(), files)
        .map(|(n, f)| {
            let file = File::open(f).unwrap();
            let reader = BufReader::new(file);
            let content: Value = serde_json::from_reader(reader).unwrap();
            (n, content)
        })
        .collect();
    let groups = {
        let mut buffer: Vec<String> = files
            .iter()
            .map(|(_, f)| {
                let root = f.as_object().unwrap();
                let (group, _) = root.iter().next().unwrap();
                group.to_owned()
            })
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        buffer.sort();
        buffer
    };
    let group_files = {
        let mut map: HashMap<String, Vec<usize>> = HashMap::new();
        for (index, (_, file)) in files.iter().enumerate() {
            let root = file.as_object().unwrap();
            let (group, _) = root.iter().next().unwrap();
            map.entry(group.to_owned()).or_default().push(index);
        }
        map
    };

    let objects: Vec<Object> = files
        .iter()
        .flat_map(|(n, f)| {
            let root = f.as_object().unwrap();
            let (group, objects) = root.iter().next().unwrap();
            objects.as_array().unwrap().iter().map(|o| {
                let id = o.as_object().unwrap()["id"].as_str().unwrap().to_owned();
                Object {
                    group: group.to_owned(),
                    file: n.to_owned(),
                    id,
                    content: o.to_owned(),
                }
            })
        })
        .collect();
    let file_objects = {
        let mut map: HashMap<String, Vec<usize>> = HashMap::new();
        for (index, object) in objects.iter().enumerate() {
            let file = &object.file;
            map.entry(file.to_owned()).or_default().push(index);
        }
        map
    };

    Data {
        files: file_names,
        groups,
        objects,
        group_files,
        file_objects,
    }
}
