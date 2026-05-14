use forge_query::facade::{
    BasisLifecycleAdapterOutcome, BasisLifecycleAdapterProof, BasisLifecycleMigrationSurface,
};

fn main() {
    let _proof = BasisLifecycleAdapterProof {
        surface: BasisLifecycleMigrationSurface::BranchPreviewAdmission,
        entrypoint: "host-forged-entrypoint",
        target_lifecycle_phase: "forged-phase",
        operation_lane: "inspection",
        outcome: BasisLifecycleAdapterOutcome::ScopedCapability,
        source_digest: "source".to_string(),
        lifecycle_proof_digest: "lifecycle".to_string(),
        adapter_proof_digest: "adapter".to_string(),
    };
}
