use super::super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::super::coverage_target::KernelCompiledProductConsumerCoverageTarget;
use super::super::dependency_row::KernelCompiledProductConsumerClusterIdentity;
use super::super::error::KernelCompiledProductConsumerDependencyError;
use super::super::family_class::KernelCompiledProductFamilyClass;
use super::super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::super::proof_basis::KernelCompiledProductProofBasis;

pub(super) fn current_replay_undo_consumer_rows() -> Result<
    Vec<KernelCompiledProductConsumerCoverageTarget>,
    KernelCompiledProductConsumerDependencyError,
> {
    Ok(vec![KernelCompiledProductConsumerCoverageTarget::new(
        KernelCompiledProductConsumerClusterIdentity::ReplayUndoBoundary,
        "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/current_route_authority.rs",
        "current_replay_undo_boundary_route_authority",
        KernelCompiledProductConsumerResponsibility::OrdinarySweep,
        KernelCompiledProductFamilyClass::ReplayUndoBoundaryProof,
        KernelCompiledProductFutureCutoverLane::ReplayUndoCompiledProductConsumerCutover,
        KernelCompiledProductProofBasis::new(
            "completed split route authority plus replay/undo boundary proof authority",
            "replay-scope and undo-scope locality footprint",
            "current replay/undo boundary proof and inventory coverage witness",
            "transaction packet identity plus replay/undo scope identities",
            "replay_undo.boundary.consumer:v1",
        ),
        None,
        "ordinary consumer sweep still depends on replay/undo proof as a local route authority chain",
        &[],
    )])
}
