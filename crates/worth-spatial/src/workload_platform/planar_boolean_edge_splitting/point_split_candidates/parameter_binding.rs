use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanPointEvent, PlanarBooleanPointEventSegmentParameterFact,
};

use super::denial::{
    PlanarBooleanPointSplitCandidateDenial, PlanarBooleanPointSplitCandidateDenialKind,
};

pub(crate) enum PointEventCarrierParameterBinding<'a> {
    Bound(&'a PlanarBooleanPointEventSegmentParameterFact),
    RetainedParticipantWithoutParameter,
}

pub(crate) fn bind_point_event_to_source_edge_parameter<'a>(
    event: &'a PlanarBooleanPointEvent,
    carrier_identity: &str,
) -> Result<PointEventCarrierParameterBinding<'a>, PlanarBooleanPointSplitCandidateDenial> {
    let carrier_participates = event_participates_in_carrier(event, carrier_identity);
    let operand_a = matching_carrier_parameter(event.operand_a_parameter(), carrier_identity);
    let operand_b = matching_carrier_parameter(event.operand_b_parameter(), carrier_identity);
    match (carrier_participates, operand_a, operand_b) {
        (true, None, None) => {
            Ok(PointEventCarrierParameterBinding::RetainedParticipantWithoutParameter)
        }
        (false, None, None) => Err(missing_parameter_denial(event.event_identity())),
        (false, Some(_), _) | (false, _, Some(_)) => {
            Err(missing_parameter_denial(event.event_identity()))
        }
        (true, Some(parameter), None) | (true, None, Some(parameter)) => {
            Ok(PointEventCarrierParameterBinding::Bound(parameter))
        }
        (true, Some(first), Some(second))
            if first.parameter_fact_identity() == second.parameter_fact_identity() =>
        {
            Ok(PointEventCarrierParameterBinding::Bound(first))
        }
        (true, Some(_), Some(_)) => Err(conflicting_parameter_denial(event.event_identity())),
    }
}

fn event_participates_in_carrier(event: &PlanarBooleanPointEvent, carrier_identity: &str) -> bool {
    event
        .participating_carrier_identities()
        .iter()
        .any(|carrier| carrier == carrier_identity)
}

fn matching_carrier_parameter<'a>(
    parameter: &'a PlanarBooleanPointEventSegmentParameterFact,
    carrier_identity: &str,
) -> Option<&'a PlanarBooleanPointEventSegmentParameterFact> {
    (parameter.carrier_identity() == carrier_identity).then_some(parameter)
}

fn missing_parameter_denial(event_identity: &str) -> PlanarBooleanPointSplitCandidateDenial {
    PlanarBooleanPointSplitCandidateDenial::with_rejected_missing_parameter_fact(
        PlanarBooleanPointSplitCandidateDenialKind::MissingCarrierParameter,
        event_identity,
        "point event participation row has no matching carrier parameter fact",
    )
}

fn conflicting_parameter_denial(event_identity: &str) -> PlanarBooleanPointSplitCandidateDenial {
    PlanarBooleanPointSplitCandidateDenial::with_rejected_conflicting_parameter_fact(
        PlanarBooleanPointSplitCandidateDenialKind::ConflictingCarrierParameterFacts,
        event_identity,
        "point event has conflicting parameter facts for the same carrier",
    )
}
