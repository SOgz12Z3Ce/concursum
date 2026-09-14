use crate::data::files::File;
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
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

    pub(crate) fn texts<'a>(&'a self) -> Text<'a> {
        let Text {
            mut labels,
            mut descriptions,
        } = Text::default();

        // Labels
        // General label
        if let Some(label) = self.properties.get("label") {
            let label = label.as_str().unwrap();
            labels.push(label);
        }

        // Slot label
        if let Some(label) = self
            .properties
            .get("slot")
            .and_then(|slot| slot.as_object().unwrap().get("label"))
        {
            let label = label.as_str().unwrap();
            labels.push(label);
        }

        // Slots label
        if let Some(slots) = self
            .properties
            .get("slots")
            .and_then(|slots| slots.as_array())
        {
            for slot in slots {
                let Some(label) = slot.as_object().unwrap().get("label") else {
                    continue;
                };
                let label = label.as_str().unwrap();
                labels.push(label);
            }
        }

        // Internal deck label
        if let Some(label) = self
            .properties
            .get("internaldeck")
            .and_then(|internal_deck| internal_deck.as_object().unwrap().get("label"))
        {
            let label = label.as_str().unwrap();
            labels.push(label);
        }

        // Recipes label
        if let Some(alts) = self.properties.get("alt") {
            let alts = alts.as_array().unwrap();
            for alt in alts {
                let alt = alt.as_object().unwrap();
                if let Some(label) = alt.get("label") {
                    let label = label.as_str().unwrap();
                    labels.push(label);
                }
            }
        }
        if let Some(linkeds) = self.properties.get("linked") {
            let linkeds = linkeds.as_array().unwrap();
            for linked in linkeds {
                let linked = linked.as_object().unwrap();
                if let Some(label) = linked.get("label") {
                    let label = label.as_str().unwrap();
                    labels.push(label);
                }
            }
        }

        // Descriptions
        // General description
        if let Some(description) = self.properties.get("description") {
            let description = description.as_str().unwrap();
            descriptions.push(description);
        }
        
        // Recipe start description
        if let Some(description) = self.properties.get("startdescription") {
            let description = description.as_str().unwrap();
            descriptions.push(description);
        }

        // Achievement description
        if let Some(description) = self.properties.get("descriptionunlocked") {
            let description = description.as_str().unwrap();
            descriptions.push(description);
        }

        // Internal deck description
        if let Some(description) = self
            .properties
            .get("internaldeck")
            .and_then(|internal_deck| internal_deck.as_object().unwrap().get("description"))
        {
            let description = description.as_str().unwrap();
            descriptions.push(description);
        }
        
        // Slot description
        if let Some(description) = self
            .properties
            .get("slot")
            .and_then(|slot| slot.as_object().unwrap().get("description"))
        {
            let description = description.as_str().unwrap();
            descriptions.push(description);
        }

        // Slots description
        if let Some(slots) = self
            .properties
            .get("slots")
            .and_then(|slots| slots.as_array())
        {
            for slot in slots {
                let Some(description) = slot.as_object().unwrap().get("description") else {
                    continue;
                };
                let description = description.as_str().unwrap();
                descriptions.push(description);
            }
        }
        
        // Recipes description
        if let Some(alts) = self.properties.get("alt") {
            let alts = alts.as_array().unwrap();
            for alt in alts {
                let alt = alt.as_object().unwrap();
                if let Some(description) = alt.get("description") {
                    let description = description.as_str().unwrap();
                    descriptions.push(description);
                }
                if let Some(description) = alt.get("startdescription") {
                    let description = description.as_str().unwrap();
                    descriptions.push(description);
                }
            }
        }
        if let Some(linkeds) = self.properties.get("linked") {
            let linkeds = linkeds.as_array().unwrap();
            for linked in linkeds {
                let linked = linked.as_object().unwrap();
                if let Some(description) = linked.get("startdescription") {
                    let description = description.as_str().unwrap();
                    descriptions.push(description);
                }
            }
        }
        
        // Draw message description
        if let Some(draw_messages) = self.properties.get("drawmessages") {
            let draw_messages = draw_messages.as_object().unwrap();
            for (_, message) in draw_messages {
                descriptions.push(message.as_str().unwrap());
            }
        }

        Text {
            labels,
            descriptions,
        }
    }
}

// Consider objects with same group and same id are game object and its
// localization object.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Key {
    pub(crate) group: String,
    pub(crate) id: String,
}

#[derive(Debug, Default)]
pub(crate) struct Text<'a> {
    pub(crate) labels: Vec<&'a str>,
    pub(crate) descriptions: Vec<&'a str>,
}

#[derive(Debug)]
pub(crate) struct Texts<'a> {
    pub(crate) labels: Vec<Vec<&'a str>>,
    pub(crate) descriptions: Vec<Vec<&'a str>>,
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
