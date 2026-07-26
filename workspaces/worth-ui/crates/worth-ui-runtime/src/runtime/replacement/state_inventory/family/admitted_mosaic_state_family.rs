use crate::capability::{
    FrozenMosaicStateSlotEntry, MosaicStateOwnerIdentity, MosaicStatePersistencePolicy,
    MosaicStateReplacementRule, MosaicStateSlotKind,
};
use crate::runtime::{
    WorthUiDurableStateFamily, WorthUiDurableStateFamilyId, WorthUiDurableStateReplacementPolicy,
    WorthUiStateOwnerIdentity, WorthUiStatePersistencePosture,
};

use super::WorthUiDurableStateFamilyDefinition;

pub(in crate::runtime::replacement::state_inventory) fn admitted_mosaic_state_family(
    entry: &FrozenMosaicStateSlotEntry,
) -> WorthUiDurableStateFamily {
    let descriptor = entry.descriptor();
    WorthUiDurableStateFamily::from_admitted_definition(WorthUiDurableStateFamilyDefinition {
        id: WorthUiDurableStateFamilyId::custom(descriptor.id().as_str()),
        owner_identity: admitted_owner_identity(
            descriptor
                .owner_identity()
                .expect("frozen state slot carries admitted owner identity"),
        ),
        replacement_policy: admitted_replacement_policy(
            descriptor
                .replacement_rule()
                .expect("frozen state slot carries admitted replacement rule"),
        ),
        persistence_posture: admitted_persistence_posture(
            descriptor
                .persistence_policy()
                .expect("frozen state slot carries admitted persistence policy"),
        ),
        lane_constrained: state_kind_is_lane_constrained(descriptor.kind()),
        contract_digest: crate::declaration::stable_text_digest(
            entry.reconciliation_key().as_str(),
        ),
    })
}

fn admitted_owner_identity(owner: &MosaicStateOwnerIdentity) -> WorthUiStateOwnerIdentity {
    match owner {
        MosaicStateOwnerIdentity::MosaicRegionKind(_) | MosaicStateOwnerIdentity::Surface(_) => {
            WorthUiStateOwnerIdentity::node_identity(owner.digest_basis())
        }
        MosaicStateOwnerIdentity::RuntimeScope(_) => {
            WorthUiStateOwnerIdentity::shell_local_interaction(owner.digest_basis())
        }
        MosaicStateOwnerIdentity::MissingForDiagnostics => {
            unreachable!("frozen state slot cannot carry missing owner identity")
        }
    }
}

fn admitted_replacement_policy(
    rule: &MosaicStateReplacementRule,
) -> WorthUiDurableStateReplacementPolicy {
    match rule {
        MosaicStateReplacementRule::PreserveWhenOwnerMatches => {
            WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState
        }
        MosaicStateReplacementRule::DiscardWhenOwnerChanges => {
            WorthUiDurableStateReplacementPolicy::DropOnReplacement
        }
        MosaicStateReplacementRule::RemapWhenRuntimeSuppliesAlias => {
            WorthUiDurableStateReplacementPolicy::ReplaceOnReplacement
        }
        MosaicStateReplacementRule::MissingForDiagnostics => {
            unreachable!("frozen state slot cannot carry missing replacement rule")
        }
    }
}

fn admitted_persistence_posture(
    policy: &MosaicStatePersistencePolicy,
) -> WorthUiStatePersistencePosture {
    match policy {
        MosaicStatePersistencePolicy::EphemeralDuringRuntime => {
            WorthUiStatePersistencePosture::RuntimeOnly
        }
        MosaicStatePersistencePolicy::RestoreAcrossHotReload => {
            WorthUiStatePersistencePosture::SessionRecorded
        }
        MosaicStatePersistencePolicy::PersistAcrossRuntimeRestart => {
            WorthUiStatePersistencePosture::WorkspaceRecordedForLater
        }
        MosaicStatePersistencePolicy::MissingForDiagnostics => {
            unreachable!("frozen state slot cannot carry missing persistence policy")
        }
    }
}

fn state_kind_is_lane_constrained(kind: &MosaicStateSlotKind) -> bool {
    matches!(
        kind,
        MosaicStateSlotKind::ScrollPosition
            | MosaicStateSlotKind::FocusedRegion
            | MosaicStateSlotKind::SelectionToken
            | MosaicStateSlotKind::DraftInputState
    )
}
