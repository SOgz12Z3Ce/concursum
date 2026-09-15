use crate::{
    data::cs::object::{Group, Object},
    error::Error,
};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Text<'a> {
    Achievement {
        label: &'a str,
        description_unlocked: Option<&'a str>,
    },
    Culture,
    Deck {
        label: Option<&'a str>,
        description: Option<&'a str>,
    },
    Dictum,
    Element {
        label: Option<&'a str>,
        slots: Option<Vec<SlotText<'a>>>,
        description: Option<&'a str>,
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
        slots: Option<Vec<SlotText<'a>>>,
        internal_deck: Option<DeckText<'a>>,
        alt: Option<Vec<RecipeText<'a>>>,
        linked: Option<Vec<RecipeText<'a>>>,
        description: Option<&'a str>,
        start_description: Option<&'a str>,
    },
    Setting,
    Verb {
        label: &'a str,
        slot: Option<SlotText<'a>>,
        description: &'a str,
    },
}

#[derive(Debug)]
pub(crate) struct SlotText<'a> {
    pub(crate) label: Option<&'a str>,
    pub(crate) description: Option<&'a str>,
}

#[derive(Debug)]
pub(crate) struct DeckText<'a> {
    pub(crate) label: Option<&'a str>,
    pub(crate) description: Option<&'a str>,
    pub(crate) draw_messages: Option<HashMap<&'a String, &'a str>>,
}

#[derive(Debug)]
pub(crate) struct RecipeText<'a> {
    pub(crate) label: Option<&'a str>,
    pub(crate) description: Option<&'a str>,
    pub(crate) start_description: Option<&'a str>,
}

impl<'a> Object<'a> {
    pub(crate) fn text(&self) -> Result<Text, Error> {
        let properties = self.properties();

        match self.group()? {
            Group::Achievements => Ok(Text::Achievement {
                label: text(properties, "label")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected achievement object has member 'label'"),
                })?,
                description_unlocked: text(properties, "descriptionunlocked")?,
            }),
            Group::Cultures => Ok(Text::Culture),
            Group::Decks => Ok(Text::Deck {
                label: text(properties, "label")?,
                description: text(properties, "description")?,
            }),
            Group::Dicta => Ok(Text::Dictum),
            Group::Elements => Ok(Text::Element {
                label: text(properties, "label")?,
                slots: slots_text(properties, "slots")?,
                description: text(properties, "description")?,
            }),
            Group::Endings => Ok(Text::Ending {
                label: text(properties, "label")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected ending object has member 'label'"),
                })?,
                description: text(properties, "description")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected ending object has member 'description'"),
                })?,
            }),
            Group::Legacies => Ok(Text::Legacy {
                label: text(properties, "label")?,
                description: text(properties, "description")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected legacy object has member 'description'"),
                })?,
                start_description: text(properties, "startdescription")?,
            }),
            Group::Levers => Ok(Text::Lever),
            Group::Portals => Ok(Text::Portal {
                label: text(properties, "label")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected portal object has member 'label'"),
                })?,
                description: text(properties, "description")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected portal object has member 'description'"),
                })?,
            }),
            Group::Recipes => Ok(Text::Recipe {
                label: text(properties, "label")?,
                slots: slots_text(properties, "slots")?,
                internal_deck: deck_text(properties, "internaldeck")?,
                alt: recipes_text(properties, "alt")?,
                linked: recipes_text(properties, "linked")?,
                description: text(properties, "description")?,
                start_description: text(properties, "startdescription")?,
            }),
            Group::Settings => Ok(Text::Setting),
            Group::Verbs => Ok(Text::Verb {
                label: text(properties, "label")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected verb object has member 'label'"),
                })?,
                slot: slot_text(properties, "slot")?,
                description: text(properties, "description")?.ok_or_else(|| Error::JsonSchema {
                    value: Value::Object(properties.to_owned()),
                    message: String::from("expected verb object has member 'description'"),
                })?,
            }),
        }
    }
}

