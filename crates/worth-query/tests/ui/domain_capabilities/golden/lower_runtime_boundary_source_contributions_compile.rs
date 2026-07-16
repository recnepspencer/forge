#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::domain::{
    WorthQueryAftermathContributionAuthoring, WorthQueryExplanationContributionAuthoring,
    WorthQueryInvariantCapabilityContributionAuthoring, WorthQuerySupportContributionAuthoring,
};
use worth_query::facade::runtime::{
    LiveViewDeclarationAdmissionBoundaryReceipt, SignalInvalidationBoundaryReceipt,
    SubscriptionActivationBoundaryReceipt, WorthQueryLowerRuntimeBoundaryEnvelope,
    WorthQueryLowerRuntimeBoundaryEnvelopeSource, WriteAuthorityExecutionReceipt,
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
    let installation = installed_domain::install("lower-runtime-source-golden");
    let domain = installation.contributions();
    let _ = domain.for_lower_runtime_boundary_source(live);
    let _ = domain.for_lower_runtime_boundary_source(write);
    let _ = domain.for_lower_runtime_boundary_source(signal);
    let _ = domain.for_lower_runtime_boundary_source(subscription);
    let _ = domain.for_lower_runtime_boundary_source(envelope);
}

fn main() {}
