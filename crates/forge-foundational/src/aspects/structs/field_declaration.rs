use crate::aspects::contracts::AbsenceLaw;
use crate::aspects::evolution::AspectEvolutionPolicy;
use crate::values::ScalarAspectType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldKey(String);

impl FieldKey {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.trim().is_empty() || value.contains('.') || value.contains(char::is_whitespace) {
            None
        } else {
            Some(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FieldRequirement {
    Required,
    Optional,
    Defaulted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclaration {
    key: FieldKey,
    value_type: ScalarAspectType,
    requirement: FieldRequirement,
    absence: AbsenceLaw,
    evolution: AspectEvolutionPolicy,
}

impl FieldDeclaration {
    pub fn new(
        key: FieldKey,
        value_type: ScalarAspectType,
        requirement: FieldRequirement,
        absence: AbsenceLaw,
        evolution: AspectEvolutionPolicy,
    ) -> Self {
        Self {
            key,
            value_type,
            requirement,
            absence,
            evolution,
        }
    }

    pub fn key(&self) -> &FieldKey {
        &self.key
    }

    pub fn value_type(&self) -> ScalarAspectType {
        self.value_type
    }

    pub fn requirement(&self) -> FieldRequirement {
        self.requirement
    }

    pub fn absence(&self) -> AbsenceLaw {
        self.absence
    }

    pub fn evolution(&self) -> AspectEvolutionPolicy {
        self.evolution
    }
}
