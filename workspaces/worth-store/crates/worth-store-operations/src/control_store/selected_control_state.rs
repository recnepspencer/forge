use worth_store_authority::{
    ControlStoreFencingAuthority, ControlStoreFencingProviderDenial, SelectedControlStoreGeneration,
};
use worth_store_physical_backend::{
    ControlMediaFault, ControlRecoveryObjectHandle, PhysicalControlStoreSummary,
};

use super::{
    decode_control_record, ControlStoreAvailabilityDenial, ControlStoreSelectionIndeterminate,
    ControlStoreTrustPosture, OperationalControlHistorySummary, OperationalControlReplayBudget,
    OperationalControlReplayResource, OperationalControlStore, PersistedControlRecordDecodeDenial,
    SelectedControlReplay, SelectedControlReplayDenial, SelectedOperationalControlState,
};

pub fn inspect_control_store_copies(
    stores: &[&OperationalControlStore],
    fencing_authority: &ControlStoreFencingAuthority<'_>,
) -> ControlStoreTrustPosture {
    inspect_control_store_copies_with_budget(
        stores,
        fencing_authority,
        OperationalControlReplayBudget::default(),
    )
}

pub fn inspect_control_store_copies_with_budget(
    stores: &[&OperationalControlStore],
    fencing_authority: &ControlStoreFencingAuthority<'_>,
    budget: OperationalControlReplayBudget,
) -> ControlStoreTrustPosture {
    if stores.is_empty() {
        return ControlStoreTrustPosture::Empty;
    }
    let selected = match fencing_authority.select_generation() {
        Ok(selected) => selected,
        Err(ControlStoreFencingProviderDenial::Unsupported) => {
            return ControlStoreTrustPosture::Unavailable(
                ControlStoreAvailabilityDenial::FencingUnsupported,
            )
        }
        Err(ControlStoreFencingProviderDenial::Unavailable) => {
            return ControlStoreTrustPosture::Unavailable(
                ControlStoreAvailabilityDenial::FencingUnavailable,
            )
        }
    };
    let mut selected_stores = Vec::new();
    if selected_stores.try_reserve(stores.len()).is_err() {
        return ControlStoreTrustPosture::Unavailable(ControlStoreAvailabilityDenial::Media(
            ControlMediaFault::AllocationFailed,
        ));
    }
    selected_stores.extend(stores.iter().copied().filter(|store| {
        store.media_identity().fingerprint() == selected.media_identity_fingerprint()
    }));
    if selected_stores.is_empty() {
        return ControlStoreTrustPosture::Indeterminate(
            ControlStoreSelectionIndeterminate::SelectedMediaUnavailable {
                media_identity_fingerprint: selected.media_identity_fingerprint(),
            },
        );
    }
    let selected_store = selected_stores[0];
    for store in &selected_stores {
        let summary = match store.physical().inspect_summary() {
            Ok(summary) => summary,
            Err(error) => {
                return ControlStoreTrustPosture::Unavailable(
                    ControlStoreAvailabilityDenial::Media(error),
                )
            }
        };
        if let Some(damage) = summary.damage() {
            return ControlStoreTrustPosture::Damaged(clone_fault(damage));
        }
        if summary.last_generation() != Some(selected.generation()) {
            return ControlStoreTrustPosture::Indeterminate(
                ControlStoreSelectionIndeterminate::SelectedGenerationNotReadable {
                    selected: selected.generation(),
                    observed: summary.last_generation(),
                },
            );
        }
        if summary.prefix_digest() != selected.prefix_digest() {
            return ControlStoreTrustPosture::Indeterminate(
                ControlStoreSelectionIndeterminate::SelectedPrefixDigestMismatch {
                    selected: selected.prefix_digest(),
                    observed: summary.prefix_digest(),
                },
            );
        }
    }
    decode_selected_state(stores, selected_store, selected, budget)
}

