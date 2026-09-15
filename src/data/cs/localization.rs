use crate::{
    data::cs::object::{Group, Key, Object, OwnedKey},
    error::Error,
};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct LocalizedObject<'a> {
    core: Object<'a>,
    localization: Localization<'a>,
}

impl<'a> LocalizedObject<'a> {
    fn new(core: Object<'a>, localization: Localization<'a>) -> Self {
        Self { core, localization }
    }

    pub(crate) fn from_objects(
        cores: Vec<Object<'a>>,
        localizations: Vec<LocalizationObject<'a>>,
    ) -> Result<Vec<Self>, Error> {
        let mut key_localizations: HashMap<OwnedKey, Localization> = HashMap::new();
        for localization in localizations {
            key_localizations
                .entry(localization.key()?.into())
                .or_default()
                .add(localization)?;
        }

        let localized_objects = cores
            .into_iter()
            .map(|core| {
                let localization = key_localizations
                    .remove(&core.key()?.into())
                    .unwrap_or_default();
                Ok(Self { core, localization })
            })
            .collect::<Result<_, Error>>()?;
        if !key_localizations.is_empty() {
            println!("warning: drop object due to no core object: {key_localizations:?}");
        }
        Ok(localized_objects)
    }

    pub(crate) fn group(&self) -> Result<Group, Error> {
        self.core.group()
    }

    pub(crate) fn location(&self) -> &String {
        self.core.location()
    }

    pub(crate) fn id(&self) -> Result<&str, Error> {
        self.core.id()
    }

    pub(crate) fn icon(&self) -> Result<Option<String>, Error> {
        match self.group()? {
            Group::Achievements => todo!(),
            Group::Cultures => todo!(),
            Group::Decks => todo!(),
            Group::Dicta => todo!(),
            Group::Elements => todo!(),
            Group::Endings => todo!(),
            Group::Legacies => todo!(),
            Group::Levers => todo!(),
            Group::Portals => todo!(),
            Group::Recipes => todo!(),
            Group::Settings => todo!(),
            Group::Verbs => todo!(),
        }

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

#[derive(Debug, Default)]
pub(crate) struct Localization<'a> {
    de: Option<Object<'a>>,
    es: Option<Object<'a>>,
    fr: Option<Object<'a>>,
    ja: Option<Object<'a>>,
    ru: Option<Object<'a>>,
    zh_hans: Option<Object<'a>>,
}

impl<'a> Localization<'a> {
    #[allow(unused)] // We may not be interested in this language.
    pub(crate) fn de(&self) -> Option<&Object<'a>> {
        self.de.as_ref()
    }

    #[allow(unused)] // We may not be interested in this language.
    pub(crate) fn es(&self) -> Option<&Object<'a>> {
        self.es.as_ref()
    }

    #[allow(unused)] // We may not be interested in this language.
    pub(crate) fn fr(&self) -> Option<&Object<'a>> {
        self.fr.as_ref()
    }

    #[allow(unused)] // We may not be interested in this language.
    pub(crate) fn ja(&self) -> Option<&Object<'a>> {
        self.ja.as_ref()
    }

    #[allow(unused)] // We may not be interested in this language.
    pub(crate) fn ru(&self) -> Option<&Object<'a>> {
        self.ru.as_ref()
    }

    pub(crate) fn zh_hans(&self) -> Option<&Object<'a>> {
        self.zh_hans.as_ref()
    }

    pub(crate) fn add(&mut self, localization_object: LocalizationObject<'a>) -> Result<(), Error> {
        let container = match localization_object.locale {
            Locale::De => &mut self.de,
            Locale::Es => &mut self.es,
            Locale::Fr => &mut self.fr,
            Locale::Ja => &mut self.ja,
            Locale::Ru => &mut self.ru,
            Locale::ZhHans => &mut self.zh_hans,
        };
        match container {
            Some(object) => return Err(Error::DuplicatedLocalizationObject(object.key()?.into())),
            None => container.insert(localization_object.object),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct LocalizationObject<'a> {
    locale: Locale,
    object: Object<'a>,
}

impl<'a> LocalizationObject<'a> {
    pub(crate) fn new(locale: Locale, object: Object<'a>) -> Self {
        Self { locale, object }
    }

    fn key(&self) -> Result<Key, Error> {
        self.object.key()
    }

    // fn locale(&self) -> Locale {
    //     self.locale
    // }

    // fn object(&self) -> &Object<'a> {
    //     &self.object
    // }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Locale {
    De,
    Es,
    Fr,
    Ja,
    Ru,
    ZhHans,
}

impl Locale {
    pub(crate) fn folder(&self) -> &'static str {
        match self {
            Locale::De => "loc_de",
            Locale::Es => "loc_es",
            Locale::Fr => "loc_fr",
            Locale::Ja => "loc_jp",
            Locale::Ru => "loc_ru",
            Locale::ZhHans => "loc_zh-hans",
        }
    }
}
