use crate::error::{BridgeErrorContext, BridgePatchCoordinate, BridgeRouteError, BridgeRouteErrorKind};
use crate::input::envelope::{BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem};
use crate::mapping::{
    BridgeMappingFallbackClass, BridgeMappingLookup, FrozenBridgeMappingRegistration,
    FrozenAspectMappingRegistry, FrozenMappingRegistry,
};
use crate::routing::counters::BridgeRoutingCounters;
use crate::routing::matching::{classify_truth_delta_surface, FineGrainedSurfaceMatch, FineGrainedMatchStatus};
use crate::routing::surfaces::{
    derive_normalized_truth_delta_surface_set, truth_delta_surface_count, TruthDeltaSurface,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EligibleRouteEntry {
    item: BridgeCommittedPatchItem,
    normalized_surface: TruthDeltaSurface,
    registration: FrozenBridgeMappingRegistration,
    fallback_class: Option<BridgeMappingFallbackClass>,
    fine_grained_match: FineGrainedSurfaceMatch,
}

impl EligibleRouteEntry {
    pub(crate) fn item(&self) -> &BridgeCommittedPatchItem {
        &self.item
    }

    pub(crate) fn normalized_surface(&self) -> &TruthDeltaSurface {
        &self.normalized_surface
    }

    pub(crate) fn registration(&self) -> &FrozenBridgeMappingRegistration {
        &self.registration
    }

    pub(crate) fn fallback_class(&self) -> Option<BridgeMappingFallbackClass> {
        self.fallback_class
    }

    pub(crate) fn fine_grained_match(&self) -> &FineGrainedSurfaceMatch {
        &self.fine_grained_match
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EligibleRouteRequest {
    envelope: BridgeCommittedPatchEnvelope,
    entries: Vec<EligibleRouteEntry>,
    counters: BridgeRoutingCounters,
}

impl EligibleRouteRequest {
    pub(crate) fn entries(&self) -> &[EligibleRouteEntry] {
        &self.entries
    }

    pub(crate) fn counters(&self) -> BridgeRoutingCounters {
        self.counters
    }
}

pub(crate) fn validate_route_request(
    envelope: BridgeCommittedPatchEnvelope,
    registry: &FrozenMappingRegistry,
    aspect_registry: &FrozenAspectMappingRegistry,
) -> Result<EligibleRouteRequest, BridgeRouteError> {
    let truth_delta_surface_set =
        derive_normalized_truth_delta_surface_set(&envelope, aspect_registry)?;
    let mut entries = Vec::with_capacity(envelope.patch_body().canonical_items().len());
    let mut counters = BridgeRoutingCounters::from_patch_counts(
        envelope.patch_summary().patch_item_count(),
        envelope.patch_summary().normalized_patch_item_count(),
    )
    .with_truth_delta_surface_counts(
        truth_delta_surface_count(&envelope),
        truth_delta_surface_set.len(),
    );

    for (item, normalized_surface) in envelope
        .patch_body()
        .canonical_items()
        .iter()
        .zip(truth_delta_surface_set.item_surfaces().iter())
    {
        let fine_grained_match = classify_truth_delta_surface(normalized_surface, aspect_registry);
        counters = match fine_grained_match.status() {
            FineGrainedMatchStatus::Matched => counters.with_planned_slice_match(),
            FineGrainedMatchStatus::FallbackAdmitted => {
                counters.with_planned_slice_match().with_slice_fallback()
            }
            FineGrainedMatchStatus::SuppressedByRegistrationPolicy => {
                counters.with_slice_suppression()
            }
            FineGrainedMatchStatus::UnsupportedSurfaceCategory
            | FineGrainedMatchStatus::AmbiguousRegistration => counters,
        };
        counters = counters.with_mapping_lookup();
        match registry.lookup_truth_surface(normalized_surface) {
            BridgeMappingLookup::Exact { resolved } => entries.push(EligibleRouteEntry {
                item: item.clone(),
                normalized_surface: normalized_surface.clone(),
                registration: resolved.registration().clone(),
                fallback_class: None,
                fine_grained_match,
            }),
            BridgeMappingLookup::Fallback { resolved } => {
                counters = counters.with_mapping_fallback();
                entries.push(EligibleRouteEntry {
                    item: item.clone(),
                    normalized_surface: normalized_surface.clone(),
                    registration: resolved.registration().clone(),
                    fallback_class: resolved.registration().fallback_class(),
                    fine_grained_match,
                });
            }
            BridgeMappingLookup::Missing => {
                return Err(BridgeRouteError::new(
                    BridgeRouteErrorKind::MissingMappingRegistration,
                    format!(
                        "No bridge mapping registration matched committed patch item `{}/{}/{}`.",
                        item.entity_identity(),
                        item.aspect_label(),
                        item.surface_label()
                    ),
                )
                .with_context(BridgeErrorContext::routing(BridgePatchCoordinate::new(
                    item.entity_identity(),
                    item.aspect_label(),
                    item.surface_label(),
                ))));
            }
        }
    }

    debug_assert_eq!(
        envelope.patch_body().canonical_items().len(),
        truth_delta_surface_set.item_surfaces().len(),
        "normalized truth-delta surface derivation must preserve canonical patch-item cardinality"
    );

    Ok(EligibleRouteRequest {
        envelope,
        entries,
        counters,
    })
}
