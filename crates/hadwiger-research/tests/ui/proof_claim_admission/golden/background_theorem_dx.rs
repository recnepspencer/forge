use hadwiger_research::facade::{
    retain_background_plane_seven_upper_bound_checked, HadwigerProofClaimAdmissionError,
    HadwigerResearchHandle, RetainedBackgroundTheorem,
};

fn main() {
    let _: fn(
        &HadwigerResearchHandle,
        String,
        String,
        String,
    ) -> Result<RetainedBackgroundTheorem, HadwigerProofClaimAdmissionError> =
        retain_background_plane_seven_upper_bound_checked;
}
