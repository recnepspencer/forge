use worth_kernel::workload_composition::{
    KernelCompiledProductConsumerClusterIdentity, KernelCompiledProductConsumerDependencyRow,
    KernelCompiledProductConsumerResponsibility, KernelCompiledProductFamilyClass,
    KernelCompiledProductFutureCutoverLane, KernelCompiledProductProofBasis,
};

fn main() {
    let _ = KernelCompiledProductConsumerDependencyRow {
        cluster_identity: KernelCompiledProductConsumerClusterIdentity::LookupConsumedWorkload,
        current_source_path: "fake",
        current_consumer_surface: "fake",
        responsibility: KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived,
        family_class: KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex,
        future_cutover_lane:
            KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
        proof_basis: KernelCompiledProductProofBasis::new("a", "b", "c", "d", "e"),
        query_boundary_lane: None,
        reason: "fake",
    };
}
