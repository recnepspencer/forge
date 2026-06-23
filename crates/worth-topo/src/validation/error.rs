use crate::validation::rule_registry::rule_identity_for_validator;
use crate::validation::TopologyValidationRuleIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyValidationError {
    validator: &'static str,
    rule_identity: Option<TopologyValidationRuleIdentity>,
    message: String,
}

impl TopologyValidationError {
    pub fn new(validator: &'static str, message: impl Into<String>) -> Self {
        Self {
            validator,
            rule_identity: rule_identity_for_validator(validator),
            message: message.into(),
        }
    }

    pub fn validator(&self) -> &'static str {
        self.validator
    }

    pub fn rule_identity(&self) -> Option<&TopologyValidationRuleIdentity> {
        self.rule_identity.as_ref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for TopologyValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.validator, self.message)
    }
}

impl std::error::Error for TopologyValidationError {}
