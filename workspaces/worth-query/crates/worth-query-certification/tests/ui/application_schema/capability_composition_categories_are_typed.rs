use worth_query_decl::facade::application_capability::{
    ApplicationCapabilityAllowRule, ApplicationCapabilityConflictRule,
    ApplicationCapabilityDecisionComposition, ApplicationCapabilityDenyRule,
    ApplicationCapabilityGraphRequirement, ApplicationCapabilityGraphRule,
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
    let _ = |left: ApplicationCapabilityGraphRequirement,
             right: ApplicationCapabilityGraphRequirement| {
        ApplicationCapabilityAllowRule::new(ApplicationCapabilityGraphRule::all([left, right]))
    };
}
