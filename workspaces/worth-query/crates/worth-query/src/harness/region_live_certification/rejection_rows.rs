use super::bundles::rejection_row;
use super::row_catalog::REJECTION_ROW_SPECS;
use crate::harness::live_certification::LiveRejectionRow;
use crate::harness::profiles::CertificationProfile;

pub(super) fn rejection_rows() -> Vec<LiveRejectionRow> {
    REJECTION_ROW_SPECS
        .iter()
        .map(|spec| {
            rejection_row(
                spec.row_name,
                spec.perturbation_class,
                (spec.control_lane)(CertificationProfile::DirectConstruction),
                (spec.hostile_lane)(CertificationProfile::BindingVariation),
                (spec.parity_lane)(CertificationProfile::ReplayParity),
            )
        })
        .collect()
}
