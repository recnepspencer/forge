use std::collections::HashMap;

use super::{OperationalControlHistoryViolationKind, OperationalOperationId};

#[derive(Clone)]
pub(super) struct ReplayedRecoveryPublication {
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_kind: super::RecoveryPublicationOperationKind,
    binding: super::control_record::RecoveryPublicationControlBinding,
    state: RecoveryPublicationReplayState,
    terminal_disposition: Option<(super::TerminalRecoveryPublicationDisposition, [u8; 32])>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RecoveryPublicationReplayState {
    Prepared,
    Published,
    Terminal,
    FenceReleased,
}

pub(super) fn observe_prepared(
    map: &mut HashMap<OperationalOperationId, ReplayedRecoveryPublication>,
    operation: &OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    binding: super::control_record::RecoveryPublicationControlBinding,
) -> Result<(), OperationalControlHistoryViolationKind> {
    if binding.cutover_plan_fingerprint() == [0; 32]
        || binding.publication_identity() == [0; 32]
        || binding.candidate_media_identity() == [0; 32]
        || binding.publication_plan_fingerprint() == [0; 32]
        || binding.fence_identity() == [0; 32]
        || binding.fence_plan_fingerprint() == [0; 32]
        || binding
            .admission_policy()
            .validate(binding.authority_posture())
            .is_err()
        || map.contains_key(operation)
    {
        return Err(OperationalControlHistoryViolationKind::DuplicateRecoveryPublication);
    }
    map.try_reserve(1)
        .map_err(|_| OperationalControlHistoryViolationKind::DuplicateRecoveryPublication)?;
    let operation_kind = match binding.operation_tag() {
        1 => super::RecoveryPublicationOperationKind::BackupRestore,
        2 => super::RecoveryPublicationOperationKind::PointInTimeRecovery,
        3 => super::RecoveryPublicationOperationKind::Rollback,
        4 => super::RecoveryPublicationOperationKind::AuthorityAffectingRepair,
        _ => return Err(OperationalControlHistoryViolationKind::DuplicateRecoveryPublication),
    };
    map.insert(
        operation.clone(),
        ReplayedRecoveryPublication {
            authority_identity,
            operation_kind,
            binding,
            state: RecoveryPublicationReplayState::Prepared,
            terminal_disposition: None,
        },
    );
    Ok(())
}

pub(super) fn observe_published(
    map: &mut HashMap<OperationalOperationId, ReplayedRecoveryPublication>,
    operation: &OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    binding: super::control_record::RecoveryPublicationControlBinding,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let publication = map
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::RecoveryPublicationBeforePreparation)?;
    if publication.state != RecoveryPublicationReplayState::Prepared
        || publication.authority_identity != authority_identity
        || publication.binding != binding
    {
        return Err(OperationalControlHistoryViolationKind::RecoveryPublicationBindingMismatch);
    }
    publication.state = RecoveryPublicationReplayState::Published;
    Ok(())
}

impl ReplayedRecoveryPublication {
    pub(super) fn pending_handle(
        self,
        operation: OperationalOperationId,
    ) -> Option<super::PendingRecoveryPublicationHandle> {
        if self.state == RecoveryPublicationReplayState::Published {
            Some(super::PendingRecoveryPublicationHandle::new(
                operation,
                self.authority_identity,
                self.operation_kind,
                self.binding,
            ))
        } else {
            None
        }
    }

    pub(super) fn prepared_handle(
        self,
        operation: OperationalOperationId,
    ) -> Option<super::PreparedRecoveryPublicationHandle> {
        if self.state == RecoveryPublicationReplayState::Prepared {
            Some(super::PreparedRecoveryPublicationHandle::new(
                operation,
                self.authority_identity,
                self.operation_kind,
                self.binding,
            ))
        } else {
            None
        }
    }

    pub(super) fn terminal_fence_release_handle(
        self,
        operation: OperationalOperationId,
    ) -> Option<super::TerminalRecoveryFenceReleaseHandle> {
        let (disposition, basis) = self.terminal_disposition?;
        (self.state == RecoveryPublicationReplayState::Terminal).then(|| {
            super::TerminalRecoveryFenceReleaseHandle::new(
                operation,
                self.authority_identity,
                self.binding,
                disposition,
                basis,
            )
        })
    }
}

pub(super) fn observe_fence_released(
    map: &mut HashMap<OperationalOperationId, ReplayedRecoveryPublication>,
    operation: &OperationalOperationId,
    record_authority: worth_store_authority::StoreCurrentAuthorityIdentity,
    publication_identity: [u8; 32],
    fence_identity: [u8; 32],
    fence_plan_fingerprint: [u8; 32],
    disposition_tag: u8,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let publication = map
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::FenceReleaseBeforeDisposition)?;
    let disposition = publication
        .terminal_disposition
        .ok_or(OperationalControlHistoryViolationKind::FenceReleaseBeforeDisposition)?
        .0;
    if publication.state != RecoveryPublicationReplayState::Terminal
        || publication.authority_identity != record_authority
        || publication.binding.publication_identity() != publication_identity
        || publication.binding.fence_identity() != fence_identity
        || publication.binding.fence_plan_fingerprint() != fence_plan_fingerprint
        || super::TerminalRecoveryPublicationDisposition::from_tag(disposition_tag)
            != Some(disposition)
    {
        return Err(OperationalControlHistoryViolationKind::FenceReleaseBindingMismatch);
    }
    publication.state = RecoveryPublicationReplayState::FenceReleased;
    Ok(())
}

