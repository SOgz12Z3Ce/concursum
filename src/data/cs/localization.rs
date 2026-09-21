use crate::{
    data::cs::{
        group::Group,
        object::{Key, Object},
        text::Summary,
    },
    error::{Error, JsonSchemaError},
};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub(crate) struct LocalizedObject<'a, 'b> {
    core: Object<'a>,
    localization: Localization<'b>,
}

impl<'a, 'b> LocalizedObject<'a, 'b> {
    pub(crate) fn from_objects(
        cores: Vec<Object<'a>>,
        localizations: Vec<LocalizationObject<'b>>,
    ) -> Result<Vec<Self>, Error> {
        let mut key_localizations: HashMap<Key, Localization> = HashMap::new();
        for localization_object in localizations {
            key_localizations
                .entry(localization_object.key()?)
                .or_default()
                .add(localization_object)
                .inspect_err(|error| println!("warning: {error}"))
                .ok();
        }

        let localized_objects = cores
            .into_iter()
            .map(|core| {
                let localization = key_localizations.remove(&core.key()?).unwrap_or_default();
                Ok(Self { core, localization })
            })
            .collect::<Result<_, Error>>()?;
        if !key_localizations.is_empty() {
            println!("warning: dropping object due to no core object:");
            for key in key_localizations.keys() {
                println!("'{key}'")
            }
        }
        Ok(localized_objects)
    }

    pub(crate) fn core(&self) -> &Object<'a> {
        &self.core
    }
    
    pub(crate) fn localization(&self) -> &Localization<'b> {
        &self.localization
    }

    pub(crate) fn group(&self) -> Result<Group, Error> {
        self.core.group()
    }

    pub(crate) fn id(&self) -> Result<&'a str, JsonSchemaError> {
        self.core.id()
    }

    pub(crate) fn key(&self) -> Result<Key<'a>, Error> {
        self.core.key()
    }

    pub(crate) fn icon(&self) -> Result<(Option<String>, Option<&'static str>), Error> {
        self.core.icon()
    }

    pub(crate) fn summary(&self) -> Result<Summaries<'_>, Error> {
        Ok(Summaries {
            en_gb: self.core.summary()?,
            zh_hans: self
                .localization
                .zh_hans()
                .map(|object| object.summary())
                .transpose()?,
        })
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
            Some(object) => {
                return Err(Error::DuplicatedLocalizationObject {
                    locale: localization_object.locale,
                    key: object.key()?.into(),
                });
            }
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

    pub(crate) fn key(&self) -> Result<Key<'a>, Error> {
        self.object.key()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum Locale {
    De,
    Es,
    Fr,
    Ja,
    Ru,
    ZhHans,
}

impl Locale {
    pub(crate) const ALL: [Self; 6] = [
        Self::De,
        Self::Es,
        Self::Fr,
        Self::Ja,
        Self::Ru,
        Self::ZhHans,
    ];

    pub(crate) fn folder(&self) -> &'static str {
        match self {
            Self::De => "loc_de",
            Self::Es => "loc_es",
            Self::Fr => "loc_fr",
            Self::Ja => "loc_jp",
            Self::Ru => "loc_ru",
            Self::ZhHans => "loc_zh-hans",
        }
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::De => write!(f, "de (Deutsch)"),
            Self::Es => write!(f, "es (Español)"),
            Self::Fr => write!(f, "fr (Français)"),
            Self::Ja => write!(f, "ja (日本語)"),
            Self::Ru => write!(f, "ru (Русский)"),
            Self::ZhHans => write!(f, "zh-Hans (简体中文)"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Summaries<'a> {
    pub(crate) en_gb: Summary<'a>,
    pub(crate) zh_hans: Option<Summary<'a>>,
}

impl<'a> Summaries<'a> {
    pub(crate) fn labels(&self) -> Vec<&'a str> {
        let en_gb_labels = self.en_gb.labels().into_iter();
        let zh_hans_labels = self
            .zh_hans
            .as_ref()
            .map(|summary| summary.labels())
            .into_iter()
            .flatten();

        en_gb_labels.chain(zh_hans_labels).collect()
    }

    pub(crate) fn descriptions(&self) -> Vec<&'a str> {
        let en_gb_descriptions = self.en_gb.descriptions().into_iter();
        let zh_hans_descriptions = self
            .zh_hans
            .as_ref()
            .map(|summary| summary.descriptions())
            .into_iter()
            .flatten();

        en_gb_descriptions.chain(zh_hans_descriptions).collect()
    }
}
