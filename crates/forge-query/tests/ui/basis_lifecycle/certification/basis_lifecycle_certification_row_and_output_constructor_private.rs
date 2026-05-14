use forge_query::facade::{
    BasisEligibilityCounters, BasisFamily, BasisLifecycleCertificationLane,
    BasisLifecycleCertificationOutputDigest, BasisLifecycleCertificationOutputPosture,
    BasisLifecycleCertificationRow,
};

fn main() {
    let _ = BasisLifecycleCertificationRow {
        lane: BasisLifecycleCertificationLane::Admitted,
        basis_family: BasisFamily::CurrentHead,
        operation_lane: "observation",
        artifact_digest: String::new(),
        failure_digest: None,
        counter_snapshot_digest: BasisEligibilityCounters::default().digest(),
        row_digest: String::new(),
    };

    let _ = BasisLifecycleCertificationOutputDigest {
        name: "basis_target_dx_digest",
        posture: BasisLifecycleCertificationOutputPosture::Deferred,
        digest: String::new(),
    };
}
