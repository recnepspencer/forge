use crate::IoSchedulerBackendCapabilityRequirement;

use super::proof::prove_foreground_reservation_progression;
use super::resource_contract::{require_declared_resource_budget, require_lane_resource_contract};
use super::{
    ForegroundLatencyEnvelopeKind, ForegroundReservationAdmissionDenial,
    ForegroundReservationAdmissionOutcome, ForegroundReservationAdmissionRequest,
    ForegroundReservationCounterSnapshot, ForegroundReservationDenied, ForegroundReservationHeld,
    ForegroundReservationReceipt, ForegroundReservationStaleRebindRequired,
    ForegroundResourceBudget,
};

pub fn admit_foreground_reservation(
    request: ForegroundReservationAdmissionRequest<'_>,
) -> ForegroundReservationAdmissionOutcome {
    let lane = request.lane();
    let envelope = match lane.envelope() {
        Some(envelope) => envelope,
        None => {
            return denied(
                &request,
                ForegroundReservationAdmissionDenial::MissingLaneEnvelope,
            )
        }
    };
    if envelope.kind() == ForegroundLatencyEnvelopeKind::CertificationOnlyTarget {
        let counters = denied_counters(&request, ForegroundResourceBudget::new());
        return ForegroundReservationAdmissionOutcome::Held(ForegroundReservationHeld::new(
            lane.lane(),
            envelope,
            counters,
            ForegroundReservationAdmissionDenial::CertificationOnlyEnvelopeCannotExecute,
        ));
    }
    if let Err(denial) = require_declared_resource_budget(lane.requested_budget()) {
        return denied(&request, denial);
    }
    if let Err(denial) = require_lane_resource_contract(lane.lane(), lane.requested_budget()) {
        return denied(&request, denial);
    }
    if !lane.backend_requirement_is_store_owned() {
        return denied(
            &request,
            ForegroundReservationAdmissionDenial::LaneBackendRequirementNotStoreOwned {
                lane: lane.lane(),
                backend_requirement: lane.backend_requirement(),
            },
        );
    }
    if let Err(denial) = require_backend_requirement_matches_lane(&request) {
        return denied(&request, denial);
    }
    if let Err(denial) = require_arbitration_matches_lane(&request) {
        return denied(&request, denial);
    }
    if let Err(denial) = require_secure_frame_scope(&request) {
        return denied(&request, denial);
    }
    if let Err(denial) = require_capacity_admission_matches_request(&request) {
        return denied(&request, denial);
    }
    match prove_foreground_reservation_progression(&request).into_raw() {
        worth_proof::TransitionOutcome::Success(_) => {}
        worth_proof::TransitionOutcome::Denied(denial) => return denied(&request, denial),
        worth_proof::TransitionOutcome::Deferred(deferred) => match deferred {},
        worth_proof::TransitionOutcome::Stale(_) => return stale_rebind_required(&request),
        worth_proof::TransitionOutcome::RebindRequired(_) => {
            return stale_rebind_required(&request)
        }
        worth_proof::TransitionOutcome::Failed(failed) => match failed {},
    }

    let readiness_counters = request.stable_readiness().counters();
    let reservation_counters = ForegroundReservationCounterSnapshot::admitted(
        lane.requested_budget(),
        request.capacity_admission().assumed_backend_limits(),
        request.capacity_admission().admitted_budget(),
        readiness_counters.wait_count(),
        readiness_counters.retry_count(),
    );
    let security_identity = request.security_scope().permission().identity();

    ForegroundReservationAdmissionOutcome::Admitted(ForegroundReservationReceipt::admitted(
        lane.lane(),
        super::ForegroundReservationBackendBasis::new(
            lane.backend_requirement(),
            request.backend().profile(),
            request.backend().evidence_class(),
        ),
        envelope,
        request.arbitration(),
        reservation_counters,
        security_identity,
    ))
}

