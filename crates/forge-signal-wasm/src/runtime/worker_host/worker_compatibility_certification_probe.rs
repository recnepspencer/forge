use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::core::RuntimeCore;

use super::{
    committed_truth_digest_for_runtime, compare_worker_async_lifecycle_truth,
    compare_worker_diagnostics_truth, compare_worker_observation_truth,
    probe_worker_branch_lifecycle_parity, publish_definition_envelope_into_worker_runtime,
    WorkerCompatibilityCertificationReport, WorkerCompatibilityCertificationScenario,
    WorkerCompatibilityTruthReport, WorkerRuntimeNonHostIsolationReport, WorkerRuntimeShell,
};

pub fn certify_worker_compatibility(
    scenario: WorkerCompatibilityCertificationScenario,
) -> Result<WorkerCompatibilityCertificationReport, ForgeSignalJsError> {
    let mut worker_shell = WorkerRuntimeShell::new(scenario.publication.policy.clone())?;
    let worker_publication_summary = worker_shell.publish_graph(scenario.publication.clone())?;
    worker_shell.observe_signal_for_runtime_certification(&scenario.observed_signal_id)?;

    let mut compatibility_runtime = RuntimeCore::new(scenario.publication.policy.clone())?;
    let compatibility_publication_summary = publish_definition_envelope_into_worker_runtime(
        &mut compatibility_runtime,
        scenario.publication.clone().into_definition_envelope(),
    )?;
    compatibility_runtime.observe_signal_for_runtime_certification(&scenario.observed_signal_id)?;

    let worker_envelope =
        worker_shell.apply_committed_transaction(scenario.transaction_ops.clone())?;
    compatibility_runtime.apply_transaction(scenario.transaction_ops.clone())?;

    let committed_truth_report = WorkerCompatibilityTruthReport::compare(
        &worker_envelope,
        committed_truth_digest_for_runtime(&compatibility_runtime)?,
    );
    let worker_async_lifecycle = worker_shell.certify_async_lifecycle(
        &scenario.async_signal_id,
        scenario.async_payload_contract_id,
        scenario.async_payload_byte_len,
    )?;
    let compatibility_async_lifecycle = compatibility_runtime.certify_runtime_async_lifecycle(
        &scenario.async_signal_id,
        scenario.async_payload_contract_id,
        scenario.async_payload_byte_len,
    )?;
    let async_lifecycle_report = compare_worker_async_lifecycle_truth(
        worker_async_lifecycle,
        compatibility_async_lifecycle,
    )?;
    let published_recipe_ids = scenario
        .publication
        .recipes
        .iter()
        .map(|recipe| recipe.id.clone())
        .collect::<Vec<_>>();
    let branch_lifecycle_report = probe_worker_branch_lifecycle_parity(
        scenario.publication,
        scenario.feature_transaction_ops,
        scenario.main_transaction_ops,
    )?;
    let observation_report =
        compare_worker_observation_truth(&worker_shell, &compatibility_runtime)?;
    let diagnostics_report =
        compare_worker_diagnostics_truth(&worker_shell, &compatibility_runtime)?;
    let isolation_report = WorkerRuntimeNonHostIsolationReport::from_certified_worker_run(
        &scenario.independent_region_recipe_ids,
        &published_recipe_ids,
        scenario.transaction_ops.len() as u64,
        &worker_envelope,
        &worker_publication_summary,
    )?;

    Ok(WorkerCompatibilityCertificationReport {
        committed_truth_report,
        async_lifecycle_report,
        branch_lifecycle_report,
        observation_report,
        diagnostics_report,
        isolation_report,
        worker_publication_summary,
        compatibility_publication_summary,
    })
}
