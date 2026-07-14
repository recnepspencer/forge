use super::{
    UiAllocationInvalidationNarrowingCounters, UiAllocationInvalidationNarrowingDenial,
    UiAllocationInvalidationTarget,
};

pub(super) fn enforce_drag_resize_target_budget(
    ordinal: u16,
    target_count: usize,
    maximum: u16,
    counters: &UiAllocationInvalidationNarrowingCounters,
) -> Result<(), UiAllocationInvalidationNarrowingDenial> {
    let attempted = usize::from(counters.emitted_targets())
        .checked_add(target_count)
        .ok_or(UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal })?;
    let attempted = u16::try_from(attempted).map_err(|_| {
        UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal }
    })?;
    if attempted > maximum {
        return Err(
            UiAllocationInvalidationNarrowingDenial::DragResizeTargetBudgetExceeded {
                ordinal,
                attempted,
                maximum,
            },
        );
    }
    Ok(())
}

pub(super) fn map_lookup_denial(
    denial: super::authority::UiInvalidationAuthorityLookupDenial,
    _ordinal: u16,
) -> UiAllocationInvalidationNarrowingDenial {
    match denial {
        _ => unreachable!("graph and durable lookups cannot deny after active projection lookup"),
    }
}

pub(super) fn target_count(target: &UiAllocationInvalidationTarget) -> Option<usize> {
    Some(match target {
        UiAllocationInvalidationTarget::Graph(target)
        | UiAllocationInvalidationTarget::ResizePreview { target, .. }
        | UiAllocationInvalidationTarget::QueryProjection { target, .. }
        | UiAllocationInvalidationTarget::HostMeasurement { target, .. }
        | UiAllocationInvalidationTarget::DurableResize { target, .. } => {
            target.neighborhood_count()
        }
        UiAllocationInvalidationTarget::PortalAnchor { movement } => {
            movement.target().neighborhood_count()
        }
        UiAllocationInvalidationTarget::ScrollOwnedContentExtent { bindings, .. }
        | UiAllocationInvalidationTarget::ScrollOwnedExtent { bindings, .. } => {
            bindings.iter().try_fold(0usize, |total, binding| {
                total.checked_add(binding.target().neighborhood_count())
            })?
        }
    })
}