fn require_capacity_admission_matches_request(
    request: &ForegroundReservationAdmissionRequest<'_>,
) -> Result<(), ForegroundReservationAdmissionDenial> {
    let capacity = request.capacity_admission();
    let lane = request.lane();
    if capacity.lane() != lane.lane() {
        return Err(
            ForegroundReservationAdmissionDenial::CapacityAdmissionLaneMismatch {
                requested: lane.lane(),
                admitted: capacity.lane(),
            },
        );
    }
    if capacity.requested_budget() != lane.requested_budget() {
        return Err(
            ForegroundReservationAdmissionDenial::CapacityAdmissionBudgetMismatch {
                lane_requested: lane.requested_budget(),
                capacity_requested: capacity.requested_budget(),
            },
        );
    }
    if capacity.backend_requirement() != request.backend().requirement()
        || capacity.backend_profile() != request.backend().profile()
        || capacity.backend_evidence_class() != request.backend().evidence_class()
    {
        return Err(ForegroundReservationAdmissionDenial::CapacityAdmissionBackendMismatch);
    }
    match lane.envelope() {
        Some(envelope) if capacity.envelope() == envelope => {}
        _ => return Err(ForegroundReservationAdmissionDenial::CapacityAdmissionEnvelopeMismatch),
    }
    if capacity.arbitration() != request.arbitration() {
        return Err(ForegroundReservationAdmissionDenial::CapacityAdmissionArbitrationMismatch);
    }
    if capacity.security_scope_identity() != request.security_scope().permission().identity() {
        return Err(ForegroundReservationAdmissionDenial::CapacityAdmissionSecurityScopeMismatch);
    }
    let readiness_counters = request.stable_readiness().counters();
    if capacity.stable_read_wait_count() != readiness_counters.wait_count()
        || capacity.stable_read_retry_count() != readiness_counters.retry_count()
    {
        return Err(
            ForegroundReservationAdmissionDenial::CapacityAdmissionReadinessCounterMismatch,
        );
    }
    Ok(())
}

fn require_backend_requirement_matches_lane(
    request: &ForegroundReservationAdmissionRequest<'_>,
) -> Result<(), ForegroundReservationAdmissionDenial> {
    let lane_required = request.lane().backend_requirement();
    let admitted = request.backend().requirement();
    if lane_required == admitted {
        Ok(())
    } else {
        Err(
            ForegroundReservationAdmissionDenial::LaneBackendRequirementMismatch {
                lane_required,
                admitted,
            },
        )
    }
}

fn require_arbitration_matches_lane(
    request: &ForegroundReservationAdmissionRequest<'_>,
) -> Result<(), ForegroundReservationAdmissionDenial> {
    let declared = request.lane().lane();
    let attempted = request.arbitration().declared_lane();
    if declared == attempted {
        Ok(())
    } else {
        Err(
            ForegroundReservationAdmissionDenial::ForegroundPriorityLaundering {
                declared,
                attempted,
            },
        )
    }
}

fn require_secure_frame_scope(
    request: &ForegroundReservationAdmissionRequest<'_>,
) -> Result<(), ForegroundReservationAdmissionDenial> {
    if request.lane().backend_requirement()
        != IoSchedulerBackendCapabilityRequirement::SecureFrameIo
    {
        return Ok(());
    }
    if !request.backend().security_scope_bound() {
        return Err(ForegroundReservationAdmissionDenial::SecureFrameBackendWasNotSecurityBound);
    }
    Ok(())
}

fn denied(
    request: &ForegroundReservationAdmissionRequest<'_>,
    denial: ForegroundReservationAdmissionDenial,
) -> ForegroundReservationAdmissionOutcome {
    denied_with_budget(request, denial, ForegroundResourceBudget::new())
}

fn denied_with_budget(
    request: &ForegroundReservationAdmissionRequest<'_>,
    denial: ForegroundReservationAdmissionDenial,
    denied_budget: ForegroundResourceBudget,
) -> ForegroundReservationAdmissionOutcome {
    let counters = denied_counters(request, denied_budget);
    ForegroundReservationAdmissionOutcome::Denied(ForegroundReservationDenied::new(
        request.lane().lane(),
        counters,
        denial,
    ))
}

fn denied_counters(
    request: &ForegroundReservationAdmissionRequest<'_>,
    denied_budget: ForegroundResourceBudget,
) -> ForegroundReservationCounterSnapshot {
    ForegroundReservationCounterSnapshot::denied_capacity(
        request.lane().requested_budget(),
        request.capacity_admission().assumed_backend_limits(),
        denied_budget,
    )
}

fn stale_rebind_required(
    request: &ForegroundReservationAdmissionRequest<'_>,
) -> ForegroundReservationAdmissionOutcome {
    let counters = ForegroundReservationCounterSnapshot::denied_capacity(
        request.lane().requested_budget(),
        request.capacity_admission().assumed_backend_limits(),
        request.capacity_admission().requested_budget(),
    );
    ForegroundReservationAdmissionOutcome::StaleRebindRequired(
        ForegroundReservationStaleRebindRequired::new(
            request.lane().lane(),
            counters,
            ForegroundReservationAdmissionDenial::ReservationBasisRebindRequired,
        ),
    )
}
