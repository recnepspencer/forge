use worth_store_tiering::ColdPlacementState;

use crate::handoffs::BlobHarnessPlacementClass;
use crate::{BlobChunkReachabilityProofSet, BlobPlacementAdmissionAuthority, BlobPlacementIntent};

use super::backend::admitted_backend;
use super::certification_test_authority::{cold_posture, external_recovery};

pub(super) fn admit_placement(
    _case: &str,
    reachability: &BlobChunkReachabilityProofSet,
    placement_class: BlobHarnessPlacementClass,
) -> crate::AdmittedBlobPlacement {
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());
    let intent = match placement_class {
        BlobHarnessPlacementClass::StoreLocal => BlobPlacementIntent::inline(),
        BlobHarnessPlacementClass::ExternalPlacementObserved => {
            BlobPlacementIntent::external(external_recovery(reachability))
        }
        BlobHarnessPlacementClass::ColdTierObserved => BlobPlacementIntent::cold(
            cold_posture(reachability.security_metadata().identity()),
            ColdPlacementState::ColdAvailable,
        ),
    };
    authority.admit(reachability, intent).expect("placement")
}
