use crate::application_schema::application_authorization_path_canonical_components;
use worth_foundational::facade::canonical_basis_value_for_aspect_value;

use super::{
    canonical_components::{append_field, structural_count, text},
    ApplicationCapabilityCanonicalComponent, ApplicationCapabilityConflictRule,
    ApplicationCapabilityDelegationRule, ApplicationCapabilityDenyRule,
    ApplicationCapabilityDisclosureRule, ApplicationCapabilityDistinctActorRule,
    ApplicationCapabilityGraphRule, ApplicationCapabilityScopeGuard,
    ApplicationCapabilitySeparationOfDutyRule, ErasedApplicationCapabilityContract,
};

pub(super) fn append_composition(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    contract: &ErasedApplicationCapabilityContract,
) {
    let composition = contract.composition();
    append_graph_rule(
        components,
        "composition.allow",
        composition.decision().allow().graph(),
    );
    append_optional_graph_rule(
        components,
        "composition.deny",
        optional_deny_rule(composition.decision().deny()),
    );
    append_optional_graph_rule(
        components,
        "composition.conflict",
        optional_conflict_rule(composition.decision().conflict()),
    );
    append_optional_graph_rule(
        components,
        "composition.separation-of-duty",
        optional_separation_rule(composition.actors().separation_of_duty()),
    );
    append_optional_graph_rule(
        components,
        "composition.distinct-actor",
        optional_distinct_actor_rule(composition.actors().distinct_actor()),
    );
    append_delegation(
        components,
        "composition.delegation",
        composition.propagation().delegation(),
    );
    append_disclosure(
        components,
        "composition.disclosure",
        composition.propagation().disclosure(),
    );
}

fn append_optional_graph_rule(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    rule: Option<&ApplicationCapabilityGraphRule>,
) {
    match rule {
        None => text(components, format!("{prefix}.posture"), "not-applicable"),
        Some(rule) => {
            text(components, format!("{prefix}.posture"), "graph-rule");
            append_graph_rule(components, prefix, rule);
        }
    }
}

fn append_graph_rule(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    rule: &ApplicationCapabilityGraphRule,
) {
    structural_count(
        components,
        format!("{prefix}.clause-count"),
        rule.clauses().len(),
    );
    for (clause_ordinal, clause) in rule.clauses().iter().enumerate() {
        let clause_prefix = format!("{prefix}.clause[{clause_ordinal}]");
        for component in application_authorization_path_canonical_components(clause.path()) {
            components.push(ApplicationCapabilityCanonicalComponent::new(
                format!("{clause_prefix}.path.{}", component.locus()),
                component.value().clone(),
            ));
        }
        append_guard(
            components,
            &format!("{clause_prefix}.guard"),
            clause.guard(),
        );
    }
}

fn append_guard(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    guard: &ApplicationCapabilityScopeGuard,
) {
    structural_count(
        components,
        format!("{prefix}.requirement-count"),
        guard.requirements().len(),
    );
    for (requirement_ordinal, requirement) in guard.requirements().iter().enumerate() {
        let requirement_prefix = format!("{prefix}.requirement[{requirement_ordinal}]");
        append_field(
            components,
            &format!("{requirement_prefix}.field-binding"),
            requirement.field(),
        );
        structural_count(
            components,
            format!("{requirement_prefix}.accepted-value-count"),
            requirement.values().len(),
        );
        for (value_ordinal, value) in requirement.values().iter().enumerate() {
            components.push(ApplicationCapabilityCanonicalComponent::new(
                format!("{requirement_prefix}.accepted-value[{value_ordinal}]"),
                canonical_basis_value_for_aspect_value(value),
            ));
        }
    }
}

fn append_delegation(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    rule: ApplicationCapabilityDelegationRule,
) {
    let posture = match rule {
        ApplicationCapabilityDelegationRule::Forbidden => "forbidden",
        ApplicationCapabilityDelegationRule::NarrowAllDimensions => "narrow-all-dimensions",
    };
    text(components, format!("{prefix}.posture"), posture);
}

fn append_disclosure(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    rule: &ApplicationCapabilityDisclosureRule,
) {
    match rule {
        ApplicationCapabilityDisclosureRule::NotApplicable => {
            text(components, format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityDisclosureRule::Permit(guards) => {
            text(components, format!("{prefix}.posture"), "permit");
            structural_count(components, format!("{prefix}.guard-count"), guards.len());
            for (guard_ordinal, guard) in guards.iter().enumerate() {
                append_guard(
                    components,
                    &format!("{prefix}.guard[{guard_ordinal}]"),
                    guard,
                );
            }
        }
    }
}

const fn optional_deny_rule(
    rule: &ApplicationCapabilityDenyRule,
) -> Option<&ApplicationCapabilityGraphRule> {
    rule.graph()
}

const fn optional_conflict_rule(
    rule: &ApplicationCapabilityConflictRule,
) -> Option<&ApplicationCapabilityGraphRule> {
    rule.graph()
}

const fn optional_separation_rule(
    rule: &ApplicationCapabilitySeparationOfDutyRule,
) -> Option<&ApplicationCapabilityGraphRule> {
    rule.graph()
}

const fn optional_distinct_actor_rule(
    rule: &ApplicationCapabilityDistinctActorRule,
) -> Option<&ApplicationCapabilityGraphRule> {
    rule.graph()
}
