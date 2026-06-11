use super::CertifiedSignedArea2DBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CertifiedSignedArea2DIdentityEntry {
    locus: String,
    value: String,
}

impl CertifiedSignedArea2DIdentityEntry {
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

pub(crate) fn certified_signed_area_2d_identity_entries(
    basis: &CertifiedSignedArea2DBasis,
) -> Vec<CertifiedSignedArea2DIdentityEntry> {
    vec![
        entry(
            "geometry.signed_area_2d.primary_loop",
            basis.primary_loop_identity(),
        ),
        entry(
            "geometry.signed_area_2d.planar_neighborhood",
            basis.planar_neighborhood_identity(),
        ),
        entry(
            "geometry.signed_area_2d.frame_identity",
            basis.frame_identity(),
        ),
        entry(
            "geometry.signed_area_2d.movement_rotation",
            basis.movement_rotation_posture_identity(),
        ),
        entry(
            "geometry.signed_area_2d.tolerance_policy",
            basis.tolerance_policy_identity(),
        ),
        entry(
            "geometry.signed_area_2d.winding_fact",
            basis.winding_receipt().fact_digest(),
        ),
        entry(
            "geometry.signed_area_2d.precision_fact",
            basis.precision_receipt().fact_digest(),
        ),
        entry(
            "geometry.signed_area_2d.degeneracy_policy",
            basis.degeneracy_policy().as_str(),
        ),
        entry(
            "geometry.signed_area_2d.orientation",
            basis.orientation().as_str(),
        ),
        entry(
            "geometry.signed_area_2d.degeneracy",
            basis.degeneracy().as_str(),
        ),
        entry(
            "geometry.signed_area_2d.signed_area_twice",
            basis.signed_area_twice_decimal(),
        ),
        entry(
            "geometry.signed_area_2d.localized_cause",
            basis
                .localized_cause()
                .map(|cause| cause.identity())
                .unwrap_or_else(|| "none".to_string()),
        ),
    ]
}

fn entry(locus: impl Into<String>, value: impl Into<String>) -> CertifiedSignedArea2DIdentityEntry {
    CertifiedSignedArea2DIdentityEntry::new(locus, value)
}
