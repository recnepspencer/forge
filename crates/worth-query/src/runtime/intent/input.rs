use crate::runtime::{WorthQueryAspectTouch, WorthQueryEffectPayload};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentInput {
    value: WorthQueryIntentInputValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthQueryIntentInputValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    AspectTouch(WorthQueryAspectTouch),
    Array(Vec<WorthQueryIntentInputValue>),
    Object(BTreeMap<String, WorthQueryIntentInputValue>),
}

impl WorthQueryIntentInput {
    pub fn null() -> Self {
        Self {
            value: WorthQueryIntentInputValue::Null,
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            value: WorthQueryIntentInputValue::Bool(value),
        }
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: WorthQueryIntentInputValue::String(value.into()),
        }
    }

    pub fn integer(value: i64) -> Self {
        Self {
            value: WorthQueryIntentInputValue::Number(value.to_string()),
        }
    }

    pub fn unsigned_integer(value: u64) -> Self {
        Self {
            value: WorthQueryIntentInputValue::Number(value.to_string()),
        }
    }

    pub fn decimal_text(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !is_canonical_number_text(&value) {
            return Err(format!(
                "intent input number `{value}` is not valid canonical number text"
            ));
        }
        Ok(Self {
            value: WorthQueryIntentInputValue::Number(value),
        })
    }

    pub fn array(values: impl IntoIterator<Item = WorthQueryIntentInput>) -> Self {
        Self {
            value: WorthQueryIntentInputValue::Array(
                values.into_iter().map(|value| value.value).collect(),
            ),
        }
    }

    pub fn object(
        fields: impl IntoIterator<Item = (impl Into<String>, WorthQueryIntentInput)>,
    ) -> Self {
        Self {
            value: WorthQueryIntentInputValue::Object(
                fields
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.value))
                    .collect(),
            ),
        }
    }

    pub(in crate::runtime) fn from_effect_payload(payload: &WorthQueryEffectPayload) -> Self {
        let Some(condition) = payload.condition() else {
            return Self {
                value: WorthQueryIntentInputValue::Null,
            };
        };
        let mut fields = BTreeMap::from([
            (
                "condition".to_string(),
                WorthQueryIntentInputValue::String(condition.to_string()),
            ),
            (
                "changed_aspects".to_string(),
                intent_input_aspect_touch_array(payload.changed_aspect_touches()),
            ),
        ]);
        if !payload.input_aspect_touches().is_empty() {
            fields.insert(
                "input_aspects".to_string(),
                intent_input_aspect_touch_array(payload.input_aspect_touches()),
            );
        }
        if !payload.output_aspect_touches().is_empty() {
            fields.insert(
                "output_aspects".to_string(),
                intent_input_aspect_touch_array(payload.output_aspect_touches()),
            );
        }
        Self {
            value: WorthQueryIntentInputValue::Object(fields),
        }
    }

    pub fn string_field(&self, field: &str) -> Option<&str> {
        let WorthQueryIntentInputValue::Object(fields) = &self.value else {
            return None;
        };
        let WorthQueryIntentInputValue::String(value) = fields.get(field)? else {
            return None;
        };
        Some(value)
    }

    pub(super) fn digest_material(&self) -> String {
        intent_input_digest_material(&self.value)
    }
}

fn intent_input_aspect_touch_array(
    touches: &[WorthQueryAspectTouch],
) -> WorthQueryIntentInputValue {
    WorthQueryIntentInputValue::Array(
        touches
            .iter()
            .cloned()
            .map(WorthQueryIntentInputValue::AspectTouch)
            .collect(),
    )
}

fn intent_input_digest_material(input: &WorthQueryIntentInputValue) -> String {
    match input {
        WorthQueryIntentInputValue::Null => "null".to_string(),
        WorthQueryIntentInputValue::Bool(value) => format!("bool:{value}"),
        WorthQueryIntentInputValue::Number(value) => format!("number:{value}"),
        WorthQueryIntentInputValue::String(value) => format!("string:{}:{value}", value.len()),
        WorthQueryIntentInputValue::AspectTouch(touch) => {
            format!("aspect_touch:{}", touch.admitted_touch_digest_part())
        }
        WorthQueryIntentInputValue::Array(values) => {
            let values = values
                .iter()
                .map(intent_input_digest_material)
                .collect::<Vec<_>>();
            format!("array:[{}]", values.join(","))
        }
        WorthQueryIntentInputValue::Object(fields) => {
            let fields = fields
                .iter()
                .map(|(key, value)| {
                    format!(
                        "field:{}:{key}={}",
                        key.len(),
                        intent_input_digest_material(value)
                    )
                })
                .collect::<Vec<_>>();
            format!("object:{{{}}}", fields.join(","))
        }
    }
}

fn is_canonical_number_text(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut index = 0;
    if bytes.is_empty() {
        return false;
    }
    if bytes[index] == b'-' {
        index += 1;
        if index == bytes.len() {
            return false;
        }
    }
    match bytes[index] {
        b'0' => {
            index += 1;
            if index < bytes.len() && bytes[index].is_ascii_digit() {
                return false;
            }
        }
        b'1'..=b'9' => {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
        }
        _ => return false,
    }
    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;
        if index == bytes.len() || !bytes[index].is_ascii_digit() {
            return false;
        }
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
    }
    if index < bytes.len() && matches!(bytes[index], b'e' | b'E') {
        index += 1;
        if index < bytes.len() && matches!(bytes[index], b'+' | b'-') {
            index += 1;
        }
        if index == bytes.len() || !bytes[index].is_ascii_digit() {
            return false;
        }
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
    }
    index == bytes.len()
}
