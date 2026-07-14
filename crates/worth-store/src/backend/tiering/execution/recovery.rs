use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};
use crate::failure::StoreError;

use super::{recall_recovery::recover_recall_state, shared::manifest_from_state};

pub(crate) fn canonical_residency_manifest<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::CanonicalResidencyManifest {
    backend.counters().record_placement_state_manifest_loads(1);
    manifest_from_state(backend.state())
}

pub(crate) fn recover_tiering_state<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<crate::CanonicalResidencyManifest, StoreError> {
    recover_recall_state(backend)?;
    backend.counters().record_placement_state_recovery(1);
    Ok(manifest_from_state(backend.state()))
}
