use std::str::FromStr;

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
