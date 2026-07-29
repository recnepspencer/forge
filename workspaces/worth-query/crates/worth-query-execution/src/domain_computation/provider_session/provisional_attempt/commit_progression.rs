use super::{
    WorthQueryInvariantProgressionAuthority, WorthQueryProposedPostState,
    WorthQueryProposedStateInspection,
};
use crate::domain_computation::provider_session::{
    WorthQueryDecisionReadSetFailure, WorthQueryDecisionReadSetFreshnessOutcome,
    WorthQueryProviderSessionFailure, WorthQuerySessionCommitOrAbortOutcome,
    WorthQueryStaleDecisionReadSet,
};

pub struct WorthQueryInvariantApprovedProposedState<'run> {
    proposed: WorthQueryProposedPostState<'run>,
    progression: WorthQueryInvariantProgressionAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryProviderCommitAdmissionDenial {
    ForeignInvariantProgression,
}

#[derive(Debug)]
pub enum WorthQueryProviderCompareAndCommitOutcome {
    Committed {
        plan_identity: String,
        token_identity: String,
        provider_receipt: String,
    },
    Stale(WorthQueryStaleDecisionReadSet),
    Denied(WorthQueryProviderCompareAndCommitDenial),
    Indeterminate(WorthQueryProviderSessionFailure),
}

#[derive(Debug)]
pub enum WorthQueryProviderCompareAndCommitDenial {
    DecisionReadSet(WorthQueryDecisionReadSetFailure),
    ProviderSession(WorthQueryProviderSessionFailure),
}

impl<'run> WorthQueryProposedStateInspection<'run> {
    pub fn bind_invariant_progression(
        self,
        progression: WorthQueryInvariantProgressionAuthority,
    ) -> Result<
        WorthQueryInvariantApprovedProposedState<'run>,
        (
            WorthQueryProviderCommitAdmissionDenial,
            WorthQueryProposedStateInspection<'run>,
        ),
    > {
        let attempt = &self.proposed.attempt;
        let plan = attempt.staged.plan();
        if !progression.belongs_to(
            plan.provider_identity(),
            plan.provider_generation(),
            attempt.staged.provisional_binding_identity(),
            plan.basis_identity(),
            self.proposed.identity(),
            self.proposed.generation(),
        ) {
            return Err((
                WorthQueryProviderCommitAdmissionDenial::ForeignInvariantProgression,
                self,
            ));
        }
        Ok(WorthQueryInvariantApprovedProposedState {
            proposed: self.proposed,
            progression,
        })
    }
}

impl WorthQueryInvariantApprovedProposedState<'_> {
    pub fn invariant_receipt_identities(&self) -> &[std::sync::Arc<str>] {
        self.progression.receipt_identities()
    }

    pub fn compare_and_commit(mut self) -> WorthQueryProviderCompareAndCommitOutcome {
        let fresh = self.proposed.attempt.read_set;
        let compared = self
            .proposed
            .attempt
            .staged
            .read_authority()
            .recompare_fresh_decision_read_set(fresh);
        match compared {
            Ok(WorthQueryDecisionReadSetFreshnessOutcome::Stale(stale)) => {
                let _ = self.proposed.attempt.overlay.discard();
                let _ = self.proposed.attempt.staged.abort();
                WorthQueryProviderCompareAndCommitOutcome::Stale(stale)
            }
            Err(failure) => {
                let _ = self.proposed.attempt.overlay.discard();
                let _ = self.proposed.attempt.staged.abort();
                WorthQueryProviderCompareAndCommitOutcome::Denied(
                    WorthQueryProviderCompareAndCommitDenial::DecisionReadSet(failure),
                )
            }
            Ok(WorthQueryDecisionReadSetFreshnessOutcome::Fresh(fresh)) => {
                self.proposed.attempt.read_set = fresh;
                self.commit_fresh()
            }
        }
    }

    pub fn discard(mut self) {
        let _ = self.proposed.attempt.overlay.discard();
        let _ = self.proposed.attempt.staged.abort();
    }

    fn commit_fresh(mut self) -> WorthQueryProviderCompareAndCommitOutcome {
        let prepared = match self.proposed.attempt.staged.prepare_for_commit() {
            Ok(prepared) => prepared,
            Err(failure) => {
                let _ = self.proposed.attempt.overlay.discard();
                return WorthQueryProviderCompareAndCommitOutcome::Denied(
                    WorthQueryProviderCompareAndCommitDenial::ProviderSession(failure),
                );
            }
        };
        match prepared.commit() {
            WorthQuerySessionCommitOrAbortOutcome::Committed {
                plan_identity,
                token_identity,
                provider_receipt,
                ..
            } => {
                self.proposed
                    .attempt
                    .overlay
                    .release_to_provider_resolution();
                WorthQueryProviderCompareAndCommitOutcome::Committed {
                    plan_identity,
                    token_identity,
                    provider_receipt,
                }
            }
            WorthQuerySessionCommitOrAbortOutcome::CommitRecoveryRequired(failure) => {
                self.proposed
                    .attempt
                    .overlay
                    .release_to_provider_resolution();
                WorthQueryProviderCompareAndCommitOutcome::Indeterminate(failure)
            }
            WorthQuerySessionCommitOrAbortOutcome::Aborted { .. }
            | WorthQuerySessionCommitOrAbortOutcome::AbortRecoveryRequired(_) => {
                unreachable!("commit transition cannot produce an abort outcome")
            }
        }
    }
}
