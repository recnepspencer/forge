use worth_foundational::facade::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
    CanonicalEquivalenceBasis,
};
use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityAcceptedValues, ApplicationCapabilityActorComposition,
        ApplicationCapabilityAllowRule, ApplicationCapabilityCardinalityDimension,
        ApplicationCapabilityComposition, ApplicationCapabilityConflictRule,
        ApplicationCapabilityConstraintDefinition, ApplicationCapabilityContextEntitySlotRef,
        ApplicationCapabilityContextRef, ApplicationCapabilityContractBuilder,
        ApplicationCapabilityDecisionComposition, ApplicationCapabilityDelegationDefinition,
        ApplicationCapabilityDelegationRule, ApplicationCapabilityDenyRule,
        ApplicationCapabilityDisclosureRule, ApplicationCapabilityDistinctActorRule,
        ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
        ApplicationCapabilityGraphClause, ApplicationCapabilityGraphRequirement,
        ApplicationCapabilityGraphRule, ApplicationCapabilityPathContextAnchor,
        ApplicationCapabilityPropagationComposition, ApplicationCapabilityProvenanceRef,
        ApplicationCapabilityRef, ApplicationCapabilityRelationBinding,
        ApplicationCapabilityRelationDimension, ApplicationCapabilityScopeGuard,
        ApplicationCapabilitySeparationOfDutyRule, ApplicationCapabilityTargetDefinition,
        ApplicationCapabilityValidityDefinition, ApplicationCapabilityValueBinding,
        ErasedApplicationCapabilityContract,
    },
    application_schema::{
        ApplicationAuthorizationPath, ApplicationAuthorizationPathBuilder, ApplicationEntityRef,
        ApplicationFieldRef, ApplicationOperationRef, ApplicationRelationRef, EqualityPredicate,
        NoApplicationCurrency, ReadOnly,
    },
};

use super::canonical_basis::prepare_capability_basis;

mod budgets;
mod residue;

pub(super) struct Schema;
struct Capability;
struct Operation;
struct Grant;
struct Resource;
struct Principal;
struct Facts;
struct Action;
struct Purpose;
struct Field;
struct Amount;
struct Workflow;
struct ValidFrom;
struct ValidThrough;
struct DelegationLimit;
struct ResourceRelation;
struct ScopedRelation;
struct PrincipalResource;
struct Parent;
struct Grantor;
struct Grantee;
struct Context;
struct OtherContext;
struct Provenance;
struct OtherProvenance;
struct ResourceSlot;

#[derive(Clone, Copy)]
pub(super) enum Axis {
    Action,
    Resource,
    Relation,
    Field,
    Purpose,
    Amount,
    Cardinality,
    Workflow,
    Validity,
    Delegation,
    Provenance,
    Context,
    Rule(usize),
    ContextAnchor,
    AlternativeGrouping,
    ConjunctiveGrouping,
    OversizedComposition,
}

mod identity_axes;

pub(super) fn contract(axis: Option<Axis>) -> ErasedApplicationCapabilityContract {
    let context_name = if matches!(axis, Some(Axis::Context)) {
        "ChangedContext"
    } else {
        "Context"
    };
    let provenance_name = if matches!(axis, Some(Axis::Provenance)) {
        "ChangedProvenance"
    } else {
        "Provenance"
    };
    contract_with_axis(
        axis,
        ApplicationCapabilityContextRef::<Schema, Context>::from_schema_identifier(context_name),
        ApplicationCapabilityProvenanceRef::<Schema, Provenance>::from_schema_identifier(
            provenance_name,
        ),
    )
}

pub(super) fn contract_with_context<ContextMarker>(
    context: ApplicationCapabilityContextRef<Schema, ContextMarker>,
) -> ErasedApplicationCapabilityContract {
    contract_with_axis(
        None,
        context,
        ApplicationCapabilityProvenanceRef::<Schema, Provenance>::from_schema_identifier(
            "Provenance",
        ),
    )
}

fn contract_with_markers<ContextMarker, ProvenanceMarker>(
    context: ApplicationCapabilityContextRef<Schema, ContextMarker>,
    provenance: ApplicationCapabilityProvenanceRef<Schema, ProvenanceMarker>,
) -> ErasedApplicationCapabilityContract {
    contract_with_axis(None, context, provenance)
}

