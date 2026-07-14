use std::collections::BTreeSet;
use std::sync::Arc;

use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::mapping::SubscriptionSliceKind;

use super::declaration_identity::BridgeSubscriptionDeclarationFamilyIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionDeclarationFamilyKind {
    DetailExact,
    CollectionMembership,
}

impl BridgeSubscriptionDeclarationFamilyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DetailExact => "detail_exact",
            Self::CollectionMembership => "collection_membership",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeclarationFamily {
    family_identity: BridgeSubscriptionDeclarationFamilyIdentity,
    family_kind: BridgeSubscriptionDeclarationFamilyKind,
    family_label: Arc<str>,
    supported_slice_kinds: Arc<[SubscriptionSliceKind]>,
    allows_delivery_intent_identity: bool,
}

impl BridgeSubscriptionDeclarationFamily {
    pub(crate) fn new(
        family_identity: BridgeSubscriptionDeclarationFamilyIdentity,
        family_kind: BridgeSubscriptionDeclarationFamilyKind,
        family_label: impl Into<Arc<str>>,
        supported_slice_kinds: Vec<SubscriptionSliceKind>,
        allows_delivery_intent_identity: bool,
    ) -> Result<Self, BridgeBuildError> {
        if supported_slice_kinds.is_empty() {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                format!(
                    "Bridge subscription family `{}` must admit at least one supported slice kind.",
                    family_kind.as_str()
                ),
            ));
        }

        let unique_slice_kinds = supported_slice_kinds
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if unique_slice_kinds.len() != supported_slice_kinds.len() {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::BuilderConfigurationConflict,
                format!(
                    "Bridge subscription family `{}` contained duplicate supported slice kinds.",
                    family_kind.as_str()
                ),
            ));
        }

        Ok(Self {
            family_identity,
            family_kind,
            family_label: family_label.into(),
            supported_slice_kinds: unique_slice_kinds.into_iter().collect::<Vec<_>>().into(),
            allows_delivery_intent_identity,
        })
    }

    pub fn family_identity(&self) -> &BridgeSubscriptionDeclarationFamilyIdentity {
        &self.family_identity
    }

    pub fn family_kind(&self) -> BridgeSubscriptionDeclarationFamilyKind {
        self.family_kind
    }

    pub fn family_label(&self) -> &str {
        self.family_label.as_ref()
    }

    pub fn supported_slice_kinds(&self) -> &[SubscriptionSliceKind] {
        &self.supported_slice_kinds
    }

    pub fn allows_delivery_intent_identity(&self) -> bool {
        self.allows_delivery_intent_identity
    }
}
