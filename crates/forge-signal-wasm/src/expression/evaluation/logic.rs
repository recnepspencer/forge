use crate::boundary::errors::ForgeSignalJsError;

use super::super::model::{Expr, SignalValue};
use super::environment::ExprEnvironment;

impl<'a> ExprEnvironment<'a> {
    pub(super) fn evaluate_and(&self, args: &[Expr]) -> Result<SignalValue, ForgeSignalJsError> {
        for arg in args {
            if !self.require_bool(&self.evaluate(arg)?)? {
                return Ok(SignalValue::Bool(false));
            }
        }
        Ok(SignalValue::Bool(true))
    }

    pub(super) fn evaluate_or(&self, args: &[Expr]) -> Result<SignalValue, ForgeSignalJsError> {
        for arg in args {
            if self.require_bool(&self.evaluate(arg)?)? {
                return Ok(SignalValue::Bool(true));
            }
        }
        Ok(SignalValue::Bool(false))
    }

    pub(super) fn evaluate_if(
        &self,
        condition: &Expr,
        then_expr: &Expr,
        else_expr: &Expr,
    ) -> Result<SignalValue, ForgeSignalJsError> {
        if self.require_bool(&self.evaluate(condition)?)? {
            self.evaluate(then_expr)
        } else {
            self.evaluate(else_expr)
        }
    }

    pub(super) fn compare_numbers<F>(
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

    pub(super) fn require_bool(&self, value: &SignalValue) -> Result<bool, ForgeSignalJsError> {
        match value {
            SignalValue::Bool(value) => Ok(*value),
            _ => Err(ForgeSignalJsError::invalid_input(
                "expression expected a boolean value",
            )),
        }
    }

    pub(super) fn require_number(&self, value: &SignalValue) -> Result<f64, ForgeSignalJsError> {
        match value {
            SignalValue::Number(value) => Ok(*value),
            _ => Err(ForgeSignalJsError::invalid_input(
                "expression expected a numeric value",
            )),
        }
    }

    pub(super) fn require_stringish(
        &self,
        value: &SignalValue,
    ) -> Result<String, ForgeSignalJsError> {
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
