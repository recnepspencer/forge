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
pub struct ResolvedBridgeMappings<'a> {
    resolved: Vec<ResolvedBridgeMapping<'a>>,
}

impl<'a> ResolvedBridgeMappings<'a> {
    pub(crate) fn new(resolved: Vec<ResolvedBridgeMapping<'a>>) -> Self {
        Self { resolved }
    }

    pub(crate) fn registrations(
        &self,
    ) -> impl Iterator<Item = &'a FrozenBridgeMappingRegistration> + '_ {
        self.resolved
            .iter()
            .map(ResolvedBridgeMapping::registration)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeMappingLookup<'a> {
    Exact {
        resolved: ResolvedBridgeMappings<'a>,
    },
    Fallback {
        resolved: ResolvedBridgeMappings<'a>,
    },
    Missing,
}

impl FrozenMappingRegistry {
    pub(crate) fn lookup<'a>(&'a self, key: BridgeMappingLookupKey<'_>) -> BridgeMappingLookup<'a> {
        let mut matches = self
            .registrations
            .iter()
            .filter(|registration| registration.truth_scope().matches_key(key))
            .collect::<Vec<_>>();
        if matches.is_empty() {
            return BridgeMappingLookup::Missing;
        }

        let most_specific_rank = matches
            .iter()
            .map(|registration| registration.truth_scope().specificity_rank())
            .max()
            .expect("non-empty matching registrations should have a specificity rank");
        matches.retain(|registration| {
            registration.truth_scope().specificity_rank() == most_specific_rank
        });

        let is_fallback = matches
            .first()
            .and_then(|registration| registration.fallback_class())
            .is_some();
        let resolved = ResolvedBridgeMappings::new(
            matches
                .into_iter()
                .map(ResolvedBridgeMapping::new)
                .collect(),
        );
        match is_fallback {
            false => BridgeMappingLookup::Exact { resolved },
            true => BridgeMappingLookup::Fallback { resolved },
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
