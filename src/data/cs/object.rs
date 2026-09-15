use std::{fmt::Display, str::FromStr};

use crate::{data::cs::file::File, error::Error};
use serde_json::{Map, Value};

#[derive(Debug)]
pub(crate) struct Object<'a> {
    file: &'a File,
    properties: &'a Map<String, Value>,
}

impl<'a> Object<'a> {
    pub(crate) fn from_file(file: &File) -> Result<Vec<Object>, Error> {
        file.objects()?
            .iter()
            .map(|properties| {
                let Some(properties) = properties.as_object() else {
                    return Err(Error::JsonSchema {
                        value: properties.to_owned(),
                        message: String::from("expected an object"),
                    });
                };
                Ok(Object { file, properties })
            })
            .collect()
    }

    pub(crate) fn group(&self) -> Result<Group, Error> {
        self.file.group()?.parse()
    }

    pub(crate) fn location(&self) -> &String {
        self.file.location()
    }

    pub(crate) fn key(&self) -> Result<Key, Error> {
        Ok(Key::new(self.group()?, self.id()?))
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
    Achievements,
    Cultures,
    Decks,
    Dicta,
    Elements,
    Endings,
    Legacies,
    Levers,
    Portals,
    Recipes,
    Settings,
    Verbs,
}

impl FromStr for Group {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "achievements" => Ok(Self::Achievements),
            "cultures" => Ok(Self::Cultures),
            "decks" => Ok(Self::Decks),
            "dicta" => Ok(Self::Dicta),
            "elements" => Ok(Self::Elements),
            "endings" => Ok(Self::Endings),
            "legacies" => Ok(Self::Legacies),
            "levers" => Ok(Self::Levers),
            "portals" => Ok(Self::Portals),
            "recipes" => Ok(Self::Recipes),
            "settings" => Ok(Self::Settings),
            "verbs" => Ok(Self::Verbs),
            _ => Err(Error::Group(s.to_owned())),
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Achievements => write!(f, "achievements"),
            Self::Cultures => write!(f, "cultures"),
            Self::Decks => write!(f, "decks"),
            Self::Dicta => write!(f, "dicta"),
            Self::Elements => write!(f, "elements"),
            Self::Endings => write!(f, "endings"),
            Self::Legacies => write!(f, "legacies"),
            Self::Levers => write!(f, "levers"),
            Self::Portals => write!(f, "portals"),
            Self::Recipes => write!(f, "recipes"),
            Self::Settings => write!(f, "settings"),
            Self::Verbs => write!(f, "verbs"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct Key<'a> {
    group: Group,
    id: &'a str,
}

impl<'a> Key<'a> {
    pub(crate) fn new(group: Group, id: &str) -> Key {
        Key { group, id }
    }

    pub(crate) fn group(&self) -> Group {
        self.group
    }

    pub(crate) fn id(&self) -> &str {
        self.id
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct OwnedKey {
    group: Group,
    id: String,
}

impl<'a> From<Key<'a>> for OwnedKey {
    fn from(value: Key<'a>) -> Self {
        Self {
            group: value.group,
            id: value.id.to_owned(),
        }
    }
}

impl Display for OwnedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.group, self.id)
    }
}
