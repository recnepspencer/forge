use super::PlanarLocalFrameBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarLocalFrameBasisIdentityEntry {
    query_locus: &'static str,
    digest_label: &'static str,
    value: String,
}

impl PlanarLocalFrameBasisIdentityEntry {
    const fn new(query_locus: &'static str, digest_label: &'static str, value: String) -> Self {
        Self {
            query_locus,
            digest_label,
            value,
        }
    }

    pub(crate) fn query_locus(&self) -> &'static str {
        self.query_locus
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("{}:{}", self.digest_label, self.value)
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

pub(crate) fn planar_local_frame_basis_identity_entries(
    basis: &PlanarLocalFrameBasis,
) -> Vec<PlanarLocalFrameBasisIdentityEntry> {
    vec![
        entry(
            "geometry.planar_local_frame.frame_identity",
            "frame",
            basis.frame_identity(),
        ),
        entry(
            "geometry.planar_local_frame.origin",
            "origin",
            format!("{:?}", basis.origin()),
        ),
        entry(
            "geometry.planar_local_frame.normal",
            "normal",
            format!("{:?}", basis.normal()),
        ),
        entry(
            "geometry.planar_local_frame.u_axis",
            "u_axis",
            format!("{:?}", basis.u_axis()),
        ),
        entry(
            "geometry.planar_local_frame.v_axis",
            "v_axis",
            format!("{:?}", basis.v_axis()),
        ),
        entry(
            "geometry.planar_local_frame.w_axis",
            "w_axis",
            format!("{:?}", basis.w_axis()),
        ),
        entry(
            "geometry.planar_local_frame.local_feature_scale",
            "local_order",
            basis.local_feature_scale_order().to_string(),
        ),
        entry(
            "geometry.planar_local_frame.world_magnitude",
            "world_order",
            basis.world_magnitude_order().to_string(),
        ),
        entry(
            "geometry.planar_local_frame.normalization_scale",
            "normalization",
            basis.normalization_scale().to_bits().to_string(),
        ),
        entry(
            "geometry.planar_local_frame.scale_separation",
            "scale_separation",
            basis.scale_separation_orders().to_string(),
        ),
        entry(
            "geometry.planar_local_frame.transform_chain",
            "transform_chain",
            basis.transform_chain_digest(),
        ),
        entry(
            "geometry.planar_local_frame.movement_rotation",
            "movement",
            basis.movement_rotation_posture_identity(),
        ),
        entry(
            "geometry.planar_local_frame.tolerance_policy",
            "tolerance",
            basis.tolerance_policy_identity(),
        ),
        entry(
            "geometry.planar_local_frame.precision_fact",
            "precision_fact",
            basis.precision_fact_digest(),
        ),
        entry(
            "geometry.planar_local_frame.precision_declaration",
            "precision_declaration",
            basis.precision_declaration_digest(),
        ),
        entry(
            "geometry.planar_local_frame.precision_envelope",
            "precision_envelope",
            basis.precision_envelope_digest(),
        ),
    ]
}

fn entry(
    query_locus: &'static str,
    digest_label: &'static str,
    value: impl Into<String>,
) -> PlanarLocalFrameBasisIdentityEntry {
    PlanarLocalFrameBasisIdentityEntry::new(query_locus, digest_label, value.into())
}
