use std::collections::BTreeMap;

use super::error::WorthQueryProgramError;
use super::values::WorthQueryProgramValue;

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryValueExpr {
    Literal(WorthQueryProgramValue),
    Input(String),
    Object(BTreeMap<String, WorthQueryValueExpr>),
    Array(Vec<WorthQueryValueExpr>),
}

impl WorthQueryValueExpr {
    pub fn literal(value: WorthQueryProgramValue) -> Self {
        Self::Literal(value)
    }

    pub fn input(name: impl Into<String>) -> Self {
        Self::Input(name.into())
    }

    pub fn object(fields: impl IntoIterator<Item = (String, WorthQueryValueExpr)>) -> Self {
        Self::Object(fields.into_iter().collect())
    }

    pub fn array(items: impl IntoIterator<Item = WorthQueryValueExpr>) -> Self {
        Self::Array(items.into_iter().collect())
    }

    pub(crate) fn evaluate(
        &self,
        inputs: &BTreeMap<String, WorthQueryProgramValue>,
    ) -> Result<WorthQueryProgramValue, WorthQueryProgramError> {
        match self {
            Self::Literal(value) => Ok(value.clone()),
            Self::Input(name) => inputs.get(name).cloned().ok_or_else(|| {
                WorthQueryProgramError::new(format!("missing bound input `{name}`"))
            }),
            Self::Object(fields) => fields
                .iter()
                .map(|(key, value)| Ok((key.clone(), value.evaluate(inputs)?)))
                .collect::<Result<Vec<_>, _>>()
                .map(WorthQueryProgramValue::object),
            Self::Array(items) => items
                .iter()
                .map(|item| item.evaluate(inputs))
                .collect::<Result<Vec<_>, _>>()
                .map(WorthQueryProgramValue::array),
        }
    }
}
