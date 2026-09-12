mod files;
pub(crate) mod localization;
pub(crate) mod object;

use crate::data::{
    files::File,
    localization::LocalizedObject,
    object::{Key, Object},
};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::LazyLock,
};

pub(crate) static DATA: LazyLock<Data> = LazyLock::new(|| {
    let base = Path::new("content/cs");
    let core_files = files::load(base.join("core"));
    let localization_files: [Vec<File>; localization::LOCALIZATION_COUNT] =
        localization::LOCALIZATION_STRS
            .iter()
            .map(|localization_str| files::load(base.join(localization_str.folder)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

    let core_objects = {
        let mut objects = Object::from_files(core_files);
        objects.sort_by(|a, b| {
            a.group
                .cmp(&b.group)
                .then_with(|| a.location.cmp(&b.location))
        });
        objects
    };
    let localization_objects: [Vec<Object>; localization::LOCALIZATION_COUNT] = localization_files
        .into_iter()
        .map(|files| Object::from_files(files))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let localized_objects = LocalizedObject::from_objects(core_objects, localization_objects);

    let (index, localized_objects): (HashMap<Key, usize>, Vec<LocalizedObject>) = localized_objects
        .into_iter()
        .enumerate()
        .map(|(index, (key, object))| ((key, index), object))
        .collect();
    let groups: Vec<String> = localized_objects
        .iter()
        .map(|object| object.group().to_owned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let files: Vec<String> = localized_objects
        .iter()
        .map(|object| object.location().to_owned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let mut group_files: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut file_objects: HashMap<String, Vec<usize>> = HashMap::new();
    for (index, object) in localized_objects.iter().enumerate() {
        let group = object.group();
        let file = object.location();

        group_files
            .entry(group.to_owned())
            .or_default()
            .insert(files.iter().position(|f| f == file).unwrap());
        file_objects.entry(file.to_owned()).or_default().push(index);
    }

    let group_files = group_files
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().collect::<Vec<_>>()))
        .collect();

    Data {
        index,
        localized_objects,
        groups,
        files,
        group_files,
        file_objects,
    }
});

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) index: HashMap<Key, usize>,

    pub(crate) localized_objects: Vec<LocalizedObject>,
    pub(crate) groups: Vec<String>,
    pub(crate) files: Vec<String>,

    pub(crate) group_files: HashMap<String, Vec<usize>>,
    pub(crate) file_objects: HashMap<String, Vec<usize>>,
}

impl Data {
    pub(crate) fn object(&self, key: &Key) -> Option<&LocalizedObject> {
        let index = self.index.get(&key);
        index.and_then(|index| Some(&self.localized_objects[*index]))
    }

    pub(crate) fn texts<'a>(&'a self) -> Texts<'a> {
        let Texts {
            mut labels,
            mut descriptions,
        };
        labels = Vec::new();
        descriptions = Vec::new();

        for object in &self.localized_objects {
            let Some(object) = &object.localizations[5] else {
                // ZH only for now
                continue;
            };
            labels.push(
                object
                    .properties
                    .get("label")
                    .and_then(|label| label.as_str()),
            );
            descriptions.push(
                object
                    .properties
                    .get("description")
                    .and_then(|label| label.as_str()),
            );
        }

        Texts {
            labels,
            descriptions,
        }
    }
}

// TODO: Add more fileds.
#[derive(Debug)]
pub(crate) struct Texts<'a> {
    pub(crate) labels: Vec<Option<&'a str>>,
    pub(crate) descriptions: Vec<Option<&'a str>>,
}