fn decode_selected_state(
    stores: &[&OperationalControlStore],
    selected_store: &OperationalControlStore,
    selected: SelectedControlStoreGeneration,
    budget: OperationalControlReplayBudget,
) -> ControlStoreTrustPosture {
    let mut replay = Some(SelectedControlReplay::new(budget));
    let mut durable_records = Vec::new();
    let mut denial = None;
    let mut record_index = 0u64;
    let visit = selected_store.physical().visit_records(|raw| {
        if denial.is_some() {
            return;
        }
        let decoded = decode_control_record(raw.payload()).and_then(|record| {
            record.into_domain(|handle| {
                if handle.bytes() > budget.max_single_recovery_object_bytes() {
                    return Err(PersistedControlRecordDecodeDenial::ReplayBudgetExceeded {
                        required: handle.bytes(),
                        limit: budget.max_single_recovery_object_bytes(),
                    });
                }
                read_recovery_object_from_selected_copies(stores, selected, handle)
                    .map_err(PersistedControlRecordDecodeDenial::Media)
            })
        });
        match decoded {
            Ok(record) if record.authority_identity() == selected.authority_identity() => {
                let Some(current) = replay.as_mut() else {
                    denial = Some(ControlStoreTrustPosture::Unavailable(
                        ControlStoreAvailabilityDenial::Media(
                            ControlMediaFault::DerivedTransitionIndexCorrupt,
                        ),
                    ));
                    return;
                };
                if durable_records.try_reserve(1).is_err() {
                    denial = Some(ControlStoreTrustPosture::Unavailable(
                        ControlStoreAvailabilityDenial::Media(ControlMediaFault::AllocationFailed),
                    ));
                    return;
                }
                durable_records.push(record.clone());
                if let Err(error) = current.observe(record_index, record) {
                    denial = Some(map_replay_denial(error));
                    return;
                }
            }
            Ok(record) => {
                denial = Some(ControlStoreTrustPosture::Indeterminate(
                    ControlStoreSelectionIndeterminate::SelectedAuthorityMismatch {
                        selected: selected.authority_identity().fingerprint(),
                        observed: record.authority_identity().fingerprint(),
                    },
                ));
            }
            Err(PersistedControlRecordDecodeDenial::Media(fault)) => {
                denial = Some(ControlStoreTrustPosture::Damaged(fault));
            }
            Err(PersistedControlRecordDecodeDenial::AllocationFailed) => {
                denial = Some(ControlStoreTrustPosture::Unavailable(
                    ControlStoreAvailabilityDenial::Media(ControlMediaFault::AllocationFailed),
                ));
            }
            Err(PersistedControlRecordDecodeDenial::ReplayBudgetExceeded { required, limit }) => {
                denial = Some(ControlStoreTrustPosture::Unavailable(
                    ControlStoreAvailabilityDenial::ReplayBudgetExceeded {
                        resource: OperationalControlReplayResource::SingleRecoveryObjectBytes,
                        required,
                        limit,
                    },
                ));
            }
            Err(PersistedControlRecordDecodeDenial::InvalidEncoding) => {
                denial = Some(ControlStoreTrustPosture::Damaged(
                    ControlMediaFault::CorruptRecord {
                        offset: 0,
                        generation: Some(raw.generation()),
                    },
                ));
            }
        }
        record_index = match record_index.checked_add(1) {
            Some(next) => next,
            None => {
                denial = Some(ControlStoreTrustPosture::Unavailable(
                    ControlStoreAvailabilityDenial::Media(ControlMediaFault::GenerationExhausted),
                ));
                record_index
            }
        };
    });
    let summary = match visit {
        Ok(summary) => summary,
        Err(fault) => {
            return ControlStoreTrustPosture::Unavailable(ControlStoreAvailabilityDenial::Media(
                fault,
            ))
        }
    };
    if let Some(denial) = denial {
        return denial;
    }
    if let Some(posture) = reject_changed_selected_prefix(&summary, selected) {
        return posture;
    }
    let Some(replay) = replay.take() else {
        return ControlStoreTrustPosture::Unavailable(ControlStoreAvailabilityDenial::Media(
            ControlMediaFault::DerivedTransitionIndexCorrupt,
        ));
    };
    let replayed = match replay.finish() {
        Ok(replayed) => replayed,
        Err(denial) => return map_replay_denial(denial),
    };
    let history_summary = OperationalControlHistorySummary::new(
        summary.record_count(),
        replayed.completed_backups,
        replayed.abandoned_backups,
    );
    ControlStoreTrustPosture::Selected(SelectedOperationalControlState::new(
        selected,
        summary.identity(),
        history_summary,
        durable_records,
        replayed,
    ))
}

fn map_replay_denial(denial: SelectedControlReplayDenial) -> ControlStoreTrustPosture {
    match denial {
        SelectedControlReplayDenial::AllocationFailed => ControlStoreTrustPosture::Unavailable(
            ControlStoreAvailabilityDenial::Media(ControlMediaFault::AllocationFailed),
        ),
        SelectedControlReplayDenial::CounterOverflow => ControlStoreTrustPosture::Unavailable(
            ControlStoreAvailabilityDenial::Media(ControlMediaFault::GenerationExhausted),
        ),
        SelectedControlReplayDenial::DerivedIndex(fault) => {
            ControlStoreTrustPosture::Unavailable(ControlStoreAvailabilityDenial::Media(fault))
        }
        SelectedControlReplayDenial::BudgetExceeded {
            resource,
            required,
            limit,
        } => ControlStoreTrustPosture::Unavailable(
            ControlStoreAvailabilityDenial::ReplayBudgetExceeded {
                resource,
                required,
                limit,
            },
        ),
        SelectedControlReplayDenial::Invalid(violation) => ControlStoreTrustPosture::Indeterminate(
            ControlStoreSelectionIndeterminate::InvalidHistory(violation),
        ),
    }
}

