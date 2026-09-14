use crate::{
    data::{
        JsonObjectExt as _, JsonValueExt,
        cs::{group::Group, object::Object},
    },
    error::{Error, JsonSchemaError},
};
use serde_json::{Map, Value};
use std::{collections::HashMap, iter};

static ALT_KEY: &'static str = "alt";
static DESCRIPTION_KEY: &'static str = "description";
static DESCRIPTION_UNLOCKED_KEY: &'static str = "descriptionunlocked";
static DRAW_MESSAGES_KEY: &'static str = "drawmessages";
static INTERNAL_DECK_KEY: &'static str = "internaldeck";
static LABEL_KEY: &'static str = "label";
static LINKED_KEY: &'static str = "linked";
static SLOT_KEY: &'static str = "slot";
static SLOTS_KEY: &'static str = "slots";
static START_DESCRIPTION_KEY: &'static str = "startdescription";

trait JsonObjectExt {
    fn try_get_text<'a>(&'a self, key: &str) -> Result<&'a str, JsonSchemaError>;

    fn get_text<'a>(&'a self, key: &str) -> Result<Option<&'a str>, JsonSchemaError>;

    fn get_slot_text<'a>(&'a self, key: &str) -> Result<Option<SlotText<'a>>, JsonSchemaError>;

    fn get_slots_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<Vec<SlotText<'a>>>, JsonSchemaError>;

    fn get_deck_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<InternalDeckText<'a>>, JsonSchemaError>;

    fn get_draw_messages_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<HashMap<&'a String, &'a str>>, JsonSchemaError>;

    fn get_recipes_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<Vec<RecipeText<'a>>>, JsonSchemaError>;
}

impl JsonObjectExt for Map<String, Value> {
    fn try_get_text<'a>(&'a self, key: &str) -> Result<&'a str, JsonSchemaError> {
        self.try_get(key)?.try_as_str()
    }

    fn get_text<'a>(&'a self, key: &str) -> Result<Option<&'a str>, JsonSchemaError> {
        self.get(key).map(|value| value.try_as_str()).transpose()
    }

    fn get_slot_text<'a>(&'a self, key: &str) -> Result<Option<SlotText<'a>>, JsonSchemaError> {
        let Some(value) = self.get(key) else {
            return Ok(None);
        };
        let slot = value.try_as_object()?;
        let slot = slot_text(slot)?;
        Ok(Some(slot))
    }

    fn get_slots_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<Vec<SlotText<'a>>>, JsonSchemaError> {
        let Some(value) = self.get(key) else {
            return Ok(None);
        };
        let slots = value.try_as_array()?;
        let slots = slots
            .iter()
            .map(|slot| slot_text(slot.try_as_object()?))
            .collect::<Result<_, _>>()?;
        Ok(Some(slots))
    }

    fn get_deck_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<InternalDeckText<'a>>, JsonSchemaError> {
        let Some(deck) = self.get(key) else {
            return Ok(None);
        };
        let deck = deck.try_as_object()?;

        let label = deck
            .get(LABEL_KEY)
            .map(|value| value.try_as_str())
            .transpose()?;
        let description = deck
            .get(DESCRIPTION_KEY)
            .map(|value| value.try_as_str())
            .transpose()?;
        Ok(Some(InternalDeckText {
            label: label,
            description,
        }))
    }

    fn get_draw_messages_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<HashMap<&'a String, &'a str>>, JsonSchemaError> {
        let Some(draw_messages) = self.get(key) else {
            return Ok(None);
        };
        let draw_messages = draw_messages.try_as_object()?;

        let draw_messages = draw_messages
            .iter()
            .map(|(key, value)| Ok((key, value.try_as_str()?)))
            .collect::<Result<_, _>>()?;
        Ok(Some(draw_messages))
    }

    fn get_recipes_text<'a>(
        &'a self,
        key: &str,
    ) -> Result<Option<Vec<RecipeText<'a>>>, JsonSchemaError> {
        let Some(recipes) = self.get(key) else {
            return Ok(None);
        };
        let recipes = recipes.try_as_array()?;

        let recipes = recipes
            .iter()
            .map(|recipe| {
                let recipe = recipe.try_as_object()?;
                Ok(RecipeText {
                    label: recipe.get_text(LABEL_KEY)?,
                    description: recipe.get_text(DESCRIPTION_KEY)?,
                    start_description: recipe.get_text(START_DESCRIPTION_KEY)?,
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Some(recipes))
    }
}

fn slot_text(slot: &Map<String, Value>) -> Result<SlotText<'_>, JsonSchemaError> {
    Ok(SlotText {
        label: slot.get_text(LABEL_KEY)?,
        description: slot.get_text(DESCRIPTION_KEY)?,
    })
}

