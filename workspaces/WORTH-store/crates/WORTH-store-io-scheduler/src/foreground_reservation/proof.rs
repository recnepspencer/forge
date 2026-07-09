use core::convert::Infallible;

use worth_proof::prelude::AuthorityWitness;
use worth_proof::prelude::ProofOutcome;

use super::capacity_admission::{
    ForegroundReservationCapacityAuthority, ForegroundReservationCapacityFreshness,
};
use super::{
    ForegroundIoLaneKind, ForegroundReservationAdmissionDenial,
    ForegroundReservationAdmissionRequest,
};

pub type ForegroundReservationProgressionOutcome<'a> = ProofOutcome<
    ForegroundReservationReady<'a>,
    ForegroundReservationAdmissionDenial,
    Infallible,
    ForegroundReservationStale,
    ForegroundReservationRebindRequired,
>;

#[derive(Debug, Eq, PartialEq)]
pub struct ForegroundReservationReady<'a> {
    lane: ForegroundIoLaneKind,
    authority_witness: &'a AuthorityWitness<ForegroundReservationCapacityAuthority>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundReservationStale {
    lane: ForegroundIoLaneKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundReservationRebindRequired {
    lane: ForegroundIoLaneKind,
}

pub(super) fn prove_foreground_reservation_progression<'request>(
    request: &'request ForegroundReservationAdmissionRequest<'_>,
) -> ForegroundReservationProgressionOutcome<'request> {
    let lane = request.lane().lane();
    match request.capacity_admission().freshness() {
        ForegroundReservationCapacityFreshness::Current => {
            worth_proof::TransitionOutcome::success(ForegroundReservationReady {
                lane,
                authority_witness: request.capacity_admission().authority_witness(),
            })
            .into()
        }
        ForegroundReservationCapacityFreshness::RebindRequired => {
            worth_proof::TransitionOutcome::rebind_required(ForegroundReservationRebindRequired {
                lane,
            })
            .into()
        }
    }
}
