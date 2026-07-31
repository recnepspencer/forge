use worth_query_decl::facade::application_capability::{
    ApplicationCapabilityAllowRule, ApplicationCapabilityConflictRule,
    ApplicationCapabilityDecisionComposition, ApplicationCapabilityDenyRule,
};

fn compose(
    allow: ApplicationCapabilityAllowRule,
    deny: ApplicationCapabilityDenyRule,
    conflict: ApplicationCapabilityConflictRule,
) -> ApplicationCapabilityDecisionComposition {
    ApplicationCapabilityDecisionComposition::new(allow, deny, conflict)
}

fn main() {
    let _ = compose;
}
