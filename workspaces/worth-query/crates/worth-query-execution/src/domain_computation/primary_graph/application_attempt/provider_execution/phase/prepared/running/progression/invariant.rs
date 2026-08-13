use worth_query_installation::facade::APPLICATION_INVARIANT_SLOT;

use crate::domain_computation::primary_graph::application_attempt::provider_execution::outcome::{
    progression_denied, WorthQueryProviderProgressionOutcome,
};
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitDenialStage as DenialStage;
use crate::domain_computation::WorthQueryInvariantStateLocator;

pub(super) fn progress_invariant_candidate<'run>(
    staged: crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
    fresh: crate::domain_computation::WorthQueryFreshDecisionReadSet,
    steps: Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
) -> Result<
    crate::domain_computation::WorthQueryInvariantApprovedProposedState<'run>,
    WorthQueryProviderProgressionOutcome,
> {
    let lowered = match staged
        .effect_authority()
        .lower_provisional_program(&fresh, steps)
    {
        Ok(lowered) => lowered,
        Err(_) => {
            let _ = staged.abort();
            return Err(progression_denied(DenialStage::EffectLowering));
        }
    };
    let inspection = staged
        .begin_provisional_attempt(fresh, lowered)
        .map_err(|_| progression_denied(DenialStage::ProvisionalState))?
        .materialize_proposed_state()
        .inspect();
    let locators = inspection
        .facts()
        .iter()
        .map(|fact| {
            WorthQueryInvariantStateLocator::new("application-proposed-state", fact.identity())
        })
        .collect::<Result<Vec<_>, _>>();
    let receipt = match locators.and_then(|locators| {
        inspection
            .select_installed_invariant(APPLICATION_INVARIANT_SLOT)?
            .admit_state_load_plan(locators)?
            .execute()
    }) {
        Ok(receipt) => receipt,
        Err(_) => {
            inspection.discard();
            return Err(progression_denied(DenialStage::InvariantExecution));
        }
    };
    let progression = match inspection.admit_invariant_progression([receipt]) {
        Ok(progression) => progression,
        Err(_) => {
            inspection.discard();
            return Err(progression_denied(DenialStage::InvariantExecution));
        }
    };
    inspection
        .bind_invariant_progression(progression)
        .map_err(|(_, inspection)| {
            inspection.discard();
            progression_denied(DenialStage::InvariantExecution)
        })
}
