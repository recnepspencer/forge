use worth_query::facade::runtime::{
    WorthQueryGraphIndexLifecycleClass, WorthQueryGraphIndexLifecycleOwner,
    WorthQueryGraphIndexPosture, WorthQueryGraphIndexSupportRow, WorthQueryGraphIndexSupportState,
    WorthQueryGraphReadAccessComplexityContract, WorthQueryGraphReadAccessInvalidationBasis,
    WorthQueryGraphReadAccessRebuildBasis, WorthQueryGraphReadAccessRequirementKind,
};

fn main() {
    let _ = WorthQueryGraphIndexSupportRow {
        digest: String::new(),
        requirement_kind: WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        supported_relation_direction: None,
        lifecycle_owner: WorthQueryGraphIndexLifecycleOwner::QueryRuntime,
        lifecycle_class: WorthQueryGraphIndexLifecycleClass::RuntimeMaintained,
        rebuild_basis: WorthQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
        invalidation_basis: WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
        complexity_contract: WorthQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
        posture: WorthQueryGraphIndexPosture::Verified,
        support_state: WorthQueryGraphIndexSupportState::Available,
        owning_milestone: None,
    };
}