fn text<'a>(object: &'a Map<String, Value>, field: &str) -> Result<Option<&'a str>, Error> {
    let Some(value) = object.get(field) else {
        return Ok(None);
    };
    let Some(value) = value.as_str() else {
        return Err(Error::JsonSchema {
            value: value.to_owned(),
            message: format!("expected value of \"{field}\" member is a string"),
        });
    };
    Ok(Some(value))
}

fn slot_text<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<Option<SlotText<'a>>, Error> {
    let Some(slot) = object.get(field) else {
        return Ok(None);
    };
    let Some(slot) = slot.as_object() else {
        return Err(Error::JsonSchema {
            value: slot.to_owned(),
            message: format!("expected value of \"{field}\" member is an object"),
        });
    };
    let SlotText { label, description };
    label = text(slot, "label")?;
    description = text(slot, "description")?;
    Ok(Some(SlotText { label, description }))
}

fn slots_text<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<Option<Vec<SlotText<'a>>>, Error> {
    let Some(slots) = object.get(field) else {
        return Ok(None);
    };
    let Some(slots) = slots.as_array() else {
        return Err(Error::JsonSchema {
            value: slots.to_owned(),
            message: format!("expected value of \"{field}\" member is an array"),
        });
    };

    let slots = slots
        .iter()
        .map(|slot| {
            let Some(slot) = slot.as_object() else {
                return Err(Error::JsonSchema {
                    value: slot.to_owned(),
                    message: format!("expected value of \"{field}\" member is an object"),
                });
            };
            let SlotText { label, description };
            label = text(slot, "label")?;
            description = text(slot, "description")?;
            Ok(SlotText { label, description })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some(slots))
}

fn deck_text<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<Option<DeckText<'a>>, Error> {
    let Some(deck) = object.get(field) else {
        return Ok(None);
    };
    let Some(deck) = deck.as_object() else {
        return Err(Error::JsonSchema {
            value: deck.to_owned(),
            message: format!("expected value of \"{field}\" member is an object"),
        });
    };

    let DeckText {
        label,
        description,
        draw_messages,
    };
    label = text(deck, "label")?;
    description = text(deck, "description")?;
    draw_messages = draw_messages_text(deck, "drawmessages")?;
    Ok(Some(DeckText {
        label,
        description,
        draw_messages,
    }))
}

fn draw_messages_text<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<Option<HashMap<&'a String, &'a str>>, Error> {
    let Some(draw_messages) = object.get(field) else {
        return Ok(None);
    };
    let Some(draw_messages) = draw_messages.as_object() else {
        return Err(Error::JsonSchema {
            value: draw_messages.to_owned(),
            message: format!("expected value of \"{field}\" member is an object"),
        });
    };

    let draw_messages = draw_messages
        .iter()
        .map(|(id, message)| {
            let Some(message) = message.as_str() else {
                return Err(Error::JsonSchema {
                    value: message.to_owned(),
                    message: format!("expected value of \"{id}\" member is a string"),
                });
            };
            Ok((id, message))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;
    Ok(Some(draw_messages))
}

fn recipes_text<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<Option<Vec<RecipeText<'a>>>, Error> {
    let Some(recipes) = object.get(field) else {
        return Ok(None);
    };
    let Some(recipes) = recipes.as_array() else {
        return Err(Error::JsonSchema {
            value: recipes.to_owned(),
            message: format!("expected value of \"{field}\" member is an array"),
        });
    };

    let recipes = recipes
        .iter()
        .map(|recipe| {
            let Some(recipe) = recipe.as_object() else {
                return Err(Error::JsonSchema {
                    value: recipe.to_owned(),
                    message: format!("expected value of \"{field}\" member is an object"),
                });
            };
            let RecipeText {
                label,
                description,
                start_description,
            };
            label = text(recipe, "label")?;
            description = text(recipe, "description")?;
            start_description = text(recipe, "startdescription")?;
            Ok(RecipeText {
                label,
                description,
                start_description,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some(recipes))
}
