use crate::reachability::types::BlobChunkReachabilityRegistry;
use crate::reachability::verification::authority_match::verify_hold_authority;
use crate::{BlobReachabilityDenial, BlobReachabilityProtectedHold};

pub(crate) fn transition_admit_hold(
    registry: &mut BlobChunkReachabilityRegistry,
    hold: BlobReachabilityProtectedHold,
) -> Result<(), BlobReachabilityDenial> {
    if let Some(seed_authority) = verify_hold_authority(registry, &hold)? {
        registry.set_authority(seed_authority);
    }
    if registry
        .holds()
        .iter()
        .any(|existing| existing.identity() == hold.identity())
    {
        return Ok(());
    }
    registry.set_stored_counters(registry.stored_counters().with_hold());
    registry.holds_mut().push(hold);
    registry
        .holds_mut()
        .sort_by(|left, right| left.identity().as_str().cmp(right.identity().as_str()));
    Ok(())
}
