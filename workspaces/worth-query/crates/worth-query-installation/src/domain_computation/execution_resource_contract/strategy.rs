use super::{WorthQueryExecutionProviderRequirements, WorthQueryExecutionResourceEnvelope};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryExecutionStrategyName(String);

impl WorthQueryExecutionStrategyName {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("empty-execution-strategy-name");
        }
        if value.trim() != value || value.chars().any(char::is_control) {
            return Err("invalid-execution-strategy-name");
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionStrategyContract {
    name: WorthQueryExecutionStrategyName,
    envelope: WorthQueryExecutionResourceEnvelope,
    provider_requirements: WorthQueryExecutionProviderRequirements,
}

impl WorthQueryExecutionStrategyContract {
    pub fn new(
        name: WorthQueryExecutionStrategyName,
        envelope: WorthQueryExecutionResourceEnvelope,
        provider_requirements: WorthQueryExecutionProviderRequirements,
    ) -> Self {
        Self {
            name,
            envelope,
            provider_requirements,
        }
    }

    pub fn name(&self) -> &WorthQueryExecutionStrategyName {
        &self.name
    }

    pub fn envelope(&self) -> &WorthQueryExecutionResourceEnvelope {
        &self.envelope
    }

    pub fn provider_requirements(&self) -> &WorthQueryExecutionProviderRequirements {
        &self.provider_requirements
    }
}

#[cfg(test)]
mod tests {
    use super::WorthQueryExecutionStrategyName;

    #[test]
    fn strategy_names_reject_nonportable_boundaries() {
        assert!(WorthQueryExecutionStrategyName::new(" bounded").is_err());
        assert!(WorthQueryExecutionStrategyName::new("bounded\nstrategy").is_err());
    }
}
