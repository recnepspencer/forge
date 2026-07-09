use std::collections::BTreeMap;

use crate::boundary::errors::WORTHSignalJsError;

use super::super::model::{Expr, SignalValue};

#[derive(Debug, Clone)]
pub struct ExprEnvironment<'a> {
    pub(super) reads: &'a BTreeMap<String, SignalValue>,
}

impl<'a> ExprEnvironment<'a> {
    pub fn new(reads: &'a BTreeMap<String, SignalValue>) -> Self {
        Self { reads }
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<SignalValue, WORTHSignalJsError> {
        match expr {
            Expr::Value { value } => Ok(value.clone()),
            Expr::Read { id } => {
                self.reads.get(id).cloned().ok_or_else(|| {
                    WORTHSignalJsError::invalid_input(format!("unknown read `{id}`"))
                })
            }
            Expr::Get { target, field } => self.evaluate_get(target, field),
            Expr::At { target, index } => self.evaluate_at(target, index),
            Expr::First { target } => self.evaluate_first(target),
            Expr::Last { target } => self.evaluate_last(target),
            Expr::Slice { target, start, end } => {
                self.evaluate_slice(target, start, end.as_ref().map(|value| &**value))
            }
            Expr::Join { target, separator } => self.evaluate_join(target, separator),
            Expr::Flatten { target } => self.evaluate_flatten(target),
            Expr::Object { fields } => self.evaluate_object(fields),
            Expr::Array { items } => self.evaluate_array(items),
            Expr::Sum { args } => self.fold_numbers(args, 0.0, |left, right| left + right),
            Expr::Multiply { args } => self.fold_numbers(args, 1.0, |left, right| left * right),
            Expr::Concat { args } => self.evaluate_concat(args),
            Expr::Coalesce { args } => self.evaluate_coalesce(args),
            Expr::Length { target } => self.evaluate_length(target),
            Expr::Contains { target, value } => self.evaluate_contains(target, value),
            Expr::MergeObjects { args } => self.evaluate_merge_objects(args),
            Expr::Keys { target } => self.evaluate_keys(target),
            Expr::Values { target } => self.evaluate_values(target),
            Expr::HasField { target, field } => self.evaluate_has_field(target, field),
            Expr::Pick { target, fields } => self.evaluate_pick(target, fields),
            Expr::Omit { target, fields } => self.evaluate_omit(target, fields),
            Expr::Append { target, value } => self.evaluate_append(target, value),
            Expr::Abs { target } => self.evaluate_abs(target),
            Expr::Min { args } => self.extreme_number(args, f64::min, "min"),
            Expr::Max { args } => self.extreme_number(args, f64::max, "max"),
            Expr::Sqrt { target } => self.evaluate_sqrt(target),
            Expr::Sin { target } => self.evaluate_sin(target),
            Expr::Cos { target } => self.evaluate_cos(target),
            Expr::Floor { target } => self.evaluate_floor(target),
            Expr::Mod { left, right } => self.evaluate_mod(left, right),
            Expr::Clamp { value, min, max } => self.evaluate_clamp(value, min, max),
            Expr::Atan2 { y, x } => self.evaluate_atan2(y, x),
            Expr::Subtract { left, right } => self.evaluate_subtract(left, right),
            Expr::Divide { left, right } => self.evaluate_divide(left, right),
            Expr::Eq { left, right } => Ok(SignalValue::Bool(
                self.evaluate(left)? == self.evaluate(right)?,
            )),
            Expr::Neq { left, right } => Ok(SignalValue::Bool(
                self.evaluate(left)? != self.evaluate(right)?,
            )),
            Expr::Gt { left, right } => self.compare_numbers(left, right, |l, r| l > r),
            Expr::Gte { left, right } => self.compare_numbers(left, right, |l, r| l >= r),
            Expr::Lt { left, right } => self.compare_numbers(left, right, |l, r| l < r),
            Expr::Lte { left, right } => self.compare_numbers(left, right, |l, r| l <= r),
            Expr::And { args } => self.evaluate_and(args),
            Expr::Or { args } => self.evaluate_or(args),
            Expr::Not { arg } => Ok(SignalValue::Bool(!self.require_bool(&self.evaluate(arg)?)?)),
            Expr::If {
                condition,
                then_expr,
                else_expr,
            } => self.evaluate_if(condition, then_expr, else_expr),
        }
    }
}
