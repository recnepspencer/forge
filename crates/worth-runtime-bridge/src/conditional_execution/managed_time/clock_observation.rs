use super::{
    BridgeManagedClockAcceptedObservation, BridgeManagedClockObservationOutcome,
    BridgeManagedClockObservationParts, BridgeManagedTemporalDenial,
    BridgeManagedTemporalDenialKind,
};
use crate::conditional_execution::BridgeOwnedSignalRuntime;

enum ObservationAdmission {
    Advance,
    Duplicate,
    Stale,
    Reordered,
}

impl BridgeOwnedSignalRuntime {
    pub fn observe_managed_clock(
        &mut self,
        parts: BridgeManagedClockObservationParts<'_>,
    ) -> Result<BridgeManagedClockObservationOutcome, BridgeManagedTemporalDenial> {
        let binding_identity = parts.binding.binding_identity.clone();
        let lane = self.managed_clock_lane_mut(parts.binding)?;
        validate_observation_source(lane, &parts)?;
        let admission = classify_observation(
            lane.last_observation(),
            parts.sequence,
            parts.observed_coordinate,
        );
        match admission {
            ObservationAdmission::Stale => Ok(BridgeManagedClockObservationOutcome::Stale),
            ObservationAdmission::Reordered => Ok(BridgeManagedClockObservationOutcome::Reordered),
            ObservationAdmission::Duplicate => {
                let due = lane.promote_due(&binding_identity)?;
                Ok(BridgeManagedClockObservationOutcome::Duplicate(
                    BridgeManagedClockAcceptedObservation {
                        sequence: parts.sequence,
                        observed_coordinate: parts.observed_coordinate,
                        signal_advance_ordinal: None,
                        due,
                    },
                ))
            }
            ObservationAdmission::Advance => {
                let signal_advance_ordinal =
                    lane.advance_signal_clock(parts.observed_coordinate)?;
                lane.record_observation(parts.sequence, parts.observed_coordinate);
                let due = lane.promote_due(&binding_identity)?;
                Ok(BridgeManagedClockObservationOutcome::Accepted(
                    BridgeManagedClockAcceptedObservation {
                        sequence: parts.sequence,
                        observed_coordinate: parts.observed_coordinate,
                        signal_advance_ordinal: Some(signal_advance_ordinal),
                        due,
                    },
                ))
            }
        }
    }
}

fn validate_observation_source(
    lane: &super::BridgeManagedClockLane,
    parts: &BridgeManagedClockObservationParts<'_>,
) -> Result<(), BridgeManagedTemporalDenial> {
    if lane.source_identity.as_ref() != parts.source_identity {
        return Err(BridgeManagedTemporalDenial::new(
            BridgeManagedTemporalDenialKind::ForeignClockSource,
            "clock observation came from another installed source",
        ));
    }
    if lane.timeline_identity.as_ref() != parts.timeline_identity {
        return Err(BridgeManagedTemporalDenial::new(
            BridgeManagedTemporalDenialKind::ForeignClockTimeline,
            "clock observation came from another installed timeline",
        ));
    }
    Ok(())
}

fn classify_observation(
    last: Option<(u64, u64)>,
    sequence: u64,
    coordinate: u64,
) -> ObservationAdmission {
    let Some((last_sequence, last_coordinate)) = last else {
        return ObservationAdmission::Advance;
    };
    if sequence == last_sequence && coordinate == last_coordinate {
        ObservationAdmission::Duplicate
    } else if sequence <= last_sequence {
        ObservationAdmission::Stale
    } else if coordinate < last_coordinate {
        ObservationAdmission::Reordered
    } else {
        ObservationAdmission::Advance
    }
}
