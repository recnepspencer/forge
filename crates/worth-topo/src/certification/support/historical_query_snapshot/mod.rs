use schema::facade::topology_authoring::DerivedTopologyReadBasis;

mod derived_snapshot;
mod full_snapshot;

pub(crate) use derived_snapshot::historical_derived_surface_snapshot_for_read_basis;
pub(crate) use full_snapshot::historical_query_snapshot_for_read_basis;

fn ensure_snapshot_matches_read_basis(
    equivalence_facts: &forge_query::facade::ForgeQueryRetainedScalarFactSet,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<
    (),
    crate::projection::runtime_boundary::declared_query_surfaces::TopologyQuerySurfaceError,
> {
    use crate::projection::runtime_boundary::declared_query_surfaces::TopologyQuerySurfaceError;

    let expected_authority_snapshot_id = serde_json::json!(read_basis.snapshot().snapshot_id.0);
    if equivalence_facts.field_value("authority_snapshot_id")
        != Some(&expected_authority_snapshot_id)
    {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot authority snapshot id `{:?}` did not match requested read basis snapshot `{}`",
            equivalence_facts.field_value("authority_snapshot_id"),
            read_basis.snapshot().snapshot_id.0
        )));
    }
    let expected_authority_branch_id = serde_json::json!(read_basis.branch_id().0);
    if equivalence_facts.field_value("authority_branch_id") != Some(&expected_authority_branch_id) {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot authority branch id `{:?}` did not match requested read basis branch `{}`",
            equivalence_facts.field_value("authority_branch_id"),
            read_basis.branch_id().0
        )));
    }
    let expected_authoritative_origin =
        serde_json::to_value(read_basis.authoritative_mutation_origin()).map_err(|error| {
            TopologyQuerySurfaceError::new(format!(
                "requested read-basis authoritative mutation origin failed to encode: {error}"
            ))
        })?;
    if equivalence_facts.field_value("authoritative_mutation_origin")
        != Some(&expected_authoritative_origin)
    {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot authoritative mutation origin diverged from requested read basis",
        ));
    }
    let expected_derivation_origin =
        serde_json::to_value(read_basis.derivation_origin()).map_err(|error| {
            TopologyQuerySurfaceError::new(format!(
                "requested read-basis derivation origin failed to encode: {error}"
            ))
        })?;
    if equivalence_facts.field_value("derivation_origin") != Some(&expected_derivation_origin) {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot derivation origin diverged from requested read basis",
        ));
    }
    let expected_truth_basis = serde_json::json!(
        read_basis
            .authority
            .truth_basis_identity
            .mutation_digest_hex
    );
    if equivalence_facts.field_value("truth_basis_digest_hex") != Some(&expected_truth_basis) {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot truth basis digest diverged from requested read basis",
        ));
    }
    let expected_touched_aspect_count = serde_json::json!(read_basis.touched_aspects().len());
    if equivalence_facts.field_value("touched_aspect_count") != Some(&expected_touched_aspect_count)
    {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot touched-aspect count `{:?}` did not match requested read basis count `{}`",
            equivalence_facts.field_value("touched_aspect_count"),
            read_basis.touched_aspects().len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
