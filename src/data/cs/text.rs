use std::collections::HashMap;

use crate::{data::cs::object::Object, error::Error};
use serde_json::{Map, Value};

#[derive(Debug)]
pub(crate) struct Text<'a> {
    pub(crate) label: Option<&'a str>,
    pub(crate) slot: Option<SlotText<'a>>,
    pub(crate) slots: Option<Vec<SlotText<'a>>>,
    pub(crate) internal_deck: Option<DeckText<'a>>,
    pub(crate) alt: Option<Vec<RecipeText<'a>>>,
    pub(crate) linked: Option<Vec<RecipeText<'a>>>,
    pub(crate) description: Option<&'a str>,
    pub(crate) start_description: Option<&'a str>,
    pub(crate) description_unlocked: Option<&'a str>,
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
        let Text {
            label,
            slot,
            slots,
            internal_deck,
            alt,
            linked,
            description,
            start_description,
            description_unlocked,
        };

        // General
        label = text(properties, "label")?;
        description = text(properties, "description")?;
        start_description = text(properties, "startdescription")?;
        description_unlocked = text(properties, "descriptionunlocked")?;

        // Slot
        slot = slot_text(properties, "slot")?;
        slots = slots_text(properties, "slots")?;

        // Internal deck
        internal_deck = deck_text(properties, "internaldeck")?;

        // Recipes
        alt = recipes_text(properties, "alt")?;
        linked = recipes_text(properties, "linked")?;

        Ok(Text {
            label,
            slot,
            slots,
            internal_deck,
            alt,
            linked,
            description,
            start_description,
            description_unlocked,
        })
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
