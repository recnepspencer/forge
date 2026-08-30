use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_integrity::PhysicalIntegrityScrubWindow;

#[derive(Debug, Clone, Copy)]
pub(super) enum ScheduledIntegrityScrubWindow<'media> {
    Inspect(PhysicalIntegrityScrubWindow<'media>),
    DeferredAllocation(PhysicalIntegrityScrubWindow<'media>),
    RejectedStoreScope { ordinal: u64 },
}

pub(super) fn schedule_window<'media>(
    window: PhysicalIntegrityScrubWindow<'media>,
    store: StableStoreIdentity,
    allocation_bytes: u64,
) -> ScheduledIntegrityScrubWindow<'media> {
    if window.scope().store_identity() != store {
        return ScheduledIntegrityScrubWindow::RejectedStoreScope {
            ordinal: window.ordinal(),
        };
    }
    if window.artifact().byte_count() <= allocation_bytes {
        ScheduledIntegrityScrubWindow::Inspect(window)
    } else {
        ScheduledIntegrityScrubWindow::DeferredAllocation(window)
    }
}

#[cfg(test)]
mod tests {
    use worth_store_physical_format::store_namespace::{
        ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
    };
    use worth_store_physical_integrity::{
        PhysicalArtifactScope, PhysicalByteRange, UntrustedPhysicalArtifact,
    };

    use super::*;

    fn store(byte: u8) -> StableStoreIdentity {
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([byte; 16]).unwrap(),
        )
        .published_identity()
    }

    #[test]
    fn one_window_is_admitted_against_one_bounded_allocation() {
        let store = store(31);
        let bytes = [1_u8; 4];
        let window = PhysicalIntegrityScrubWindow::new(
            1,
            PhysicalArtifactScope::current_root_selector(
                store,
                PhysicalByteRange::new(0, 4).unwrap(),
            ),
            UntrustedPhysicalArtifact::from_bounded_bytes(&bytes),
        );

        assert!(matches!(
            schedule_window(window, store, 4),
            ScheduledIntegrityScrubWindow::Inspect(_)
        ));
        let ScheduledIntegrityScrubWindow::DeferredAllocation(deferred) =
            schedule_window(window, store, 3)
        else {
            panic!("an undersized allocation must retain the deferred window");
        };
        assert_eq!(deferred.ordinal(), 1);
        assert_eq!(deferred.artifact().byte_count(), 4);
        assert!(matches!(
            schedule_window(window, self::store(32), 4),
            ScheduledIntegrityScrubWindow::RejectedStoreScope { ordinal: 1 }
        ));
    }
}
