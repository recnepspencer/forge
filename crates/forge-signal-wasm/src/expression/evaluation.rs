use std::collections::BTreeMap;

use crate::boundary::errors::ForgeSignalJsError;

use super::model::{Expr, SignalValue};

#[derive(Debug, Clone)]
pub struct ExprEnvironment<'a> {
    reads: &'a BTreeMap<String, SignalValue>,
}

impl<'a> ExprEnvironment<'a> {
    pub fn new(reads: &'a BTreeMap<String, SignalValue>) -> Self {
        Self { reads }
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<SignalValue, ForgeSignalJsError> {
        match expr {
            Expr::Value { value } => Ok(value.clone()),
            Expr::Read { id } => self
                .reads
                .get(id)
                .cloned()
                .ok_or_else(|| ForgeSignalJsError::invalid_input(format!("unknown read `{id}`"))),
            Expr::Get { target, field } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Object(fields) => fields
                        .into_iter()
                        .find(|(name, _)| name == field)
                        .map(|(_, value)| value)
                        .ok_or_else(|| {
                            ForgeSignalJsError::invalid_input(format!(
                                "field `{field}` was not present on object"
                            ))
                        }),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "field access requires an object target",
                    )),
                }
            }
            Expr::At { target, index } => {
                let target = self.evaluate(target)?;
                let index = self.require_number(&self.evaluate(index)?)? as usize;
                match target {
                    SignalValue::Array(items) => items.get(index).cloned().ok_or_else(|| {
                        ForgeSignalJsError::invalid_input(format!(
                            "array index `{index}` was out of bounds"
                        ))
                    }),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "index access requires an array target",
                    )),
                }
            }
            Expr::First { target } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Array(items) => items.into_iter().next().ok_or_else(|| {
                        ForgeSignalJsError::invalid_input("first requires a non-empty array target")
                    }),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "first requires an array target",
                    )),
                }
            }
            Expr::Last { target } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Array(items) => items.into_iter().last().ok_or_else(|| {
                        ForgeSignalJsError::invalid_input("last requires a non-empty array target")
                    }),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "last requires an array target",
                    )),
                }
            }
            Expr::Slice { target, start, end } => {
                let target = self.evaluate(target)?;
                let start = self.require_number(&self.evaluate(start)?)? as usize;
                match target {
                    SignalValue::Array(items) => {
                        let end = match end {
                            Some(end) => self.require_number(&self.evaluate(end)?)? as usize,
                            None => items.len(),
                        };
                        if start > end || end > items.len() {
                            return Err(ForgeSignalJsError::invalid_input(
                                "slice range was out of bounds",
                            ));
                        }
                        Ok(SignalValue::Array(items[start..end].to_vec()))
                    }
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "slice requires an array target",
                    )),
                }
            }
            Expr::Join { target, separator } => {
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
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "join requires an array target",
                    )),
                }
            }
            Expr::Flatten { target } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Array(items) => {
                        let mut flattened = Vec::new();
                        for item in items {
                            match item {
                                SignalValue::Array(nested) => flattened.extend(nested),
                                _ => {
                                    return Err(ForgeSignalJsError::invalid_input(
                                        "flatten requires an array of arrays",
                                    ))
                                }
                            }
                        }
                        Ok(SignalValue::Array(flattened))
                    }
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "flatten requires an array target",
                    )),
                }
            }
            Expr::Object { fields } => {
                let mut resolved = Vec::with_capacity(fields.len());
                for (name, value) in fields {
                    resolved.push((name.clone(), self.evaluate(value)?));
                }
                Ok(SignalValue::Object(resolved))
            }
            Expr::Array { items } => {
                let mut resolved = Vec::with_capacity(items.len());
                for item in items {
                    resolved.push(self.evaluate(item)?);
                }
                Ok(SignalValue::Array(resolved))
            }
            Expr::Sum { args } => self.fold_numbers(args, 0.0, |left, right| left + right),
            Expr::Multiply { args } => self.fold_numbers(args, 1.0, |left, right| left * right),
            Expr::Concat { args } => {
                let mut output = String::new();
                for arg in args {
                    output.push_str(&self.require_stringish(&self.evaluate(arg)?)?);
                }
                Ok(SignalValue::String(output))
            }
            Expr::Coalesce { args } => {
                for arg in args {
                    let value = self.evaluate(arg)?;
                    if !matches!(value, SignalValue::Null) {
                        return Ok(value);
                    }
                }
                Ok(SignalValue::Null)
            }
            Expr::Length { target } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Array(items) => Ok(SignalValue::Number(items.len() as f64)),
                    SignalValue::Object(fields) => Ok(SignalValue::Number(fields.len() as f64)),
                    SignalValue::String(text) => Ok(SignalValue::Number(text.len() as f64)),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "length requires an array, object, or string target",
                    )),
                }
            }
            Expr::Contains { target, value } => {
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
                        return Err(ForgeSignalJsError::invalid_input(
                            "contains requires an array, object, or string target",
                        ))
                    }
                };
                Ok(SignalValue::Bool(contains))
            }
            Expr::MergeObjects { args } => {
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
                            return Err(ForgeSignalJsError::invalid_input(
                                "mergeObjects requires object inputs",
                            ))
                        }
                    }
                }
                Ok(SignalValue::Object(merged))
            }
            Expr::Keys { target } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Object(fields) => Ok(SignalValue::Array(
                        fields
                            .into_iter()
                            .map(|(name, _)| SignalValue::String(name))
                            .collect(),
                    )),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "keys requires an object target",
                    )),
                }
            }
            Expr::Values { target } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Object(fields) => Ok(SignalValue::Array(
                        fields.into_iter().map(|(_, value)| value).collect(),
                    )),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "values requires an object target",
                    )),
                }
            }
            Expr::HasField { target, field } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Object(fields) => Ok(SignalValue::Bool(
                        fields.iter().any(|(name, _)| name == field),
                    )),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "hasField requires an object target",
                    )),
                }
            }
            Expr::Pick { target, fields } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Object(entries) => Ok(SignalValue::Object(
                        entries
                            .into_iter()
                            .filter(|(name, _)| fields.iter().any(|field| field == name))
                            .collect(),
                    )),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "pick requires an object target",
                    )),
                }
            }
            Expr::Omit { target, fields } => {
                let target = self.evaluate(target)?;
                match target {
                    SignalValue::Object(entries) => Ok(SignalValue::Object(
                        entries
                            .into_iter()
                            .filter(|(name, _)| !fields.iter().any(|field| field == name))
                            .collect(),
                    )),
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "omit requires an object target",
                    )),
                }
            }
            Expr::Append { target, value } => {
                let target = self.evaluate(target)?;
                let value = self.evaluate(value)?;
                match target {
                    SignalValue::Array(mut items) => {
                        items.push(value);
                        Ok(SignalValue::Array(items))
                    }
                    _ => Err(ForgeSignalJsError::invalid_input(
                        "append requires an array target",
                    )),
                }
            }
            Expr::Abs { target } => Ok(SignalValue::Number(
                self.require_number(&self.evaluate(target)?)?.abs(),
            )),
            Expr::Min { args } => self.extreme_number(args, f64::min, "min"),
            Expr::Max { args } => self.extreme_number(args, f64::max, "max"),
            Expr::Sqrt { target } => {
                let value = self.require_number(&self.evaluate(target)?)?;
                if value < 0.0 {
                    return Err(ForgeSignalJsError::invalid_input(
                        "sqrt requires a non-negative input",
                    ));
                }
                Ok(SignalValue::Number(value.sqrt()))
            }
            Expr::Sin { target } => Ok(SignalValue::Number(
                self.require_number(&self.evaluate(target)?)?.sin(),
            )),
            Expr::Cos { target } => Ok(SignalValue::Number(
                self.require_number(&self.evaluate(target)?)?.cos(),
            )),
            Expr::Floor { target } => Ok(SignalValue::Number(
                self.require_number(&self.evaluate(target)?)?.floor(),
            )),
            Expr::Mod { left, right } => {
                let divisor = self.require_number(&self.evaluate(right)?)?;
                if divisor == 0.0 {
                    return Err(ForgeSignalJsError::invalid_input("mod by zero"));
                }
                Ok(SignalValue::Number(
                    self.require_number(&self.evaluate(left)?)? % divisor,
                ))
            }
            Expr::Clamp { value, min, max } => {
                let value = self.require_number(&self.evaluate(value)?)?;
                let min = self.require_number(&self.evaluate(min)?)?;
                let max = self.require_number(&self.evaluate(max)?)?;
                if min > max {
                    return Err(ForgeSignalJsError::invalid_input(
                        "clamp requires min <= max",
                    ));
                }
                Ok(SignalValue::Number(value.clamp(min, max)))
            }
            Expr::Atan2 { y, x } => Ok(SignalValue::Number(
                self.require_number(&self.evaluate(y)?)?
                    .atan2(self.require_number(&self.evaluate(x)?)?),
            )),
            Expr::Subtract { left, right } => Ok(SignalValue::Number(
                self.require_number(&self.evaluate(left)?)?
                    - self.require_number(&self.evaluate(right)?)?,
            )),
            Expr::Divide { left, right } => {
                let denominator = self.require_number(&self.evaluate(right)?)?;
                if denominator == 0.0 {
                    return Err(ForgeSignalJsError::invalid_input("division by zero"));
                }
                Ok(SignalValue::Number(
                    self.require_number(&self.evaluate(left)?)? / denominator,
                ))
            }
            Expr::Eq { left, right } => Ok(SignalValue::Bool(self.evaluate(left)? == self.evaluate(right)?)),
            Expr::Neq { left, right } => {
                Ok(SignalValue::Bool(self.evaluate(left)? != self.evaluate(right)?))
            }
            Expr::Gt { left, right } => self.compare_numbers(left, right, |l, r| l > r),
            Expr::Gte { left, right } => self.compare_numbers(left, right, |l, r| l >= r),
            Expr::Lt { left, right } => self.compare_numbers(left, right, |l, r| l < r),
            Expr::Lte { left, right } => self.compare_numbers(left, right, |l, r| l <= r),
            Expr::And { args } => {
                for arg in args {
                    if !self.require_bool(&self.evaluate(arg)?)? {
                        return Ok(SignalValue::Bool(false));
                    }
                }
                Ok(SignalValue::Bool(true))
            }
            Expr::Or { args } => {
                for arg in args {
                    if self.require_bool(&self.evaluate(arg)?)? {
                        return Ok(SignalValue::Bool(true));
                    }
                }
                Ok(SignalValue::Bool(false))
            }
            Expr::Not { arg } => Ok(SignalValue::Bool(!self.require_bool(&self.evaluate(arg)?)?)),
            Expr::If {
                condition,
                then_expr,
                else_expr,
            } => {
                if self.require_bool(&self.evaluate(condition)?)? {
                    self.evaluate(then_expr)
                } else {
                    self.evaluate(else_expr)
                }
            }
        }
    }

    fn fold_numbers<F>(
        &self,
        args: &[Expr],
        seed: f64,
        fold: F,
    ) -> Result<SignalValue, ForgeSignalJsError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut total = seed;
        for arg in args {
            total = fold(total, self.require_number(&self.evaluate(arg)?)?);
        }
        Ok(SignalValue::Number(total))
    }

    fn compare_numbers<F>(
        &self,
        left: &Expr,
        right: &Expr,
        compare: F,
    ) -> Result<SignalValue, ForgeSignalJsError>
    where
        F: Fn(f64, f64) -> bool,
    {
        Ok(SignalValue::Bool(compare(
            self.require_number(&self.evaluate(left)?)?,
            self.require_number(&self.evaluate(right)?)?,
        )))
    }

    fn extreme_number<F>(
        &self,
        args: &[Expr],
        select: F,
        op_name: &'static str,
    ) -> Result<SignalValue, ForgeSignalJsError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut iter = args.iter();
        let Some(first) = iter.next() else {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "{op_name} requires at least one input"
            )));
        };
        let mut current = self.require_number(&self.evaluate(first)?)?;
        for arg in iter {
            current = select(current, self.require_number(&self.evaluate(arg)?)?);
        }
        Ok(SignalValue::Number(current))
    }

    fn require_bool(&self, value: &SignalValue) -> Result<bool, ForgeSignalJsError> {
        match value {
            SignalValue::Bool(value) => Ok(*value),
            _ => Err(ForgeSignalJsError::invalid_input(
                "expression expected a boolean value",
            )),
        }
    }

    fn require_number(&self, value: &SignalValue) -> Result<f64, ForgeSignalJsError> {
        match value {
            SignalValue::Number(value) => Ok(*value),
            _ => Err(ForgeSignalJsError::invalid_input(
                "expression expected a numeric value",
            )),
        }
    }

    fn require_stringish(&self, value: &SignalValue) -> Result<String, ForgeSignalJsError> {
        match value {
            SignalValue::String(value) => Ok(value.clone()),
            SignalValue::Number(value) => Ok(value.to_string()),
            SignalValue::Bool(value) => Ok(value.to_string()),
            SignalValue::Null => Ok("null".to_owned()),
            _ => Err(ForgeSignalJsError::invalid_input(
                "concat requires string, number, boolean, or null inputs",
            )),
        }
    }
}
