use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCounters, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeclarationIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDeclarationRejectionKind {
    UnsupportedSliceKindForFamily,
    EmptyDeclaration,
    CanonicalizationFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeclarationRejection {
    declaration_identity: BridgeSubscriptionDeclarationIdentity,
    requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
    rejection_kind: BridgeSubscriptionDeclarationRejectionKind,
    normalized_slice_intent_count: usize,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeclarationRejection {
    pub(crate) fn new(
        declaration_identity: BridgeSubscriptionDeclarationIdentity,
        requested_family_kind: BridgeSubscriptionDeclarationFamilyKind,
        rejection_kind: BridgeSubscriptionDeclarationRejectionKind,
        declaration_input_slice_intent_count: usize,
        normalized_slice_intent_count: usize,
        declaration_deduplicated_slice_intent_count: usize,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-declaration-rejection|id={}|family={}|kind:{rejection_kind:?}|normalized-slice-intent-count={}",
            declaration_identity.as_str(),
            requested_family_kind.as_str(),
            normalized_slice_intent_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_identity,
            requested_family_kind,
            rejection_kind,
            normalized_slice_intent_count,
            counters: BridgeSubscriptionCounters::from_rejection(
                declaration_input_slice_intent_count,
                normalized_slice_intent_count,
                declaration_deduplicated_slice_intent_count,
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-declaration-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn declaration_identity(&self) -> &BridgeSubscriptionDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn requested_family_kind(&self) -> BridgeSubscriptionDeclarationFamilyKind {
        self.requested_family_kind
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionDeclarationRejectionKind {
        self.rejection_kind
    }

    pub fn normalized_slice_intent_count(&self) -> usize {
        self.normalized_slice_intent_count
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
