use super::*;

pub(super) fn resolve_relation(
    counters: &mut CompatibilityAdmissionCounters,
    edge_registry: &CompatibilityEdgeRegistry,
    family_id: &ArtifactFamilyId,
    from_semantic_version: ArtifactSemanticVersion,
    to_semantic_version: ArtifactSemanticVersion,
    path: CompatibilityAdmissionPath,
) -> Result<CompatibilityRelation, CompatibilityRejection> {
    counters.record_relation_recheck();
    let Some(edge) = edge_registry.get(family_id, from_semantic_version, to_semantic_version)
    else {
        counters.record_edge_missing_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MissingCompatibilityEdge,
            family_id.clone(),
            "declared compatibility edge is missing",
        ));
    };
    let relation = edge.relation();
    if relation == CompatibilityRelation::Incompatible {
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DeclaredIncompatibleRelation,
            family_id.clone(),
            "declared compatibility edge explicitly rejects this semantic relation",
        ));
    }
    admit_adapter_cost(counters, family_id, edge, path)?;
    Ok(relation)
}
