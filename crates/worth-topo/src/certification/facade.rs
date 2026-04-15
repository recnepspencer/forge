use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use worth_schema::facade::{
    DerivedTopologyReadBasis, VerifiedTopologyCommit, WorthMilestoneOnePrimitiveCase,
    WorthMilestoneOnePrimitiveScenario,
};

use crate::certification::closeout::certify_milestone_one_closeout_impl;
use crate::certification::corpus::{
    certify_milestone_one_branch_local_primitive_scenarios_impl,
    certify_milestone_one_default_primitive_corpus_impl,
    certify_milestone_one_primitive_corpus_impl,
    certify_milestone_one_primitive_scenarios_impl,
};
use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::milestone_two::{
    certify_milestone_two_closeout_impl, certify_milestone_two_default_derived_corpus_impl,
    certify_milestone_two_read_view_impl, certify_milestone_two_verified_commit_impl,
};
use crate::certification::read_view::{
    certify_milestone_one_read_view_impl, certify_verified_topology_commit_impl,
};
use crate::certification::report::{
    WorthMilestoneOneCertificationReport, WorthMilestoneOneCloseoutReport,
    WorthMilestoneTwoCloseoutReport,
    WorthMilestoneTwoDerivedCorpusReport, WorthMilestoneTwoDerivedReadReport,
    WorthPrimitiveCorpusReport,
};

pub fn certify_milestone_one_read_view(
    read_view: &RelationalReadView,
    read_basis: DerivedTopologyReadBasis,
) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
    certify_milestone_one_read_view_impl(read_view, read_basis)
}

pub fn certify_verified_topology_commit(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
    certify_verified_topology_commit_impl(runtime, verified)
}

pub fn certify_milestone_two_read_view(
    read_view: &RelationalReadView,
    read_basis: DerivedTopologyReadBasis,
) -> Result<WorthMilestoneTwoDerivedReadReport, WorthMilestoneOneCertificationError> {
    certify_milestone_two_read_view_impl(read_view, read_basis)
}

pub fn certify_milestone_two_verified_topology_commit(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<WorthMilestoneTwoDerivedReadReport, WorthMilestoneOneCertificationError> {
    certify_milestone_two_verified_commit_impl(runtime, verified)
}

pub fn certify_milestone_one_primitive_corpus<F>(
    runtime_factory: F,
    stem: &str,
    primitives: &[WorthMilestoneOnePrimitiveCase],
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_primitive_corpus_impl(runtime_factory, stem, primitives)
}

pub fn certify_milestone_one_default_primitive_corpus<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_default_primitive_corpus_impl(runtime_factory, stem)
}

pub fn certify_milestone_two_default_derived_corpus<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneTwoDerivedCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_two_default_derived_corpus_impl(runtime_factory, stem)
}

pub fn certify_milestone_two_closeout<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneTwoCloseoutReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_two_closeout_impl(runtime_factory, stem)
}

pub fn certify_milestone_one_closeout<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneOneCloseoutReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_closeout_impl(runtime_factory, stem)
}

pub fn certify_milestone_one_primitive_scenarios<F>(
    runtime_factory: &mut F,
    stem: &str,
    scenarios: &[WorthMilestoneOnePrimitiveScenario],
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    certify_milestone_one_primitive_scenarios_impl(runtime_factory, stem, scenarios)
}

pub fn certify_milestone_one_branch_local_primitive_scenarios<F>(
    runtime_factory: &mut F,
    stem: &str,
    branch_id: &str,
    scenarios: &[WorthMilestoneOnePrimitiveScenario],
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
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
