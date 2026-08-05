use worth_query_admission::facade::graph_read_access::{
    WorthQueryGraphIndexInventory, WorthQueryGraphIndexLifecycleClass,
    WorthQueryGraphIndexLifecycleOwner, WorthQueryGraphIndexPosture,
    WorthQueryGraphIndexSupportRow, WorthQueryGraphIndexSupportState,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadAccessRequirementRow,
    WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadOrderingPosture,
};

use super::super::schema_layout::WorthQueryPrimaryGraphLayout;
use worth_query_installation::facade::WorthQueryInstalledApplicationContinuationContract;
use worth_query_installation::facade::WorthQueryInstalledApplicationLiveContract;

pub(in crate::domain_computation::primary_graph) fn primary_graph_support_inventory(
    layout: &WorthQueryPrimaryGraphLayout,
    continuation: Option<&WorthQueryInstalledApplicationContinuationContract>,
    live: Option<&WorthQueryInstalledApplicationLiveContract>,
    requirements: &WorthQueryGraphReadAccessRequirementSet,
) -> WorthQueryGraphIndexInventory {
    WorthQueryGraphIndexInventory::from_rows(
        requirements
            .rows()
            .iter()
            .filter(|requirement| runtime_supports(layout, continuation, live, requirement))
            .map(exact_support_row)
            .collect(),
    )
}

fn runtime_supports(
    layout: &WorthQueryPrimaryGraphLayout,
    continuation: Option<&WorthQueryInstalledApplicationContinuationContract>,
    live: Option<&WorthQueryInstalledApplicationLiveContract>,
    requirement: &WorthQueryGraphReadAccessRequirementRow,
) -> bool {
    match requirement.kind() {
        WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency
        | WorthQueryGraphReadAccessRequirementKind::ReverseAdjacency => requirement
            .relation_name()
            .is_some_and(|relation| layout.relation(relation).is_some()),
        WorthQueryGraphReadAccessRequirementKind::PredicateSupport => requirement
            .predicate_field_authorities()
            .iter()
            .all(|field| {
                layout.supports_equality_field(field.native_aspect_key(), field.native_field_key())
            }),
        WorthQueryGraphReadAccessRequirementKind::OrderingSupport => match requirement
            .ordering_posture()
        {
            Some(WorthQueryGraphReadOrderingPosture::BoundedProjectedCollection) => requirement
                .ordering_field_authorities()
                .iter()
                .all(|field| {
                    layout.supports_projection_field(
                        field.native_aspect_key(),
                        field.native_field_key(),
                    )
                }),
            Some(WorthQueryGraphReadOrderingPosture::IndexedRelatedCollectionSeek) => {
                continuation.is_some_and(|contract| layout.supports_continuation_ordering(contract))
            }
            _ => false,
        },
        WorthQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport => {
            live.is_some_and(|contract| {
                layout.supports_equality_field(
                    contract.scope_identity().aspect_key(),
                    contract.scope_identity().field_key(),
                ) && layout.supports_equality_field(
                    contract.target_identity().aspect_key(),
                    contract.target_identity().field_key(),
                )
            })
        }
        WorthQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration => false,
        WorthQueryGraphReadAccessRequirementKind::TraversalWorkset
        | WorthQueryGraphReadAccessRequirementKind::VisitedSet
        | WorthQueryGraphReadAccessRequirementKind::DedupSet
        | WorthQueryGraphReadAccessRequirementKind::ProofSupport
        | WorthQueryGraphReadAccessRequirementKind::ResultBuffer
        | WorthQueryGraphReadAccessRequirementKind::MaterializationLifecycle => true,
    }
}

fn exact_support_row(
    requirement: &WorthQueryGraphReadAccessRequirementRow,
) -> WorthQueryGraphIndexSupportRow {
    let mut row = if requirement.kind()
        == &WorthQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport
    {
        WorthQueryGraphIndexSupportRow::with_runtime_support_posture(
            requirement.kind().clone(),
            WorthQueryGraphIndexLifecycleOwner::QueryRuntime,
            WorthQueryGraphIndexLifecycleClass::RuntimeMaintained,
            WorthQueryGraphIndexPosture::Verified,
            WorthQueryGraphIndexSupportState::Available,
            None,
        )
    } else {
        WorthQueryGraphIndexSupportRow::for_requirement_kind(requirement.kind().clone())
    };
    if let Some(direction) = requirement.relation_direction() {
        row = row.with_supported_relation_direction(direction.clone());
    }
    if let Some(family) = requirement.predicate_family() {
        row = row.with_supported_predicate_family(family.clone());
    }
    if let Some(posture) = requirement.ordering_posture() {
        row = row.with_supported_ordering_posture(posture.clone());
    }
    if let Some(lifecycle) = requirement.lifecycle_class() {
        row = row.with_supported_requirement_lifecycle(lifecycle.clone());
    }
    row
}
