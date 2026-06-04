use std::sync::Arc;

use forge_foundational::facade::{AspectFieldLocator, AspectKey, AspectLocator};

use crate::input::envelope::BridgeCommittedPatchTarget;
use crate::mapping::{SubscriptionSliceKind, TruthDeltaSurfaceKind};
use crate::snapshot::{SnapshotReadCorrelationId, SnapshotReadTargetIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePatchTargetCoordinate {
    entity_identity: Arc<str>,
    target: BridgeCommittedPatchTarget,
}

impl BridgePatchTargetCoordinate {
    pub(crate) fn new(
        entity_identity: impl Into<Arc<str>>,
        target: BridgeCommittedPatchTarget,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            target,
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_key(&self) -> &AspectKey {
        self.target.aspect_key()
    }

    pub fn aspect_locator(&self) -> &AspectLocator {
        self.target.aspect_locator()
    }

    pub fn field_locator(&self) -> Option<&AspectFieldLocator> {
        self.target.field_locator()
    }

    pub fn surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.target.surface_kind()
    }

    pub fn target(&self) -> &BridgeCommittedPatchTarget {
        &self.target
    }

    pub fn target_canonical_basis(&self) -> String {
        self.target.canonical_basis()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSnapshotReadCoordinate {
    correlation_id: SnapshotReadCorrelationId,
    entity_identity: Arc<str>,
    aspect_key: AspectKey,
    target_identity: Option<SnapshotReadTargetIdentity>,
    slice_kind: Option<SubscriptionSliceKind>,
}

impl BridgeSnapshotReadCoordinate {
    pub(crate) fn new_coarse(
        correlation_id: SnapshotReadCorrelationId,
        entity_identity: impl Into<Arc<str>>,
        aspect_key: AspectKey,
    ) -> Self {
        Self {
            correlation_id,
            entity_identity: entity_identity.into(),
            aspect_key,
            target_identity: None,
            slice_kind: None,
        }
    }

    pub(crate) fn new_subscription_slice(
        correlation_id: SnapshotReadCorrelationId,
        entity_identity: impl Into<Arc<str>>,
        aspect_key: AspectKey,
        target_identity: SnapshotReadTargetIdentity,
        slice_kind: SubscriptionSliceKind,
    ) -> Self {
        Self {
            correlation_id,
            entity_identity: entity_identity.into(),
            aspect_key,
            target_identity: Some(target_identity),
            slice_kind: Some(slice_kind),
        }
    }

    pub fn correlation_id(&self) -> &SnapshotReadCorrelationId {
        &self.correlation_id
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn target_identity(&self) -> Option<&SnapshotReadTargetIdentity> {
        self.target_identity.as_ref()
    }

    pub fn slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        self.slice_kind.as_ref()
    }
}
