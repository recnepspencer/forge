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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryCancellationSafePointFamily(String);

impl WorthQueryCancellationSafePointFamily {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("empty-cancellation-safe-point-family");
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
