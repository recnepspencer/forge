use worth_store_tiering::ColdPlacementState;

use crate::handoffs::BlobHarnessPlacementClass;
use crate::{BlobChunkReachabilityProofSet, BlobPlacementAdmissionAuthority, BlobPlacementIntent};

use super::backend::admitted_backend;
use super::certification_test_authority::{external_recovery, placement_readiness};

pub(super) fn admit_placement(
    _case: &str,
    reachability: &BlobChunkReachabilityProofSet,
    placement_class: BlobHarnessPlacementClass,
) -> crate::AdmittedBlobPlacement {
    let authority = BlobPlacementAdmissionAuthority::from_admitted_backend(admitted_backend());
    let readiness = placement_readiness(reachability.security_metadata().identity());
    let intent = match placement_class {
        BlobHarnessPlacementClass::StoreLocal => BlobPlacementIntent::inline(readiness),
        BlobHarnessPlacementClass::ExternalPlacementObserved => {
            BlobPlacementIntent::external(readiness, external_recovery(reachability))
        }
        BlobHarnessPlacementClass::ColdTierObserved => {
            BlobPlacementIntent::cold(readiness, ColdPlacementState::ColdAvailable)
        }
    };
    authority.admit(reachability, intent).expect("placement")
}