pub(super) fn observe_disposition(
    map: &mut HashMap<OperationalOperationId, ReplayedRecoveryPublication>,
    operation: &OperationalOperationId,
    record_authority: worth_store_authority::StoreCurrentAuthorityIdentity,
    publication_identity: [u8; 32],
    disposition_tag: u8,
    disposition_basis: [u8; 32],
    observed_authority: worth_store_authority::StoreCurrentAuthorityIdentity,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let publication = map
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::RecoveryDispositionBeforePublication)?;
    if publication.binding.publication_identity() != publication_identity {
        return Err(OperationalControlHistoryViolationKind::RecoveryPublicationIdentityMismatch);
    }
    if publication.authority_identity != record_authority
        || (disposition_tag != 2 && observed_authority != publication.authority_identity)
    {
        return Err(OperationalControlHistoryViolationKind::RecoveryPublicationIdentityMismatch);
    }
    let terminal_from_published = publication.state == RecoveryPublicationReplayState::Published;
    let abandoned_before_publication =
        publication.state == RecoveryPublicationReplayState::Prepared && disposition_tag == 3;
    if (!terminal_from_published && !abandoned_before_publication)
        || !(1..=4).contains(&disposition_tag)
        || disposition_basis == [0; 32]
    {
        return Err(
            OperationalControlHistoryViolationKind::DuplicateRecoveryPublicationDisposition,
        );
    }
    publication.state = RecoveryPublicationReplayState::Terminal;
    publication.terminal_disposition = Some((
        super::TerminalRecoveryPublicationDisposition::from_tag(disposition_tag).ok_or(
            OperationalControlHistoryViolationKind::DuplicateRecoveryPublicationDisposition,
        )?,
        disposition_basis,
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control_store::control_record::RecoveryPublicationControlBinding;

    #[test]
    fn replay_retains_every_coordinate_required_for_typed_fresh_process_recovery() {
        let mut map = HashMap::new();
        let operation = OperationalOperationId::new("restore-pending").unwrap();
        observe_prepared(&mut map, &operation, authority(), binding(1, [3; 32])).unwrap();
        observe_published(&mut map, &operation, authority(), binding(1, [3; 32])).unwrap();

        let handle = map
            .remove(&operation)
            .unwrap()
            .pending_handle(operation)
            .unwrap();
        assert_eq!(
            handle.operation_kind(),
            super::super::RecoveryPublicationOperationKind::BackupRestore
        );
        assert_eq!(handle.cutover_plan_fingerprint(), [2; 32]);
        assert_eq!(handle.publication_identity(), [4; 32]);
        assert_eq!(handle.candidate_media_identity(), [5; 32]);
        assert_eq!(handle.fence_identity(), [6; 32]);
        assert_eq!(handle.fence_plan_fingerprint(), [7; 32]);
    }

    #[test]
    fn replay_rejects_unknown_operation_class_and_zero_fence_binding() {
        let operation = OperationalOperationId::new("invalid-pending").unwrap();
        let mut map = HashMap::new();
        assert!(observe_prepared(&mut map, &operation, authority(), binding(9, [3; 32])).is_err());
        assert!(observe_prepared(&mut map, &operation, authority(), binding(1, [0; 32])).is_err());
        assert!(map.is_empty());
    }

    fn authority() -> worth_store_authority::StoreCurrentAuthorityIdentity {
        worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint([1; 32])
    }

    fn binding(operation_tag: u8, publication_plan: [u8; 32]) -> RecoveryPublicationControlBinding {
        RecoveryPublicationControlBinding::from_persisted(
            operation_tag,
            [2; 32],
            publication_plan,
            [4; 32],
            [5; 32],
            [6; 32],
            [7; 32],
            authority_posture(),
            worth_store_authority::RecoveryAuthorityAdmissionPolicy::fully_trusted_only(),
        )
    }

    fn authority_posture() -> worth_store_authority::RecoveryAuthorityAdmissionPosture {
        let trusted =
            worth_store_authority::RecoveryAuthorityRegionPosture::observed([9; 32], 1).unwrap();
        let empty =
            worth_store_authority::RecoveryAuthorityRegionPosture::observed([0; 32], 0).unwrap();
        worth_store_authority::RecoveryAuthorityAdmissionPosture::from_independent_post_verification(
            [8; 32],
            [trusted, empty, empty, empty, empty],
        )
        .unwrap()
    }
}
