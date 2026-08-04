use worth_store::physical_runtime::PhysicalMutationAcknowledgment;
use worth_store_formal_models::DurabilityRecoveryAction;

pub(super) fn map_physical_mutation_acknowledgment(
    acknowledgment: &PhysicalMutationAcknowledgment,
) -> [DurabilityRecoveryAction; 11] {
    assert!(acknowledgment.completed_breadth().record_count() > 0);
    assert!(acknowledgment.completed_breadth().data_effect_count() > 0);
    assert!(acknowledgment.completed_breadth().current_root_generation() > 0);
    [
        DurabilityRecoveryAction::WalAppendProposed,
        DurabilityRecoveryAction::WalAppendCompletedInMemory,
        DurabilityRecoveryAction::WalFenceRequested,
        DurabilityRecoveryAction::WalFenceCompleted,
        DurabilityRecoveryAction::PageFlushRequested,
        DurabilityRecoveryAction::PageFlushCompleted,
        DurabilityRecoveryAction::CheckpointBegun,
        DurabilityRecoveryAction::CheckpointDurable,
        DurabilityRecoveryAction::DirectorySyncCompleted,
        DurabilityRecoveryAction::CheckpointPublished,
        DurabilityRecoveryAction::PhysicalMutationAcknowledged,
    ]
}
