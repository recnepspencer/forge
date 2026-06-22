use super::denial::{
    PlanarBooleanEdgeSplitScopeAdmissionDenial, PlanarBooleanEdgeSplitScopeAdmissionDenialKind,
};
use super::input::PlanarBooleanEdgeSplitScopeAdmissionInput;
use super::policy_outcome::PlanarBooleanEdgeSplitPolicyOutcome;
use super::scope_class::PlanarBooleanEdgeSplitScopeClass;

pub(crate) fn classify_edge_split_scope(
    input: &PlanarBooleanEdgeSplitScopeAdmissionInput<'_>,
) -> Result<PlanarBooleanEdgeSplitScopeClass, PlanarBooleanEdgeSplitScopeAdmissionDenial> {
    let split_request = input.split_request();
    if split_request.counters().segment_carrier_count() == 0 {
        return Err(PlanarBooleanEdgeSplitScopeAdmissionDenial::new(
            PlanarBooleanEdgeSplitScopeAdmissionDenialKind::UnsupportedEmptySourceCarrierScope,
            split_request.split_request_identity(),
            PlanarBooleanEdgeSplitPolicyOutcome::unsupported(
                split_request.event_ledger_identity(),
                split_request.split_request_identity(),
            ),
            "edge split scope admission requires source-edge carrier provenance before recovery can begin",
        ));
    }
    Ok(PlanarBooleanEdgeSplitScopeClass::PlanarBRepLineSegmentEdgeSurgery)
}
