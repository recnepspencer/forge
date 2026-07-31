use std::sync::atomic::{AtomicU64, Ordering};

use super::{RelationalAuthorizationObservationIdentity, RelationalAuthorizationObservationPlan};

static NEXT_AUTHORIZATION_OBSERVATION_ORDINAL: AtomicU64 = AtomicU64::new(1);

pub(super) fn mint_observation_identity(
    plan: &RelationalAuthorizationObservationPlan,
) -> Option<RelationalAuthorizationObservationIdentity> {
    let ordinal = NEXT_AUTHORIZATION_OBSERVATION_ORDINAL
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .ok()?;
    let snapshot = plan.snapshot();
    let mut bytes = [0_u8; 32];
    bytes[0..8].copy_from_slice(&snapshot.runtime_instance_id.to_le_bytes());
    bytes[8..16].copy_from_slice(&snapshot.snapshot_id.0.to_le_bytes());
    bytes[16..24].copy_from_slice(&snapshot.version_id.as_u64().to_le_bytes());
    bytes[24..32].copy_from_slice(&ordinal.to_le_bytes());
    Some(RelationalAuthorizationObservationIdentity::mint(bytes))
}
