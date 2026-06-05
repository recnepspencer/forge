use hadwiger_research::facade::{
    admit_plane_lower_bound_claim_checked, GraphVersion, PlaneLowerBoundClaimRequest,
};

fn main() {
    let _ = admit_plane_lower_bound_claim_checked;
    let _: fn(String, &GraphVersion) -> PlaneLowerBoundClaimRequest =
        PlaneLowerBoundClaimRequest::new;
}
