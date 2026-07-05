use super::counters::PlanarBooleanOverlapRegionIdentityLineageCounters;
use super::denial::PlanarBooleanOverlapRegionIdentityLineageDenial;
use super::identity::{
    bundle_identity, identity_map_set_identity, overlap_region_identity,
    persistent_name_map_set_identity, persistent_name_row_identity, subshape_signature_identity,
    subshape_signature_map_set_identity,
};
use super::input::PlanarBooleanOverlapRegionIdentityLineageInput;
use super::product::{
    PlanarBooleanOverlapRegionIdentityLineageBundle, PlanarBooleanOverlapRegionIdentityMap,
    PlanarBooleanOverlapRegionPersistentNamePropagationMap,
    PlanarBooleanOverlapRegionSubshapeSignatureMap,
};
use super::rows::{
    PlanarBooleanOverlapRegionIdentityRow, PlanarBooleanOverlapRegionPersistentNamePropagationRow,
    PlanarBooleanOverlapRegionSubshapeSignatureRow,
};
use super::validation::{
    canonical_signature_basis, validate_input_identities, validate_persistent_name_rows,
    validate_unique_region_identities,
};

pub(super) fn build_identity_lineage_bundle(
    input: PlanarBooleanOverlapRegionIdentityLineageInput<'_>,
) -> Result<
    PlanarBooleanOverlapRegionIdentityLineageBundle,
    PlanarBooleanOverlapRegionIdentityLineageDenial,
> {
    let mut counters = PlanarBooleanOverlapRegionIdentityLineageCounters::default();
    validate_input_identities(input, &counters)?;

    let canonical = input
        .post_admission_normalization()
        .overlap_region_canonical_winding();
    let request_identity = canonical.request_identity().to_string();
    let arrangement_graph_identity = canonical.arrangement_graph_identity().to_string();
    let cell_set_identity = canonical.cell_set_identity().to_string();
    let ordering_basis_identity = canonical.ordering_basis_identity().to_string();

    let mut canonical_rows = canonical.rows().iter().collect::<Vec<_>>();
    canonical_rows.sort_by(|left, right| {
        left.canonical_winding_identity()
            .cmp(right.canonical_winding_identity())
    });

    let mut identity_rows = Vec::new();
    let mut persistent_name_rows = Vec::new();
    let mut signature_rows = Vec::new();

    for row in canonical_rows {
        counters.examined_canonical_row();
        let region_identity = overlap_region_identity(
            &request_identity,
            &arrangement_graph_identity,
            &cell_set_identity,
            &ordering_basis_identity,
            row,
        );
        identity_rows.push(PlanarBooleanOverlapRegionIdentityRow::new(
            region_identity.clone(),
            row.canonical_winding_identity().to_string(),
            row.source_kind(),
            row.source_identity().to_string(),
            row.island_identity().to_string(),
            row.neighborhood_identity().to_string(),
            row.area_overlap_component_identity().map(str::to_string),
            row.canonical_operand_side(),
            row.canonical_winding_sign(),
            row.canonical_boundary_segment_identities().to_vec(),
            row.canonical_source_loop_identities().to_vec(),
            row.lineage_identities().to_vec(),
            row.source_edge_identities().to_vec(),
            row.boundary_roles().to_vec(),
        ));
        counters.admitted_identity_row();

        for persistent_name_identity in row.propagated_persistent_name_identities() {
            persistent_name_rows.push(PlanarBooleanOverlapRegionPersistentNamePropagationRow::new(
                persistent_name_row_identity(&region_identity, persistent_name_identity),
                region_identity.clone(),
                row.canonical_winding_identity().to_string(),
                persistent_name_identity.to_string(),
            ));
        }
        counters.admitted_persistent_name_row(row.propagated_persistent_name_identities().len());

        signature_rows.push(PlanarBooleanOverlapRegionSubshapeSignatureRow::new(
            subshape_signature_identity(&region_identity, row.canonical_winding_identity()),
            region_identity,
            row.canonical_winding_identity().to_string(),
            canonical_signature_basis(row),
            row.canonical_winding_sign().is_none(),
        ));
        counters.admitted_subshape_signature_row();
    }

    persistent_name_rows.sort_by(|left, right| {
        left.propagation_identity()
            .cmp(right.propagation_identity())
    });
    signature_rows.sort_by(|left, right| {
        left.signature_identity().cmp(right.signature_identity())
    });

    validate_unique_region_identities(&identity_rows, &mut counters)?;
    validate_persistent_name_rows(&persistent_name_rows, &identity_rows, &mut counters)?;

    let identity_row_identities = identity_rows
        .iter()
        .map(|row| row.region_identity().to_string())
        .collect::<Vec<_>>();
    let persistent_name_row_identities = persistent_name_rows
        .iter()
        .map(|row| row.propagation_identity().to_string())
        .collect::<Vec<_>>();
    let signature_row_identities = signature_rows
        .iter()
        .map(|row| row.signature_identity().to_string())
        .collect::<Vec<_>>();
    let identity_map_identity = identity_map_set_identity(&request_identity, &identity_row_identities);
    let persistent_name_map_identity = persistent_name_map_set_identity(
        &request_identity,
        &persistent_name_row_identities,
    );
    let subshape_signature_map_identity = subshape_signature_map_set_identity(
        &request_identity,
        &signature_row_identities,
    );

    Ok(PlanarBooleanOverlapRegionIdentityLineageBundle::new(
        bundle_identity(
            &request_identity,
            &identity_map_identity,
            &persistent_name_map_identity,
            &subshape_signature_map_identity,
        ),
        PlanarBooleanOverlapRegionIdentityMap::new(
            identity_map_identity,
            request_identity.clone(),
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            identity_rows,
        ),
        PlanarBooleanOverlapRegionPersistentNamePropagationMap::new(
            persistent_name_map_identity,
            request_identity.clone(),
            persistent_name_rows,
        ),
        PlanarBooleanOverlapRegionSubshapeSignatureMap::new(
            subshape_signature_map_identity,
            request_identity,
            signature_rows,
        ),
        input.post_admission_normalization().clone(),
        counters,
    ))
}
