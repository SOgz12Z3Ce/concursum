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
    pub(crate) draw_messages: Vec<&'a str>,
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

    let slots_text= slots
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
        .collect()
        ;
    Ok(Some(slots_text))
}
