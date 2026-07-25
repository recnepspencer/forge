#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryExecutionMode {
    Synchronous,
    Asynchronous,
}

impl WorthQueryExecutionMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Synchronous => "synchronous",
            Self::Asynchronous => "asynchronous",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryExecutionDegradation {
    PartialResult,
    YieldedProgress,
    RetainedProgress,
}

impl WorthQueryExecutionDegradation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PartialResult => "partial-result",
            Self::YieldedProgress => "yielded-progress",
            Self::RetainedProgress => "retained-progress",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryPartialEffectPosture {
    EffectFree,
    PartialEffectsMayRemain,
}

impl WorthQueryPartialEffectPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectFree => "effect-free",
            Self::PartialEffectsMayRemain => "partial-effects-may-remain",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryCancellationSafePointFamily(String);

impl WorthQueryCancellationSafePointFamily {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("empty-cancellation-safe-point-family");
        }
        if value.trim() != value || value.chars().any(char::is_control) {
            return Err("invalid-cancellation-safe-point-family");
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::WorthQueryCancellationSafePointFamily;

    #[test]
    fn safe_point_families_reject_nonportable_boundaries() {
        assert!(WorthQueryCancellationSafePointFamily::new(" chunk").is_err());
        assert!(WorthQueryCancellationSafePointFamily::new("chunk\nboundary").is_err());
    }
}
