use super::InstalledIdentityEvolutionOutcome;
use crate::identity_evolution::{
    IdentityEvolutionOutcomeFamily as Family, IdentityEvolutionResultBundle,
};

impl InstalledIdentityEvolutionOutcome {
    /// Compares replay-visible lineage meaning without equating operational
    /// receipt, basis-binding, or run identities.
    pub(crate) fn semantic_replay_eq(&self, candidate: &Self) -> bool {
        self.artifact.family() == candidate.artifact.family()
            && self.artifact.prediction_drift_outcome()
                == candidate.artifact.prediction_drift_outcome()
            && self.artifact.counters() == candidate.artifact.counters()
            && result_semantics_eq(
                self.artifact.result_bundle(),
                candidate.artifact.result_bundle(),
            )
            && continuity_semantics_eq(self.continuity.as_ref(), candidate.continuity.as_ref())
            && self.lifecycle_target == candidate.lifecycle_target
            && self.establishing_entity_targets == candidate.establishing_entity_targets
    }
}

fn result_semantics_eq(
    subject: &IdentityEvolutionResultBundle,
    candidate: &IdentityEvolutionResultBundle,
) -> bool {
    result_metadata_eq(subject, candidate) && result_outcome_eq(subject, candidate)
}

fn result_metadata_eq(
    subject: &IdentityEvolutionResultBundle,
    candidate: &IdentityEvolutionResultBundle,
) -> bool {
    let left_metadata = subject.metadata();
    let right_metadata = candidate.metadata();
    left_metadata.outcome_family() == right_metadata.outcome_family()
        && left_metadata.branch_locality_class() == right_metadata.branch_locality_class()
        && left_metadata.promotion_or_merge_authority_state()
            == right_metadata.promotion_or_merge_authority_state()
        && left_metadata.complexity_report() == right_metadata.complexity_report()
}

fn result_outcome_eq(
    subject: &IdentityEvolutionResultBundle,
    candidate: &IdentityEvolutionResultBundle,
) -> bool {
    if subject.outcome_family() != candidate.outcome_family() {
        return false;
    }
    continuity_outcome_eq(subject, candidate)
        .or_else(|| terminal_outcome_eq(subject, candidate))
        .unwrap_or(false)
}

fn continuity_outcome_eq(
    subject: &IdentityEvolutionResultBundle,
    candidate: &IdentityEvolutionResultBundle,
) -> Option<bool> {
    Some(match subject.outcome_family() {
        Family::SingularIdentityContinuity => {
            subject
                .as_singular_identity_continuity()
                .map(|value| value.authoritative_identity())
                == candidate
                    .as_singular_identity_continuity()
                    .map(|value| value.authoritative_identity())
        }
        Family::PluralIdentitySuccessorSet => {
            subject
                .as_plural_identity_successor_set()
                .map(|value| value.successor_identities())
                == candidate
                    .as_plural_identity_successor_set()
                    .map(|value| value.successor_identities())
        }
        Family::AdvisoryIdentityCandidateSet => {
            subject
                .as_advisory_identity_candidate_set()
                .map(|value| value.advisory_candidate_identities())
                == candidate
                    .as_advisory_identity_candidate_set()
                    .map(|value| value.advisory_candidate_identities())
        }
        Family::GeneratedIdentity => {
            subject
                .as_generated_identity()
                .map(|value| value.authoritative_identity())
                == candidate
                    .as_generated_identity()
                    .map(|value| value.authoritative_identity())
        }
        _ => return None,
    })
}

fn terminal_outcome_eq(
    subject: &IdentityEvolutionResultBundle,
    candidate: &IdentityEvolutionResultBundle,
) -> Option<bool> {
    Some(match subject.outcome_family() {
        Family::Ambiguity => {
            subject.as_ambiguity().map(|value| value.ambiguity_reason())
                == candidate
                    .as_ambiguity()
                    .map(|value| value.ambiguity_reason())
        }
        Family::IdentityBreak => {
            subject
                .as_identity_break()
                .map(|value| value.identity_break_reason())
                == candidate
                    .as_identity_break()
                    .map(|value| value.identity_break_reason())
        }
        Family::RetiredIdentity => {
            subject
                .as_retired_identity()
                .map(|value| value.authoritative_identity())
                == candidate
                    .as_retired_identity()
                    .map(|value| value.authoritative_identity())
        }
        Family::Denied => {
            subject.as_denied().map(|value| value.denial_reason())
                == candidate.as_denied().map(|value| value.denial_reason())
        }
        _ => return None,
    })
}

fn continuity_semantics_eq(
    subject: Option<&crate::runtime::WorthQueryContinuityMutationEvidence>,
    candidate: Option<&crate::runtime::WorthQueryContinuityMutationEvidence>,
) -> bool {
    match (subject, candidate) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            left.family() == right.family()
                && left.outcome_class() == right.outcome_class()
                && left
                    .prior_authoritative_identity()
                    .is_same_authority_as(right.prior_authoritative_identity())
                && authority_sequences_match(
                    left.successor_authoritative_identities(),
                    right.successor_authoritative_identities(),
                )
                && optional_current_entities_match(
                    left.resolved_target_entity_identity(),
                    right.resolved_target_entity_identity(),
                )
                && left.target_collection() == right.target_collection()
        }
        _ => false,
    }
}

fn authority_sequences_match(
    subject: &[crate::runtime::WorthQueryMutationAuthorityIdentity],
    candidate: &[crate::runtime::WorthQueryMutationAuthorityIdentity],
) -> bool {
    subject.len() == candidate.len()
        && subject
            .iter()
            .zip(candidate)
            .all(|(subject, candidate)| subject.is_same_authority_as(candidate))
}

fn optional_current_entities_match(
    subject: Option<&crate::memory_workspace::WorthQueryEntityIdentity>,
    candidate: Option<&crate::memory_workspace::WorthQueryEntityIdentity>,
) -> bool {
    match (subject, candidate) {
        (Some(subject), Some(candidate)) => subject.is_same_current_identity_as(candidate),
        (None, None) => true,
        _ => false,
    }
}
