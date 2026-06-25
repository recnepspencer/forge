#[cfg(test)]
use crate::certification::support::commit_certification_input::TopologyCommitCertificationInput;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::topology_authoring::{
    DerivedTopologyReadBasis, MilestoneOnePrimitiveCase, MilestoneOnePrimitiveScenario,
};

use crate::certification::authority_closeout::certify_milestone_one_closeout_impl;
use crate::certification::authority_closeout::read_view::{
    MilestoneOneCertificationHarness, TracedMilestoneOneCertificationReport,
};
use crate::certification::bridge_registration_closeout::{
    certify_topology_bridge_registration_closeout as certify_topology_bridge_registration_closeout_impl,
    TopologyBridgeRegistrationCloseoutReport,
};
use crate::certification::committed_artifact_alignment_closeout::{
    certify_topology_committed_artifact_alignment_closeout as certify_topology_committed_artifact_alignment_closeout_impl,
    TopologyCommittedArtifactAlignmentCloseoutReport,
};
#[cfg(test)]
use crate::certification::derived_topology_closeout::certify_milestone_two_commit_input_traced_impl;
use crate::certification::derived_topology_closeout::TracedMilestoneTwoDerivedReadReport;
use crate::certification::derived_topology_closeout::{
    certify_milestone_two_closeout_impl, certify_milestone_two_default_derived_corpus_impl,
    certify_milestone_two_read_basis_runtime_traced_impl,
};
use crate::certification::error::{MilestoneOneCertificationError, TopologyCertificationError};
use crate::certification::historical_materialization_closeout::{
    certify_topology_historical_materialization_closeout as certify_topology_historical_materialization_closeout_impl,
    TopologyHistoricalMaterializationCloseoutReport,
};
use crate::certification::primitive_corpus::{
    certify_milestone_one_branch_local_primitive_scenarios_impl,
    certify_milestone_one_default_primitive_corpus_impl,
    certify_milestone_one_primitive_corpus_impl, certify_milestone_one_primitive_scenarios_impl,
};
use crate::certification::query_boundary_cleanup_closeout::{
    certify_topology_query_boundary_cleanup_closeout as certify_topology_query_boundary_cleanup_closeout_impl,
    TopologyQueryBoundaryCleanupCloseoutReport,
};
use crate::certification::support::reporting::{
    MilestoneOneCloseoutReport, MilestoneTwoCloseoutReport, MilestoneTwoDerivedCorpusReport,
    PrimitiveCorpusReport,
};
use crate::certification::topology_operator_closeout::{
    certify_milestone_three_ambiguous_local_rewire_continuity_impl,
    certify_milestone_three_bowtie_adjacent_rewire_impl,
    certify_milestone_three_broken_radial_localization_impl,
    certify_milestone_three_cancellation_chain_parity_impl, certify_milestone_three_closeout_impl,
    certify_milestone_three_hostile_suite_impl, certify_milestone_three_split_collapse_churn_impl,
    certify_topology_operator_selected_obligation_cutover_impl,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
};
use crate::certification::BoundaryFailure;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologySelectedGraphObligationEnforcementCloseout,
};

pub fn certify_milestone_one_read_basis_traced(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneOneCertificationReport, BoundaryFailure<MilestoneOneCertificationError>>
{
    MilestoneOneCertificationHarness::certify_read_basis_with_runtime_traced(
        runtime, read_basis, None, 0,
    )
}

#[cfg(test)]
pub(crate) fn certify_topology_commit_input_traced(
    runtime: &mut RelationalRuntime,
    commit_input: &TopologyCommitCertificationInput,
) -> Result<TracedMilestoneOneCertificationReport, BoundaryFailure<MilestoneOneCertificationError>>
{
    MilestoneOneCertificationHarness::certify_commit_input_traced(runtime, commit_input)
}

pub fn certify_milestone_two_read_basis_traced(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    certify_milestone_two_read_basis_runtime_traced_impl(runtime, read_basis)
}

#[cfg(test)]
pub(crate) fn certify_milestone_two_topology_commit_input_traced(
    runtime: &mut RelationalRuntime,
    commit_input: &TopologyCommitCertificationInput,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    certify_milestone_two_commit_input_traced_impl(runtime, commit_input)
}

pub fn certify_milestone_one_primitive_corpus<F>(
    runtime_factory: F,
    stem: &str,
    primitives: &[MilestoneOnePrimitiveCase],
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_primitive_corpus_impl(runtime_factory, stem, primitives)
}

pub fn certify_milestone_one_default_primitive_corpus<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_default_primitive_corpus_impl(runtime_factory, stem)
}