fn reject_changed_selected_prefix(
    summary: &PhysicalControlStoreSummary,
    selected: SelectedControlStoreGeneration,
) -> Option<ControlStoreTrustPosture> {
    if summary.last_generation() != Some(selected.generation()) {
        return Some(ControlStoreTrustPosture::Indeterminate(
            ControlStoreSelectionIndeterminate::SelectedGenerationNotReadable {
                selected: selected.generation(),
                observed: summary.last_generation(),
            },
        ));
    }
    if summary.prefix_digest() != selected.prefix_digest() {
        return Some(ControlStoreTrustPosture::Indeterminate(
            ControlStoreSelectionIndeterminate::SelectedPrefixDigestMismatch {
                selected: selected.prefix_digest(),
                observed: summary.prefix_digest(),
            },
        ));
    }
    None
}

fn read_recovery_object_from_selected_copies(
    stores: &[&OperationalControlStore],
    selected: SelectedControlStoreGeneration,
    handle: ControlRecoveryObjectHandle,
) -> Result<Vec<u8>, ControlMediaFault> {
    let mut content = None;
    for store in stores.iter().copied().filter(|store| {
        store.media_identity().fingerprint() == selected.media_identity_fingerprint()
    }) {
        let observed = store.physical().read_recovery_object(handle)?;
        if let Some(expected) = &content {
            if expected != &observed {
                return Err(ControlMediaFault::CorruptRecoveryObject {
                    digest: handle.digest(),
                });
            }
        } else {
            content = Some(observed);
        }
    }
    content.ok_or(ControlMediaFault::MissingRecoveryObject {
        digest: handle.digest(),
    })
}

fn clone_fault(fault: &ControlMediaFault) -> ControlMediaFault {
    match fault {
        ControlMediaFault::Io(error) => {
            ControlMediaFault::Io(std::io::Error::new(error.kind(), error.to_string()))
        }
        ControlMediaFault::TornTail { offset } => ControlMediaFault::TornTail { offset: *offset },
        ControlMediaFault::CorruptRecord { offset, generation } => {
            ControlMediaFault::CorruptRecord {
                offset: *offset,
                generation: *generation,
            }
        }
        ControlMediaFault::GenerationMismatch { expected, actual } => {
            ControlMediaFault::GenerationMismatch {
                expected: *expected,
                actual: *actual,
            }
        }
        ControlMediaFault::DuplicateTransitionConflict => {
            ControlMediaFault::DuplicateTransitionConflict
        }
        ControlMediaFault::DerivedTransitionIndexCorrupt => {
            ControlMediaFault::DerivedTransitionIndexCorrupt
        }
        ControlMediaFault::RecordTooLarge {
            transition_bytes,
            payload_bytes,
        } => ControlMediaFault::RecordTooLarge {
            transition_bytes: *transition_bytes,
            payload_bytes: *payload_bytes,
        },
        ControlMediaFault::MissingRecoveryObject { digest } => {
            ControlMediaFault::MissingRecoveryObject { digest: *digest }
        }
        ControlMediaFault::RecoveryObjectLengthMismatch {
            digest,
            expected,
            actual,
        } => ControlMediaFault::RecoveryObjectLengthMismatch {
            digest: *digest,
            expected: *expected,
            actual: *actual,
        },
        ControlMediaFault::CorruptRecoveryObject { digest } => {
            ControlMediaFault::CorruptRecoveryObject { digest: *digest }
        }
        ControlMediaFault::EmptyRecoveryObject => ControlMediaFault::EmptyRecoveryObject,
        ControlMediaFault::MissingControlMediaIdentity => {
            ControlMediaFault::MissingControlMediaIdentity
        }
        ControlMediaFault::CorruptControlMediaIdentity => {
            ControlMediaFault::CorruptControlMediaIdentity
        }
        ControlMediaFault::ControlMediaIdentityUnavailable => {
            ControlMediaFault::ControlMediaIdentityUnavailable
        }
        ControlMediaFault::ControlMediaIdentityChanged { expected, observed } => {
            ControlMediaFault::ControlMediaIdentityChanged {
                expected: *expected,
                observed: *observed,
            }
        }
        ControlMediaFault::ControlHistoryChanged => ControlMediaFault::ControlHistoryChanged,
        ControlMediaFault::ControlHistoryRewound {
            expected_bytes,
            observed_bytes,
        } => ControlMediaFault::ControlHistoryRewound {
            expected_bytes: *expected_bytes,
            observed_bytes: *observed_bytes,
        },
        ControlMediaFault::IdentityEntropyUnavailable => {
            ControlMediaFault::IdentityEntropyUnavailable
        }
        ControlMediaFault::AllocationFailed => ControlMediaFault::AllocationFailed,
        ControlMediaFault::GenerationExhausted => ControlMediaFault::GenerationExhausted,
    }
}
