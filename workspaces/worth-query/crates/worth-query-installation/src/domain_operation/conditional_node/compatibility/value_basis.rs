use worth_foundational::facade::{
    canonical_basis_value_for_aspect_value, CanonicalBasisValue, CanonicalIntegerWidth,
};

use crate::domain_operation::{
    WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalConditionClass, WorthQueryConditionalConsequenceRole,
    WorthQueryConditionalNodeOutput, WorthQueryConditionalTrigger, WorthQueryDeltaComparisonDomain,
    WorthQueryOutputEquivalenceRequirement, WorthQueryPortableConditionParameterValue,
    WorthQueryQuantityValueFamily, WorthQuerySemanticLocality, WorthQueryTemporalCondition,
    WorthQueryThresholdBoundary,
};

pub(super) fn aspect_value(value: &worth_foundational::facade::AspectValue) -> CanonicalBasisValue {
    canonical_basis_value_for_aspect_value(value)
}

fn text(value: impl Into<String>) -> CanonicalBasisValue {
    CanonicalBasisValue::ExactText(value.into().into())
}

fn signed(width: CanonicalIntegerWidth, value: impl Into<i128>) -> CanonicalBasisValue {
    CanonicalBasisValue::SignedInteger {
        width,
        value: value.into(),
    }
}

fn unsigned_width(width: CanonicalIntegerWidth, value: impl Into<u128>) -> CanonicalBasisValue {
    CanonicalBasisValue::UnsignedInteger {
        width,
        value: value.into(),
    }
}

pub(super) fn parameter_values(
    value: &WorthQueryPortableConditionParameterValue,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    let (kind, value) = match value {
        WorthQueryPortableConditionParameterValue::Bool(value) => {
            ("bool", CanonicalBasisValue::Bool(*value))
        }
        WorthQueryPortableConditionParameterValue::U64(value) => {
            ("u64", unsigned_width(CanonicalIntegerWidth::Bits64, *value))
        }
        WorthQueryPortableConditionParameterValue::I64(value) => {
            ("i64", signed(CanonicalIntegerWidth::Bits64, *value))
        }
        WorthQueryPortableConditionParameterValue::Text(value) => ("text", text(value.clone())),
        WorthQueryPortableConditionParameterValue::NativeValue(value) => {
            ("native", canonical_basis_value_for_aspect_value(value))
        }
    };
    vec![("kind", text(kind)), ("value", value)]
}

pub(super) fn locality_values(
    locality: &WorthQuerySemanticLocality,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    match locality {
        WorthQuerySemanticLocality::SourceRecord => vec![("kind", text("source-record"))],
        WorthQuerySemanticLocality::SourcePartition(role) => vec![
            ("kind", text("source-partition")),
            ("partition-role", text(role.as_str())),
        ],
        WorthQuerySemanticLocality::WholeLogicalGraph => {
            vec![("kind", text("whole-logical-graph"))]
        }
    }
}

pub(super) fn trigger_values(
    trigger: &WorthQueryConditionalTrigger,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    match trigger {
        WorthQueryConditionalTrigger::DependencyChange => {
            vec![("kind", text("dependency-change"))]
        }
        WorthQueryConditionalTrigger::OnDemand(owner) => {
            vec![("kind", text("on-demand")), ("owner", text(owner.as_str()))]
        }
        WorthQueryConditionalTrigger::Temporal(wake) => vec![
            ("kind", text("temporal")),
            (
                "wake",
                text(match wake {
                    crate::domain_operation::WorthQueryTemporalWake::MonotonicClock => {
                        "monotonic-clock"
                    }
                    crate::domain_operation::WorthQueryTemporalWake::WallClock => "wall-clock",
                    crate::domain_operation::WorthQueryTemporalWake::OnSnapshotAdvance => {
                        "snapshot-advance"
                    }
                }),
            ),
        ],
    }
}

pub(super) fn comparator_values(
    value: &WorthQueryComparatorRequirement,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    match value {
        WorthQueryComparatorRequirement::ExactCanonicalValue => {
            vec![("kind", text("exact-canonical-value"))]
        }
        WorthQueryComparatorRequirement::FoundationalContractEquivalence => {
            vec![("kind", text("foundational-contract-equivalence"))]
        }
        WorthQueryComparatorRequirement::Registered(family) => vec![
            ("kind", text("registered")),
            ("family", text(family.as_str())),
        ],
    }
}

pub(super) fn output_equivalence_values(
    value: &WorthQueryOutputEquivalenceRequirement,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    match value {
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue => {
            vec![("kind", text("exact-canonical-value"))]
        }
        WorthQueryOutputEquivalenceRequirement::FoundationalContractEquivalence => {
            vec![("kind", text("foundational-contract-equivalence"))]
        }
        WorthQueryOutputEquivalenceRequirement::OutputIdentity => {
            vec![("kind", text("output-identity"))]
        }
        WorthQueryOutputEquivalenceRequirement::Registered(family) => vec![
            ("kind", text("registered")),
            ("family", text(family.as_str())),
        ],
    }
}