// TODO: add comments here?
#[derive(Debug)]
enum Text<'a> {
    Achievement {
        label: &'a str,
        description_unlocked: Option<&'a str>,
    },
    Culture,
    Deck {
        label: Option<&'a str>,
        description: Option<&'a str>,
        draw_messages: Option<HashMap<&'a String, &'a str>>,
    },
    Dictum,
    Element {
        label: Option<&'a str>,
        description: Option<&'a str>,
        slots: Option<Vec<SlotText<'a>>>,
    },
    Ending {
        label: &'a str,
        description: &'a str,
    },
    Legacy {
        label: Option<&'a str>,
        description: &'a str,
        start_description: Option<&'a str>,
    },
    Lever,
    Portal {
        label: &'a str,
        description: &'a str,
    },
    Recipe {
        label: Option<&'a str>,
        description: Option<&'a str>,
        start_description: Option<&'a str>,
        slots: Option<Vec<SlotText<'a>>>,
        internal_deck: Option<InternalDeckText<'a>>,
        alt: Option<Vec<RecipeText<'a>>>,
        linked: Option<Vec<RecipeText<'a>>>,
    },
    Setting,
    Verb {
        label: &'a str,
        description: &'a str,
        slot: Option<SlotText<'a>>,
    },
}

#[derive(Debug)]
struct SlotText<'a> {
    label: Option<&'a str>,
    description: Option<&'a str>,
}

#[derive(Debug)]
struct InternalDeckText<'a> {
    label: Option<&'a str>,
    description: Option<&'a str>,
}

#[derive(Debug)]
struct RecipeText<'a> {
    label: Option<&'a str>,
    description: Option<&'a str>,
    start_description: Option<&'a str>,
}

#[derive(Debug, Default)]
pub(crate) struct Summary<'a> {
    pub(crate) label: Option<&'a str>,
    pub(crate) description: Option<&'a str>,
    pub(crate) slots: Option<Vec<SlotAddition<'a>>>,
    pub(crate) recipes: Option<Vec<RecipeAddition<'a>>>,
    pub(crate) deck: Option<DeckAddition<'a>>,
    pub(crate) legacy: Option<LegacyAddition<'a>>,
}

impl<'a> Summary<'a> {
    pub(crate) fn labels(&self) -> Vec<&'a str> {
        let this = iter::once(self.label);
        let slots = self.slots.iter().flatten().map(|slot| slot.label);
        let recipes = self
            .recipes
            .iter()
            .flatten()
            .filter_map(|recipe| match recipe {
                RecipeAddition::This { .. } => None,
                RecipeAddition::Other { label, .. } => Some(*label),
            });
        let deck = self.deck.iter().map(|deck| match deck {
            DeckAddition::This { .. } => None,
            DeckAddition::Internal { label, .. } => *label,
        });