pub fn certify_milestone_two_default_derived_corpus<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneTwoDerivedCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_two_default_derived_corpus_impl(runtime_factory, stem)
}

pub fn certify_milestone_two_closeout<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneTwoCloseoutReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_two_closeout_impl(runtime_factory, stem)
}

pub fn certify_milestone_three_bowtie_adjacent_rewire<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_three_bowtie_adjacent_rewire_impl(runtime_factory, stem)
}

pub fn certify_milestone_three_ambiguous_local_rewire_continuity<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_three_ambiguous_local_rewire_continuity_impl(runtime_factory, stem)
}

pub fn certify_milestone_three_cancellation_chain_parity<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_three_cancellation_chain_parity_impl(runtime_factory, stem)
}

pub fn certify_milestone_three_split_collapse_churn<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_three_split_collapse_churn_impl(runtime_factory, stem)
}

pub fn certify_milestone_three_broken_radial_localization<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_three_broken_radial_localization_impl(runtime_factory, stem)
}

pub fn certify_milestone_three_hostile_suite<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileSuiteReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_three_hostile_suite_impl(runtime_factory, stem)
}

pub fn certify_milestone_three_closeout<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileSuiteReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_three_closeout_impl(runtime_factory, stem)
}

pub fn certify_topology_operator_selected_obligation_cutover(
    enforcement_closeout: &WorthTopologySelectedGraphObligationEnforcementCloseout,
) -> Result<WorthTopologyOperatorCertificationCutoverCloseout, WorthTopologyLegalityCatalogError> {
    certify_topology_operator_selected_obligation_cutover_impl(enforcement_closeout)
}

pub fn certify_milestone_one_closeout<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<MilestoneOneCloseoutReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_closeout_impl(runtime_factory, stem)
}

pub fn certify_topology_query_boundary_cleanup_closeout(
) -> Result<TopologyQueryBoundaryCleanupCloseoutReport, TopologyCertificationError> {
    certify_topology_query_boundary_cleanup_closeout_impl()
}

pub fn certify_topology_bridge_registration_closeout(
) -> Result<TopologyBridgeRegistrationCloseoutReport, TopologyCertificationError> {
    certify_topology_bridge_registration_closeout_impl()
}

pub fn certify_topology_historical_materialization_closeout(
) -> Result<TopologyHistoricalMaterializationCloseoutReport, TopologyCertificationError> {
    certify_topology_historical_materialization_closeout_impl()
}

pub fn certify_topology_committed_artifact_alignment_closeout(
) -> Result<TopologyCommittedArtifactAlignmentCloseoutReport, TopologyCertificationError> {
    certify_topology_committed_artifact_alignment_closeout_impl()
}

pub fn certify_milestone_one_primitive_scenarios<F>(
    runtime_factory: &mut F,
    stem: &str,
    scenarios: &[MilestoneOnePrimitiveScenario],
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_primitive_scenarios_impl(runtime_factory, stem, scenarios)
}

pub fn certify_milestone_one_branch_local_primitive_scenarios<F>(
    runtime_factory: &mut F,
    stem: &str,
    branch_id: &str,
    scenarios: &[MilestoneOnePrimitiveScenario],
) -> Result<PrimitiveCorpusReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_branch_local_primitive_scenarios_impl(
        runtime_factory,
        stem,
        branch_id,
        scenarios,
    )
}