pub(super) fn artifact_reuse_values(
    value: &WorthQueryArtifactReuseEquivalence,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    match value {
        WorthQueryArtifactReuseEquivalence::NotReusable => {
            vec![("kind", text("not-reusable"))]
        }
        WorthQueryArtifactReuseEquivalence::DependencyAndOutputEquivalent => {
            vec![("kind", text("dependency-and-output-equivalent"))]
        }
        WorthQueryArtifactReuseEquivalence::OutputEquivalent => {
            vec![("kind", text("output-equivalent"))]
        }
        WorthQueryArtifactReuseEquivalence::Registered(family) => vec![
            ("kind", text("registered")),
            ("family", text(family.as_str())),
        ],
    }
}

pub(super) fn temporal_condition_values(
    condition: WorthQueryTemporalCondition,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    let (kind, amount) = match condition {
        WorthQueryTemporalCondition::AfterNanoseconds(value) => ("after-ns", Some(value)),
        WorthQueryTemporalCondition::AtOrAfterUnixNanoseconds(value) => {
            ("at-or-after-unix-ns", Some(value))
        }
        WorthQueryTemporalCondition::DebounceNanoseconds(value) => ("debounce-ns", Some(value)),
        WorthQueryTemporalCondition::ThrottleNanoseconds(value) => ("throttle-ns", Some(value)),
        WorthQueryTemporalCondition::StaleAfterNanoseconds(value) => {
            ("stale-after-ns", Some(value))
        }
        WorthQueryTemporalCondition::IntervalNanoseconds(value) => ("interval-ns", Some(value)),
        WorthQueryTemporalCondition::SnapshotAdvance => ("snapshot-advance", None),
    };
    let mut values = vec![("kind", text(kind))];
    if let Some(amount) = amount {
        values.push((
            "amount",
            unsigned_width(CanonicalIntegerWidth::Bits64, amount),
        ));
    }
    values
}

pub(super) fn role_name(
    role: crate::domain_operation::WorthQueryConditionalNodeRole,
) -> &'static str {
    match role {
        crate::domain_operation::WorthQueryConditionalNodeRole::Computed => "computed",
        crate::domain_operation::WorthQueryConditionalNodeRole::WorkflowStage => "workflow-stage",
        crate::domain_operation::WorthQueryConditionalNodeRole::OperationGate => "operation-gate",
    }
}

pub(super) fn output_kind(
    output: &crate::domain_operation::WorthQueryConditionalNodeOutput,
) -> &'static str {
    match output {
        WorthQueryConditionalNodeOutput::DerivedAspect { .. } => "derived-aspect",
        WorthQueryConditionalNodeOutput::OperationOutput { .. } => "operation-output",
        WorthQueryConditionalNodeOutput::WorkflowStageOutput { .. } => "workflow-stage-output",
    }
}

pub(super) fn consequence_kind(
    consequence: &crate::domain_operation::WorthQueryConditionalConsequenceRole,
) -> &'static str {
    match consequence {
        WorthQueryConditionalConsequenceRole::DerivedOnly => "derived-only",
        WorthQueryConditionalConsequenceRole::Touch(_) => "touch",
        WorthQueryConditionalConsequenceRole::Effect(_) => "effect",
    }
}

pub(super) fn condition_class_name(
    value: crate::domain_operation::WorthQueryConditionalConditionClass,
) -> &'static str {
    match value {
        WorthQueryConditionalConditionClass::AlwaysEligible => "always-eligible",
        WorthQueryConditionalConditionClass::AspectFiltered => "aspect-filtered",
        WorthQueryConditionalConditionClass::DeltaThreshold => "delta-threshold",
        WorthQueryConditionalConditionClass::OnDemand => "on-demand",
        WorthQueryConditionalConditionClass::Temporal => "temporal",
        WorthQueryConditionalConditionClass::DomainSpecific => "domain-specific",
    }
}

pub(super) fn comparison_domain_name(
    value: crate::domain_operation::WorthQueryDeltaComparisonDomain,
) -> &'static str {
    match value {
        WorthQueryDeltaComparisonDomain::AbsoluteDifference => "absolute-difference",
        WorthQueryDeltaComparisonDomain::RelativeRatio => "relative-ratio",
    }
}

pub(super) fn boundary_name(
    value: crate::domain_operation::WorthQueryThresholdBoundary,
) -> &'static str {
    match value {
        WorthQueryThresholdBoundary::Inclusive => "inclusive",
        WorthQueryThresholdBoundary::Exclusive => "exclusive",
    }
}

pub(super) fn value_family_name(
    value: crate::domain_operation::WorthQueryQuantityValueFamily,
) -> &'static str {
    match value {
        WorthQueryQuantityValueFamily::Integer => "integer",
        WorthQueryQuantityValueFamily::Float32 => "float32",
        WorthQueryQuantityValueFamily::Float64 => "float64",
    }
}

pub(super) fn workflow_value_contract_name(
    value: crate::domain_operation::WorthQueryWorkflowValueContract,
) -> &'static str {
    use crate::domain_operation::WorthQueryWorkflowValueContract as Contract;
    match value {
        Contract::NotRequired => "not-required",
        Contract::Bool => "bool",
        Contract::I64 => "i64",
        Contract::U64 => "u64",
        Contract::Text => "text",
        Contract::EntityIdentity => "entity-identity",
        Contract::Projection => "projection",
    }
}
