use forge_query::facade::runtime::{
    ForgeQueryGraphIndexLifecycleClass, ForgeQueryGraphIndexLifecycleOwner,
    ForgeQueryGraphIndexPosture, ForgeQueryGraphIndexSupportRow, ForgeQueryGraphIndexSupportState,
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessRebuildBasis, ForgeQueryGraphReadAccessRequirementKind,
};

fn main() {
    let _ = ForgeQueryGraphIndexSupportRow {
        digest: String::new(),
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        supported_relation_direction: None,
        lifecycle_owner: ForgeQueryGraphIndexLifecycleOwner::QueryRuntime,
        lifecycle_class: ForgeQueryGraphIndexLifecycleClass::RuntimeMaintained,
        rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
        invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
        complexity_contract: ForgeQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
        posture: ForgeQueryGraphIndexPosture::Verified,
        support_state: ForgeQueryGraphIndexSupportState::Available,
        owning_milestone: None,
    };
}
