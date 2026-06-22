use super::ProjectPointToCertifiedPlane2DBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectPointToCertifiedPlane2DIdentityEntry {
    locus: &'static str,
    value: String,
}

impl ProjectPointToCertifiedPlane2DIdentityEntry {
    fn new(locus: &'static str, value: impl Into<String>) -> Self {
        Self {
            locus,
            value: value.into(),
        }
    }

    pub(crate) fn locus(&self) -> &'static str {
        self.locus
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

pub(crate) fn project_point_to_certified_plane_2d_identity_entries(
    basis: &ProjectPointToCertifiedPlane2DBasis,
) -> Vec<ProjectPointToCertifiedPlane2DIdentityEntry> {
    vec![
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.source_point_identity",
            basis.source_point_identity(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.source_point",
            format!("{:?}", basis.source_point()),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.source_point_basis",
            basis.source_point_basis_digest(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.local_delta",
            format!("{:?}", basis.local_delta_from_frame_origin()),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.local_frame_fact",
            basis.local_frame_fact_digest(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.local_frame_declaration",
            basis.local_frame_declaration_digest(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.local_frame_envelope",
            basis.local_frame_envelope_digest(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.frame_identity",
            basis.frame_identity(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.frame_origin",
            format!("{:?}", basis.frame_origin()),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.u_axis",
            format!("{:?}", basis.u_axis()),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.v_axis",
            format!("{:?}", basis.v_axis()),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.w_axis",
            format!("{:?}", basis.w_axis()),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.transform_chain",
            basis.transform_chain_digest(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.movement_rotation",
            basis.movement_rotation_posture_identity(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.tolerance_policy",
            basis.tolerance_policy_identity(),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.point_2d",
            format!("{:?}", basis.point_2d()),
        ),
        ProjectPointToCertifiedPlane2DIdentityEntry::new(
            "geometry.planar_projection.signed_distance",
            basis.signed_distance_to_plane_bits().to_string(),
        ),
    ]
}
