use super::super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::super::coverage_target::KernelCompiledProductConsumerCoverageTarget;
use super::super::dependency_row::KernelCompiledProductConsumerClusterIdentity;
use super::super::error::KernelCompiledProductConsumerDependencyError;
use super::super::family_class::KernelCompiledProductFamilyClass;
use super::super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::super::proof_basis::KernelCompiledProductProofBasis;
use crate::workload_composition::CompiledProductReuseSurfaceIdentity as Surface;

pub(super) fn current_spatial_consumer_rows() -> Result<
    Vec<KernelCompiledProductConsumerCoverageTarget>,
    KernelCompiledProductConsumerDependencyError,
> {
    Ok(vec![
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::LookupConsumedWorkload,
            "crates/worth-kernel/src/workload_composition/worth_workload/lookup_consumed_workload/mod.rs",
            "WorthWorkload::admit_lookup_consumed_workload",
            KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived,
            KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex,
            KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "workload evidence stage index plus evidence-lookup stage receipt authority",
                "spatial evidence lookup execution footprint",
                "lookup-consumed workload handoff admission",
                "evidence lookup execution receipt and workload handoff counters",
                "spatial.selected_equivalence.evidence_lookup:v1",
            ),
            None,
            "workload entry still treats lookup-consumed reuse pressure as a local composition detail",
            &[
                Surface::LookupConsumedWorkloadCompositionAdmit,
                Surface::WorthWorkloadAdmitLookupConsumedWorkload,
            ],
        ),
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::LookupConsumedBatchExecution,
            "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/lookup_consumed_cluster.rs",
            "WorthWorkload::admit_lookup_consumed_batch_execution_cluster",
            KernelCompiledProductConsumerResponsibility::SpatialEvidenceDerived,
            KernelCompiledProductFamilyClass::SpatialEvidenceLookupIndex,
            KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "workload evidence stage index, selected lookup handoff identity, and attached batch-execution receipt authority",
                "lookup-consumed batch-execution cluster footprint",
                "lookup-consumed workload admission plus bound batch execution receipt",
                "typed lookup handoff counters carried through grouped workload admission",
                "spatial.selected_equivalence.evidence_lookup:v1",
            ),
            None,
            "lookup-consumed batch execution is one spatial/evidence-derived consumer cluster and must stay distinct from retained replay carry-forward pressure",
            &[Surface::WorthWorkloadAdmitLookupConsumedBatchExecutionCluster],
        ),
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::RetainedReplayBatchExecutionCarryForward,
            "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/lookup_consumed_cluster.rs",
            "WorthWorkload::admit_lookup_consumed_batch_execution_cluster",
            KernelCompiledProductConsumerResponsibility::RetainedReplay,
            KernelCompiledProductFamilyClass::SpatialRetainedReplayWorkload,
            KernelCompiledProductFutureCutoverLane::SpatialCompiledProductConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "workload authority plus batch-admission execution receipt authority",
                "retained replay and grouped workload cluster footprint",
                "lookup-consumed workload admission plus attached batch execution receipt",
                "retained replay receipt identity carried through the completed split cluster",
                "spatial.retained_replay.workload:v1",
            ),
            None,
            "retained replay carry-forward pressure on the batch execution cluster is a distinct consumer cluster and must not hide behind lookup-consumed admission",
            &[Surface::WorthWorkloadAdmitLookupConsumedBatchExecutionCluster],
        ),
    ])
}
