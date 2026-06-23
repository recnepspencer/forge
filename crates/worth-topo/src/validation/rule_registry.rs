use crate::validation::facade::{
    TopologyValidationInputClass, TopologyValidationPhase, TopologyValidationRow,
};
use crate::validation::rule_identity::{
    loop_wiring_rule, ownership_rule, radial_rings_rule, shell_closure_rule, vertex_disks_rule,
};
use crate::validation::TopologyValidationRuleIdentity;

#[derive(Debug, Clone, Copy)]
pub(crate) struct TopologyValidationRuleSpec {
    pub(crate) name: &'static str,
    pub(crate) identity: fn() -> TopologyValidationRuleIdentity,
    pub(crate) phase: TopologyValidationPhase,
    pub(crate) input_class: TopologyValidationInputClass,
}

pub(crate) const DERIVED_TOPOLOGY_RULE_SPECS: [TopologyValidationRuleSpec; 5] = [
    TopologyValidationRuleSpec {
        name: "ownership",
        identity: ownership_rule,
        phase: TopologyValidationPhase::DerivedMaterialization,
        input_class: TopologyValidationInputClass::MaterializedTopologyView,
    },
    TopologyValidationRuleSpec {
        name: "loop_wiring",
        identity: loop_wiring_rule,
        phase: TopologyValidationPhase::DerivedMaterialization,
        input_class: TopologyValidationInputClass::MaterializedTopologyView,
    },
    TopologyValidationRuleSpec {
        name: "radial_rings",
        identity: radial_rings_rule,
        phase: TopologyValidationPhase::DerivedInterpretation,
        input_class: TopologyValidationInputClass::InterpretedTopologyView,
    },
    TopologyValidationRuleSpec {
        name: "shell_closure",
        identity: shell_closure_rule,
        phase: TopologyValidationPhase::DerivedInterpretation,
        input_class: TopologyValidationInputClass::InterpretedTopologyView,
    },
    TopologyValidationRuleSpec {
        name: "vertex_disks",
        identity: vertex_disks_rule,
        phase: TopologyValidationPhase::DerivedInterpretation,
        input_class: TopologyValidationInputClass::InterpretedTopologyView,
    },
];

pub(crate) fn rule_spec_for_name(name: &str) -> Option<&'static TopologyValidationRuleSpec> {
    DERIVED_TOPOLOGY_RULE_SPECS
        .iter()
        .find(|spec| spec.name == name)
}

pub(crate) fn rule_identity_for_validator(
    validator: &str,
) -> Option<TopologyValidationRuleIdentity> {
    let rule_name = validator
        .split_once('.')
        .map_or(validator, |(name, _)| name);
    rule_spec_for_name(rule_name).map(|spec| (spec.identity)())
}

pub(crate) fn validate_row_against_spec(
    row: &TopologyValidationRow,
    spec: &TopologyValidationRuleSpec,
) -> Result<(), String> {
    if row.validator != spec.name {
        return Err(format!(
            "expected validator `{}`, found `{}`",
            spec.name, row.validator
        ));
    }
    let expected_identity = (spec.identity)();
    if row.rule_identity != expected_identity {
        return Err(format!(
            "expected rule identity `{}`, found `{}`",
            expected_identity.stable_key(),
            row.rule_identity.stable_key()
        ));
    }
    if row.phase != spec.phase {
        return Err(format!(
            "rule `{}` declared wrong phase {:?}",
            spec.name, row.phase
        ));
    }
    if row.input_class != spec.input_class {
        return Err(format!(
            "rule `{}` declared wrong input class {:?}",
            spec.name, row.input_class
        ));
    }
    Ok(())
}
