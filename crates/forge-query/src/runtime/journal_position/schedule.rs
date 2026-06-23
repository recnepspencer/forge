use std::collections::BTreeSet;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::identity::{ForgeQueryJournalPosition, ForgeQueryJournalPositionAuthority};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryJournalPositionScheduleViolationKind {
    DuplicateCommittedPosition,
    NonMonotonicCommittedPosition,
    MixedAuthorityForCommittedSchedule,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalPositionScheduleViolation {
    kind: ForgeQueryJournalPositionScheduleViolationKind,
    index: usize,
}

#[allow(dead_code)]
impl ForgeQueryJournalPositionScheduleViolation {
    fn new(kind: ForgeQueryJournalPositionScheduleViolationKind, index: usize) -> Self {
        Self { kind, index }
    }

    pub fn kind(&self) -> ForgeQueryJournalPositionScheduleViolationKind {
        self.kind
    }

    #[allow(dead_code)]
    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalPositionSchedule {
    positions: Vec<ForgeQueryJournalPosition>,
    violations: Vec<ForgeQueryJournalPositionScheduleViolation>,
    schedule_digest: ForgeQueryEvidenceIdentity,
}

#[allow(dead_code)]
impl ForgeQueryJournalPositionSchedule {
    pub fn derive<I>(positions: I) -> Self
    where
        I: IntoIterator<Item = ForgeQueryJournalPosition>,
    {
        let positions = positions.into_iter().collect::<Vec<_>>();
        let violations = derive_schedule_violations(&positions);
        let position_identities = positions
            .iter()
            .map(ForgeQueryJournalPosition::evidence_identity)
            .collect::<Vec<_>>();
        let schedule_digest =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalPositionIdentity)
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("journal_position_identity"),
                    position_identities.iter(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("journal_position_count"),
                    positions.len(),
                )
                .seal();
        Self {
            positions,
            violations,
            schedule_digest,
        }
    }

    #[allow(dead_code)]
    pub fn positions(&self) -> &[ForgeQueryJournalPosition] {
        &self.positions
    }

    #[allow(dead_code)]
    pub fn violations(&self) -> &[ForgeQueryJournalPositionScheduleViolation] {
        &self.violations
    }

    pub fn expected_position_count(&self) -> usize {
        self.positions.len()
    }

    pub fn monotonic_position_count(&self) -> usize {
        if self.has_violation(
            ForgeQueryJournalPositionScheduleViolationKind::NonMonotonicCommittedPosition,
        ) || self.has_violation(
            ForgeQueryJournalPositionScheduleViolationKind::MixedAuthorityForCommittedSchedule,
        ) {
            0
        } else {
            self.positions.len()
        }
    }

    pub fn collision_free_count(&self) -> usize {
        if self.has_violation(
            ForgeQueryJournalPositionScheduleViolationKind::DuplicateCommittedPosition,
        ) {
            0
        } else {
            self.positions.len()
        }
    }

    pub fn stable_replay_count(&self, replay: &Self) -> usize {
        self.positions
            .iter()
            .zip(replay.positions.iter())
            .filter(|(left, right)| left.evidence_identity_ref() == right.evidence_identity_ref())
            .count()
    }

    pub fn schedule_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.schedule_digest
    }

    fn has_violation(&self, kind: ForgeQueryJournalPositionScheduleViolationKind) -> bool {
        self.violations
            .iter()
            .any(|violation| violation.kind() == kind)
    }
}

#[allow(dead_code)]
fn derive_schedule_violations(
    positions: &[ForgeQueryJournalPosition],
) -> Vec<ForgeQueryJournalPositionScheduleViolation> {
    let mut violations = Vec::new();
    let mut seen_committed_ordinals = BTreeSet::new();
    let mut previous_committed_ordinal = None;
    for (index, position) in positions.iter().enumerate() {
        if position.authority() != ForgeQueryJournalPositionAuthority::Committed {
            violations.push(ForgeQueryJournalPositionScheduleViolation::new(
                ForgeQueryJournalPositionScheduleViolationKind::MixedAuthorityForCommittedSchedule,
                index,
            ));
            continue;
        }
        let ordinal = position.ordinal_for_reporting();
        if !seen_committed_ordinals.insert(ordinal) {
            violations.push(ForgeQueryJournalPositionScheduleViolation::new(
                ForgeQueryJournalPositionScheduleViolationKind::DuplicateCommittedPosition,
                index,
            ));
        }
        if previous_committed_ordinal
            .map(|previous| ordinal <= previous)
            .unwrap_or(false)
        {
            violations.push(ForgeQueryJournalPositionScheduleViolation::new(
                ForgeQueryJournalPositionScheduleViolationKind::NonMonotonicCommittedPosition,
                index,
            ));
        }
        previous_committed_ordinal = Some(ordinal);
    }
    violations
}
