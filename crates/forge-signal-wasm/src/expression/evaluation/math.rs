use crate::boundary::errors::ForgeSignalJsError;

use super::super::model::{Expr, SignalValue};
use super::environment::ExprEnvironment;

impl<'a> ExprEnvironment<'a> {
    pub(super) fn evaluate_concat(&self, args: &[Expr]) -> Result<SignalValue, ForgeSignalJsError> {
        let mut output = String::new();
        for arg in args {
            output.push_str(&self.require_stringish(&self.evaluate(arg)?)?);
        }
        Ok(SignalValue::String(output))
    }

    pub(super) fn evaluate_coalesce(
        &self,
        args: &[Expr],
    ) -> Result<SignalValue, ForgeSignalJsError> {
        for arg in args {
            let value = self.evaluate(arg)?;
            if !matches!(value, SignalValue::Null) {
                return Ok(value);
            }
        }
        Ok(SignalValue::Null)
    }

    pub(super) fn evaluate_abs(&self, target: &Expr) -> Result<SignalValue, ForgeSignalJsError> {
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(target)?)?.abs(),
        ))
    }

    pub(super) fn evaluate_sqrt(&self, target: &Expr) -> Result<SignalValue, ForgeSignalJsError> {
        let value = self.require_number(&self.evaluate(target)?)?;
        if value < 0.0 {
            return Err(ForgeSignalJsError::invalid_input(
                "sqrt requires a non-negative input",
            ));
        }
        Ok(SignalValue::Number(value.sqrt()))
    }

    pub(super) fn evaluate_sin(&self, target: &Expr) -> Result<SignalValue, ForgeSignalJsError> {
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(target)?)?.sin(),
        ))
    }

    pub(super) fn evaluate_cos(&self, target: &Expr) -> Result<SignalValue, ForgeSignalJsError> {
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(target)?)?.cos(),
        ))
    }

    pub(super) fn evaluate_floor(&self, target: &Expr) -> Result<SignalValue, ForgeSignalJsError> {
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(target)?)?.floor(),
        ))
    }

    pub(super) fn evaluate_mod(
        &self,
        left: &Expr,
        right: &Expr,
    ) -> Result<SignalValue, ForgeSignalJsError> {
        let divisor = self.require_number(&self.evaluate(right)?)?;
        if divisor == 0.0 {
            return Err(ForgeSignalJsError::invalid_input("mod by zero"));
        }
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(left)?)? % divisor,
        ))
    }

    pub(super) fn evaluate_clamp(
        &self,
        value: &Expr,
        min: &Expr,
        max: &Expr,
    ) -> Result<SignalValue, ForgeSignalJsError> {
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

    pub(super) fn evaluate_atan2(
        &self,
        y: &Expr,
        x: &Expr,
    ) -> Result<SignalValue, ForgeSignalJsError> {
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(y)?)?
                .atan2(self.require_number(&self.evaluate(x)?)?),
        ))
    }

    pub(super) fn evaluate_subtract(
        &self,
        left: &Expr,
        right: &Expr,
    ) -> Result<SignalValue, ForgeSignalJsError> {
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(left)?)?
                - self.require_number(&self.evaluate(right)?)?,
        ))
    }

    pub(super) fn evaluate_divide(
        &self,
        left: &Expr,
        right: &Expr,
    ) -> Result<SignalValue, ForgeSignalJsError> {
        let denominator = self.require_number(&self.evaluate(right)?)?;
        if denominator == 0.0 {
            return Err(ForgeSignalJsError::invalid_input("division by zero"));
        }
        Ok(SignalValue::Number(
            self.require_number(&self.evaluate(left)?)? / denominator,
        ))
    }

    pub(super) fn fold_numbers<F>(
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

    pub(super) fn extreme_number<F>(
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
}
