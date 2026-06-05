use hadwiger_research::facade::{
    admit_plane_exact_value_claim_checked, PlaneExactValueClaimRequest, ProofClaim,
    RetainedBackgroundTheorem, WholePlaneColoringVerification,
};

fn main() {
    let _ = admit_plane_exact_value_claim_checked;
    let _: fn(String, &ProofClaim, &WholePlaneColoringVerification) -> PlaneExactValueClaimRequest =
        PlaneExactValueClaimRequest::from_checked_upper_bound;
    let _: fn(String, &ProofClaim, &RetainedBackgroundTheorem) -> PlaneExactValueClaimRequest =
        PlaneExactValueClaimRequest::from_background_upper_bound;
}
