use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalMergeStrategyWitnessDenialKind {
    MissingMergeStrategyIdentity,
    MissingInvalidationStrategyIdentity,
    MissingDeliveryStrategyIdentity,
    EmptyDigestField,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalMergeStrategyWitnessDenial {
    kind: SignalMergeStrategyWitnessDenialKind,
    message: String,
}

impl SignalMergeStrategyWitnessDenial {
    pub(crate) fn new(
        kind: SignalMergeStrategyWitnessDenialKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &SignalMergeStrategyWitnessDenialKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

pub(crate) fn ensure_non_empty_digest(
    label: &str,
    digest: &str,
    kind: SignalMergeStrategyWitnessDenialKind,
) -> Result<(), SignalMergeStrategyWitnessDenial> {
    if digest.trim().is_empty() {
        return Err(SignalMergeStrategyWitnessDenial::new(
            kind,
            format!("{label} digest must not be empty"),
        ));
    }
    Ok(())
}

pub(crate) fn canonical_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("strategy witness serialization");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
