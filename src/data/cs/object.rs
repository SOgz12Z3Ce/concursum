use crate::{
    data::{
        JsonObjectExt, JsonValueExt,
        cs::{file::File, group::Group},
    },
    error::{Error, JsonSchemaError},
};
use serde_json::{Map, Value};
use std::{fmt::Display, vec::IntoIter};

#[derive(Debug)]
pub(crate) struct Object<'a> {
    file: &'a File,
    properties: &'a Map<String, Value>,
}

impl<'a> Object<'a> {
    pub(crate) fn from_file(file: &'a File) -> Result<Vec<Self>, JsonSchemaError> {
        file.objects()?
            .iter()
            .map(|properties| {
                Ok(Self {
                    file,
                    properties: properties.try_as_object()?,
                })
            })
            .collect()
    }

    pub(crate) fn from_files(files: &'a [File]) -> Result<Vec<Self>, JsonSchemaError> {
        files.iter().try_fold(Vec::new(), |mut acc, file| {
            acc.extend(Self::from_file(file)?);
            Ok(acc)
        })
    }

    pub(crate) fn group(&self) -> Result<Group, Error> {
        self.file.group()?.parse()
    }

    pub(crate) fn key(&self) -> Result<Key<'a>, Error> {
        Ok(Key::new(self.group()?, self.id()?))
    }

    pub(crate) fn properties(&self) -> &'a Map<String, Value> {
        self.properties
    }

    pub(crate) fn id(&self) -> Result<&'a str, JsonSchemaError> {
        self.properties.try_get("id")?.try_as_str()
    }

    pub(crate) fn icon(&self) -> Result<(Option<String>, Option<&'static str>), Error> {
        let properties = self.properties;
        let (icon, fallback) = match self.group()? {
            Group::Achievements => {
                let icon = properties
                    .get("iconUnlocked")
                    .map(|value| value.try_as_str())
                    .transpose()?
                    .map(|icon| format!("aspects/{icon}.png"));
                let fallback = Some("aspects/_x.png");
                (icon, fallback)
            }
            Group::Cultures => (None, None),
            Group::Decks => (None, None),
            Group::Dicta => (None, None),
            Group::Elements => {
                let is_aspect = properties
                    .get("isAspect")
                    .map(|value| value.try_as_bool())
                    .transpose()?
                    .unwrap_or(false);
                let (prefix, fallback) = match is_aspect {
                    true => ("aspects", "aspects/_x.png"),
                    false => ("elements", "elements/_x.png"),
                };
                let icon = properties
                    .get("icon")
                    .map(|value| value.try_as_str())
                    .unwrap_or_else(|| self.id())?;

                let icon = Some(format!("{prefix}/{icon}.png"));
                let fallback = Some(fallback);
                (icon, fallback)
            }
            Group::Endings => {
                let icon = properties
                    .get("image")
                    .map(|value| value.try_as_str())
                    .transpose()?
                    .map(|icon| format!("endings/{icon}.png"));
                (icon, None)
            }
            Group::Legacies => {
                // TODO: $derive goes here.
                let icon = properties
                    .get("image")
                    .map(|value| value.try_as_str())
                    .transpose()?
                    .map(|icon| format!("legacies/{icon}.png"));
                (icon, None)
            }
            Group::Levers => (None, None),
            Group::Portals => (None, None), // TODO: The images are Texture2D resources.
            Group::Recipes => (None, None),
            Group::Settings => (None, None),
            Group::Verbs => (
                Some(format!("verbs/{}.png", self.id()?)),
                Some("verbs/_x.png"),
            ),
        };
        Ok((
            icon.or_else(|| fallback.map(|path| String::from(path))),
            fallback,
        ))
    }
}

#[derive(Debug)]
pub(crate) struct SortedObjects<'a> {
    objects: Vec<Object<'a>>,
}

impl<'a> SortedObjects<'a> {
    pub(crate) fn new<T: IntoIterator<Item = Object<'a>>>(objects: T) -> Result<Self, Error> {
        let mut objects: Vec<Object> = objects.into_iter().collect();
        let mut indices: Vec<usize> = {
            let mut key_indices = objects
                .iter()
                .enumerate()
                .map(|(index, object)| Ok((index, object.key()?)))
                .collect::<Result<Vec<_>, Error>>()?;
            key_indices.sort_by(|(_, a), (_, b)| a.cmp(b));
            key_indices.into_iter().map(|(idx, _)| idx).collect()
        };
        for i in 0..objects.len() {
            while indices[i] != i {
                let dest = indices[i];
                objects.swap(i, dest);
                indices.swap(i, dest);
            }
        }
        Ok(Self { objects })
    }
}

impl<'a> IntoIterator for SortedObjects<'a> {
    type Item = Object<'a>;

    type IntoIter = IntoIter<Object<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.into_iter()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Key<'a> {
    group: Group,
    id: &'a str,
}

impl<'a> Key<'a> {
    pub(crate) fn new(group: Group, id: &'a str) -> Self {
        Self { group, id }
    }

    pub(crate) fn group(&self) -> Group {
        self.group
    }
}

impl<'a> Display for Key<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.group, self.id)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
