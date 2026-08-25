use worth_foundational::facade::AuthoritativeRecordAspectState;

use super::observation::{invalid, string_value, ObservationError};
use super::schema::{
    BookingStatus, HazardClass, InspectionResult, OperatingPosture, Region, VesselClass,
    VoyageStatus,
};
use super::semantic_key::EntityKey;

pub(super) fn debug_text(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<String, ObservationError> {
    string_value(key, state, name)
}

pub(super) fn debug_region(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<Region, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "NorthReach" => Ok(Region::NorthReach),
        "SouthReach" => Ok(Region::SouthReach),
        other => other
            .strip_prefix("Generated(")
            .and_then(|v| v.strip_suffix(')'))
            .and_then(|v| v.parse().ok())
            .map(Region::Generated)
            .ok_or_else(|| invalid(key, name, "unknown region")),
    }
}

pub(super) fn debug_posture(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<OperatingPosture, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Open" => Ok(OperatingPosture::Open),
        "Maintenance" => Ok(OperatingPosture::Maintenance),
        "Retired" => Ok(OperatingPosture::Retired),
        _ => Err(invalid(key, name, "unknown operating posture")),
    }
}

pub(super) fn debug_class(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<VesselClass, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Feeder" => Ok(VesselClass::Feeder),
        "Panamax" => Ok(VesselClass::Panamax),
        "HeavyLift" => Ok(VesselClass::HeavyLift),
        _ => Err(invalid(key, name, "unknown vessel class")),
    }
}

pub(super) fn debug_status(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<VoyageStatus, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Planned" => Ok(VoyageStatus::Planned),
        "Delayed" => Ok(VoyageStatus::Delayed),
        "Rerouted" => Ok(VoyageStatus::Rerouted),
        "Held" => Ok(VoyageStatus::Held),
        _ => Err(invalid(key, name, "unknown voyage status")),
    }
}

pub(super) fn debug_hazard(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<HazardClass, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "General" => Ok(HazardClass::General),
        "Medical" => Ok(HazardClass::Medical),
        "Industrial" => Ok(HazardClass::Industrial),
        "HazardousV2" => Ok(HazardClass::HazardousV2),
        _ => Err(invalid(key, name, "unknown hazard class")),
    }
}

pub(super) fn debug_booking(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<BookingStatus, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Available" => Ok(BookingStatus::Available),
        "Booked" => Ok(BookingStatus::Booked),
        "Held" => Ok(BookingStatus::Held),
        _ => Err(invalid(key, name, "unknown booking status")),
    }
}

pub(super) fn debug_result(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<InspectionResult, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Pending" => Ok(InspectionResult::Pending),
        "Passed" => Ok(InspectionResult::Passed),
        "Flagged" => Ok(InspectionResult::Flagged),
        _ => Err(invalid(key, name, "unknown inspection result")),
    }
}
