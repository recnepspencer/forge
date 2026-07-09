use worth_query::facade::runtime::{
    worth_query_domain, WorthQueryAftermathContributionAuthoring,
    WorthQueryExplanationContributionAuthoring, WorthQueryInvariantCapabilityContributionAuthoring,
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryEnvelopeSource,
    WorthQuerySupportContributionAuthoring, LiveViewDeclarationAdmissionBoundaryReceipt,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    WriteAuthorityExecutionReceipt,
};

fn proof_authoring_accepts_any_boundary_source<S>(
    source: &S,
    explanation: WorthQueryExplanationContributionAuthoring,
)
where
    S: WorthQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
{
    let _ = WorthQuerySupportContributionAuthoring::narrowed_support(
        "routing.boundary",
        "source carries a real lower-runtime boundary envelope",
    )
    .for_lower_runtime_boundary_source(source);
    let _ = WorthQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
        "graph.invariant",
        ["collection"],
        ["symbol"],
        ["target"],
        ["lifecycle"],
        "program",
        "breadth",
        "counters",
        "graph.invariant",
        "denied",
    )
    .for_lower_runtime_boundary_source(source);
    let _ = explanation.for_lower_runtime_boundary_source(source);
    let _ = WorthQueryAftermathContributionAuthoring::declares_residue(
        "boundary.residue",
        "source-bound aftermath",
    )
    .for_lower_runtime_boundary_source(source);
}

fn common_lane_accepts_receipt_sources(
    live: &LiveViewDeclarationAdmissionBoundaryReceipt,
    write: &WriteAuthorityExecutionReceipt,
    signal: &SignalInvalidationBoundaryReceipt,
    subscription: &SubscriptionActivationBoundaryReceipt,
    envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
) {
    let _ = worth_query_domain("worth.spatial").for_lower_runtime_boundary_source(live);
    let _ = worth_query_domain("worth.spatial").for_lower_runtime_boundary_source(write);
    let _ = worth_query_domain("worth.spatial").for_lower_runtime_boundary_source(signal);
    let _ = worth_query_domain("worth.spatial").for_lower_runtime_boundary_source(subscription);
    let _ = worth_query_domain("worth.spatial").for_lower_runtime_boundary_source(envelope);
}

fn main() {}
