use forge_relational::facade::runtime::RelationalRuntime;
use worth_schema::facade::{
    DerivedTopologyReadBasis, VerifiedTopologyCommit, WorthMilestoneOnePrimitiveCase,
    WorthMilestoneOnePrimitiveScenario,
};

use crate::certification::closeout::certify_milestone_one_closeout_impl;
use crate::certification::corpus::{
    certify_milestone_one_branch_local_primitive_scenarios_impl,
    certify_milestone_one_default_primitive_corpus_impl,
    certify_milestone_one_primitive_corpus_impl, certify_milestone_one_primitive_scenarios_impl,
};
use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::milestone_two::WorthTracedMilestoneTwoDerivedReadReport;
use crate::certification::milestone_two::{
    certify_milestone_two_closeout_impl, certify_milestone_two_default_derived_corpus_impl,
    certify_milestone_two_read_basis_runtime_traced_impl,
    certify_milestone_two_verified_commit_traced_impl,
};
use crate::certification::read_view::WorthMilestoneOneCertificationHarness;
use crate::certification::read_view::WorthTracedMilestoneOneCertificationReport;
use crate::certification::report::{
    WorthMilestoneOneCloseoutReport, WorthMilestoneTwoCloseoutReport,
    WorthMilestoneTwoDerivedCorpusReport, WorthPrimitiveCorpusReport,
};
use worth_schema::facade::WorthBoundaryFailure;

pub fn certify_milestone_one_read_basis_traced(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    WorthMilestoneOneCertificationHarness::certify_read_basis_with_runtime_traced(
        runtime, read_basis, None, 0,
    )
}

pub fn certify_verified_topology_commit_traced(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    WorthMilestoneOneCertificationHarness::certify_verified_commit_traced(runtime, verified)
}

pub fn certify_milestone_two_read_basis_traced(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_two_read_basis_runtime_traced_impl(runtime, read_basis)
}

pub fn certify_milestone_two_verified_topology_commit_traced(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_two_verified_commit_traced_impl(runtime, verified)
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
