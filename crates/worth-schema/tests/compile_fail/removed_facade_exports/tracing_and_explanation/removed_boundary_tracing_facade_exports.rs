use schema::facade::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure,
    BridgeTraceAnchor, BridgeTraceEvidence, DecisionTrace, DerivedTraceAnchor,
    DerivedTraceEvidence, IntegrityMarkers, NamedCounter, PerformanceAccounting,
    SignalTraceAnchor, SignalTraceEvidence, TraceAvailability, TraceWarning,
};

fn main() {
    let _ = (
        None::<AuthorityTraceAnchor>,
        None::<AuthorityTraceEvidence>,
        None::<BoundaryEnvelope<()>>,
        None::<BoundaryFailure<()>>,
        None::<BridgeTraceAnchor>,
        None::<BridgeTraceEvidence>,
        None::<DecisionTrace>,
        None::<DerivedTraceAnchor>,
        None::<DerivedTraceEvidence>,
        None::<IntegrityMarkers>,
        None::<NamedCounter>,
        None::<PerformanceAccounting>,
        None::<SignalTraceAnchor>,
        None::<SignalTraceEvidence>,
        None::<TraceAvailability>,
        None::<TraceWarning>,
    );
}
