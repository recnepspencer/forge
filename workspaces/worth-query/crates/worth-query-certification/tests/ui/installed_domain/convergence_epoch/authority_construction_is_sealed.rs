use worth_query_host::facade::admission::convergence_epoch::WorthQueryAdmittedConvergenceContract;
use worth_query_host::facade::admission::basis::{
    AdmittedBasisCapability, ObservationLaneWitness,
};
use worth_query_host::facade::convergence_epoch::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceAssessment,
    WorthQueryConverged, WorthQueryConvergenceDomainDecision,
    WorthQueryConvergenceDomainWorkEvidence, WorthQueryDirectConvergenceTerminal,
    WorthQueryIteratingDirectConvergenceEpoch, WorthQueryRetainedConvergenceCandidateEvidence,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryDirectExecutionResourceAttempt,
    WorthQueryExecutionBoundOperationAuthority, WorthQueryExecutionRuntime,
    WorthQueryProviderWorkReport, WorthQueryYieldedDirectRun,
};
use worth_query_host::facade::domain::{
    WorthQueryExecutionResourceEnvelope, WorthQueryInstalledGraphParticipationAuthority,
};

fn forge_assessment<'a>(
    contract: &'a WorthQueryAdmittedConvergenceContract,
    receipt: &'a WorthQueryBoundGraphExecutionReceipt,
    incumbents: &'a [WorthQueryRetainedConvergenceCandidateEvidence],
) -> WorthQueryConvergenceAssessment<'a> {
    WorthQueryConvergenceAssessment::new(contract, receipt, 1, incumbents)
}

fn forge_report(
    decision: WorthQueryConvergenceDomainDecision,
    work: WorthQueryProviderWorkReport,
) -> WorthQueryBoundConvergenceReport {
    WorthQueryBoundConvergenceReport::new(
        "forged-report",
        "forged-provider",
        "forged-graph",
        1,
        decision,
        WorthQueryConvergenceDomainWorkEvidence::new(1, 1, 1),
        work,
    )
}

fn invoke_one_step(epoch: WorthQueryIteratingDirectConvergenceEpoch) {
    let _ = epoch.resolve_one_step();
}

fn duplicate_terminal(terminal: WorthQueryDirectConvergenceTerminal<WorthQueryConverged>) {
    let _duplicate = terminal.clone();
}

fn resource_attempt_cannot_substitute(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryExecutionBoundOperationAuthority,
    contract: WorthQueryAdmittedConvergenceContract,
    attempt: WorthQueryDirectExecutionResourceAttempt,
    graph: WorthQueryInstalledGraphParticipationAuthority,
) {
    let _ = runtime.admit_direct_convergence_epoch(operation, contract, attempt, graph);
}

fn yielded_checkpoint_cannot_substitute(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryExecutionBoundOperationAuthority,
    contract: WorthQueryAdmittedConvergenceContract,
    yielded: WorthQueryYieldedDirectRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
) {
    let _ = runtime.admit_direct_convergence_epoch(operation, contract, yielded, graph);
}

fn raw_basis_cannot_substitute(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryExecutionBoundOperationAuthority,
    contract: WorthQueryAdmittedConvergenceContract,
    basis: AdmittedBasisCapability<ObservationLaneWitness>,
    graph: WorthQueryInstalledGraphParticipationAuthority,
) {
    let _ = runtime.admit_direct_convergence_epoch(operation, contract, basis, graph);
}

fn raw_envelope_cannot_substitute(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryExecutionBoundOperationAuthority,
    contract: WorthQueryAdmittedConvergenceContract,
    envelope: WorthQueryExecutionResourceEnvelope,
    graph: WorthQueryInstalledGraphParticipationAuthority,
) {
    let _ = runtime.admit_direct_convergence_epoch(operation, contract, envelope, graph);
}

fn main() {}
