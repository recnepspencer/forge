//! Affinity-keyed transient state for one Primary Graph application attempt.

use crate::domain_computation::{
    provider_session::WorthQueryProviderSessionTerminalBinding, WorthQueryProposedFact,
    WorthQueryProviderSessionAffinityIdentity, WorthQueryProvisionalEffectStep,
};
use std::collections::BTreeMap;

use super::{
    mutation_work::WorthQueryPrimaryMutationWorkCounters, WorthQueryPrimaryGraphApplicationAttempt,
    WorthQueryPrimaryGraphApplicationDecisionFact,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorthQueryApplicationAttemptLookupKey(WorthQueryProviderSessionAffinityIdentity);

impl WorthQueryApplicationAttemptLookupKey {
    const fn from_binding(binding: &WorthQueryProviderSessionTerminalBinding) -> Self {
        Self(binding.affinity_identity())
    }

    const fn from_identity(identity: WorthQueryProviderSessionAffinityIdentity) -> Self {
        Self(identity)
    }
}

#[derive(Default)]
pub(super) struct WorthQueryPrimaryGraphApplicationAttemptStore {
    attempts: BTreeMap<WorthQueryApplicationAttemptLookupKey, WorthQueryApplicationAttemptState>,
    next_overlay: u64,
}

struct WorthQueryApplicationAttemptState {
    attempt: WorthQueryPrimaryGraphApplicationAttempt,
    phase: WorthQueryApplicationAttemptPhase,
}

enum WorthQueryApplicationAttemptPhase {
    Registered,
    OverlayStaged(WorthQueryPrimaryGraphOverlay),
    InvariantApproved {
        overlay: WorthQueryPrimaryGraphOverlay,
        candidate: worth_relational::facade::transactions::ValidatedRelationalMutation,
        work: WorthQueryPrimaryMutationWorkCounters,
    },
    InvariantApprovedAfterOverlayDiscard,
}

struct WorthQueryPrimaryGraphOverlay {
    identity: String,
    facts: Vec<WorthQueryProposedFact>,
}

pub(super) struct WorthQueryStagedApplicationAttempt<'attempt> {
    attempt: &'attempt WorthQueryPrimaryGraphApplicationAttempt,
    overlay: &'attempt WorthQueryPrimaryGraphOverlay,
}

pub(super) struct WorthQueryPreparedProviderApplicationAttempt {
    attempt: WorthQueryPrimaryGraphApplicationAttempt,
    candidate: worth_relational::facade::transactions::ValidatedRelationalMutation,
    work: WorthQueryPrimaryMutationWorkCounters,
}

