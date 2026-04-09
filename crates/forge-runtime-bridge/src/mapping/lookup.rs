use crate::input::envelope::BridgeCommittedPatchItem;
use crate::mapping::freezing::{FrozenBridgeMappingRegistration, FrozenMappingRegistry};
use crate::routing::surfaces::TruthDeltaSurface;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeMappingLookupKey<'a> {
    entity_identity: &'a str,
    aspect_label: &'a str,
    surface_label: &'a str,
}

impl<'a> BridgeMappingLookupKey<'a> {
    pub fn new(entity_identity: &'a str, aspect_label: &'a str, surface_label: &'a str) -> Self {
        Self {
            entity_identity,
            aspect_label,
            surface_label,
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label
    }
}

impl<'a> From<&'a BridgeCommittedPatchItem> for BridgeMappingLookupKey<'a> {
    fn from(value: &'a BridgeCommittedPatchItem) -> Self {
        Self::new(
            value.entity_identity(),
            value.aspect_label(),
            value.surface_label(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedBridgeMapping<'a> {
    registration: &'a FrozenBridgeMappingRegistration,
}

impl<'a> ResolvedBridgeMapping<'a> {
    pub(crate) fn new(registration: &'a FrozenBridgeMappingRegistration) -> Self {
        Self { registration }
    }

    pub(crate) fn registration(&self) -> &'a FrozenBridgeMappingRegistration {
        self.registration
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeMappingLookup<'a> {
    Exact { resolved: ResolvedBridgeMapping<'a> },
    Fallback { resolved: ResolvedBridgeMapping<'a> },
    Missing,
}

impl FrozenMappingRegistry {
    pub(crate) fn lookup<'a>(&'a self, key: BridgeMappingLookupKey<'_>) -> BridgeMappingLookup<'a> {
        let Some(registration) = self
            .registrations
            .iter()
            .find(|registration| registration.truth_scope().matches_key(key))
        else {
            return BridgeMappingLookup::Missing;
        };

        let resolved = ResolvedBridgeMapping::new(registration);
        match registration.fallback_class() {
            None => BridgeMappingLookup::Exact { resolved },
            Some(_) => BridgeMappingLookup::Fallback { resolved },
        }
    }
    pub(crate) fn lookup_truth_surface<'a>(
        &'a self,
        surface: &'a TruthDeltaSurface,
    ) -> BridgeMappingLookup<'a> {
        self.lookup(BridgeMappingLookupKey::new(
            surface.entity_identity(),
            surface.aspect_label(),
            surface.surface_label(),
        ))
    }
}
