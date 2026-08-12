use crate::handoff::RecoveryCleanupDeferralEvidence;
use crate::progression::ReopenedPhysicalRecovery;

use super::accounting::RecoveryCleanupAccounting;
use super::attempt::{RecoveryCleanupAttempt, RecoveryCleanupAttemptBasis};
use super::command_basis::RecoveryCleanupCommandBasis;
use super::plan::{build_plan, RecoveryCleanupPlanBasis};
use super::{
    PhysicalRecoveryCleanupCancellation, RecoveryCleanupDeferralReason,
    RecoveryCleanupDispositionKind, RecoveryCleanupTarget,
};

pub(crate) fn execute(
    reopened: ReopenedPhysicalRecovery,
    cancellation: Option<PhysicalRecoveryCleanupCancellation>,
) -> crate::entry::PhysicalRecoveryOutcome {
    let limits = reopened.state.authority.limits.declaration();
    let mut plan = build_plan(RecoveryCleanupPlanBasis {
        selection: &reopened.state.selection,
        base: &reopened.state.base,
        publication: &reopened.expectation,
        fates: &reopened.state.fates,
        limits,
    });
    let command_basis =
        RecoveryCleanupCommandBasis::from_reopened(&reopened, plan.identity(), plan.candidates());
    if let Some(command_basis) = &command_basis {
        debug_assert_eq!(command_basis.descriptive_plan_identity(), plan.identity());
        plan.bind_authority_identity(command_basis.plan_identity());
    }
    let mut accounting = RecoveryCleanupAccounting::begin(&plan);
    let candidates = plan.take_eligibilities();
    let cancellation_at = match cancellation {
        Some(cancellation) => match cancellation.admit(plan.identity(), candidates.len() as u64) {
            Some(safe_point) => Some(safe_point),
            None => {
                defer_for_invalid_cancellation(&mut plan, &mut accounting, &candidates);
                return finish(reopened, plan, accounting);
            }
        },
        None => None,
    };
    let mut policy = None;
    for (index, candidate) in candidates.into_iter().enumerate() {
        if cancellation_at == Some(index as u64) {
            defer_for_cancellation(&mut plan, &mut accounting, candidate, index as u64);
            break;
        }
        let artifact = candidate.artifact();
        let byte_count = candidate.byte_count();
        let attempt = RecoveryCleanupAttemptBasis::new(&reopened, &plan, command_basis.as_ref())
            .execute(policy, candidate);
        match attempt {
            RecoveryCleanupAttempt::Completed {
                freshness,
                performed,
                revalidation,
            } => {
                policy.get_or_insert(freshness.policy_identity());
                accounting.record_freshness_sample(freshness);
                accounting.record_completed(byte_count, performed, revalidation);
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
    finish(reopened, plan, accounting)
}

fn defer_for_cancellation(
    plan: &mut super::RecoveryCleanupPlan,
    accounting: &mut RecoveryCleanupAccounting,
    candidate: super::RecoveryCleanupEligibility,
    settled_actions: u64,
) {
    let target = RecoveryCleanupTarget::Wal(candidate.artifact());
    let remaining_actions = plan
        .dispositions()
        .iter()
        .filter(|disposition| disposition.kind() == RecoveryCleanupDispositionKind::Eligible)
        .count() as u64;
    let remaining_bytes = plan
        .dispositions()
        .iter()
        .filter(|disposition| disposition.kind() == RecoveryCleanupDispositionKind::Eligible)
        .map(|disposition| disposition.byte_count())
        .sum();
    plan.defer_remaining(RecoveryCleanupDeferralReason::Cancelled);
    accounting.record_cancellation(
        remaining_actions,
        remaining_bytes,
        RecoveryCleanupDeferralEvidence::Cancelled {
            target,
            settled_actions,
        },
    );
}

fn defer_for_invalid_cancellation(
    plan: &mut super::RecoveryCleanupPlan,
    accounting: &mut RecoveryCleanupAccounting,
    candidates: &[super::RecoveryCleanupEligibility],
) {
    let Some(candidate) = candidates.first() else {
        return;
    };
    let bytes = candidates
        .iter()
        .map(|candidate| candidate.byte_count())
        .sum();
    plan.defer_remaining(RecoveryCleanupDeferralReason::CancellationBindingMismatch);
    accounting.record_cancellation(
        candidates.len() as u64,
        bytes,
        RecoveryCleanupDeferralEvidence::CancellationBindingMismatch {
            target: RecoveryCleanupTarget::Wal(candidate.artifact()),
        },
    );
}

fn finish(
    reopened: ReopenedPhysicalRecovery,
    plan: super::RecoveryCleanupPlan,
    accounting: RecoveryCleanupAccounting,
) -> crate::entry::PhysicalRecoveryOutcome {
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
        RecoveryCleanupDeferralEvidence::Cancelled { .. } => {
            RecoveryCleanupDeferralReason::Cancelled
        }
        RecoveryCleanupDeferralEvidence::CancellationBindingMismatch { .. } => {
            RecoveryCleanupDeferralReason::CancellationBindingMismatch
        }
        RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { .. } => {
            RecoveryCleanupDeferralReason::DeniedBeforeEffect
        }
        RecoveryCleanupDeferralEvidence::IndeterminateEffect { .. } => {
            RecoveryCleanupDeferralReason::IndeterminateEffect
        }
    }
}
