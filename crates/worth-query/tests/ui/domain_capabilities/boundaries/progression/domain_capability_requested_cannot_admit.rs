use worth_query::facade::runtime::{
    admit_eligible_domain_capability_contribution, WorthQueryAdmissionContributionAuthoring,
    WorthQueryIntentDeclaration, WorthQueryIntentInput,
};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "rotate",
        "worth.spatial.rotate",
        "1",
        "worth.spatial.intent",
        WorthQueryIntentInput::object([("entity", WorthQueryIntentInput::string("edge:42"))]),
    );
    let requested = WorthQueryAdmissionContributionAuthoring::advisory(
        "arbitration.requires_clarification",
        "multiple candidates remain",
    )
    .for_intent_declaration(&declaration);
    let _ = admit_eligible_domain_capability_contribution(requested);
}
