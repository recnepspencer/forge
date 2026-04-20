use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

use super::shared::manifest_from_state;

pub(crate) fn canonical_residency_manifest<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::CanonicalResidencyManifest {
    backend.counters().record_placement_state_manifest_loads(1);
    manifest_from_state(backend.state())
}

pub(crate) fn recover_tiering_state<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::CanonicalResidencyManifest {
    backend.counters().record_placement_state_recovery(1);
    manifest_from_state(backend.state())
}
