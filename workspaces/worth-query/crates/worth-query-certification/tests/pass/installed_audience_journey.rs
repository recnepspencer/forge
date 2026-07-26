use worth_query_host::facade::{
    admission::resource_admission::{
        WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionDenial,
    },
    declaration::domain_computation::WorthQueryExecutionResourceRequest,
    domain::{WorthQueryInstalledDomainOperationAuthority, WorthQueryPortableDomainPackage},
    installed::{
        domain_computation::{
            WorthQueryArtifactChunkRequest, WorthQueryArtifactNativeAccessCounters,
            WorthQueryArtifactNativeAccessDenial, WorthQueryDirectYieldOutcome,
            WorthQueryDirectYieldRecoveryRequired, WorthQueryPausedDirectGraphExecution,
            WorthQueryPausedWorkflowGraphExecution, WorthQueryTransferredArtifactHandle,
            WorthQueryWorkflowYieldOutcome, WorthQueryWorkflowYieldRecoveryRequired,
        },
        provider_session::{
            WorthQueryCooperativeGraphProviderExecution, WorthQueryExecutionProviderSession,
            WorthQueryGraphParticipationProvider, WorthQueryGraphProviderCall,
            WorthQueryGraphProviderCheckpoint, WorthQueryGraphProviderExecution,
            WorthQueryGraphProviderExecutionStart, WorthQueryGraphProviderFailure,
            WorthQueryGraphProviderRestoreMemory, WorthQueryGraphProviderRetainedMemory,
            WorthQueryGraphProviderStep, WorthQueryGraphProviderStepDisposition,
            WorthQueryProviderCheckpointExport,
        },
    },
    publication::domain_computation::WorthQueryDomainEvidenceMaterial,
    runtime::{WorthQueryExecutionRuntime, WorthQueryExecutionRuntimeInstaller},
};
use worth_query_replay::facade::WorthQueryCertificationReplayCounters;

fn install_and_inspect(
    installer: WorthQueryExecutionRuntimeInstaller,
    package: WorthQueryPortableDomainPackage,
) {
    let _ = (installer, package);
}

fn inspect_runtime_and_installed_operation(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryInstalledDomainOperationAuthority,
) {
    let _ = runtime.authority_identity();
    let _ = runtime.installed_packages().validate_domain_operation(operation);
}

fn inspect_resource_admission(
    plan: &WorthQueryAdmittedExecutionResourcePlan,
    denial: &WorthQueryExecutionResourceAdmissionDenial,
    session: &WorthQueryExecutionProviderSession,
) {
    let _: &WorthQueryExecutionResourceRequest = plan.request();
    let _ = plan.request_identity();
    let _ = plan.strategy();
    let _ = plan.envelope();
    let _ = denial.kind();
    let _ = session.identity();
    let _ = session.attempt_identity();
}

fn carry_artifact_and_publication(
    artifact: WorthQueryTransferredArtifactHandle,
    request: WorthQueryArtifactChunkRequest,
    counters: WorthQueryArtifactNativeAccessCounters,
    denial: WorthQueryArtifactNativeAccessDenial,
    evidence: WorthQueryDomainEvidenceMaterial,
) {
    let _ = (artifact, request, counters, denial, evidence);
}

fn yield_from_consumed_direct_safe_point(paused: WorthQueryPausedDirectGraphExecution) {
    match paused.yield_run() {
        WorthQueryDirectYieldOutcome::Yielded(yielded) => {
            let _ = yielded.checkpoint();
            let _ = yielded.cleanup();
        }
        WorthQueryDirectYieldOutcome::Denied(denied) => {
            let _ = denied.into_paused();
        }
        WorthQueryDirectYieldOutcome::RecoveryRequired(recovery) => {
            let _ = recovery.into_paused();
        }
    }
}

fn release_terminalized_direct_yield_recovery(recovery: WorthQueryDirectYieldRecoveryRequired) {
    let _ = recovery.cleanup_terminalized();
}

fn yield_from_consumed_workflow_safe_point(paused: WorthQueryPausedWorkflowGraphExecution) {
    match paused.yield_run() {
        WorthQueryWorkflowYieldOutcome::Yielded(yielded) => {
            let _ = yielded.artifact_evidence();
            let _ = yielded.cleanup();
        }
        WorthQueryWorkflowYieldOutcome::Denied(denied) => {
            let _ = denied.into_paused();
        }
        WorthQueryWorkflowYieldOutcome::RecoveryRequired(recovery) => {
            let _ = recovery.into_paused();
        }
    }
}

fn release_terminalized_workflow_yield_recovery(recovery: WorthQueryWorkflowYieldRecoveryRequired) {
    let _ = recovery.release_terminalized();
}

fn certification_entry(counters: WorthQueryCertificationReplayCounters) {
    let _ = counters;
}

struct CompilePassGraph;
struct CompilePassProvider;
struct CompilePassExecution;

impl WorthQueryGraphProviderExecution for CompilePassExecution {
    fn advance(
        &mut self,
        _step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        unimplemented!()
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<CompilePassGraph> for CompilePassProvider {
    type Execution = CompilePassExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_host::facade::admission::resource_admission::WorthQueryExecutionResourceSupport
    {
        unimplemented!()
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        start
            .admit_cooperative_execution(|| CompilePassExecution)
            .map_err(|denial| WorthQueryGraphProviderFailure::new(denial.detail()))
    }
}

fn carry_provider_authoring_contract(
    start: &mut WorthQueryGraphProviderExecutionStart,
    retained: WorthQueryGraphProviderRetainedMemory,
    restore: &mut WorthQueryGraphProviderRestoreMemory,
    checkpoint: &dyn WorthQueryGraphProviderCheckpoint,
    export: WorthQueryProviderCheckpointExport,
) {
    let _ = (start, retained, restore, checkpoint, export);
}

fn main() {}
