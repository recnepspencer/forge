use worth_store_formal_models::runner::{CanonicalProtocolAction, CanonicalProtocolTrace};

use super::{ControlledProtocolMutant, CounterexamplePhysicalReplayDenial};

pub(super) fn require_mapped_guard(
    mutant: ControlledProtocolMutant,
    transcript: &CanonicalProtocolTrace,
) -> Result<(), CounterexamplePhysicalReplayDenial> {
    mapped_guard_is_present(mutant, transcript)
        .then_some(())
        .ok_or(CounterexamplePhysicalReplayDenial::MappedGuardMissing)
}

fn mapped_guard_is_present(
    mutant: ControlledProtocolMutant,
    transcript: &CanonicalProtocolTrace,
) -> bool {
    let actions = transcript.actions();
    match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => {
            actions.iter().any(wal_fence_requested)
                && !actions.iter().any(wal_fence_completed)
                && !actions.iter().any(physical_mutation_acknowledged)
        }
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => {
            actions.iter().any(source_quarantined)
        }
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => {
            ordered(actions, compaction_lowered(), compaction_published())
        }
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease
        | ControlledProtocolMutant::SharedReachableAuthorityReclaimed => {
            actions.iter().any(reclaim_blocked)
        }
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => {
            actions.iter().any(quarantine_denied)
        }
        ControlledProtocolMutant::ImportPublicationWithoutDurability => {
            actions.iter().any(import_crashed)
        }
        ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => {
            actions.iter().any(replication_diverged)
        }
    }
}

fn wal_fence_requested(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::DurabilityRecovery(
            worth_store_formal_models::DurabilityRecoveryAction::WalFenceRequested
        )
    )
}

fn wal_fence_completed(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::DurabilityRecovery(
            worth_store_formal_models::DurabilityRecoveryAction::WalFenceCompleted
        )
    )
}

fn physical_mutation_acknowledged(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::DurabilityRecovery(
            worth_store_formal_models::DurabilityRecoveryAction::PhysicalMutationAcknowledged
        )
    )
}

fn compaction_lowered() -> CanonicalProtocolAction {
    CanonicalProtocolAction::CompactionVisibility(
        worth_store_formal_models::CompactionVisibilityAction::LowerRewrite,
    )
}

fn compaction_published() -> CanonicalProtocolAction {
    CanonicalProtocolAction::CompactionVisibility(
        worth_store_formal_models::CompactionVisibilityAction::PublishRewrite,
    )
}

fn source_quarantined(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::RecoverySourcePrecedence(source)
            if source.kind() == worth_store_formal_models::SourcePrecedenceActionKind::SourceQuarantined
    )
}

fn reclaim_blocked(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::LeaseReclaim(
            worth_store_formal_models::LeaseReclaimAction::ReclaimDeniedByLiveLease
        ) | CanonicalProtocolAction::SharedFrontier(
            worth_store_formal_models::SharedFrontierAction::ReclaimDeferred
        )
    )
}

fn quarantine_denied(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::QuarantineReadmission(
            worth_store_formal_models::QuarantineReadmissionState::Denied
        )
    )
}

fn import_crashed(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::ImportPublication(
            worth_store_formal_models::ImportPublicationAction::CrashBeforePublication
        )
    )
}

fn replication_diverged(action: &CanonicalProtocolAction) -> bool {
    matches!(
        action,
        CanonicalProtocolAction::ReplicationAdmission(
            worth_store_formal_models::ReplicationAdmissionAction::LineageDivergenceDetected
        )
    )
}

fn ordered(
    actions: &[CanonicalProtocolAction],
    first: CanonicalProtocolAction,
    second: CanonicalProtocolAction,
) -> bool {
    ordered_positions(actions, first, second).is_some()
}

fn ordered_positions(
    actions: &[CanonicalProtocolAction],
    first: CanonicalProtocolAction,
    second: CanonicalProtocolAction,
) -> Option<(usize, usize)> {
    let first = actions.iter().position(|action| *action == first)?;
    let second = actions.iter().position(|action| *action == second)?;
    (first < second).then_some((first, second))
}

#[cfg(test)]
mod tests {
    use worth_store_formal_models::runner::ProtocolFrontierIdentity;
    use worth_store_formal_models::ProtocolFamily;

    use super::*;

    #[test]
    fn an_unmapped_guard_is_a_typed_gap() {
        let trace = CanonicalProtocolTrace::admit(
            ProtocolFamily::DurabilityRecovery,
            ProtocolFrontierIdentity::Durability,
            [CanonicalProtocolAction::DurabilityRecovery(
                worth_store_formal_models::DurabilityRecoveryAction::WalAppendProposed,
            )],
        )
        .unwrap();
        assert_eq!(
            require_mapped_guard(
                ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence,
                &trace,
            ),
            Err(CounterexamplePhysicalReplayDenial::MappedGuardMissing)
        );
    }
}
