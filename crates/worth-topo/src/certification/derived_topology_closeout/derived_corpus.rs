use super::aggregate_reports::{
    aggregate_derived_digest, aggregate_truth_basis_digest, build_derived_branch_local_report,
    build_derived_family_coverage_matrix, build_derived_family_parity_matrix,
    build_derived_replay_report, build_milestone_two_counter_report,
};
use super::*;

pub(crate) fn certify_milestone_two_default_derived_corpus_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneTwoDerivedCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_corpus =
        certify_milestone_one_default_primitive_corpus_impl(&mut runtime_factory, stem)?;
    let bridge_proof_report = certify_milestone_one_bridge_proof(&format!("{stem}.bridge"))?;

    Ok(MilestoneTwoDerivedCorpusReport {
        materialized_topology_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .materialized_topology_digest
                .clone()
        }),
        interpreted_topology_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .interpreted_topology_digest
                .clone()
        }),
        derived_validation_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .derived_validation_digest
                .clone()
        }),
        derived_truth_basis_digest: aggregate_truth_basis_digest(&primitive_corpus),
        derived_family_coverage_matrix: build_derived_family_coverage_matrix(&primitive_corpus),
        derived_family_parity_matrix: build_derived_family_parity_matrix(
            &primitive_corpus.parity_report,
        ),
        derived_branch_local_parity_report: build_derived_branch_local_report(&primitive_corpus),
        derived_replay_parity_report: build_derived_replay_report(&primitive_corpus),
        derived_bridge_family_coverage_report: bridge_proof_report.family_coverage_report.clone(),
        bridge_routing_digest: bridge_proof_report.bridge_routing_digest.clone(),
        bridge_historical_evaluation_digest: bridge_proof_report
            .bridge_historical_evaluation_digest
            .clone(),
        milestone_2_counter_report: build_milestone_two_counter_report(&primitive_corpus),
        primitive_corpus,
        bridge_proof_report,
    })
}




