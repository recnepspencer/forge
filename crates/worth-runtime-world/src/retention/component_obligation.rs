//! Opaque Runtime World retention handoffs.
//!
//! Phase 1 fixes the consumer-facing shapes only.  The later retention lane
//! owns component-owner leases, the unique-pin registry, dependency counts,
//! transfer, and reclamation.  These values therefore have no Phase 1
//! issuance path and make no claim that a live lease exists.

/// Move-only observation handoff reserved by the future retention owner.
#[derive(Debug)]
pub(crate) struct ObservationRetentionObligation {
    _sealed: (),
}

/// Move-only publication handoff reserved by the future retention owner.
#[derive(Debug)]
pub(crate) struct PublicationRetentionObligation {
    _sealed: (),
}

/// Move-only recovery handoff reserved by the future retention owner.
#[derive(Debug)]
pub(crate) struct RetainedPartialRetentionObligation {
    _sealed: (),
}
