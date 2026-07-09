use std::collections::BTreeMap;

use crate::binding::{
    IdentityBindingDescriptor, NonIdentityBindingMetadata, QueryBindingSlot, QueryBindingSubject,
};
use crate::diagnostics::{CanonicalizationCounters, CanonicalizationWarning, NormalizationEvent};

use super::errors::QueryCanonicalizationError;

pub(super) fn canonicalize_bindings(
    identity: &[IdentityBindingDescriptor],
    non_identity: &[NonIdentityBindingMetadata],
    warnings: &mut Vec<CanonicalizationWarning>,
    events: &mut Vec<NormalizationEvent>,
    _counters: &mut CanonicalizationCounters,
) -> Result<Vec<IdentityBindingDescriptor>, QueryCanonicalizationError> {
    let mut by_slot = BTreeMap::<QueryBindingSlot, QueryBindingSubject>::new();
    let mut duplicate_binding_slots = Vec::new();
    for descriptor in identity {
        match by_slot.get(descriptor.slot()) {
            Some(existing) if existing != descriptor.subject() => {
                return Err(
                    QueryCanonicalizationError::DuplicateBindingDescriptorConflict {
                        slot: descriptor.slot().as_str().to_string(),
                    },
                );
            }
            Some(_) => {
                duplicate_binding_slots.push(descriptor.slot().as_str().to_string());
            }
            None => {
                by_slot.insert(descriptor.slot().clone(), descriptor.subject().clone());
            }
        }
    }

    events.extend(
        by_slot
            .keys()
            .map(|slot| NormalizationEvent::IdentityBindingRetained {
                slot: slot.as_str().to_string(),
            }),
    );
    duplicate_binding_slots.sort();
    for slot in duplicate_binding_slots {
        events.push(NormalizationEvent::IdentityBindingCollapsedDuplicate { slot });
    }

    let mut metadata_events: Vec<_> = non_identity
        .iter()
        .map(|metadata| metadata.key().to_string())
        .collect();
    metadata_events.sort();
    for key in metadata_events {
        warnings
            .push(CanonicalizationWarning::NonIdentityBindingMetadataIgnored { key: key.clone() });
        events.push(NormalizationEvent::NonIdentityBindingIgnored { key });
    }

    Ok(by_slot
        .into_iter()
        .map(|(slot, subject)| IdentityBindingDescriptor::new(slot, subject))
        .collect())
}
