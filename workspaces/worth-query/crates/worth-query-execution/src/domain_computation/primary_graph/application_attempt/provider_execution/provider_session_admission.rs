use super::super::WorthQueryApplicationCommitDenialStage as DenialStage;
use super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};

pub(super) fn admit_provider_session<'run>(
    running: &'run mut crate::domain_computation::WorthQueryRunningDirectRun,
    graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    mutation_run: &crate::domain_computation::provider_session::WorthQueryMutationRunBinding,
) -> Result<
    crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
    WorthQueryProviderProgressionOutcome,
> {
    let staged = running
        .admit_provider_execution_plan(graph)
        .and_then(|plan| plan.readmit())
        .and_then(|session| session.prepare())
        .map(|prepared| prepared.bind_reads_and_effects())
        .map_err(|_| progression_denied(DenialStage::ProviderPlan))?;
    if mutation_run.admits(staged.plan()) {
        Ok(staged)
    } else {
        let _ = staged.abort();
        Err(progression_denied(DenialStage::ProviderPlan))
    }
}
