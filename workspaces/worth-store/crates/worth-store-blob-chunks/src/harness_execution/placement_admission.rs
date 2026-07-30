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
    match placement_class {
        BlobHarnessPlacementClass::StoreLocal => {
            authority.admit(reachability, BlobPlacementIntent::inline())
        }
        BlobHarnessPlacementClass::ExternalPlacementObserved => {
            let recoverability = external_recovery(reachability);
            authority.admit(reachability, BlobPlacementIntent::external(&recoverability))
        }
        BlobHarnessPlacementClass::ColdTierObserved => {
            let posture = cold_posture(reachability.security_metadata().identity());
            authority.admit(
                reachability,
                BlobPlacementIntent::cold(&posture, ColdPlacementState::ColdAvailable),
            )
        }
    }
    .expect("placement")
}