        this.chain(slots)
            .chain(recipes)
            .chain(deck)
            .flatten()
            .collect()
    }

    pub(crate) fn descriptions(&self) -> Vec<&'a str> {
        let this = iter::once(self.description);
        let slots = self.slots.iter().flatten().map(|slot| slot.description);
        let recipes = self
            .recipes
            .iter()
            .flatten()
            .flat_map(|recipe| match recipe {
                RecipeAddition::This { start_description } => {
                    vec![Some(*start_description)]
                }
                RecipeAddition::Other {
                    description,
                    start_description,
                    ..
                } => {
                    vec![*description, *start_description]
                }
            });
        let deck = self.deck.iter().flat_map(|deck| match deck {
            DeckAddition::This { draw_messages } => draw_messages
                .iter()
                .map(|(_, message)| Some(*message))
                .collect(),
            DeckAddition::Internal { description, .. } => vec![*description],
        });
        let legacy = self
            .legacy
            .iter()
            .map(|legacy| Some(legacy.start_description));

        this.chain(slots)
            .chain(recipes)
            .chain(deck)
            .chain(legacy)
            .flatten()
            .collect()
    }
}

#[derive(Debug)]
pub(crate) struct SlotAddition<'a> {
    pub(crate) label: Option<&'a str>,
    pub(crate) description: Option<&'a str>,
}

impl<'a> From<SlotText<'a>> for SlotAddition<'a> {
    fn from(value: SlotText<'a>) -> Self {
        Self {
            label: value.label,
            description: value.description,
        }
    }
}

#[derive(Debug)]
pub(crate) enum RecipeAddition<'a> {
    This {
        start_description: &'a str,
    },
    Other {
        label: Option<&'a str>,
        description: Option<&'a str>,
        start_description: Option<&'a str>,
    },
}

impl<'a> From<&'a str> for RecipeAddition<'a> {
    fn from(value: &'a str) -> Self {
        Self::This {
            start_description: value,
        }
    }
}

impl<'a> From<RecipeText<'a>> for RecipeAddition<'a> {
    fn from(value: RecipeText<'a>) -> Self {
        Self::Other {
            label: value.label,
            description: value.description,
            start_description: value.start_description,
        }
    }
}

#[derive(Debug)]
pub(crate) enum DeckAddition<'a> {
    This {
        draw_messages: HashMap<&'a String, &'a str>,
    },
    Internal {
        label: Option<&'a str>,
        description: Option<&'a str>,
    },
}

impl<'a> From<HashMap<&'a String, &'a str>> for DeckAddition<'a> {
    fn from(value: HashMap<&'a String, &'a str>) -> Self {
        Self::This {
            draw_messages: value,
        }
    }
}

impl<'a> From<InternalDeckText<'a>> for DeckAddition<'a> {
    fn from(value: InternalDeckText<'a>) -> Self {
        Self::Internal {
            label: value.label,
            description: value.description,
        }
    }
}

#[derive(Debug)]
pub(crate) struct LegacyAddition<'a> {
    pub(crate) start_description: &'a str,
}

impl<'a> From<&'a str> for LegacyAddition<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            start_description: value,
        }
    }
}

