#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum WorthQueryGraphObligationPreflightWitness {
    #[default]
    Missing,
    Satisfied {
        witness_digest: String,
    },
}

impl WorthQueryGraphObligationPreflightWitness {
    pub fn missing() -> Self {
        Self::Missing
    }

    pub fn satisfied(witness_digest: impl Into<String>) -> Self {
        Self::Satisfied {
            witness_digest: witness_digest.into(),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Satisfied { .. } => "satisfied",
        }
    }

    pub fn witness_digest(&self) -> Option<&str> {
        match self {
            Self::Missing => None,
            Self::Satisfied { witness_digest } => Some(witness_digest),
        }
    }

    pub fn is_satisfied(&self) -> bool {
        matches!(self, Self::Satisfied { .. })
    }
}
