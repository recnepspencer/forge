use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use worth_schema::facade::{
    DerivedTopologyReadBasis, VerifiedTopologyCommit, WorthBoundaryFailure,
};
use worth_topo::facade::{
    certify_milestone_one_read_view_traced, certify_milestone_two_read_view_traced,
    certify_milestone_two_verified_topology_commit_traced,
    certify_verified_topology_commit_traced, WorthMilestoneOneCertificationError,
    WorthTopologyReadError, WorthTopologyReader, WorthTopologyEditApplicationMode,
    WorthTopologyEditBatch, WorthTopologyEditError, WorthTopologyEditRunner,
    WorthTracedCertifiedTopologyInterpretation,
    WorthTracedDerivedEquivalenceContract, WorthTracedDerivedReadDiagnostics,
    WorthTracedMaterializedTopologyView, WorthTracedMilestoneOneCertificationReport,
    WorthTracedMilestoneTwoDerivedReadReport, WorthTracedTopologyReadArtifact,
    WorthTracedTopologyEditApplied, WorthTracedTopologyEditCommit,
};

fn _read_artifact_contract(
    reader: &WorthTopologyReader<'_>,
    basis: &DerivedTopologyReadBasis,
) -> Result<WorthTracedTopologyReadArtifact, WorthBoundaryFailure<WorthTopologyReadError>> {
    reader.read_artifact_traced(basis)
}

fn _interpret_contract(
    reader: &WorthTopologyReader<'_>,
    basis: &DerivedTopologyReadBasis,
) -> Result<
    WorthTracedCertifiedTopologyInterpretation,
    WorthBoundaryFailure<WorthTopologyReadError>,
> {
    reader.interpret_traced(basis)
}

fn _materialize_contract(
    reader: &WorthTopologyReader<'_>,
    basis: &DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMaterializedTopologyView,
    WorthBoundaryFailure<WorthTopologyReadError>,
> {
    reader.materialize_traced(basis)
}

fn _equivalence_contract(
    reader: &WorthTopologyReader<'_>,
    basis: &DerivedTopologyReadBasis,
) -> Result<
    WorthTracedDerivedEquivalenceContract,
    WorthBoundaryFailure<WorthTopologyReadError>,
> {
    reader.equivalence_contract_traced(basis)
}

fn _diagnostics_contract(
    reader: &WorthTopologyReader<'_>,
    basis: &DerivedTopologyReadBasis,
) -> Result<
    WorthTracedDerivedReadDiagnostics,
    WorthBoundaryFailure<WorthTopologyReadError>,
> {
    reader.diagnostics_traced(basis)
}

fn _m1_read_cert_contract(
    read_view: &RelationalReadView,
    basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_one_read_view_traced(read_view, basis)
}

fn _m1_commit_cert_contract(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_verified_topology_commit_traced(runtime, verified)
}

fn _m2_read_cert_contract(
    read_view: &RelationalReadView,
    basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_two_read_view_traced(read_view, basis)
}

fn _m2_commit_cert_contract(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_two_verified_topology_commit_traced(runtime, verified)
}

fn _edit_apply_contract(
    runner: &mut WorthTopologyEditRunner<'_>,
    batch: WorthTopologyEditBatch,
    mode: WorthTopologyEditApplicationMode,
) -> Result<WorthTracedTopologyEditCommit, WorthTopologyEditError> {
    runner.apply_traced(batch, mode)
}

fn _edit_apply_and_inspect_contract(
    runner: &mut WorthTopologyEditRunner<'_>,
    batch: WorthTopologyEditBatch,
    mode: WorthTopologyEditApplicationMode,
) -> Result<WorthTracedTopologyEditApplied, WorthTopologyEditError> {
    runner.apply_and_inspect_traced(batch, mode)
}

#[test]
fn worth_topo_public_traced_boundaries_compile_with_envelope_contracts() {
    let _ = _read_artifact_contract;
    let _ = _interpret_contract;
    let _ = _materialize_contract;
    let _ = _equivalence_contract;
    let _ = _diagnostics_contract;
    let _ = _m1_read_cert_contract;
    let _ = _m1_commit_cert_contract;
    let _ = _m2_read_cert_contract;
    let _ = _m2_commit_cert_contract;
    let _ = _edit_apply_contract;
    let _ = _edit_apply_and_inspect_contract;
}