impl<'a> Object<'a> {
    pub(crate) fn summary(&self) -> Result<Summary<'a>, Error> {
        let text = self.text()?;
        let summary = match text {
            Text::Achievement {
                label,
                description_unlocked,
            } => Summary {
                label: Some(label),
                description: description_unlocked,
                ..Default::default()
            },
            Text::Culture => Default::default(),
            Text::Deck {
                label,
                description,
                draw_messages,
            } => {
                let deck = draw_messages.map(Into::into);
                Summary {
                    label,
                    description,
                    deck,
                    ..Default::default()
                }
            }
            Text::Dictum => Default::default(),
            Text::Element {
                label,
                description,
                slots,
            } => {
                let slots = slots.map(|slots| slots.into_iter().map(Into::into).collect());
                Summary {
                    label,
                    description,
                    slots,
                    ..Default::default()
                }
            }
            Text::Ending { label, description } => Summary {
                label: Some(label),
                description: Some(description),
                ..Default::default()
            },
            Text::Legacy {
                label,
                description,
                start_description,
            } => {
                let legacy = start_description.map(Into::into);
                Summary {
                    label,
                    description: Some(description),
                    legacy,
                    ..Default::default()
                }
            }
            Text::Lever => Default::default(),
            Text::Portal { label, description } => Summary {
                label: Some(label),
                description: Some(description),
                ..Default::default()
            },
            Text::Recipe {
                label,
                description,
                start_description,
                slots,
                internal_deck,
                alt,
                linked,
            } => {
                let this = start_description.map(Into::into);
                let slots = slots.map(|slots| slots.into_iter().map(Into::into).collect());
                let deck = internal_deck.map(Into::into);
                let alt = alt.map(|recipes| {
                    recipes
                        .into_iter()
                        .map(Into::<RecipeAddition>::into)
                        .collect::<Vec<_>>()
                });
                let linked = linked.map(|recipes| {
                    recipes
                        .into_iter()
                        .map(Into::<RecipeAddition>::into)
                        .collect::<Vec<_>>()
                });

                let mut recipes = Vec::new();
                if let Some(this) = this {
                    recipes.push(this);
                }
                if let Some(alt) = alt {
                    recipes.extend(alt);
                }
                if let Some(linked) = linked {
                    recipes.extend(linked);
                }
                let recipes = match recipes.len() == 0 {
                    true => None,
                    false => Some(recipes),
                };

                Summary {
                    label,
                    description,
                    slots,
                    recipes,
                    deck,
                    ..Default::default()
                }
            }
            Text::Setting => Default::default(),
            Text::Verb {
                label,
                description,
                slot,
            } => {
                let slots = slot.map(|slot| vec![slot.into()]);
                Summary {
                    label: Some(label),
                    description: Some(description),
                    slots,
                    ..Default::default()
                }
            }
        };
        Ok(summary)
    }

    fn text(&self) -> Result<Text<'a>, Error> {
        let properties = self.properties();
        match self.group()? {
            Group::Achievements => Ok(Text::Achievement {
                label: properties.try_get_text(LABEL_KEY)?,
                description_unlocked: properties.get_text(DESCRIPTION_UNLOCKED_KEY)?,
            }),
            Group::Cultures => Ok(Text::Culture),
            Group::Decks => Ok(Text::Deck {
                label: properties.get_text(LABEL_KEY)?,
                description: properties.get_text(DESCRIPTION_KEY)?,
                draw_messages: properties.get_draw_messages_text(DRAW_MESSAGES_KEY)?,
            }),
            Group::Dicta => Ok(Text::Dictum),
            Group::Elements => Ok(Text::Element {
                label: properties.get_text(LABEL_KEY)?,
                slots: properties.get_slots_text(SLOTS_KEY)?,
                description: properties.get_text(DESCRIPTION_KEY)?,
            }),
            Group::Endings => Ok(Text::Ending {
                label: properties.try_get_text(LABEL_KEY)?,
                description: properties.try_get_text(DESCRIPTION_KEY)?,
            }),
            Group::Legacies => Ok(Text::Legacy {
                label: properties.get_text(LABEL_KEY)?,
                description: properties.try_get_text(DESCRIPTION_KEY)?,
                start_description: properties.get_text(START_DESCRIPTION_KEY)?,
            }),
            Group::Levers => Ok(Text::Lever),
            Group::Portals => Ok(Text::Portal {
                label: properties.try_get_text(LABEL_KEY)?,
                description: properties.try_get_text(DESCRIPTION_KEY)?,
            }),
            Group::Recipes => Ok(Text::Recipe {
                label: properties.get_text(LABEL_KEY)?,
                slots: properties.get_slots_text(SLOTS_KEY)?,
                internal_deck: properties.get_deck_text(INTERNAL_DECK_KEY)?,
                alt: properties.get_recipes_text(ALT_KEY)?,
                linked: properties.get_recipes_text(LINKED_KEY)?,
                description: properties.get_text(DESCRIPTION_KEY)?,
                start_description: properties.get_text(START_DESCRIPTION_KEY)?,
            }),
            Group::Settings => Ok(Text::Setting),
            Group::Verbs => Ok(Text::Verb {
                label: properties.try_get_text(LABEL_KEY)?,
                slot: properties.get_slot_text(SLOT_KEY)?,
                description: properties.try_get_text(DESCRIPTION_KEY)?,
            }),
        }
    }
}
