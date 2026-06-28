use forge_foundational::{
    claim_derived_projection_boundary_surface, claim_receipt_evidence_boundary_surface,
    claim_support_only_boundary_surface, plan_artifact_boundary_bundle,
    plan_descriptive_boundary_materialization, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
    FoundationalBoundaryReceiptSurface, FoundationalBoundaryReportSurface,
};

use super::super::performance::RecoveryCounterPerformanceReceipt;
use super::canonical_basis::{full_profile_set, materialized_profile_set};
use super::foundational_bundle::{
    MaterializedFoundationalRecoveryEvidenceBundle, RecoveryEvidenceBundlePrimary,
};
use super::receipt::RecoveryPhysicsReceipt;
use super::report::RecoveryPhysicsReport;

pub(crate) fn materialize_bundle(
    receipt: &RecoveryPhysicsReceipt,
    report: &RecoveryPhysicsReport,
    performance: &RecoveryCounterPerformanceReceipt,
) -> MaterializedFoundationalRecoveryEvidenceBundle {
    let profile = materialized_profile_set(
        full_profile_set().expect("full recovery evidence profile is coherent"),
    )
    .expect("full recovery evidence profile materializes");
    let primary = plan_descriptive_boundary_materialization(
        claim_derived_projection_boundary_surface(FoundationalBoundaryArtifactSurface::new(
            RecoveryEvidenceBundlePrimary::from_members(receipt, performance),
            3,
        )),
        FoundationalBoundaryMaterializationSource::NativeAuthority,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        profile,
    )
    .expect("primary recovery evidence bundle plan");
    let report_plan = plan_descriptive_boundary_materialization(
        claim_support_only_boundary_surface(
            FoundationalBoundaryReportSurface::new(report.payload().to_vec(), 1)
                .expect("report rows exist"),
        ),
        FoundationalBoundaryMaterializationSource::NativeAuthority,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        profile,
    )
    .expect("report recovery evidence bundle plan");
    let receipt_plan = plan_descriptive_boundary_materialization(
        claim_receipt_evidence_boundary_surface(
            FoundationalBoundaryReceiptSurface::new(
                "store recovery evidence bundle materialized",
                performance.exact_counter_assertions(),
            )
            .expect("receipt boundary is named"),
        ),
        FoundationalBoundaryMaterializationSource::NativeAuthority,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        profile,
    )
    .expect("receipt recovery evidence bundle plan");
    plan_artifact_boundary_bundle(primary)
        .with_report(report_plan)
        .expect("report member is legal")
        .with_receipt(receipt_plan)
        .expect("receipt member is legal")
        .materialize()
        .expect("recovery evidence bundle materializes")
}
