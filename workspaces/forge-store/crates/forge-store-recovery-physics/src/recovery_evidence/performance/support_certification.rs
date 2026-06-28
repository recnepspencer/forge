use forge_foundational::{
    attach_counter_backed_performance_receipt, bridge_certified_performance_bundle_trust_boundary,
    certify_support_expansion_performance_report,
    foundational_performance_certified_attachment_authority,
    foundational_performance_certified_readmission_authority, plan_performance_report,
    readmit_certified_performance_bundle_after_boundary, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceReportRequest,
};
use forge_proof::TransitionOutcome;

use super::super::denial::RecoveryEvidenceDenial;
use super::super::materialization::full_profile_set;
use super::receipt::{
    RecoveryAttachedCounterBackedPerformanceReceipt, RecoveryCertifiedPerformanceBundle,
    RecoveryMaterializedPerformanceReport,
};

pub(crate) fn support_expansion_report(
    counter_backed: &FoundationalCounterBackedPerformanceReceipt<
        FoundationalAuthoritativePerformanceClaim,
    >,
) -> RecoveryMaterializedPerformanceReport {
    let attached = attach_counter_backed_performance_receipt(
        forge_foundational::FoundationalPerformanceAttachmentTargetKind::SupportBundle,
        counter_backed.clone(),
    )
    .expect("counter-backed receipts can attach to support bundles");
    materialize_support_expansion_report(attached)
}

pub(crate) fn certified_support_expansion(
    counter_backed: &FoundationalCounterBackedPerformanceReceipt<
        FoundationalAuthoritativePerformanceClaim,
    >,
) -> Result<RecoveryCertifiedPerformanceBundle, RecoveryEvidenceDenial> {
    match certify_support_expansion_performance_report(
        support_expansion_report(counter_backed),
        foundational_performance_certified_attachment_authority(),
    ) {
        TransitionOutcome::Success(bundle) => Ok(bundle),
        _ => Err(RecoveryEvidenceDenial::PerformanceCertificationDenied),
    }
}

pub(crate) fn readmitted_support_expansion(
    counter_backed: &FoundationalCounterBackedPerformanceReceipt<
        FoundationalAuthoritativePerformanceClaim,
    >,
) -> Result<RecoveryCertifiedPerformanceBundle, RecoveryEvidenceDenial> {
    let certified = certified_support_expansion(counter_backed)?;
    let basis = certified.readmission_basis().clone();
    let bridged = bridge_certified_performance_bundle_trust_boundary(certified);
    Ok(readmit_certified_performance_bundle_after_boundary(
        bridged,
        basis,
        foundational_performance_certified_readmission_authority(),
    ))
}

fn materialize_support_expansion_report(
    attached: RecoveryAttachedCounterBackedPerformanceReceipt,
) -> RecoveryMaterializedPerformanceReport {
    plan_performance_report(FoundationalPerformanceReportRequest {
        source: attached,
        profile: full_profile_set().expect("full recovery evidence profile is coherent"),
        include_layout_intent: false,
        include_contract_names: false,
        include_counter_specs: true,
        include_counter_rows: true,
        include_supporting_evidence_rows: true,
        include_budget_decisions: false,
        include_denied_work: false,
        include_widened_work: false,
    })
    .materialize()
}