impl WorthQueryPrimaryGraphApplicationAttemptStore {
    pub(super) fn register(
        &mut self,
        attempt: WorthQueryPrimaryGraphApplicationAttempt,
    ) -> Result<(), &'static str> {
        let key =
            WorthQueryApplicationAttemptLookupKey::from_binding(&attempt.provider_session_binding);
        self.register_key(key, attempt)
    }

    fn register_key(
        &mut self,
        key: WorthQueryApplicationAttemptLookupKey,
        attempt: WorthQueryPrimaryGraphApplicationAttempt,
    ) -> Result<(), &'static str> {
        match self.attempts.entry(key) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(WorthQueryApplicationAttemptState {
                    attempt,
                    phase: WorthQueryApplicationAttemptPhase::Registered,
                });
                Ok(())
            }
            std::collections::btree_map::Entry::Occupied(_) => {
                Err("provider session already owns an application attempt")
            }
        }
    }

    pub(super) fn contains_observed_fact(
        &self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
        locator: &str,
    ) -> bool {
        self.attempt(affinity)
            .is_some_and(|attempt| attempt.facts.contains_key(locator))
    }

    pub(super) fn observed_fact_and_branch(
        &self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
        locator: &str,
    ) -> Option<(
        WorthQueryPrimaryGraphApplicationDecisionFact,
        worth_relational::facade::history::BranchId,
    )> {
        let attempt = self.attempt(affinity)?;
        Some((attempt.facts.get(locator)?.clone(), attempt.branch.clone()))
    }

    pub(super) fn idempotency_basis(
        &self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
    ) -> Option<(
        super::super::application_attempt::WorthQueryApplicationIdempotencyBinding,
        worth_relational::facade::history::BranchId,
    )> {
        let attempt = self.attempt(affinity)?;
        Some((attempt.idempotency, attempt.branch.clone()))
    }

    pub(super) fn stage_overlay(
        &mut self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
        expected_steps: &[WorthQueryProvisionalEffectStep],
        generation: u64,
        facts: Vec<WorthQueryProposedFact>,
    ) -> Result<(String, Vec<WorthQueryProposedFact>), &'static str> {
        let key = WorthQueryApplicationAttemptLookupKey::from_identity(affinity);
        let state = self
            .attempts
            .get_mut(&key)
            .ok_or("provider session has no registered application attempt")?;
        if state.attempt.expected_steps != expected_steps || !state.phase.accepts_overlay() {
            return Err("provider session cannot stage this application overlay");
        }
        let next_overlay = self
            .next_overlay
            .checked_add(1)
            .ok_or("primary graph overlay identity space is exhausted")?;
        self.next_overlay = next_overlay;
        let identity = format!("primary-overlay:{generation}:{next_overlay}");
        state.phase.stage_overlay(WorthQueryPrimaryGraphOverlay {
            identity: identity.clone(),
            facts: facts.clone(),
        })?;
        Ok((identity, facts))
    }

    pub(super) fn staged_attempt(
        &self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
    ) -> Option<WorthQueryStagedApplicationAttempt<'_>> {
        let state = self.attempt_state(affinity)?;
        let overlay = match &state.phase {
            WorthQueryApplicationAttemptPhase::OverlayStaged(overlay)
            | WorthQueryApplicationAttemptPhase::InvariantApproved { overlay, .. } => overlay,
            WorthQueryApplicationAttemptPhase::Registered
            | WorthQueryApplicationAttemptPhase::InvariantApprovedAfterOverlayDiscard => {
                return None
            }
        };
        Some(WorthQueryStagedApplicationAttempt {
            attempt: &state.attempt,
            overlay,
        })
    }

    pub(super) fn retain_invariant_approved(
        &mut self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
        candidate: worth_relational::facade::transactions::ValidatedRelationalMutation,
        work: WorthQueryPrimaryMutationWorkCounters,
    ) -> Result<(), &'static str> {
        let key = WorthQueryApplicationAttemptLookupKey::from_identity(affinity);
        let state = self
            .attempts
            .get_mut(&key)
            .ok_or("provider session has no registered application attempt")?;
        let prior = std::mem::replace(
            &mut state.phase,
            WorthQueryApplicationAttemptPhase::Registered,
        );
        match prior {
            WorthQueryApplicationAttemptPhase::OverlayStaged(overlay) => {
                state.phase = WorthQueryApplicationAttemptPhase::InvariantApproved {
                    overlay,
                    candidate,
                    work,
                };
                Ok(())
            }
            prior => {
                state.phase = prior;
                Err("provider session cannot retain an invariant-approved candidate in this phase")
            }
        }
    }

    pub(super) fn is_staged_session_preparable(
        &self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
    ) -> bool {
        self.attempt_state(affinity).is_some_and(|state| {
            matches!(
                state.phase,
                WorthQueryApplicationAttemptPhase::OverlayStaged(_)
                    | WorthQueryApplicationAttemptPhase::InvariantApproved { .. }
            )
        })
    }

    pub(super) fn take_commit_prepared(
        &mut self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
    ) -> Option<WorthQueryPreparedProviderApplicationAttempt> {
        let key = WorthQueryApplicationAttemptLookupKey::from_identity(affinity);
        let ready = self.attempts.get(&key).is_some_and(|state| {
            matches!(
                state.phase,
                WorthQueryApplicationAttemptPhase::InvariantApproved { .. }
            )
        });
        if !ready {
            return None;
        }
        let WorthQueryApplicationAttemptState { attempt, phase } = self
            .attempts
            .remove(&key)
            .expect("commit-ready state exists");
        let WorthQueryApplicationAttemptPhase::InvariantApproved {
            candidate, work, ..
        } = phase
        else {
            unreachable!("commit readiness was checked under the same store borrow")
        };
        Some(WorthQueryPreparedProviderApplicationAttempt {
            attempt,
            candidate,
            work,
        })
    }

    pub(super) fn discard_overlay(
        &mut self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
        physical_overlay_identity: &str,
    ) -> bool {
        let Some(state) =
            self.attempts
                .get_mut(&WorthQueryApplicationAttemptLookupKey::from_identity(
                    affinity,
                ))
        else {
            return false;
        };
        state.phase.discard_overlay(physical_overlay_identity)
    }

    pub(super) fn abort(&mut self, affinity: WorthQueryProviderSessionAffinityIdentity) {
        self.attempts
            .remove(&WorthQueryApplicationAttemptLookupKey::from_identity(
                affinity,
            ));
    }

    #[cfg(test)]
    pub(super) fn resource_count(&self) -> usize {
        self.attempts
            .values()
            .map(|state| match &state.phase {
                WorthQueryApplicationAttemptPhase::Registered => 1,
                WorthQueryApplicationAttemptPhase::OverlayStaged(_) => 3,
                WorthQueryApplicationAttemptPhase::InvariantApproved { .. } => 5,
                WorthQueryApplicationAttemptPhase::InvariantApprovedAfterOverlayDiscard => 1,
            })
            .sum()
    }

    fn attempt(
        &self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
    ) -> Option<&WorthQueryPrimaryGraphApplicationAttempt> {
        self.attempt_state(affinity).map(|state| &state.attempt)
    }

    fn attempt_state(
        &self,
        affinity: WorthQueryProviderSessionAffinityIdentity,
    ) -> Option<&WorthQueryApplicationAttemptState> {
        self.attempts
            .get(&WorthQueryApplicationAttemptLookupKey::from_identity(
                affinity,
            ))
    }
}