fn contract_with_axis<ContextMarker, ProvenanceMarker>(
    axis: Option<Axis>,
    context: ApplicationCapabilityContextRef<Schema, ContextMarker>,
    provenance: ApplicationCapabilityProvenanceRef<Schema, ProvenanceMarker>,
) -> ErasedApplicationCapabilityContract {
    let action_value = if matches!(axis, Some(Axis::Action)) {
        2
    } else {
        1
    };
    let purpose_value = if matches!(axis, Some(Axis::Purpose)) {
        2
    } else {
        1
    };
    let resource_name = if matches!(axis, Some(Axis::Resource)) {
        "ChangedResourceRelation"
    } else {
        "ResourceRelation"
    };
    let workflow_name = if matches!(axis, Some(Axis::Workflow)) {
        "ChangedWorkflow"
    } else {
        "Workflow"
    };
    let validity_name = if matches!(axis, Some(Axis::Validity)) {
        "ChangedValidFrom"
    } else {
        "ValidFrom"
    };
    let delegation_name = if matches!(axis, Some(Axis::Delegation)) {
        "ChangedParent"
    } else {
        "Parent"
    };
    let target = ApplicationCapabilityTargetDefinition::new(
        ApplicationCapabilityValueBinding::new(field::<Action>("Action"), action_value),
        relation::<ResourceRelation, Grant, Resource>(resource_name, "Grant", "Resource"),
        if matches!(axis, Some(Axis::Relation)) {
            ApplicationCapabilityRelationDimension::not_applicable()
        } else {
            ApplicationCapabilityRelationDimension::Bound(
                relation::<ScopedRelation, Grant, Resource>("ScopedRelation", "Grant", "Resource"),
            )
        },
        if matches!(axis, Some(Axis::Field)) {
            ApplicationCapabilityFieldDimension::not_applicable()
        } else {
            ApplicationCapabilityFieldDimension::bound(field::<Field>("Field"))
        },
        ApplicationCapabilityValueBinding::new(field::<Purpose>("Purpose"), purpose_value),
    );
    let constraints = ApplicationCapabilityConstraintDefinition::new(
        if matches!(axis, Some(Axis::Amount)) {
            ApplicationCapabilityFieldDimension::not_applicable()
        } else {
            ApplicationCapabilityFieldDimension::bound(field::<Amount>("Amount"))
        },
        if matches!(axis, Some(Axis::Cardinality)) {
            ApplicationCapabilityCardinalityDimension::Bounded(2)
        } else {
            ApplicationCapabilityCardinalityDimension::One
        },
        field_binding::<Workflow>(workflow_name),
        ApplicationCapabilityValidityDefinition::new(
            field_binding::<ValidFrom>(validity_name),
            field_binding::<ValidThrough>("ValidThrough"),
        ),
        context,
    );
    let delegation = ApplicationCapabilityDelegationDefinition::new(
        relation::<Parent, Grant, Grant>(delegation_name, "Grant", "Grant"),
        relation::<Grantor, Principal, Grant>("Grantor", "Principal", "Grant"),
        relation::<Grantee, Principal, Grant>("Grantee", "Principal", "Grant"),
        field_binding::<DelegationLimit>("DelegationLimit"),
        provenance,
    );
    let contract = ApplicationCapabilityContractBuilder::new(
        ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier("Capability"),
        ApplicationOperationRef::<Schema, Operation, ()>::from_schema_identifier("Operation"),
        ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
    )
    .target(target)
    .constraints(constraints)
    .delegation(delegation)
    .composition(composition(axis))
    .build();
    contract.erased().clone()
}

