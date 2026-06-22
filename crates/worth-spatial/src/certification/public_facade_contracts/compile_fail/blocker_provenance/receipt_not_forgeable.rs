use worth_spatial::facade::blocker_provenance::{
    WorkloadBlockerBoundaryKind, WorkloadBlockerProvenanceReceipt, WorkloadBlockerSourceKind,
};

fn main() {
    let _receipt = WorkloadBlockerProvenanceReceipt {
        provenance_digest: "fake".to_string(),
        source_kind: WorkloadBlockerSourceKind::DirtyTopology,
        boundary_kind: WorkloadBlockerBoundaryKind::CleanFailBoundary,
        source_identity: "fake".to_string(),
        boundary_identity: "fake".to_string(),
        human_reason: "fake".to_string(),
    };
}
