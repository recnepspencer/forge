use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;

mod derived_snapshot;
mod full_snapshot;

pub(crate) use derived_snapshot::historical_derived_surface_snapshot_for_read_basis;
pub(crate) use full_snapshot::historical_query_snapshot_for_read_basis;

fn ensure_snapshot_matches_read_basis(
    equivalence: &DerivedEquivalenceContractReport,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<
    (),
    crate::projection::runtime_boundary::declared_query_surfaces::TopologyQuerySurfaceError,
> {
    use crate::projection::runtime_boundary::declared_query_surfaces::TopologyQuerySurfaceError;

    if equivalence.authority_snapshot_id != read_basis.snapshot().snapshot_id.0 {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot authority snapshot id `{}` did not match requested read basis snapshot `{}`",
            equivalence.authority_snapshot_id,
            read_basis.snapshot().snapshot_id.0
        )));
    }
    if equivalence.authority_branch_id != read_basis.branch_id().0.as_str() {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot authority branch id `{}` did not match requested read basis branch `{}`",
            equivalence.authority_branch_id,
            read_basis.branch_id().0
        )));
    }
    if equivalence.authoritative_mutation_origin != read_basis.authoritative_mutation_origin() {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot authoritative mutation origin diverged from requested read basis",
        ));
    }
    if equivalence.derivation_origin != read_basis.derivation_origin() {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot derivation origin diverged from requested read basis",
        ));
    }
    if equivalence.truth_basis_digest_hex
        != read_basis
            .authority
            .truth_basis_identity
            .mutation_digest_hex
    {
        return Err(TopologyQuerySurfaceError::new(
            "query-derived snapshot truth basis digest diverged from requested read basis",
        ));
    }
    if equivalence.touched_aspect_count != read_basis.touched_aspects().len() {
        return Err(TopologyQuerySurfaceError::new(format!(
            "query-derived snapshot touched-aspect count `{}` did not match requested read basis count `{}`",
            equivalence.touched_aspect_count,
            read_basis.touched_aspects().len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
