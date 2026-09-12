use crate::data::files::File;
use serde_json::{Map, Value};

#[derive(Debug)]
pub(crate) struct Object {
    pub(crate) group: String,
    pub(crate) location: String,
    pub(crate) properties: Map<String, Value>, // TODO: Fill up the type
}

impl Object {
    pub(crate) fn from_file(file: File) -> Vec<Self> {
        let group = file.group();
        let location = &file.location;
        let objects = file.objects();

        objects
            .iter()
            .map(|object| {
                let properties = object.as_object().unwrap().to_owned();
                Self {
                    group: group.to_owned(),
                    location: location.to_owned(),
                    properties,
                }
            })
            .collect()
    }

    pub(crate) fn from_files(files: Vec<File>) -> Vec<Self> {
        files
            .into_iter()
            .flat_map(|file| Self::from_file(file))
            .collect()
    }

    pub(crate) fn id(&self) -> &str {
        self.properties.get("id").unwrap().as_str().unwrap()
    }

    pub(crate) fn key(&self) -> Key {
        Key {
            group: self.group.to_owned(),
            id: self.id().to_owned(),
        }
    }
}

// Consider objects with same group and same id are game object and its
// localization object.
#[derive(Debug, Hash, Eq, PartialEq)]
pub(crate) struct Key {
    pub(crate) group: String,
    pub(crate) id: String,
}

// #[derive(Debug)]
// pub(crate) struct Objects {
//     groups: Vec<String>,
//     locations: Vec<String>,
//     properties: Vec<Map<String, Value>>,
// }

// impl<T: IntoIterator<Item = Object>> From<T> for Objects {
//     fn from(value: T) -> Self {
//         value.into_iter().fold(
//             Self {
//                 groups: Vec::new(),
//                 locations: Vec::new(),
//                 properties: Vec::new(),
//             },
//             |mut acc, object| {
//                 acc.groups.push(object.group);
//                 acc.locations.push(object.location);
//                 acc.properties.push(object.properties);
//                 acc
//             },
//         )
//     }
// }

// #[derive(Debug)]
// pub(crate) struct OptionObjects {
//     groups: Vec<Option<String>>,
//     locations: Vec<Option<String>>,
//     properties: Vec<Option<Map<String, Value>>>,
// }

// impl<T: IntoIterator<Item = Option<Object>>> From<T> for OptionObjects {
//     fn from(value: T) -> Self {
//         value.into_iter().fold(
//             Self {
//                 groups: Vec::new(),
//                 locations: Vec::new(),
//                 properties: Vec::new(),
//             },
//             |mut acc, object: Option<Object>| {
//                 if let Some(object) = object {
//                     acc.groups.push(Some(object.group));
//                     acc.locations.push(Some(object.location));
//                     acc.properties.push(Some(object.properties));
//                 } else {
//                     acc.groups.push(None);
//                     acc.locations.push(None);
//                     acc.properties.push(None);
//                 }
//                 acc
//             },
//         )
//     }
// }
