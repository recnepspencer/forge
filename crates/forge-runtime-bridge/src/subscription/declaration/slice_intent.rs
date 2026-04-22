use std::sync::Arc;

use crate::mapping::SubscriptionSliceKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionDeliveryIntentClass {
    None,
    CanonicalMeaningfulChange,
}

impl BridgeSubscriptionDeliveryIntentClass {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::CanonicalMeaningfulChange => "canonical_meaningful_change",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizedSubscriptionSliceIntentErrorKind {
    EmptyEntityIdentity,
    EmptyAspectLabel,
    EmptySurfaceLabel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedSubscriptionSliceIntentError {
    kind: NormalizedSubscriptionSliceIntentErrorKind,
}

impl NormalizedSubscriptionSliceIntentError {
    const fn new(kind: NormalizedSubscriptionSliceIntentErrorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> NormalizedSubscriptionSliceIntentErrorKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedSubscriptionSliceIntent {
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Arc<str>,
    slice_kind: SubscriptionSliceKind,
}

impl NormalizedSubscriptionSliceIntent {
    pub fn try_new(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
    ) -> Result<Self, NormalizedSubscriptionSliceIntentError> {
        let entity_identity = entity_identity.into();
        if entity_identity.is_empty() {
            return Err(NormalizedSubscriptionSliceIntentError::new(
                NormalizedSubscriptionSliceIntentErrorKind::EmptyEntityIdentity,
            ));
        }
        let aspect_label = aspect_label.into();
        if aspect_label.is_empty() {
            return Err(NormalizedSubscriptionSliceIntentError::new(
                NormalizedSubscriptionSliceIntentErrorKind::EmptyAspectLabel,
            ));
        }
        let surface_label = surface_label.into();
        if surface_label.is_empty() {
            return Err(NormalizedSubscriptionSliceIntentError::new(
                NormalizedSubscriptionSliceIntentErrorKind::EmptySurfaceLabel,
            ));
        }
        Ok(Self {
            entity_identity,
            aspect_label,
            surface_label,
            slice_kind,
        })
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

    pub fn slice_kind(&self) -> &SubscriptionSliceKind {
        &self.slice_kind
    }
}
