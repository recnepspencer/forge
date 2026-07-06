use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingRow;

pub(super) fn overlap_region_identity(
    request_identity: &str,
    arrangement_graph_identity: &str,
    cell_set_identity: &str,
    ordering_basis_identity: &str,
    row: &PlanarBooleanOverlapRegionCanonicalWindingRow,
) -> String {
    let mut parts = vec![
        request_identity.to_string(),
        arrangement_graph_identity.to_string(),
        cell_set_identity.to_string(),
        ordering_basis_identity.to_string(),
        format!("{:?}", row.source_kind()),
        row.source_identity().to_string(),
        row.island_identity().to_string(),
        row.neighborhood_identity().to_string(),
        row.area_overlap_component_identity()
            .unwrap_or("boundary-only")
            .to_string(),
        format!("{:?}", row.canonical_operand_side()),
        format!("{:?}", row.canonical_winding_sign()),
    ];
    parts.extend(
        row.canonical_boundary_segment_identities()
            .iter()
            .map(|value| format!("boundary:{value}")),
    );
    parts.extend(
        row.canonical_source_loop_identities()
            .iter()
            .map(|value| format!("loop:{value}")),
    );
    parts.extend(
        row.lineage_identities()
            .iter()
            .map(|value| format!("lineage:{value}")),
    );
    parts.extend(
        row.source_edge_identities()
            .iter()
            .map(|value| format!("edge:{value}")),
    );
    format!("overlap-region-identity:{}", parts.join("|"))
}

fn digest_rows(rows: &[impl AsRef<str>]) -> String {
    rows.iter().map(AsRef::as_ref).collect::<Vec<_>>().join("|")
}

pub(super) fn identity_map_set_identity(
    request_identity: &str,
    row_identities: &[String],
) -> String {
    format!(
        "overlap-region-identity-map:{request_identity}:{}",
        digest_rows(row_identities)
    )
}

pub(super) fn persistent_name_map_set_identity(
    request_identity: &str,
    row_identities: &[String],
) -> String {
    format!(
        "overlap-region-persistent-name-map:{request_identity}:{}",
        digest_rows(row_identities)
    )
}

pub(super) fn subshape_signature_map_set_identity(
    request_identity: &str,
    row_identities: &[String],
) -> String {
    format!(
        "overlap-region-subshape-signature-map:{request_identity}:{}",
        digest_rows(row_identities)
    )
}

pub(super) fn persistent_name_row_identity(
    region_identity: &str,
    persistent_name_identity: &str,
) -> String {
    format!("overlap-region-persistent-name:{region_identity}:{persistent_name_identity}")
}

pub(super) fn subshape_signature_identity(
    region_identity: &str,
    canonical_winding_identity: &str,
) -> String {
    format!("overlap-region-subshape-signature:{region_identity}:{canonical_winding_identity}")
}

pub(super) fn bundle_identity(
    request_identity: &str,
    identity_map_identity: &str,
    persistent_name_map_identity: &str,
    subshape_signature_map_identity: &str,
) -> String {
    format!(
        "overlap-region-identity-lineage:{request_identity}:{identity_map_identity}|{persistent_name_map_identity}|{subshape_signature_map_identity}"
    )
}
