use std::sync::Arc;

use crate::mapping::SubscriptionSliceKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePatchCoordinate {
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Arc<str>,
}

impl BridgePatchCoordinate {
    pub fn new(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: surface_label.into(),
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSnapshotReadCoordinate {
    request_key: Arc<str>,
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Option<Arc<str>>,
    slice_kind: Option<SubscriptionSliceKind>,
}

impl BridgeSnapshotReadCoordinate {
    pub fn new_coarse(
        request_key: impl Into<Arc<str>>,
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            request_key: request_key.into(),
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: None,
            slice_kind: None,
        }
    }

    pub fn new_subscription_slice(
        request_key: impl Into<Arc<str>>,
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
    ) -> Self {
        Self {
            request_key: request_key.into(),
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: Some(surface_label.into()),
            slice_kind: Some(slice_kind),
        }
    }

    pub fn request_key(&self) -> &str {
        self.request_key.as_ref()
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> Option<&str> {
        self.surface_label.as_deref()
    }

    pub fn slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        self.slice_kind.as_ref()
    }
}
