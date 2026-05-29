use super::aggregate_reports::{
    build_derived_equivalence_aggregate_report, build_derived_failure_locality_report,
    build_derived_fallback_aggregate_report, build_derived_invalidation_aggregate_report,
    build_derived_rebuild_aggregate_report, build_derived_validator_coverage_report,
};
use super::closeout_assertions::{
    ensure_milestone_two_bridge_closure, ensure_milestone_two_failure_locality_closure,
    ensure_milestone_two_family_coverage_closure, ensure_milestone_two_parity_closure,
    ensure_milestone_two_required_output_closure, ensure_milestone_two_validator_closure,
};
use super::derived_corpus::certify_milestone_two_default_derived_corpus_impl;
use super::*;

pub(crate) fn certify_milestone_two_closeout_impl<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneTwoCloseoutReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let requirements = milestone_two_closeout_requirements();
    let derived_corpus = certify_milestone_two_default_derived_corpus_impl(runtime_factory, stem)?;
    let primitive_corpus = &derived_corpus.primitive_corpus;

    let closeout = MilestoneTwoCloseoutReport {
        materialized_topology_digest: derived_corpus.materialized_topology_digest.clone(),
        interpreted_topology_digest: derived_corpus.interpreted_topology_digest.clone(),
        derived_validation_digest: derived_corpus.derived_validation_digest.clone(),
        derived_truth_basis_digest: derived_corpus.derived_truth_basis_digest.clone(),
        bridge_routing_digest: derived_corpus.bridge_routing_digest.clone(),
        bridge_historical_evaluation_digest: derived_corpus
            .bridge_historical_evaluation_digest
            .clone(),
        derived_family_coverage_matrix: derived_corpus.derived_family_coverage_matrix.clone(),
        derived_family_parity_matrix: derived_corpus.derived_family_parity_matrix.clone(),
        derived_validator_coverage_report: build_derived_validator_coverage_report(
            primitive_corpus,
        ),
        derived_invalidation_report: build_derived_invalidation_aggregate_report(primitive_corpus),
        derived_rebuild_report: build_derived_rebuild_aggregate_report(primitive_corpus),
        derived_equivalence_contract_report: build_derived_equivalence_aggregate_report(
            primitive_corpus,
        ),
        derived_fallback_report: build_derived_fallback_aggregate_report(primitive_corpus),
        derived_failure_locality_report: build_derived_failure_locality_report(primitive_corpus),
        derived_branch_local_parity_report: derived_corpus
            .derived_branch_local_parity_report
            .clone(),
        derived_replay_parity_report: derived_corpus.derived_replay_parity_report.clone(),
        derived_bridge_family_coverage_report: derived_corpus
            .derived_bridge_family_coverage_report
            .clone(),
        milestone_2_counter_report: derived_corpus.milestone_2_counter_report.clone(),
        derived_corpus,
    };

    ensure_milestone_two_family_coverage_closure(
        &closeout.derived_family_coverage_matrix,
        &requirements,
    )?;
    ensure_milestone_two_parity_closure(&closeout.derived_family_parity_matrix, &requirements)?;
    ensure_milestone_two_validator_closure(
        &closeout.derived_validator_coverage_report,
        &requirements,
    )?;
    ensure_milestone_two_bridge_closure(
        &closeout.derived_bridge_family_coverage_report,
        &requirements,
    )?;
    ensure_milestone_two_failure_locality_closure(
        &closeout.derived_failure_locality_report,
        &requirements,
    )?;
    ensure_milestone_two_required_output_closure(&closeout, &requirements)?;

    Ok(closeout)
}
