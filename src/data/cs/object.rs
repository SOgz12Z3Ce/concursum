use std::str::FromStr;

use crate::{
    data::cs::{file::File, text::Text},
    error::Error,
};
use serde_json::{Map, Value};

#[derive(Debug)]
pub(crate) struct Object<'a> {
    file: &'a File,
    properties: &'a Map<String, Value>,
}

impl<'a> Object<'a> {
    pub(crate) fn group(&self) -> Result<Group, Error> {
        self.file.group()?.parse()
    }

    pub(crate) fn location(&self) -> &String {
        self.file.location()
    }

    pub(crate) fn properties(&self) -> &Map<String, Value> {
        self.properties
    }

    pub(crate) fn id(&self) -> Result<&str, Error> {
        let properties = self.properties();
        let Some(id) = properties.get("id") else {
            return Err(Error::JsonSchema {
                value: Value::Object(properties.to_owned()),
                message: String::from("expected object has an \"id\" member"),
            });
        };
        id.as_str().ok_or_else(|| Error::JsonSchema {
            value: id.to_owned(),
            message: String::from("expected value of \"id\" member is a string"),
        })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum Group {
    Achievement,
    Culture,
    Deck,
    Dictum,
    Element,
    Ending,
    Legacy,
    Lever,
    Portal,
    Recipe,
    Setting,
    Verb,
}

impl FromStr for Group {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "achievements" => Ok(Self::Achievement),
            "cultures" => Ok(Self::Culture),
            "decks" => Ok(Self::Deck),
            "dicta" => Ok(Self::Dictum),
            "elements" => Ok(Self::Element),
            "endings" => Ok(Self::Ending),
            "legacies" => Ok(Self::Legacy),
            "levers" => Ok(Self::Lever),
            "portals" => Ok(Self::Portal),
            "recipes" => Ok(Self::Recipe),
            "settings" => Ok(Self::Setting),
            "verbs" => Ok(Self::Verb),
            _ => Err(Error::Group(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct Key<'a> {
    group: Group,
    id: &'a str,
}

impl<'a> Key<'a> {
    pub(crate) fn group(&self) -> Group {
        self.group
    }

    pub(crate) fn id(&self) -> &str {
        self.id
    }
}

// impl Object {
//     pub(crate) fn from_file(file: File) -> Vec<Self> {
//         let group = file.group();
//         let location = &file.location;
//         let objects = file.objects();

//         objects
//             .iter()
//             .map(|object| {
//                 let properties = object.as_object().unwrap().to_owned();
//                 Self {
//                     group: group.to_owned(),
//                     location: location.to_owned(),
//                     properties,
//                 }
//             })
//             .collect()
//     }

//     pub(crate) fn from_files(files: Vec<File>) -> Vec<Self> {
//         files
//             .into_iter()
//             .flat_map(|file| Self::from_file(file))
//             .collect()
//     }

//     pub(crate) fn id(&self) -> &str {
//         self.properties.get("id").unwrap().as_str().unwrap()
//     }

//     pub(crate) fn key(&self) -> Key {
//         Key {
//             group: self.group.to_owned(),
//             id: self.id().to_owned(),
//         }
//     }

//     pub(crate) fn texts<'a>(&'a self) -> Text<'a> {
//         let Text {
//             mut labels,
//             mut descriptions,
//         } = Text::default();

//         Text {
//             labels,
//             descriptions,
//         }
//     }
// }

// // Consider objects with same group and same id are game object and its
// // localization object.
// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub(crate) struct Key {
//     pub(crate) group: String,
//     pub(crate) id: String,
// }

// #[derive(Debug, Default)]
// pub(crate) struct Text<'a> {
//     pub(crate) labels: Vec<&'a str>,
//     pub(crate) descriptions: Vec<&'a str>,
// }

// #[derive(Debug)]
// pub(crate) struct Texts<'a> {
//     pub(crate) labels: Vec<Vec<&'a str>>,
//     pub(crate) descriptions: Vec<Vec<&'a str>>,
// }

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
