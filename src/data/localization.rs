use crate::data::object::{Key, Object};
use std::collections::HashMap;

macro_rules! localization_str {
    ($folder:literal, $code:literal) => {
        LocalizationStr {
            folder: $folder,
            code: $code,
        }
    };
}

pub(crate) static LOCALIZATION_COUNT: usize = 6;
pub(crate) static LOCALIZATION_STRS: [LocalizationStr; LOCALIZATION_COUNT] = [
    localization_str!("loc_de", "de"),
    localization_str!("loc_es", "es"),
    localization_str!("loc_fr", "fr"),
    localization_str!("loc_jp", "ja"),
    localization_str!("loc_ru", "ru"),
    localization_str!("loc_zh-hans", "zh-Hans"),
];

#[derive(Debug)]
pub(crate) struct LocalizationStr {
    pub(crate) folder: &'static str,
    #[allow(unused)]
    pub(crate) code: &'static str,
}

#[derive(Debug)]
pub(crate) struct LocalizedObject {
    pub(crate) core: Object,
    pub(crate) localizations: Localization,
}

impl LocalizedObject {
    pub(crate) fn from_objects(
        core: Vec<Object>,
        localizations: [Vec<Object>; LOCALIZATION_COUNT],
    ) -> HashMap<Key, LocalizedObject> {
        let mut core_map: HashMap<Key, Object> = HashMap::new();
        for object in core.into_iter() {
            let key = object.key();
            if let Some(object) = core_map.insert(key, object) {
                println!("warning: drop object due to repeated key: {object:?}");
            }
        }

        let mut localization_map: HashMap<Key, Localization> = HashMap::new();
        for (index, localization) in localizations.into_iter().enumerate() {
            for object in localization {
                let key = object.key();
                localization_map.entry(key).or_default()[index] = Some(object);
            }
        }

        let localization_objects = core_map
            .into_iter()
            .map(|(key, object)| {
                let localizations = localization_map.remove(&key).unwrap_or_default();
                let localized_object = Self {
                    core: object,
                    localizations: localizations,
                };
                (key, localized_object)
            })
            .collect();
        println!("warning: drop object due to no core object: {localization_map:?}");
        localization_objects
    }

    pub(crate) fn group(&self) -> &String {
        &self.core.group
    }

    pub(crate) fn location(&self) -> &String {
        &self.core.location
    }

    pub(crate) fn id(&self) -> &str {
        self.core.id()
    }

    pub(crate) fn icon(&self) -> Option<String> {
        match self.group().as_str() {
            "achievements" => {
                Some("".to_owned()) // data needed
            }
            "decks" => None,
            "elements" => {
                let content = &self.core.properties;
                let icon = content
                    .get("icon")
                    .and_then(|icon| icon.as_str())
                    .or(Some(&self.id()))
                    .unwrap();
                if content.get("isAspect").is_some() {
                    Some(format!("aspects/{icon}.png"))
                } else {
                    Some(format!("elements/{icon}.png"))
                }
            }
            "endings" => {
                let image = &self.core.properties.get("image").unwrap().as_str().unwrap();
                Some(format!("endings/{image}.png"))
            }
            "legacies" => {
                let image = &self.core.properties.get("image").unwrap().as_str().unwrap();
                Some(format!("legacies/{image}.png"))
            }
            "recipes" => None,
            "verbs" => Some(format!("verbs/{}.png", self.id())),
            "cultures" => None,
            "dicta" => None,
            "portals" => {
                todo!() // data needed
            }
            _ => unreachable!(),
        }
    }
}

type Localization = [Option<Object>; LOCALIZATION_COUNT];