fn composition(axis: Option<Axis>) -> ApplicationCapabilityComposition {
    let changed = |index| matches!(axis, Some(Axis::Rule(candidate)) if candidate == index);
    let allow = if matches!(axis, Some(Axis::OversizedComposition)) {
        ApplicationCapabilityGraphRule::any((0..64).map(|ordinal| {
            let relation_name = Box::leak(format!("PrincipalResource{ordinal}").into_boxed_str());
            ApplicationCapabilityGraphClause::new(graph_path(true, relation_name))
        }))
    } else if matches!(axis, Some(Axis::ContextAnchor)) {
        identity_axes::anchored_graph_rule()
    } else if matches!(axis, Some(Axis::AlternativeGrouping)) {
        identity_axes::grouped_graph_rule(false)
    } else if matches!(axis, Some(Axis::ConjunctiveGrouping)) {
        identity_axes::grouped_graph_rule(true)
    } else {
        graph_rule(true, changed(0))
    };
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(allow),
            optional_deny_rule(changed(1)),
            optional_conflict_rule(changed(2)),
        ),
        ApplicationCapabilityActorComposition::new(
            optional_separation_rule(changed(3)),
            optional_distinct_actor_rule(changed(4)),
        ),
        ApplicationCapabilityPropagationComposition::new(
            if changed(5) {
                ApplicationCapabilityDelegationRule::forbidden()
            } else {
                ApplicationCapabilityDelegationRule::narrow_all_dimensions()
            },
            ApplicationCapabilityDisclosureRule::permit([
                ApplicationCapabilityScopeGuard::requiring([
                    ApplicationCapabilityAcceptedValues::one_of(
                        field::<Field>("Field"),
                        [if changed(6) { 2_u64 } else { 1_u64 }],
                    ),
                ]),
            ]),
        ),
    )
}

fn optional_deny_rule(changed: bool) -> ApplicationCapabilityDenyRule {
    if changed {
        ApplicationCapabilityDenyRule::not_applicable()
    } else {
        ApplicationCapabilityDenyRule::when(graph_rule(false, false))
    }
}

fn optional_conflict_rule(changed: bool) -> ApplicationCapabilityConflictRule {
    if changed {
        ApplicationCapabilityConflictRule::not_applicable()
    } else {
        ApplicationCapabilityConflictRule::when(graph_rule(false, false))
    }
}

fn optional_separation_rule(changed: bool) -> ApplicationCapabilitySeparationOfDutyRule {
    if changed {
        ApplicationCapabilitySeparationOfDutyRule::not_applicable()
    } else {
        ApplicationCapabilitySeparationOfDutyRule::when(graph_rule(false, false))
    }
}

fn optional_distinct_actor_rule(changed: bool) -> ApplicationCapabilityDistinctActorRule {
    if changed {
        ApplicationCapabilityDistinctActorRule::not_applicable()
    } else {
        ApplicationCapabilityDistinctActorRule::when(graph_rule(false, false))
    }
}

fn graph_rule(allow: bool, changed: bool) -> ApplicationCapabilityGraphRule {
    ApplicationCapabilityGraphRule::any([ApplicationCapabilityGraphClause::new(graph_path(
        allow,
        if changed {
            "ChangedPrincipalResource"
        } else {
            "PrincipalResource"
        },
    ))])
}

fn graph_path(allow: bool, relation_name: &'static str) -> ApplicationAuthorizationPath {
    let relation = ApplicationRelationRef::<Schema, PrincipalResource, Principal, Resource>::
        from_schema_identifiers(relation_name, "Principal", "Resource");
    let path = ApplicationAuthorizationPathBuilder::from_principal(ApplicationEntityRef::<
        Schema,
        Principal,
    >::from_schema_identifier(
        "Principal"
    ))
    .forward(relation);
    if allow {
        path.allow(ApplicationEntityRef::<Schema, Resource>::from_schema_identifier("Resource"))
    } else {
        path.deny(ApplicationEntityRef::<Schema, Resource>::from_schema_identifier("Resource"))
    }
}

fn field<FieldMarker>(
    name: &'static str,
) -> ApplicationFieldRef<
    Schema,
    Grant,
    Facts,
    FieldMarker,
    u64,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationFieldRef::from_schema_identifiers("Grant", "Facts", name)
}

fn field_binding<FieldMarker>(name: &'static str) -> ApplicationCapabilityFieldBinding {
    ApplicationCapabilityFieldBinding::from_reference(field::<FieldMarker>(name))
}

fn relation<RelationMarker, From, To>(
    name: &'static str,
    from: &'static str,
    to: &'static str,
) -> ApplicationCapabilityRelationBinding {
    ApplicationCapabilityRelationBinding::from_reference(ApplicationRelationRef::<
        Schema,
        RelationMarker,
        From,
        To,
    >::from_schema_identifiers(
        name, from, to
    ))
}

fn compare(
    left: &worth_foundational::facade::CanonicalBasisReadyArtifact,
    right: &worth_foundational::facade::CanonicalBasisReadyArtifact,
) -> CanonicalComparisonOutcome {
    let ready = prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left.clone(),
        right.clone(),
    )
    .into_result()
    .expect("capability basis comparison is supported");
    compare_canonical_basis(&ready)
}
