use super::super::super::WorthQueryApplicationCommitDenialStage as DenialStage;
use super::super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};

pub(super) fn admit_provider_session<'run>(
    running: &'run mut crate::domain_computation::WorthQueryRunningDirectRun,
    graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    mutation_run: crate::domain_computation::provider_session::WorthQueryMutationRunBinding,
) -> Result<
    (
        crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
        crate::domain_computation::provider_session::WorthQueryProviderSessionBoundMutationRun,
    ),
    (
        WorthQueryProviderProgressionOutcome,
        crate::domain_computation::provider_session::WorthQueryMutationRunBinding,
    ),
> {
    let staged = match running
        .admit_provider_execution_plan(graph)
        .and_then(|plan| plan.readmit())
        .and_then(|session| session.prepare())
        .map(|prepared| prepared.bind_reads_and_effects())
    {
        Ok(staged) => staged,
        Err(_) => return Err((progression_denied(DenialStage::ProviderPlan), mutation_run)),
    };
    let terminal_binding = staged.provider_session_terminal_binding();
    match mutation_run.bind_provider_session(terminal_binding) {
        Ok(bound) => Ok((staged, bound)),
        Err(mutation_run) => {
            let _ = staged.abort();
            Err((progression_denied(DenialStage::ProviderPlan), mutation_run))
        }
    }
}
