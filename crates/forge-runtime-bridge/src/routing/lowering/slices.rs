use std::sync::Arc;

use forge_foundational::facade::AspectKey;

use crate::mapping::SubscriptionSliceKind;
use crate::routing::matching::FineGrainedMatchStatus;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BridgeSubscriptionSlice {
    entity_identity: Arc<str>,
    aspect_key: AspectKey,
    surface_label: Arc<str>,
    slice_kind: SubscriptionSliceKind,
    match_status: FineGrainedMatchStatus,
}

impl BridgeSubscriptionSlice {
    pub(crate) fn new(
        entity_identity: impl Into<Arc<str>>,
        aspect_key: AspectKey,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
        match_status: FineGrainedMatchStatus,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            aspect_key,
            surface_label: surface_label.into(),
            slice_kind,
            match_status,
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_key.as_str()
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }

    pub fn slice_kind(&self) -> &SubscriptionSliceKind {
        &self.slice_kind
    }

    pub fn match_status(&self) -> FineGrainedMatchStatus {
        self.match_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalSubscriptionSlices {
    slices: Arc<[BridgeSubscriptionSlice]>,
}

impl CanonicalSubscriptionSlices {
    pub(crate) fn new(slices: Vec<BridgeSubscriptionSlice>) -> Self {
        Self {
            slices: Arc::from(slices),
        }
    }

    pub fn slices(&self) -> &[BridgeSubscriptionSlice] {
        &self.slices
    }

    pub(crate) fn shared(&self) -> &Arc<[BridgeSubscriptionSlice]> {
        &self.slices
    }

    pub fn len(&self) -> usize {
        self.slices.len()
    }
}
