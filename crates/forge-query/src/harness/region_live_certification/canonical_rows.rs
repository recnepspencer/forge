use super::bundles::canonical_row;
use super::row_catalog::CANONICAL_ROW_SPECS;
use crate::harness::live_certification::LiveCertificationRow;
use crate::harness::profiles::CertificationProfile;

pub(super) fn canonical_rows() -> Vec<LiveCertificationRow> {
    CANONICAL_ROW_SPECS
        .iter()
        .map(|spec| {
            canonical_row(
                spec.row_name,
                spec.perturbation_class,
                spec.hostile_expectation,
                (spec.control_lane)(CertificationProfile::DirectConstruction),
                (spec.hostile_lane)(CertificationProfile::BuilderReordering),
                (spec.parity_lane)(CertificationProfile::ReplayParity),
            )
        })
        .collect()
}
