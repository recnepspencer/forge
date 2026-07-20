use worth_store::physical_runtime::MediaAdmissionInspectionCause;
use worth_store_physical_backend::{MediaCounterSnapshot, MediaOperationRole};

pub(super) fn inspection_counters(cause: &MediaAdmissionInspectionCause) -> MediaCounterSnapshot {
    match cause {
        MediaAdmissionInspectionCause::PostEffectDenial(denial) => *denial.counters(),
        MediaAdmissionInspectionCause::BackendFailure(failure) => *failure.counters(),
    }
}

pub(super) fn assert_loser_counters(counters: MediaCounterSnapshot, effectful: bool) {
    assert_eq!(counters.ownership_acquisitions(), 0);
    assert_eq!(counters.ownership_contentions(), 1);
    assert_eq!(counters.replacements(), 0);
    assert_eq!(counters.deletions(), 0);
    assert_eq!(
        counters.attempts_for(MediaOperationRole::PositionedWrite),
        0
    );
    assert_eq!(counters.attempts_for(MediaOperationRole::Append), 0);
    assert_eq!(counters.attempts_for(MediaOperationRole::Truncate), 0);
    assert_eq!(counters.attempts_for(MediaOperationRole::Allocate), 0);
    let scaffold_effect = counters.completed_operations_for(MediaOperationRole::CreateDirectory)
        + counters.completed_operations_for(MediaOperationRole::CreateMutationLease)
        > 0;
    assert_eq!(scaffold_effect, effectful);
    assert!(counters.is_conserved());
    assert_eq!(counters.live_file_handles(), 0);
    assert_eq!(counters.live_directory_handles(), 0);
    assert_eq!(
        counters.attempts_for(MediaOperationRole::AcquireMutationLease),
        1
    );
    assert_eq!(
        counters.denied_before_effect_for(MediaOperationRole::AcquireMutationLease),
        1
    );
    for role in MediaOperationRole::ALL {
        if !is_loser_admission_role(role) {
            assert_eq!(counters.attempts_for(role), 0, "loser crossed {role:?}");
        }
    }
}

pub(super) fn exact_counter_projection(counters: MediaCounterSnapshot) -> String {
    let mut values = vec![
        counters.attempted_operations(),
        counters.completed_operations(),
        counters.denied_before_effect(),
        counters.partial_effects(),
        counters.indeterminate_effects(),
        counters.requested_bytes(),
        counters.completed_bytes(),
        counters.eof_observations(),
        counters.retry_attempts(),
        counters.listing_batches(),
        counters.listing_entries(),
        counters.qualification_transactions(),
        counters.ownership_attempts(),
        counters.ownership_acquisitions(),
        counters.ownership_contentions(),
        counters.ownership_releases(),
        counters.confinement_denials(),
        counters.stale_handle_denials(),
        counters.unsupported_capabilities(),
        counters.file_syncs(),
        counters.directory_syncs(),
        counters.file_opens(),
        counters.file_creates(),
        counters.file_closes(),
        counters.live_file_handles(),
        counters.peak_file_handles(),
        counters.directory_opens(),
        counters.directory_closes(),
        counters.live_directory_handles(),
        counters.peak_directory_handles(),
        counters.replacements(),
        counters.deletions(),
        counters.cleanup_actions(),
        counters.preserved_residue(),
        counters.peak_request_width_bytes(),
        counters.explicit_heap_allocation_events(),
        counters.requested_heap_capacity_bytes(),
    ];
    for role in MediaOperationRole::ALL {
        values.extend([
            counters.attempts_for(role),
            counters.completed_operations_for(role),
            counters.denied_before_effect_for(role),
            counters.partial_effects_for(role),
            counters.indeterminate_effects_for(role),
            counters.requested_bytes_for(role),
            counters.completed_bytes_for(role),
        ]);
    }
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn campaign_counter_projection(counters: MediaCounterSnapshot) -> String {
    let mut values = vec![
        counters.ownership_attempts(),
        counters.ownership_acquisitions(),
        counters.ownership_contentions(),
        counters.ownership_releases(),
        counters.confinement_denials(),
        counters.stale_handle_denials(),
        counters.unsupported_capabilities(),
        counters.qualification_transactions(),
        counters.file_syncs(),
        counters.directory_syncs(),
        counters.live_file_handles(),
        counters.live_directory_handles(),
        counters.replacements(),
        counters.deletions(),
        counters.cleanup_actions(),
        counters.preserved_residue(),
    ];
    for role in MediaOperationRole::ALL {
        if !is_scaffold_race(role) {
            values.extend([
                counters.attempts_for(role),
                counters.completed_operations_for(role),
                counters.denied_before_effect_for(role),
                counters.partial_effects_for(role),
                counters.indeterminate_effects_for(role),
                counters.requested_bytes_for(role),
                counters.completed_bytes_for(role),
            ]);
        }
    }
    encode(values)
}

fn is_scaffold_race(role: MediaOperationRole) -> bool {
    matches!(
        role,
        MediaOperationRole::OpenRootParent
            | MediaOperationRole::InspectNamespaceEntry
            | MediaOperationRole::CreateDirectory
            | MediaOperationRole::OpenDirectory
            | MediaOperationRole::ValidateRootIdentity
            | MediaOperationRole::OpenMutationLease
            | MediaOperationRole::CreateMutationLease
            | MediaOperationRole::ListDirectory
            | MediaOperationRole::OpenExisting
            | MediaOperationRole::PositionedRead
            | MediaOperationRole::ReadMetadata
    )
}

fn is_loser_admission_role(role: MediaOperationRole) -> bool {
    is_scaffold_race(role)
        || matches!(
            role,
            MediaOperationRole::ObserveRootProfile | MediaOperationRole::AcquireMutationLease
        )
}

fn encode(values: Vec<u64>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
