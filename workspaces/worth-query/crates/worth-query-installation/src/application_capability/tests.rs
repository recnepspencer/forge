use worth_foundational::facade::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
    CanonicalEquivalenceBasis,
};
use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityActorComposition, ApplicationCapabilityCardinalityDimension,
        ApplicationCapabilityComposition, ApplicationCapabilityConstraintDefinition,
        ApplicationCapabilityContextRef, ApplicationCapabilityContractBuilder,
        ApplicationCapabilityDecisionComposition, ApplicationCapabilityDelegationDefinition,
        ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
        ApplicationCapabilityPropagationComposition, ApplicationCapabilityProvenanceRef,
        ApplicationCapabilityRef, ApplicationCapabilityRelationBinding,
        ApplicationCapabilityRelationDimension, ApplicationCapabilityRule,
        ApplicationCapabilityTargetDefinition, ApplicationCapabilityValidityDefinition,
        ApplicationCapabilityValueBinding, ErasedApplicationCapabilityContract,
    },
    application_schema::{
        ApplicationEntityRef, ApplicationFieldRef, ApplicationOperationRef, ApplicationPolicyRef,
        ApplicationRelationRef, EqualityPredicate, NoApplicationCurrency, ReadOnly,
    },
};

use super::{
    canonical_basis::prepare_capability_basis,
    WorthQueryApplicationCapabilityInstallationDenialKind,
};

mod residue;

struct Schema;
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
struct Parent;
struct Grantor;
struct Grantee;
struct Context;
struct Provenance;
struct Policy;

#[derive(Clone, Copy)]
enum Axis {
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
}

#[test]
fn every_scope_and_composition_axis_changes_structured_and_digest_identity() {
    let package = worth_foundational::facade::CanonicalDigestId::new([1; 32]);
    let schema = worth_foundational::facade::CanonicalDigestId::new([2; 32]);
    let baseline = contract(None);
    let baseline = prepare_capability_basis(&package, &schema, &baseline).unwrap();
    let axes = [
        Axis::Action,
        Axis::Resource,
        Axis::Relation,
        Axis::Field,
        Axis::Purpose,
        Axis::Amount,
        Axis::Cardinality,
        Axis::Workflow,
        Axis::Validity,
        Axis::Delegation,
        Axis::Provenance,
        Axis::Context,
        Axis::Rule(0),
        Axis::Rule(1),
        Axis::Rule(2),
        Axis::Rule(3),
        Axis::Rule(4),
        Axis::Rule(5),
        Axis::Rule(6),
    ];
    for axis in axes {
        let changed = contract(Some(axis));
        let changed = prepare_capability_basis(&package, &schema, &changed).unwrap();
        assert_ne!(baseline.digest(), changed.digest());
        assert!(matches!(
            compare(baseline.basis(), changed.basis()),
            CanonicalComparisonOutcome::Mismatched(_)
        ));
    }
}

#[test]
fn capability_canonical_bytes_are_bounded_before_identity_derivation() {
    struct LongContext;
    let long_name = "x".repeat(17 * 1_024);
    let context = ApplicationCapabilityContextRef::<Schema, LongContext>::from_schema_identifier(
        Box::leak(long_name.into_boxed_str()),
    );
    let contract = contract_with_context(context);
    let package = worth_foundational::facade::CanonicalDigestId::new([1; 32]);
    let schema = worth_foundational::facade::CanonicalDigestId::new([2; 32]);
    let denial = prepare_capability_basis(&package, &schema, &contract).unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalByteLimitExceeded
    );
}

fn contract(axis: Option<Axis>) -> ErasedApplicationCapabilityContract {
    let context_name = if matches!(axis, Some(Axis::Context)) {
        "ChangedContext"
    } else {
        "Context"
    };
    contract_with_axis(
        axis,
        ApplicationCapabilityContextRef::<Schema, Context>::from_schema_identifier(context_name),
    )
}

fn contract_with_context<ContextMarker>(
    context: ApplicationCapabilityContextRef<Schema, ContextMarker>,
) -> ErasedApplicationCapabilityContract {
    contract_with_axis(None, context)
}

fn contract_with_axis<ContextMarker>(
    axis: Option<Axis>,
    context: ApplicationCapabilityContextRef<Schema, ContextMarker>,
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
    let provenance_name = if matches!(axis, Some(Axis::Provenance)) {
        "ChangedProvenance"
    } else {
        "Provenance"
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
        ApplicationCapabilityProvenanceRef::<Schema, Provenance>::from_schema_identifier(
            provenance_name,
        ),
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
    let rules: [ApplicationCapabilityRule; 7] = std::array::from_fn(|index| {
        if matches!(axis, Some(Axis::Rule(changed)) if changed == index) {
            ApplicationCapabilityRule::not_applicable()
        } else {
            ApplicationCapabilityRule::policy(
                ApplicationPolicyRef::<Schema, Policy>::from_schema_identifier(match index {
                    0 => "Allow",
                    1 => "Deny",
                    2 => "Conflict",
                    3 => "Separation",
                    4 => "Distinct",
                    5 => "Delegation",
                    _ => "Disclosure",
                }),
            )
        }
    });
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            rules[0].clone(),
            rules[1].clone(),
            rules[2].clone(),
        ),
        ApplicationCapabilityActorComposition::new(rules[3].clone(), rules[4].clone()),
        ApplicationCapabilityPropagationComposition::new(rules[5].clone(), rules[6].clone()),
    )
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
