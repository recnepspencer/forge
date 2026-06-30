use super::super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::super::coverage_target::KernelCompiledProductConsumerCoverageTarget;
use super::super::dependency_row::KernelCompiledProductConsumerClusterIdentity;
use super::super::error::KernelCompiledProductConsumerDependencyError;
use super::super::family_class::KernelCompiledProductFamilyClass;
use super::super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::super::proof_basis::KernelCompiledProductProofBasis;
use super::super::query_boundary_lane::KernelCompiledProductQueryBoundaryLane;
use crate::workload_composition::CompiledProductReuseSurfaceIdentity as Surface;

pub(super) fn current_query_boundary_consumer_rows() -> Result<
    Vec<KernelCompiledProductConsumerCoverageTarget>,
    KernelCompiledProductConsumerDependencyError,
> {
    Ok(vec![
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::ReplayUndoPublicCloseoutReadModel,
            "crates/worth-kernel/src/replay_undo_consumer_cutover/public_closeout/inventory_classification.rs",
            "ReplayUndoPublicCloseoutInventoryRow::from_inventory",
            KernelCompiledProductConsumerResponsibility::QueryBacked,
            KernelCompiledProductFamilyClass::QueryProjectionConsumption,
            KernelCompiledProductFutureCutoverLane::QueryProjectionConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "query-owned projection fact authority over inventory-backed public read models",
                "public replay/undo closeout row projection footprint",
                "inventory classification admitted into one read-model projection",
                "typed projection-consumption facts rather than row archaeology",
                "query.projection_consumption.read_model:v1",
            ),
            Some(KernelCompiledProductQueryBoundaryLane::ProjectionConsumption),
            "public replay/undo inventory projection is a query-backed read-model consumer and must name the real projection lane",
            &[Surface::ReplayUndoPublicCloseoutReadModelProjection],
        ),
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::KernelConflictPublicCloseoutBoundaryTraceability,
            "crates/worth-kernel/src/workload_composition/public_closeout/public_closeout.rs",
            "current_worth_touched_graph_conflict_public_closeout",
            KernelCompiledProductConsumerResponsibility::QueryBacked,
            KernelCompiledProductFamilyClass::QueryLowerRuntimeBoundaryEnvelope,
            KernelCompiledProductFutureCutoverLane::QueryBoundaryEnvelopeConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "query lower-runtime boundary envelope authority over downstream boundary explanation",
                "public boundary traceability footprint for conflict closeout consumers",
                "receipt-backed lower-runtime boundary support and traceability",
                "typed lower-runtime boundary support artifacts rather than local support folklore",
                "query.lower_runtime_boundary_envelope:v1",
            ),
            Some(KernelCompiledProductQueryBoundaryLane::LowerRuntimeBoundaryEnvelope),
            "kernel public closeout still needs one explicit Query boundary lane for downstream traceability instead of local boundary prose",
            &[Surface::KernelConflictPublicCloseoutBoundaryTraceability],
        ),
    ])
}
