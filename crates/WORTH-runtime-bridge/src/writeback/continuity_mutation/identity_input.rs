use std::sync::Arc;

use crate::identity::BridgeIdentityEvidence;

use super::BridgeContinuityMutationBundleError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityAuthoritativeIdentity {
    value: Arc<str>,
}

impl BridgeContinuityAuthoritativeIdentity {
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, BridgeContinuityMutationBundleError> {
        let value = normalize_continuity_text("authoritative identity", value)?;
        require_native_identity_projection("authoritative identity", value.as_ref())?;
        Ok(Self { value })
    }

    pub fn from_bridge_evidence(evidence_identity: &BridgeIdentityEvidence) -> Self {
        Self {
            value: Arc::from(format!(
                "bridge-continuity-authoritative:{}",
                evidence_identity.as_str()
            )),
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityResolvedTargetIdentity {
    value: Arc<str>,
}

impl BridgeContinuityResolvedTargetIdentity {
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, BridgeContinuityMutationBundleError> {
        Ok(Self {
            value: normalize_continuity_text("resolved target identity", value)?,
        })
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityTargetCollection {
    value: Arc<str>,
}

impl BridgeContinuityTargetCollection {
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, BridgeContinuityMutationBundleError> {
        Ok(Self {
            value: normalize_continuity_text("target collection", value)?,
        })
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}

pub(super) fn normalize_successor_set(
    successors: impl IntoIterator<Item = BridgeContinuityAuthoritativeIdentity>,
) -> Result<Vec<BridgeContinuityAuthoritativeIdentity>, BridgeContinuityMutationBundleError> {
    let successors = successors.into_iter().collect::<Vec<_>>();
    if successors.len() < 2 {
        return Err(BridgeContinuityMutationBundleError::new(
            "split-successor continuity requires at least two successor authoritative identities",
        ));
    }
    Ok(successors)
}

fn normalize_continuity_text(
    field_name: &str,
    value: impl Into<Arc<str>>,
) -> Result<Arc<str>, BridgeContinuityMutationBundleError> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(BridgeContinuityMutationBundleError::new(format!(
            "{field_name} must not be empty"
        )));
    }
    Ok(Arc::from(trimmed.to_owned()))
}

fn require_native_identity_projection(
    field_name: &str,
    value: &str,
) -> Result<(), BridgeContinuityMutationBundleError> {
    let mut segments = value.split(':');
    let first = segments.next();
    let second = segments.next();
    if first == Some("authority") && second == Some("sha256") {
        return Err(BridgeContinuityMutationBundleError::new(format!(
            "{field_name} must be a native continuity identity, not a derived digest projection"
        )));
    }
    Ok(())
}
