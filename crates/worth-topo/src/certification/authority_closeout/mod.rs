use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::runtime::RelationalRuntime;

use crate::certification::authority_closeout::read_view::MilestoneOneCertificationHarness;
use crate::certification::bridge::certify_milestone_one_bridge_proof;
use crate::certification::core::{CertificationRequiredOutput, CertificationSuiteRequirements};
use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::primitive_corpus::{
    certify_milestone_one_admitted_range_sweeps,
    certify_milestone_one_default_primitive_corpus_impl,
};
use crate::certification::rejections::certify_milestone_one_illegal_topology_rejections;
use crate::certification::requirements::milestone_one_closeout_requirements;
use crate::certification::shared::digest_rows;
use crate::certification::support::reporting::{
    AdmittedRangeSweepReport, DeterministicDigest, FailureLocalityReport, FailureLocalityRow,
    IllegalTopologyRejectionReport, MilestoneOneBranchLocalAggregateReport,
    MilestoneOneCertificationReport, MilestoneOneCloseoutReport, MilestoneOneCounters,
    MilestoneOneRejectionClassReport, MilestoneOneRejectionClassRow,
    MilestoneOneReplayAggregateReport, MilestoneOneValidationAggregateReport,
    MilestoneOneValidationAggregateRow, MilestoneOneValidatorCoverageReport,
    MilestoneOneValidatorCoverageRow, NamingAttachmentAggregateReport,
    NamingAttachmentAggregateRow, PrimitiveCorpusReport, ReplayParityStatus,
    TopologyLocalizationAggregateEntityRow, TopologyLocalizationAggregateRelationRow,
    TopologyLocalizationAggregateReport,
};
use crate::certification::BoundaryFailure;
use crate::test_support::primitive_corpus::validated_topology::seeded_bootstrap;

mod aggregates_a;
mod aggregates_b;
mod closures;
pub(crate) mod read_view;

use self::aggregates_a::{
    build_closeout_counter_report, build_closeout_digest, build_closeout_localization_report,
    build_closeout_naming_attachment_report, build_closeout_validation_report,
    build_closeout_validator_coverage_report,
};
use self::aggregates_b::{
    build_closeout_branch_local_report, build_closeout_rejection_class_report,
    build_closeout_replay_report, build_failure_locality_report,
};
use self::closures::{
    ensure_bridge_coverage_closure, ensure_failure_locality_closure,
    ensure_family_coverage_closure, ensure_parity_closure, ensure_rejection_class_closure,
    ensure_required_output_closure, ensure_sweep_closure, ensure_validator_expectation_closure,
};

pub fn certify_milestone_one_closeout_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneOneCloseoutReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let requirements = milestone_one_closeout_requirements();
    let mut baseline_runtime = runtime_factory();
    let seeded =
        seeded_bootstrap(&mut baseline_runtime, &format!("{stem}.bootstrap")).map_err(|error| {
            MilestoneOneCertificationError::ReadView(format!(
                " milestone one closeout failed to seed bootstrap truth: {error:?}"
            ))
        })?;
    let seeded_bootstrap =
        MilestoneOneCertificationHarness::certify_read_basis_with_runtime_traced(
            &mut baseline_runtime,
            seeded.read_basis().clone(),
            Some(&seeded.persisted_truth().batch),
            1,
        )
        .map_err(BoundaryFailure::into_error)?
        .into_primary_result();
    let primitive_corpus = certify_milestone_one_default_primitive_corpus_impl(
        &mut runtime_factory,
        &format!("{stem}.corpus"),
    )?;
    let admitted_range_sweeps = certify_milestone_one_admitted_range_sweeps(
        &mut runtime_factory,
        &format!("{stem}.sweeps"),
    )?;
    let illegal_topology_rejection_report = certify_milestone_one_illegal_topology_rejections(
        &mut runtime_factory,
        &format!("{stem}.illegal"),
    )?;
    let bridge_proof_report = certify_milestone_one_bridge_proof(&format!("{stem}.bridge"))?;
    let topology_truth_digest =
        build_closeout_digest(&seeded_bootstrap, &primitive_corpus, |report| {
            report.topology_truth_digest.clone()
        });
    let naming_truth_digest =
        build_closeout_digest(&seeded_bootstrap, &primitive_corpus, |report| {
            report.naming_truth_digest.clone()
        });
    let topology_validation_digest =
        build_closeout_digest(&seeded_bootstrap, &primitive_corpus, |report| {
            report.topology_validation_digest.clone()
        });
    let topology_validation_report =
        build_closeout_validation_report(&seeded_bootstrap, &primitive_corpus);
    let topology_localization_report =
        build_closeout_localization_report(&seeded_bootstrap, &primitive_corpus);
    let naming_attachment_report =
        build_closeout_naming_attachment_report(&seeded_bootstrap, &primitive_corpus);
    let primitive_family_coverage_matrix = primitive_corpus.coverage_matrix.clone();
    let primitive_corpus_parity_report = primitive_corpus.parity_report.clone();
    let validator_coverage_report =
        build_closeout_validator_coverage_report(&topology_validation_report);
    let branch_local_topology_report =
        build_closeout_branch_local_report(&seeded_bootstrap, &primitive_corpus);
    let milestone_1_replay_parity_report =
        build_closeout_replay_report(&seeded_bootstrap, &primitive_corpus);
    let rejection_class_report = build_closeout_rejection_class_report(
        &primitive_corpus,
        &illegal_topology_rejection_report,
    );
    let failure_locality_report =
        build_failure_locality_report(&primitive_corpus, &illegal_topology_rejection_report);
    let bridge_family_coverage_report = bridge_proof_report.family_coverage_report.clone();
    let counter_report = build_closeout_counter_report(
        &seeded_bootstrap,
        &primitive_corpus,
        &illegal_topology_rejection_report,
    );

    ensure_family_coverage_closure(&primitive_family_coverage_matrix, &requirements)?;
    ensure_parity_closure(&primitive_corpus_parity_report, &requirements)?;
    ensure_validator_expectation_closure(&validator_coverage_report, &requirements)?;
    ensure_rejection_class_closure(&rejection_class_report, &requirements)?;
    ensure_sweep_closure(&admitted_range_sweeps, &requirements)?;
    ensure_failure_locality_closure(&failure_locality_report, &requirements)?;
    ensure_bridge_coverage_closure(&bridge_proof_report.family_coverage_report, &requirements)?;

    let closeout = MilestoneOneCloseoutReport {
        topology_truth_digest,
        naming_truth_digest,
        topology_validation_digest,
        topology_validation_report,
        topology_localization_report,
        naming_attachment_report,
        primitive_family_coverage_matrix,
        primitive_corpus_parity_report,
        admitted_range_sweep_report: admitted_range_sweeps,
        validator_coverage_report,
        branch_local_topology_report,
        milestone_1_replay_parity_report,
        rejection_class_report,
        failure_locality_report,
        bridge_family_coverage_report,
        seeded_bootstrap,
        primitive_corpus,
        illegal_topology_rejection_report,
        bridge_proof_report,
        milestone_1_counter_report: counter_report,
    };

    ensure_required_output_closure(&closeout, &requirements)?;
    Ok(closeout)
}




