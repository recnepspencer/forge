use super::super::verify_pinned_eviction;
use crate::courtroom_campaign::bounded_residency_siege::protocol::BoundedResidencyPinnedEvictionObservation;

#[test]
fn pinned_eviction_oracle_rejects_every_one_field_bypass() {
    let accepted = BoundedResidencyPinnedEvictionObservation {
        forced_evictions: 9,
        pinned_frames_before: 3,
        pinned_frames_after: 3,
        pin_leases_before: 3,
        pin_leases_after: 3,
        bases_preserved: true,
    };
    assert!(verify_pinned_eviction(accepted).is_ok());
    for hostile in [
        BoundedResidencyPinnedEvictionObservation {
            forced_evictions: 0,
            ..accepted
        },
        BoundedResidencyPinnedEvictionObservation {
            pinned_frames_before: 2,
            ..accepted
        },
        BoundedResidencyPinnedEvictionObservation {
            pinned_frames_after: 2,
            ..accepted
        },
        BoundedResidencyPinnedEvictionObservation {
            pin_leases_before: 2,
            ..accepted
        },
        BoundedResidencyPinnedEvictionObservation {
            pin_leases_after: 2,
            ..accepted
        },
        BoundedResidencyPinnedEvictionObservation {
            bases_preserved: false,
            ..accepted
        },
    ] {
        assert_eq!(
            verify_pinned_eviction(hostile).unwrap_err(),
            "Courtroom C forced eviction did not preserve exact pinned-frame authority",
            "{hostile:?}"
        );
    }
}
