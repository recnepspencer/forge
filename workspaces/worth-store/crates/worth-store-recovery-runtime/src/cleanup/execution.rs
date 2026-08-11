use crate::handoff::RecoveryCleanupDeferralEvidence;
use crate::progression::ReopenedPhysicalRecovery;

use super::accounting::RecoveryCleanupAccounting;
use super::attempt::{RecoveryCleanupAttempt, RecoveryCleanupAttemptBasis};
use super::command_basis::RecoveryCleanupCommandBasis;
use super::plan::{build_plan, RecoveryCleanupPlanBasis};
use super::{RecoveryCleanupDeferralReason, RecoveryCleanupDispositionKind};

pub(crate) fn execute(reopened: ReopenedPhysicalRecovery) -> crate::entry::PhysicalRecoveryOutcome {
    let limits = reopened.state.authority.limits.declaration();
    let mut plan = build_plan(RecoveryCleanupPlanBasis {
        selection: &reopened.state.selection,
        base: &reopened.state.base,
        publication: &reopened.expectation,
        fates: &reopened.state.fates,
        limits,
    });
    let command_basis = RecoveryCleanupCommandBasis::from_reopened(&reopened);
    let mut accounting = RecoveryCleanupAccounting::begin(&plan);
    let mut policy = None;
    for candidate in plan.take_eligibilities() {
        let artifact = candidate.artifact();
        let byte_count = candidate.byte_count();
        let attempt = RecoveryCleanupAttemptBasis::new(&reopened, &plan, command_basis.as_ref())
            .execute(policy, candidate);
        match attempt {
            RecoveryCleanupAttempt::Completed {
                freshness,
                performed,
            } => {
                policy.get_or_insert(freshness.policy_identity());
                accounting.record_freshness_sample(freshness);
                accounting.record_completed(byte_count, performed);
                debug_assert!(
                    plan.transition_candidate(
                        artifact,
                        RecoveryCleanupDispositionKind::SafelyRemoved,
                    )
                );
            }
            RecoveryCleanupAttempt::Deferred {
                freshness,
                evidence,
                stop,
            } => {
                if let Some(freshness) = freshness {
                    policy.get_or_insert(freshness.policy_identity());
                    accounting.record_freshness_sample(freshness);
                }
                let reason = deferral_reason(&evidence);
                debug_assert!(plan.transition_candidate(
                    artifact,
                    RecoveryCleanupDispositionKind::Deferred(reason),
                ));
                accounting.record_deferral(evidence);
                if stop {
                    plan.defer_remaining(reason);
                    break;
                }
            }
        }
    }
    let posture = accounting.finish(plan);
    crate::orchestration::finish_recovery_without_cleanup(reopened, posture)
}

const fn deferral_reason(
    evidence: &RecoveryCleanupDeferralEvidence,
) -> RecoveryCleanupDeferralReason {
    match evidence {
        RecoveryCleanupDeferralEvidence::Freshness { .. } => {
            RecoveryCleanupDeferralReason::FreshnessUnavailable
        }
        RecoveryCleanupDeferralEvidence::PublishedGenerationChanged { .. } => {
            RecoveryCleanupDeferralReason::PublishedGenerationChanged
        }
        RecoveryCleanupDeferralEvidence::EligibilityChanged { .. } => {
            RecoveryCleanupDeferralReason::EligibilityChanged
        }
        RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { .. } => {
            RecoveryCleanupDeferralReason::DeniedBeforeEffect
        }
        RecoveryCleanupDeferralEvidence::IndeterminateEffect { .. } => {
            RecoveryCleanupDeferralReason::IndeterminateEffect
        }
    }
}
