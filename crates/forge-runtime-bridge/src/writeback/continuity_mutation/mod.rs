use std::sync::Arc;

use crate::continuity::{
    BridgeContinuityClass, BridgeContinuityOutcomeClass, BridgeContinuityRejectionClass,
};

mod digest_basis;
mod identity_input;

use digest_basis::{
    derive_lineage_digest, derive_optional_binding_basis_digest, derive_resolution_digest,
};
use identity_input::normalize_successor_set;

pub use identity_input::{
    BridgeContinuityAuthoritativeIdentity, BridgeContinuityResolvedTargetIdentity,
    BridgeContinuityTargetCollection,
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
    pub(super) fn new(message: impl Into<String>) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityMutationBundle {
    family: BridgeContinuityMutationFamily,
    outcome_class: BridgeContinuityOutcomeClass,
    prior_authoritative_identity: BridgeContinuityAuthoritativeIdentity,
    successor_authoritative_identities: Vec<BridgeContinuityAuthoritativeIdentity>,
    basis_binding_digest: Option<Arc<str>>,
    resolved_target_entity_identity: Option<BridgeContinuityResolvedTargetIdentity>,
    target_collection: Option<BridgeContinuityTargetCollection>,
    lineage_digest: Arc<str>,
    continuity_resolution_digest: Arc<str>,
}

impl BridgeContinuityMutationBundle {
    pub fn rebind_existing_target(
        outcome_class: BridgeContinuityOutcomeClass,
        prior_authoritative_identity: BridgeContinuityAuthoritativeIdentity,
        successor_authoritative_identity: Option<BridgeContinuityAuthoritativeIdentity>,
        resolved_target_entity_identity: Option<BridgeContinuityResolvedTargetIdentity>,
        target_collection: Option<BridgeContinuityTargetCollection>,
    ) -> Result<Self, BridgeContinuityMutationBundleError> {
        let family = BridgeContinuityMutationFamily::RebindExistingTarget;
        Ok(Self::from_semantic_evidence(
            family,
            outcome_class,
            prior_authoritative_identity,
            successor_authoritative_identity
                .into_iter()
                .collect::<Vec<_>>(),
            resolved_target_entity_identity,
            target_collection,
        ))
    }

    pub fn split_existing_target(
        outcome_class: BridgeContinuityOutcomeClass,
        prior_authoritative_identity: BridgeContinuityAuthoritativeIdentity,
        successor_authoritative_identities: impl IntoIterator<
            Item = BridgeContinuityAuthoritativeIdentity,
        >,
        resolved_target_entity_identity: Option<BridgeContinuityResolvedTargetIdentity>,
        target_collection: Option<BridgeContinuityTargetCollection>,
    ) -> Result<Self, BridgeContinuityMutationBundleError> {
        let family = BridgeContinuityMutationFamily::SplitExistingTarget;
        let successor_authoritative_identities =
            normalize_successor_set(successor_authoritative_identities)?;

        Ok(Self::from_semantic_evidence(
            family,
            outcome_class,
            prior_authoritative_identity,
            successor_authoritative_identities,
            resolved_target_entity_identity,
            target_collection,
        ))
    }

    fn from_semantic_evidence(
        family: BridgeContinuityMutationFamily,
        outcome_class: BridgeContinuityOutcomeClass,
        prior_authoritative_identity: BridgeContinuityAuthoritativeIdentity,
        successor_authoritative_identities: Vec<BridgeContinuityAuthoritativeIdentity>,
        resolved_target_entity_identity: Option<BridgeContinuityResolvedTargetIdentity>,
        target_collection: Option<BridgeContinuityTargetCollection>,
    ) -> Self {
        let basis_binding_digest = derive_optional_binding_basis_digest(
            prior_authoritative_identity.as_str(),
            resolved_target_entity_identity
                .as_ref()
                .map(BridgeContinuityResolvedTargetIdentity::as_str),
            target_collection
                .as_ref()
                .map(BridgeContinuityTargetCollection::as_str),
        );
        let lineage_digest = derive_lineage_digest(
            family,
            outcome_class,
            prior_authoritative_identity.as_str(),
            &successor_authoritative_identities,
        );
        let continuity_resolution_digest = derive_resolution_digest(
            family,
            outcome_class,
            prior_authoritative_identity.as_str(),
            &successor_authoritative_identities,
            basis_binding_digest.as_ref(),
            resolved_target_entity_identity
                .as_ref()
                .map(BridgeContinuityResolvedTargetIdentity::as_str),
            target_collection
                .as_ref()
                .map(BridgeContinuityTargetCollection::as_str),
        );

        Self {
            family,
            outcome_class,
            prior_authoritative_identity,
            successor_authoritative_identities,
            basis_binding_digest,
            resolved_target_entity_identity,
            target_collection,
            lineage_digest,
            continuity_resolution_digest,
        }
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
        self.prior_authoritative_identity.as_str()
    }

    pub fn successor_authoritative_identity(&self) -> Option<&str> {
        match self.successor_authoritative_identities.as_slice() {
            [only] => Some(only.as_str()),
            _ => None,
        }
    }

    pub fn successor_authoritative_identities(&self) -> &[BridgeContinuityAuthoritativeIdentity] {
        &self.successor_authoritative_identities
    }

    pub fn basis_binding_digest(&self) -> Option<&str> {
        self.basis_binding_digest.as_deref()
    }

    pub fn resolved_target_entity_identity(&self) -> Option<&str> {
        self.resolved_target_entity_identity
            .as_ref()
            .map(BridgeContinuityResolvedTargetIdentity::as_str)
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection
            .as_ref()
            .map(BridgeContinuityTargetCollection::as_str)
    }

    pub fn lineage_digest(&self) -> &str {
        self.lineage_digest.as_ref()
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        self.continuity_resolution_digest.as_ref()
    }
}
