use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessMemoryEstimateBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementRow,
};

fn main() {
    let _ = ForgeQueryGraphReadAccessRequirementRow {
        kind: ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
        relation_name: None,
        relation_authority: None,
        relation_direction: None,
        relation_depth: None,
        fanout_posture: None,
        predicate_family: None,
        predicate_field_authorities: Vec::new(),
        ordering_posture: None,
        ordering_field_authorities: Vec::new(),
        traversal_operator: None,
        lifecycle_class: None,
        result_pressure: None,
        invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
        complexity_contract: ForgeQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
        memory_estimate_basis: ForgeQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
    };
}
