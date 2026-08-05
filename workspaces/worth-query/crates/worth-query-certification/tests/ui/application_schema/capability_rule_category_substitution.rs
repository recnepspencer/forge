use worth_query_decl::facade::{
    application_capability::{
        ApplicationCapabilityConflictRule, ApplicationCapabilityDecisionComposition,
        ApplicationCapabilityDenyRule, ApplicationCapabilityGraphRequirement,
    },
    application_schema::ApplicationPolicyRef,
};

struct Schema;
struct Policy;

fn deny_rule_cannot_become_allow_rule() {
    let _ = ApplicationCapabilityDecisionComposition::new(
        ApplicationCapabilityDenyRule::not_applicable(),
        ApplicationCapabilityDenyRule::not_applicable(),
        ApplicationCapabilityConflictRule::not_applicable(),
    );
}

fn policy_name_cannot_become_allow_rule() {
    let policy = ApplicationPolicyRef::<Schema, Policy>::from_schema_identifier("Policy");
    let _ = ApplicationCapabilityDecisionComposition::new(
        policy,
        ApplicationCapabilityDenyRule::not_applicable(),
        ApplicationCapabilityConflictRule::not_applicable(),
    );
}

fn graph_requirement_cannot_become_allow_rule(
    requirement: ApplicationCapabilityGraphRequirement,
) {
    let _ = ApplicationCapabilityDecisionComposition::new(
        requirement,
        ApplicationCapabilityDenyRule::not_applicable(),
        ApplicationCapabilityConflictRule::not_applicable(),
    );
}

fn main() {}
