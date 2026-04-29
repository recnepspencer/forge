use std::sync::Arc;

use crate::continuity::{
    BridgeContinuityClass, BridgeContinuityOutcomeClass, BridgeContinuityRejectionClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeContinuityMutationFamily {
    RebindExistingTarget,
    SplitExistingTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityMutationBundleError {
    message: Arc<str>,
}

impl BridgeContinuityMutationBundleError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: Arc::from(message.into()),
        }
    }
}

impl std::fmt::Display for BridgeContinuityMutationBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_ref())
    }
}

impl std::error::Error for BridgeContinuityMutationBundleError {}

fn normalize_successors(
    successors: impl IntoIterator<Item = impl Into<String>>,
) -> Result<Vec<Arc<str>>, BridgeContinuityMutationBundleError> {
    let successors = successors
        .into_iter()
        .map(Into::into)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(Arc::<str>::from)
        .collect::<Vec<_>>();
    if successors.len() < 2 {
        return Err(BridgeContinuityMutationBundleError::new(
            "split-successor continuity requires at least two successor authoritative identities",
        ));
    }
    Ok(successors)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityMutationBundle {
    family: BridgeContinuityMutationFamily,
    outcome_class: BridgeContinuityOutcomeClass,
    prior_authoritative_identity: Arc<str>,
    successor_authoritative_identities: Vec<Arc<str>>,
    basis_binding_digest: Option<Arc<str>>,
    resolved_target_entity_identity: Option<Arc<str>>,
    target_collection: Option<Arc<str>>,
    lineage_digest: Arc<str>,
    continuity_resolution_digest: Arc<str>,
}

impl BridgeContinuityMutationBundle {
    pub fn rebind_existing_target(
        outcome_class: BridgeContinuityOutcomeClass,
        prior_authoritative_identity: impl Into<Arc<str>>,
        successor_authoritative_identity: Option<&str>,
        basis_binding_digest: Option<&str>,
        resolved_target_entity_identity: Option<&str>,
        target_collection: Option<&str>,
        lineage_digest: impl Into<Arc<str>>,
        continuity_resolution_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            family: BridgeContinuityMutationFamily::RebindExistingTarget,
            outcome_class,
            prior_authoritative_identity: prior_authoritative_identity.into(),
            successor_authoritative_identities: successor_authoritative_identity
                .into_iter()
                .map(|value| Arc::from(value.to_owned()))
                .collect(),
            basis_binding_digest: basis_binding_digest.map(|value| Arc::from(value.to_owned())),
            resolved_target_entity_identity: resolved_target_entity_identity
                .map(|value| Arc::from(value.to_owned())),
            target_collection: target_collection.map(|value| Arc::from(value.to_owned())),
            lineage_digest: lineage_digest.into(),
            continuity_resolution_digest: continuity_resolution_digest.into(),
        }
    }

    pub fn split_existing_target(
        outcome_class: BridgeContinuityOutcomeClass,
        prior_authoritative_identity: impl Into<Arc<str>>,
        successor_authoritative_identities: impl IntoIterator<Item = impl Into<String>>,
        basis_binding_digest: Option<&str>,
        resolved_target_entity_identity: Option<&str>,
        target_collection: Option<&str>,
        lineage_digest: impl Into<Arc<str>>,
        continuity_resolution_digest: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeContinuityMutationBundleError> {
        Ok(Self {
            family: BridgeContinuityMutationFamily::SplitExistingTarget,
            outcome_class,
            prior_authoritative_identity: prior_authoritative_identity.into(),
            successor_authoritative_identities: normalize_successors(
                successor_authoritative_identities,
            )?,
            basis_binding_digest: basis_binding_digest.map(|value| Arc::from(value.to_owned())),
            resolved_target_entity_identity: resolved_target_entity_identity
                .map(|value| Arc::from(value.to_owned())),
            target_collection: target_collection.map(|value| Arc::from(value.to_owned())),
            lineage_digest: lineage_digest.into(),
            continuity_resolution_digest: continuity_resolution_digest.into(),
        })
    }

    pub fn family(&self) -> BridgeContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> BridgeContinuityOutcomeClass {
        self.outcome_class
    }

    pub fn continuity_class(&self) -> Option<BridgeContinuityClass> {
        self.outcome_class.continued_class()
    }

    pub fn rejection_class(&self) -> Option<BridgeContinuityRejectionClass> {
        self.outcome_class.rejection_class()
    }

    pub fn prior_authoritative_identity(&self) -> &str {
        self.prior_authoritative_identity.as_ref()
    }

    pub fn successor_authoritative_identity(&self) -> Option<&str> {
        match self.successor_authoritative_identities.as_slice() {
            [only] => Some(only.as_ref()),
            _ => None,
        }
    }

    pub fn successor_authoritative_identities(&self) -> &[Arc<str>] {
        &self.successor_authoritative_identities
    }

    pub fn basis_binding_digest(&self) -> Option<&str> {
        self.basis_binding_digest.as_deref()
    }

    pub fn resolved_target_entity_identity(&self) -> Option<&str> {
        self.resolved_target_entity_identity.as_deref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn lineage_digest(&self) -> &str {
        self.lineage_digest.as_ref()
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        self.continuity_resolution_digest.as_ref()
    }
}
