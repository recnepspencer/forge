use super::super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::super::coverage_target::KernelCompiledProductConsumerCoverageTarget;
use super::super::dependency_row::KernelCompiledProductConsumerClusterIdentity;
use super::super::error::KernelCompiledProductConsumerDependencyError;
use super::super::family_class::KernelCompiledProductFamilyClass;
use super::super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::super::proof_basis::KernelCompiledProductProofBasis;
use crate::workload_composition::CompiledProductReuseSurfaceIdentity as Surface;

pub(super) fn current_public_closeout_consumer_rows() -> Result<
    Vec<KernelCompiledProductConsumerCoverageTarget>,
    KernelCompiledProductConsumerDependencyError,
> {
    Ok(vec![
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::OrdinaryConsumerCutoverSummary,
            "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/current_cutover.rs",
            "current_worth_workload_ordinary_consumer_cutover",
            KernelCompiledProductConsumerResponsibility::OrdinarySweep,
            KernelCompiledProductFamilyClass::KernelOrdinaryConsumerCutoverSummary,
            KernelCompiledProductFutureCutoverLane::OrdinarySweepConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "phase-eleven consumer sweep inventory plus current batch execution receipt authority",
                "ordinary consumer cutover surface footprint",
                "current route witnesses and selected-plan witness summaries",
                "cutover rows, posture classifications, and bound batch execution receipt",
                "kernel.ordinary_consumer_cutover.summary:v1",
            ),
            None,
            "ordinary consumer cutover is itself one kernel-owned closeout summary consumer and must remain visible as a separate cluster",
            &[Surface::CurrentWorthWorkloadOrdinaryConsumerCutover],
        ),
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::ConflictPublicCloseout,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_proof/current.rs",
            "current_worth_touched_graph_conflict_public_closeout",
            KernelCompiledProductConsumerResponsibility::PublicCloseout,
            KernelCompiledProductFamilyClass::KernelPublicCloseoutProofChain,
            KernelCompiledProductFutureCutoverLane::PublicCloseoutCompiledProductConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "conflict closeout proof chain authority and deletion closeout authority",
                "ordinary consumer residue chain footprint",
                "current ordinary-consumer cutover plus deletion closeout and source-firewall proof",
                "published public closeout proof-chain digest set",
                "kernel.public_closeout.proof_chain:v1",
            ),
            None,
            "public closeout now lives on the planner-owned proof lane and must stay certified there instead of reviving the deleted legacy helper path",
            &[Surface::CurrentWorthTouchedGraphConflictPublicCloseout],
        ),
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::ConflictPublicCloseoutSeed,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_proof/current.rs",
            "current_worth_touched_graph_conflict_milestone_fifteen_seed",
            KernelCompiledProductConsumerResponsibility::PublicCloseout,
            KernelCompiledProductFamilyClass::KernelPublicCloseoutSeed,
            KernelCompiledProductFutureCutoverLane::PublicCloseoutCompiledProductConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "public closeout proof chain and source-firewall authority",
                "Milestone 15 public closeout seed footprint",
                "published public closeout chain lowered into one seed artifact",
                "residue digest plus source-firewall digest preserved in seed lowering",
                "kernel.public_closeout.seed:v1",
            ),
            None,
            "Milestone 15 seed must stay certified on the planner-owned public-proof lane instead of a deleted legacy public-closeout helper",
            &[Surface::CurrentWorthTouchedGraphConflictMilestoneFourteenSeed],
        ),
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::SpatialEvidenceLookupPublicCloseout,
            "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs",
            "current_evidence_lookup_public_closeout",
            KernelCompiledProductConsumerResponsibility::PublicCloseout,
            KernelCompiledProductFamilyClass::SpatialEvidenceLookupPublicCloseout,
            KernelCompiledProductFutureCutoverLane::PublicCloseoutCompiledProductConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "evidence lookup family-stage proof authority and milestone-twelve seed authority",
                "evidence lookup public closeout row footprint",
                "receipt-backed evidence lookup stage cutover plus declared family support",
                "typed family-stage rows and public support payloads",
                "spatial.evidence_lookup.public_closeout:v1",
            ),
            None,
            "spatial evidence lookup public closeout is a covered public-closeout consumer and must be named by the kernel matrix before later read-model cutover",
            &[Surface::CurrentEvidenceLookupPublicCloseout],
        ),
    ])
}
