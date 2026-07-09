use crate::boundary::errors::WORTHSignalJsError;

use super::super::model::{Expr, SignalValue};
use super::environment::ExprEnvironment;

impl<'a> ExprEnvironment<'a> {
    pub(super) fn evaluate_get(
        &self,
        target: &Expr,
        field: &str,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Object(fields) => fields
                .into_iter()
                .find(|(name, _)| name == field)
                .map(|(_, value)| value)
                .ok_or_else(|| {
                    WORTHSignalJsError::invalid_input(format!(
                        "field `{field}` was not present on object"
                    ))
                }),
            _ => Err(WORTHSignalJsError::invalid_input(
                "field access requires an object target",
            )),
        }
    }

    pub(super) fn evaluate_at(
        &self,
        target: &Expr,
        index: &Expr,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        let index = self.require_number(&self.evaluate(index)?)? as usize;
        match target {
            SignalValue::Array(items) => items.get(index).cloned().ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "array index `{index}` was out of bounds"
                ))
            }),
            _ => Err(WORTHSignalJsError::invalid_input(
                "index access requires an array target",
            )),
        }
    }

    pub(super) fn evaluate_first(&self, target: &Expr) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Array(items) => items.into_iter().next().ok_or_else(|| {
                WORTHSignalJsError::invalid_input("first requires a non-empty array target")
            }),
            _ => Err(WORTHSignalJsError::invalid_input(
                "first requires an array target",
            )),
        }
    }

    pub(super) fn evaluate_last(&self, target: &Expr) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Array(items) => items.into_iter().last().ok_or_else(|| {
                WORTHSignalJsError::invalid_input("last requires a non-empty array target")
            }),
            _ => Err(WORTHSignalJsError::invalid_input(
                "last requires an array target",
            )),
        }
    }

    pub(super) fn evaluate_slice(
        &self,
        target: &Expr,
        start: &Expr,
        end: Option<&Expr>,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        let start = self.require_number(&self.evaluate(start)?)? as usize;
        match target {
            SignalValue::Array(items) => {
                let end = match end {
                    Some(end) => self.require_number(&self.evaluate(end)?)? as usize,
                    None => items.len(),
                };
                if start > end || end > items.len() {
                    return Err(WORTHSignalJsError::invalid_input(
                        "slice range was out of bounds",
                    ));
                }
                Ok(SignalValue::Array(items[start..end].to_vec()))
            }
            _ => Err(WORTHSignalJsError::invalid_input(
                "slice requires an array target",
            )),
        }
    }

    pub(super) fn evaluate_join(
        &self,
        target: &Expr,
        separator: &Expr,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        let separator = self.require_stringish(&self.evaluate(separator)?)?;
        match target {
            SignalValue::Array(items) => {
                let mut pieces = Vec::with_capacity(items.len());
                for item in items {
                    pieces.push(self.require_stringish(&item)?);
                }
                Ok(SignalValue::String(pieces.join(&separator)))
            }
            _ => Err(WORTHSignalJsError::invalid_input(
                "join requires an array target",
            )),
        }
    }

    pub(super) fn evaluate_flatten(
        &self,
        target: &Expr,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Array(items) => {
                let mut flattened = Vec::new();
                for item in items {
                    match item {
                        SignalValue::Array(nested) => flattened.extend(nested),
                        _ => {
                            return Err(WORTHSignalJsError::invalid_input(
                                "flatten requires an array of arrays",
                            ))
                        }
                    }
                }
                Ok(SignalValue::Array(flattened))
            }
            _ => Err(WORTHSignalJsError::invalid_input(
                "flatten requires an array target",
            )),
        }
    }

    pub(super) fn evaluate_object(
        &self,
        fields: &[(String, Expr)],
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let mut resolved = Vec::with_capacity(fields.len());
        for (name, value) in fields {
            resolved.push((name.clone(), self.evaluate(value)?));
        }
        Ok(SignalValue::Object(resolved))
    }

    pub(super) fn evaluate_array(&self, items: &[Expr]) -> Result<SignalValue, WORTHSignalJsError> {
        let mut resolved = Vec::with_capacity(items.len());
        for item in items {
            resolved.push(self.evaluate(item)?);
        }
        Ok(SignalValue::Array(resolved))
    }

    pub(super) fn evaluate_length(&self, target: &Expr) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Array(items) => Ok(SignalValue::Number(items.len() as f64)),
            SignalValue::Object(fields) => Ok(SignalValue::Number(fields.len() as f64)),
            SignalValue::String(text) => Ok(SignalValue::Number(text.len() as f64)),
            _ => Err(WORTHSignalJsError::invalid_input(
                "length requires an array, object, or string target",
            )),
        }
    }

    pub(super) fn evaluate_contains(
        &self,
        target: &Expr,
        value: &Expr,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        let value = self.evaluate(value)?;
        let contains = match target {
            SignalValue::Array(items) => items.contains(&value),
            SignalValue::Object(fields) => match value {
                SignalValue::String(key) => fields.iter().any(|(name, _)| name == &key),
                _ => false,
            },
            SignalValue::String(text) => match value {
                SignalValue::String(needle) => text.contains(&needle),
                _ => false,
            },
            _ => {
                return Err(WORTHSignalJsError::invalid_input(
                    "contains requires an array, object, or string target",
                ))
            }
        };
        Ok(SignalValue::Bool(contains))
    }

    pub(super) fn evaluate_merge_objects(
        &self,
        args: &[Expr],
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let mut merged = Vec::<(String, SignalValue)>::new();
        for arg in args {
            let value = self.evaluate(arg)?;
            match value {
                SignalValue::Object(fields) => {
                    for (name, value) in fields {
                        if let Some(existing) =
                            merged.iter_mut().find(|(existing, _)| existing == &name)
                        {
                            existing.1 = value;
                        } else {
                            merged.push((name, value));
                        }
                    }
                }
                _ => {
                    return Err(WORTHSignalJsError::invalid_input(
                        "mergeObjects requires object inputs",
                    ))
                }
            }
        }
        Ok(SignalValue::Object(merged))
    }

    pub(super) fn evaluate_keys(&self, target: &Expr) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Object(fields) => Ok(SignalValue::Array(
                fields
                    .into_iter()
                    .map(|(name, _)| SignalValue::String(name))
                    .collect(),
            )),
            _ => Err(WORTHSignalJsError::invalid_input(
                "keys requires an object target",
            )),
        }
    }

    pub(super) fn evaluate_values(&self, target: &Expr) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Object(fields) => Ok(SignalValue::Array(
                fields.into_iter().map(|(_, value)| value).collect(),
            )),
            _ => Err(WORTHSignalJsError::invalid_input(
                "values requires an object target",
            )),
        }
    }

    pub(super) fn evaluate_has_field(
        &self,
        target: &Expr,
        field: &str,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Object(fields) => Ok(SignalValue::Bool(
                fields.iter().any(|(name, _)| name == field),
            )),
            _ => Err(WORTHSignalJsError::invalid_input(
                "hasField requires an object target",
            )),
        }
    }

    pub(super) fn evaluate_pick(
        &self,
        target: &Expr,
        fields: &[String],
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Object(entries) => Ok(SignalValue::Object(
                entries
                    .into_iter()
                    .filter(|(name, _)| fields.iter().any(|field| field == name))
                    .collect(),
            )),
            _ => Err(WORTHSignalJsError::invalid_input(
                "pick requires an object target",
            )),
        }
    }

    pub(super) fn evaluate_omit(
        &self,
        target: &Expr,
        fields: &[String],
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        match target {
            SignalValue::Object(entries) => Ok(SignalValue::Object(
                entries
                    .into_iter()
                    .filter(|(name, _)| !fields.iter().any(|field| field == name))
                    .collect(),
            )),
            _ => Err(WORTHSignalJsError::invalid_input(
                "omit requires an object target",
            )),
        }
    }

    pub(super) fn evaluate_append(
        &self,
        target: &Expr,
        value: &Expr,
    ) -> Result<SignalValue, WORTHSignalJsError> {
        let target = self.evaluate(target)?;
        let value = self.evaluate(value)?;
        match target {
            SignalValue::Array(mut items) => {
                items.push(value);
                Ok(SignalValue::Array(items))
            }
            _ => Err(WORTHSignalJsError::invalid_input(
                "append requires an array target",
            )),
        }
    }
}
