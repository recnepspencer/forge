use forge_query::facade::runtime::{
    forge_query_domain, ForgeQueryAftermathContributionAuthoring,
    ForgeQueryExplanationContributionAuthoring, ForgeQueryInvariantCapabilityContributionAuthoring,
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
    ForgeQuerySupportContributionAuthoring, LiveViewDeclarationAdmissionBoundaryReceipt,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    WriteAuthorityExecutionReceipt,
};

fn proof_authoring_accepts_any_boundary_source<S>(
    source: &S,
    explanation: ForgeQueryExplanationContributionAuthoring,
)
where
    S: ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
{
    let _ = ForgeQuerySupportContributionAuthoring::narrowed_support(
        "routing.boundary",
        "source carries a real lower-runtime boundary envelope",
    )
    .for_lower_runtime_boundary_source(source);
    let _ = ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
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
    let _ = ForgeQueryAftermathContributionAuthoring::declares_residue(
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
    envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
) {
    let _ = forge_query_domain("worth.spatial").for_lower_runtime_boundary_source(live);
    let _ = forge_query_domain("worth.spatial").for_lower_runtime_boundary_source(write);
    let _ = forge_query_domain("worth.spatial").for_lower_runtime_boundary_source(signal);
    let _ = forge_query_domain("worth.spatial").for_lower_runtime_boundary_source(subscription);
    let _ = forge_query_domain("worth.spatial").for_lower_runtime_boundary_source(envelope);
}

fn main() {}