impl WorthQueryApplicationAttemptPhase {
    const fn accepts_overlay(&self) -> bool {
        matches!(self, Self::Registered)
    }

    fn stage_overlay(
        &mut self,
        overlay: WorthQueryPrimaryGraphOverlay,
    ) -> Result<(), &'static str> {
        if !self.accepts_overlay() {
            return Err("provider session cannot stage this application overlay");
        }
        *self = Self::OverlayStaged(overlay);
        Ok(())
    }

    fn discard_overlay(&mut self, physical_overlay_identity: &str) -> bool {
        let prior = std::mem::replace(self, Self::Registered);
        match prior {
            Self::OverlayStaged(overlay) if overlay.identity == physical_overlay_identity => true,
            Self::InvariantApproved {
                overlay,
                candidate: _,
                work: _,
            } if overlay.identity == physical_overlay_identity => {
                *self = Self::InvariantApprovedAfterOverlayDiscard;
                true
            }
            prior => {
                *self = prior;
                false
            }
        }
    }
}

impl WorthQueryStagedApplicationAttempt<'_> {
    pub(super) fn overlay_identity(&self) -> &str {
        &self.overlay.identity
    }

    pub(super) fn overlay_facts(&self) -> &[WorthQueryProposedFact] {
        &self.overlay.facts
    }

    pub(super) fn expected_step_count(&self) -> usize {
        self.attempt.expected_steps.len()
    }

    pub(super) fn batch(&self) -> &worth_relational::facade::transactions::WorkerIntentBatch {
        &self.attempt.batch
    }

    pub(super) fn branch(&self) -> &worth_relational::facade::history::BranchId {
        &self.attempt.branch
    }

    pub(super) fn decision_fact_count(&self) -> usize {
        self.attempt.decision_fact_count
    }

    pub(super) fn expected_branch_head(
        &self,
    ) -> Option<worth_relational::facade::transactions::ExpectedBranchHead> {
        self.attempt
            .aftermath_causality
            .as_ref()
            .map(|causality| causality.expected_head())
    }
}

impl WorthQueryPreparedProviderApplicationAttempt {
    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryPrimaryGraphApplicationAttempt,
        worth_relational::facade::transactions::ValidatedRelationalMutation,
        WorthQueryPrimaryMutationWorkCounters,
    ) {
        (self.attempt, self.candidate, self.work)
    }
}

#[cfg(test)]
mod tests;
