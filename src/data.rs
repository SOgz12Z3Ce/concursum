pub(crate) mod cs;

use crate::data::cs::{
    files::{self, File},
    localization::{self, LocalizedObject},
    object::{Key, Object},
};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub(crate) fn load<P: AsRef<Path>>(base_dir: P) -> Data {
    let base_path = base_dir.as_ref();
    let base = base_path.join("content/cs");
    let core_files = files::load(base.join("core"));
    let localization_files: [Vec<File>; localization::LOCALIZATION_COUNT] =
        localization::LOCALIZATION_STRS
            .iter()
            .map(|localization_str| files::load(base.join(localization_str.folder)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    let core_objects = Object::from_files(core_files);
    let localization_objects: [Vec<Object>; localization::LOCALIZATION_COUNT] = localization_files
        .into_iter()
        .map(|files| Object::from_files(files))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let localized_objects = LocalizedObject::from_objects(core_objects, localization_objects);
    let mut localized_objects: Vec<(Key, LocalizedObject)> =
        localized_objects.into_iter().collect();
    localized_objects.sort_by(|(_, a), (_, b)| {
        a.group()
            .cmp(b.group())
            .then_with(|| a.location().cmp(b.location()))
    });
    let (index, localized_objects): (HashMap<Key, usize>, Vec<LocalizedObject>) = localized_objects
        .into_iter()
        .enumerate()
        .map(|(index, (key, object))| ((key, index), object))
        .collect();
    let mut groups: Vec<String> = localized_objects
        .iter()
        .map(|object| object.group().to_owned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    groups.sort();
    let mut files: Vec<String> = localized_objects
        .iter()
        .map(|object| object.location().to_owned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    files.sort();
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
}

#[derive(Debug, Clone)]
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
            let mut cur_labels = Vec::new();
            let mut cur_descriptions = Vec::new();

            let core = &object.core;
            let texts = core.texts();
            cur_labels.extend(texts.labels);
            cur_descriptions.extend(texts.descriptions);

            if let Some(object) = &object.localizations[5] {
                let texts = object.texts();
                cur_labels.extend(texts.labels);
                cur_descriptions.extend(texts.descriptions);
            };

            labels.push(cur_labels);
            descriptions.push(cur_descriptions);
        }

        Texts {
            labels,
            descriptions,
        }
    }
}
