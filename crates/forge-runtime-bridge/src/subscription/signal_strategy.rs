use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSignalStrategyIdentity, BridgeSubscriptionCounters, BridgeSubscriptionDeclaration,
    BridgeSubscriptionDeclarationFamilyKind, ValidatedSubscriptionBasisBinding,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSignalStrategyKind {
    ExactFieldLensObservation,
    CollectionMembershipObservation,
}

impl BridgeSignalStrategyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFieldLensObservation => "exact_field_lens_observation",
            Self::CollectionMembershipObservation => "collection_membership_observation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSignalStrategyDescriptor {
    strategy_identity: BridgeSignalStrategyIdentity,
    strategy_kind: BridgeSignalStrategyKind,
    family_kind: BridgeSubscriptionDeclarationFamilyKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSignalStrategyDescriptor {
    pub(crate) fn lower(
        declaration: &BridgeSubscriptionDeclaration,
        basis_binding: &ValidatedSubscriptionBasisBinding,
    ) -> Self {
        let strategy_kind = match declaration.requested_family_kind() {
            BridgeSubscriptionDeclarationFamilyKind::DetailExact => {
                BridgeSignalStrategyKind::ExactFieldLensObservation
            }
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership => {
                BridgeSignalStrategyKind::CollectionMembershipObservation
            }
        };

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-signal-strategy|declaration={}|basis={}|family={}|strategy={}|slice-count={}",
            declaration.digest(),
            basis_binding.digest(),
            declaration.requested_family_kind().as_str(),
            strategy_kind.as_str(),
            declaration.normalized_slice_intent_count(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            strategy_identity: BridgeSignalStrategyIdentity::new(format!(
                "bridge-subscription-signal-strategy-id:sha256:{digest:x}"
            )),
            strategy_kind,
            family_kind: declaration.requested_family_kind(),
            counters: BridgeSubscriptionCounters::from_signal_strategy_descriptor(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-signal-strategy:sha256:{digest:x}"
            )),
        }
    }

    pub fn strategy_identity(&self) -> &BridgeSignalStrategyIdentity {
        &self.strategy_identity
    }

    pub fn strategy_kind(&self) -> BridgeSignalStrategyKind {
        self.strategy_kind
    }

    pub fn family_kind(&self) -> BridgeSubscriptionDeclarationFamilyKind {
        self.family_kind
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
