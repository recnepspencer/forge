use crate::runtime::{ForgeQueryAspectTouch, ForgeQueryEffectPayload};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentInput {
    value: ForgeQueryIntentInputValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ForgeQueryIntentInputValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<ForgeQueryIntentInputValue>),
    Object(BTreeMap<String, ForgeQueryIntentInputValue>),
}

impl ForgeQueryIntentInput {
    pub fn null() -> Self {
        Self {
            value: ForgeQueryIntentInputValue::Null,
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            value: ForgeQueryIntentInputValue::Bool(value),
        }
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: ForgeQueryIntentInputValue::String(value.into()),
        }
    }

    pub fn integer(value: i64) -> Self {
        Self {
            value: ForgeQueryIntentInputValue::Number(value.to_string()),
        }
    }

    pub fn unsigned_integer(value: u64) -> Self {
        Self {
            value: ForgeQueryIntentInputValue::Number(value.to_string()),
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
            value: ForgeQueryIntentInputValue::Number(value),
        })
    }

    pub fn array(values: impl IntoIterator<Item = ForgeQueryIntentInput>) -> Self {
        Self {
            value: ForgeQueryIntentInputValue::Array(
                values.into_iter().map(|value| value.value).collect(),
            ),
        }
    }

    pub fn object(
        fields: impl IntoIterator<Item = (impl Into<String>, ForgeQueryIntentInput)>,
    ) -> Self {
        Self {
            value: ForgeQueryIntentInputValue::Object(
                fields
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.value))
                    .collect(),
            ),
        }
    }

    pub(in crate::runtime) fn from_effect_payload(payload: &ForgeQueryEffectPayload) -> Self {
        let Some(condition) = payload.condition() else {
            return Self {
                value: ForgeQueryIntentInputValue::Null,
            };
        };
        let mut fields = BTreeMap::from([
            (
                "condition".to_string(),
                ForgeQueryIntentInputValue::String(condition.to_string()),
            ),
            (
                "changed_aspects".to_string(),
                intent_input_string_array(payload.changed_aspect_touches()),
            ),
        ]);
        if !payload.input_aspect_touches().is_empty() {
            fields.insert(
                "input_aspects".to_string(),
                intent_input_string_array(payload.input_aspect_touches()),
            );
        }
        if !payload.output_aspect_touches().is_empty() {
            fields.insert(
                "output_aspects".to_string(),
                intent_input_string_array(payload.output_aspect_touches()),
            );
        }
        Self {
            value: ForgeQueryIntentInputValue::Object(fields),
        }
    }

    pub fn string_field(&self, field: &str) -> Option<&str> {
        let ForgeQueryIntentInputValue::Object(fields) = &self.value else {
            return None;
        };
        let ForgeQueryIntentInputValue::String(value) = fields.get(field)? else {
            return None;
        };
        Some(value)
    }

    pub(super) fn digest_material(&self) -> String {
        intent_input_digest_material(&self.value)
    }
}

fn intent_input_string_array(touches: &[ForgeQueryAspectTouch]) -> ForgeQueryIntentInputValue {
    ForgeQueryIntentInputValue::Array(
        touches
            .iter()
            .map(|touch| {
                ForgeQueryIntentInputValue::String(touch.admitted_touch_digest_part().to_string())
            })
            .collect(),
    )
}

fn intent_input_digest_material(input: &ForgeQueryIntentInputValue) -> String {
    match input {
        ForgeQueryIntentInputValue::Null => "null".to_string(),
        ForgeQueryIntentInputValue::Bool(value) => format!("bool:{value}"),
        ForgeQueryIntentInputValue::Number(value) => format!("number:{value}"),
        ForgeQueryIntentInputValue::String(value) => format!("string:{}:{value}", value.len()),
        ForgeQueryIntentInputValue::Array(values) => {
            let values = values
                .iter()
                .map(intent_input_digest_material)
                .collect::<Vec<_>>();
            format!("array:[{}]", values.join(","))
        }
        ForgeQueryIntentInputValue::Object(fields) => {
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
