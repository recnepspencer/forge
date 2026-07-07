use forge_store_physical_backend::BlobBackendResidueObservationKind;

use crate::placement::admission::test_support::residue_observation;
use crate::{BlobRecoveryRecordDenialKind, BlobRecoveryRecordSet};

#[test]
fn manifest_lane_detects_orphaned_backend_residue() {
    let residue = residue_observation(
        BlobBackendResidueObservationKind::OrphanedPlacementResidue,
        "backend/object/key",
    );
    assert_eq!(
        BlobRecoveryRecordSet::reject_backend_residue(&residue).kind(),
        BlobRecoveryRecordDenialKind::BackendResidueRejected
    );
}
