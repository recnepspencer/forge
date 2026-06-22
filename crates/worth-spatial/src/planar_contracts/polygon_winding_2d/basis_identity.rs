use super::CertifiedPolygonWinding2DBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CertifiedPolygonWinding2DIdentityEntry {
    locus: String,
    value: String,
}

impl CertifiedPolygonWinding2DIdentityEntry {
    fn new(locus: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            locus: locus.into(),
            value: value.into(),
        }
    }

    pub(crate) fn locus(&self) -> &str {
        &self.locus
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

pub(crate) fn certified_polygon_winding_2d_identity_entries(
    basis: &CertifiedPolygonWinding2DBasis,
) -> Vec<CertifiedPolygonWinding2DIdentityEntry> {
    let mut entries = vec![
        entry(
            "geometry.polygon_winding_2d.primary_loop",
            basis.primary_loop_identity(),
        ),
        entry(
            "geometry.polygon_winding_2d.planar_neighborhood",
            basis.planar_neighborhood_identity(),
        ),
        entry(
            "geometry.polygon_winding_2d.winding_policy",
            basis.winding_policy_identity(),
        ),
        entry(
            "geometry.polygon_winding_2d.frame_identity",
            basis.frame_identity(),
        ),
        entry(
            "geometry.polygon_winding_2d.local_frame_fact",
            basis.local_frame_fact_digest(),
        ),
        entry(
            "geometry.polygon_winding_2d.movement_rotation",
            basis.movement_rotation_posture_identity(),
        ),
        entry(
            "geometry.polygon_winding_2d.tolerance_policy",
            basis.tolerance_policy_identity(),
        ),
        entry(
            "geometry.polygon_winding_2d.primary_winding",
            basis.primary_winding().as_str(),
        ),
    ];
    for (index, loop_basis) in basis.loop_summaries().iter().enumerate() {
        entries.push(entry(
            loop_locus(index, "identity"),
            loop_basis.loop_identity(),
        ));
        entries.push(entry(
            loop_locus(index, "topology_identity"),
            loop_basis.topology_loop_identity(),
        ));
        entries.push(entry(
            loop_locus(index, "membership_fact"),
            loop_basis.loop_membership_fact_digest(),
        ));
        entries.push(entry(
            loop_locus(index, "topology_spatial_contract"),
            loop_basis.topology_to_spatial_contract_digest(),
        ));
        entries.push(entry(
            loop_locus(index, "winding"),
            loop_basis.winding().as_str(),
        ));
        entries.push(entry(
            loop_locus(index, "containment"),
            loop_basis.containment_identity(),
        ));
    }
    for (index, digest) in basis.projected_vertex_fact_digests().iter().enumerate() {
        entries.push(entry(vertex_locus(index), *digest));
    }
    for (index, digest) in basis.winding_predicate_fact_digests().iter().enumerate() {
        entries.push(entry(predicate_locus(index), digest.as_str()));
    }
    for (index, digest) in basis.segment_contact_fact_digests().iter().enumerate() {
        entries.push(entry(segment_locus(index), digest.as_str()));
    }
    entries
}

fn entry(
    locus: impl Into<String>,
    value: impl Into<String>,
) -> CertifiedPolygonWinding2DIdentityEntry {
    CertifiedPolygonWinding2DIdentityEntry::new(locus, value)
}

fn loop_locus(index: usize, suffix: &str) -> String {
    format!("geometry.polygon_winding_2d.loop.{index}.{suffix}")
}

fn vertex_locus(index: usize) -> String {
    format!("geometry.polygon_winding_2d.vertex.{index}.projection_fact")
}

fn predicate_locus(index: usize) -> String {
    format!("geometry.polygon_winding_2d.winding_predicate.{index}.fact")
}

fn segment_locus(index: usize) -> String {
    format!("geometry.polygon_winding_2d.segment_contact.{index}.fact")
}
