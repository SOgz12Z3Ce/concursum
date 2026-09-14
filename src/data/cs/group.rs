use crate::error::Error;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl Group {
    pub(crate) const ALL: [Self; 12] = [
        Self::Achievements,
        Self::Cultures,
        Self::Decks,
        Self::Dicta,
        Self::Elements,
        Self::Endings,
        Self::Legacies,
        Self::Levers,
        Self::Portals,
        Self::Recipes,
        Self::Settings,
        Self::Verbs,
    ];
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
