use crate::{
    data::cs::object::{Key, Object},
    error::Error,
};
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
    #[allow(unused)] // TODO: Reserved for future usage.
    pub(crate) code: &'static str,
}

#[derive(Debug)]
pub(crate) struct LocalizedObject<'a> {
    pub(crate) core: Object<'a>,
    pub(crate) localizations: Localization<'a>,
}

impl<'a> LocalizedObject<'a> {
    pub(crate) fn from_objects(
        cores: Vec<Object<'a>>,
        localizations: Localization<'a>,
    ) -> Result<HashMap<Key<'a>, Self>, Error> {
        let cores = {
            let mut map = HashMap::new();
            for core in cores {
                if let Some(object) = map.insert(core.key()?, core) {
                    println!("warning: drop object due to repeated key: {object:?}");
                }
            }
            map
        };

        let localizations = {
            let mut map: HashMap<Key, [Option<Object>; LOCALIZATION_COUNT]> = HashMap::new();
            for (index, localizations) in localizations.into_iter().enumerate() {
                for localization in localizations {
                    map.entry(localization.key()?).or_default()[index] = Some(localization);
                }
            }
            map
        };

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
                let Some(image) = &self.core.properties.get("image") else {
                    return None;
                };
                let image = image.as_str().unwrap();
                Some(format!("legacies/{image}.png"))
            }
            "legcies" => None,
            "levers" => None,
            "portals" => {
                let image = &self.core.properties.get("icon").unwrap().as_str().unwrap();
                Some(format!("portals/{image}.png")) // where is it?
            }
            "recipes" => None,
            "settings" => None,
            "verbs" => Some(format!("verbs/{}.png", self.id())),
            "cultures" => None,
            "dicta" => None,
            _ => unreachable!(),
        }
    }
}

type Localization<'a> = [Vec<Object<'a>>; LOCALIZATION_COUNT];
