use std::sync::Arc;

use crate::identity::{BridgeIdentity, MappingIdTag, SignalInvalidationScopeTag};
use crate::mapping::fallback::BridgeMappingFallbackClass;

pub type BridgeMappingId = BridgeIdentity<MappingIdTag>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MappingSelector {
    Any,
    Exact(Arc<str>),
}

impl MappingSelector {
    pub fn exact(value: impl Into<Arc<str>>) -> Self {
        Self::Exact(value.into())
    }

    pub fn any() -> Self {
        Self::Any
    }

    pub(crate) fn matches(&self, value: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(expected) => expected.as_ref() == value,
        }
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => true,
            (Self::Exact(left), Self::Exact(right)) => left == right,
        }
    }

    pub(crate) fn is_exact(&self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TruthPatchScope {
    entity_selector: MappingSelector,
    aspect_selector: MappingSelector,
    surface_selector: MappingSelector,
}

impl TruthPatchScope {
    pub fn new(
        entity_selector: MappingSelector,
        aspect_selector: MappingSelector,
        surface_selector: MappingSelector,
    ) -> Self {
        Self {
            entity_selector,
            aspect_selector,
            surface_selector,
        }
    }

    pub fn entity_selector(&self) -> &MappingSelector {
        &self.entity_selector
    }

    pub fn aspect_selector(&self) -> &MappingSelector {
        &self.aspect_selector
    }

    pub fn surface_selector(&self) -> &MappingSelector {
        &self.surface_selector
    }

    pub(crate) fn specificity_rank(&self) -> u8 {
        u8::from(self.entity_selector.is_exact())
            + u8::from(self.aspect_selector.is_exact())
            + u8::from(self.surface_selector.is_exact())
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        self.entity_selector.overlaps(&other.entity_selector)
            && self.aspect_selector.overlaps(&other.aspect_selector)
            && self.surface_selector.overlaps(&other.surface_selector)
    }

    pub(crate) fn fallback_class(&self) -> Option<BridgeMappingFallbackClass> {
        let widened_entity = !self.entity_selector.is_exact();
        let widened_aspect = !self.aspect_selector.is_exact();
        let widened_surface = !self.surface_selector.is_exact();

        match (widened_entity, widened_aspect, widened_surface) {
            (false, false, false) => None,
            (true, false, false) => Some(BridgeMappingFallbackClass::Entity),
            (false, true, false) => Some(BridgeMappingFallbackClass::Aspect),
            (false, false, true) => Some(BridgeMappingFallbackClass::Surface),
            (true, true, false) => Some(BridgeMappingFallbackClass::EntityAspect),
            (true, false, true) => Some(BridgeMappingFallbackClass::EntitySurface),
            (false, true, true) => Some(BridgeMappingFallbackClass::AspectSurface),
            (true, true, true) => Some(BridgeMappingFallbackClass::EntityAspectSurface),
        }
    }
}

pub type SignalInvalidationScope = BridgeIdentity<SignalInvalidationScopeTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoarseRoutingMode {
    Direct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMappingRegistration {
    mapping_id: BridgeMappingId,
    truth_scope: TruthPatchScope,
    signal_scope: SignalInvalidationScope,
    routing_mode: CoarseRoutingMode,
}

impl BridgeMappingRegistration {
    pub fn new(
        mapping_id: BridgeMappingId,
        truth_scope: TruthPatchScope,
        signal_scope: SignalInvalidationScope,
        routing_mode: CoarseRoutingMode,
    ) -> Self {
        Self {
            mapping_id,
            truth_scope,
            signal_scope,
            routing_mode,
        }
    }

    pub fn mapping_id(&self) -> &BridgeMappingId {
        &self.mapping_id
    }

    pub fn truth_scope(&self) -> &TruthPatchScope {
        &self.truth_scope
    }

    pub fn signal_scope(&self) -> &SignalInvalidationScope {
        &self.signal_scope
    }

    pub fn routing_mode(&self) -> CoarseRoutingMode {
        self.routing_mode
    }

    pub(crate) fn semantic_duplicate_of(&self, other: &Self) -> bool {
        self.truth_scope == other.truth_scope
            && self.signal_scope == other.signal_scope
            && self.routing_mode == other.routing_mode
    }
}
