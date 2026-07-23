use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, AspectMask, CanonicalMismatchKind, FieldDeclaration, FieldKey, FieldRequirement,
    ProjectionMask, ScalarAspectType, StructAspectShape,
};
use worth_query_declaration::facade::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, RootEntityKey,
};
use worth_query_declaration::facade::binding::{
    NonIdentityBindingMetadata, QueryBindingDescriptor,
};
use worth_query_declaration::facade::canonicalization::canonicalize_request;
use worth_query_declaration::facade::identity::CanonicalEquivalence;

use super::*;
use crate::domain_operation::*;

mod canonical_identity;

#[test]
fn equivalent_operation_retains_owner_work_without_claiming_hidden_scans() {
    let left = operation("inspect", 1, semantics(1, "Entity"));
    let right = operation("inspect", 1, semantics(1, "Entity"));
    let WorthQueryPortableOperationComparisonOutcome::Equivalent(equivalent) =
        compare_portable_domain_operations(&left, &right)
    else {
        panic!("same owner meaning must compare equivalent");
    };

    let work = equivalent.work();
    assert!(work.owner_dimensions_inspected() >= 40);
    assert_eq!(work.direct_foundational_comparison_requests(), 2);
    assert_eq!(work.canonical_export_comparison_requests(), 1);
    assert_eq!(work.conditional_owner_comparison_requests(), 1);
    assert_eq!(
        work.delegated_conditional_foundational_comparison_requests(),
        1
    );
    assert_eq!(work.subject_conditional_nodes_submitted(), 0);
    assert_eq!(work.candidate_conditional_nodes_submitted(), 0);
    assert_eq!(work.variable_items_submitted(), 6);
}

#[test]
fn native_revision_drift_preserves_foundational_category_and_dimension() {
    let left = operation("inspect", 1, semantics(1, "Entity"));
    let right = operation("inspect", 1, semantics(2, "Entity"));
    let WorthQueryPortableOperationComparisonOutcome::Mismatched(mismatch) =
        compare_portable_domain_operations(&left, &right)
    else {
        panic!("native revision drift must mismatch");
    };

    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableOperationDimension::NativeContract
    );
    assert_eq!(
        mismatch.category(),
        WorthQueryPortableOperationComparisonMismatchCategory::Foundational
    );
    assert_eq!(
        mismatch.foundational_basis().unwrap().kind(),
        CanonicalMismatchKind::ValueMismatch
    );
    assert_eq!(mismatch.work().direct_foundational_comparison_requests(), 1);
}

#[test]
fn canonical_query_drift_uses_the_declaration_owner_not_digest_authority() {
    let left = operation("inspect", 1, semantics(1, "EntityA"));
    let right = operation("inspect", 1, semantics(1, "EntityB"));
    let WorthQueryPortableOperationComparisonOutcome::Mismatched(mismatch) =
        compare_portable_domain_operations(&left, &right)
    else {
        panic!("declaration-owner query drift must mismatch");
    };

    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableOperationDimension::CanonicalQuery
    );
    assert_eq!(
        mismatch.category(),
        WorthQueryPortableOperationComparisonMismatchCategory::DeclarationOwner
    );
    assert!(mismatch.foundational_basis().is_none());
}

#[test]
fn declaration_owner_equivalence_ignores_nonsemantic_authoring_diagnostics() {
    let left = operation("inspect", 1, semantics(1, "Entity"));
    let mut right_semantics = semantics(1, "Entity");
    right_semantics.canonical_query = canonical_bundle_with_route_metadata("Entity");
    assert_ne!(
        left.semantics().canonical_query,
        right_semantics.canonical_query
    );
    assert_eq!(
        left.semantics()
            .canonical_query
            .equivalence_to(&right_semantics.canonical_query),
        CanonicalEquivalence::Equivalent
    );
    let right = operation("inspect", 1, right_semantics);

    assert!(matches!(
        compare_portable_domain_operations(&left, &right),
        WorthQueryPortableOperationComparisonOutcome::Equivalent(_)
    ));
}

#[test]
fn support_drift_reports_the_exact_owner_field() {
    let left = operation("inspect", 1, semantics(1, "Entity"));
    let mut right_semantics = semantics(1, "Entity");
    right_semantics.support.sharing = WorthQuerySupportRequirement::Required;
    let right = operation("inspect", 1, right_semantics);
    let WorthQueryPortableOperationComparisonOutcome::Mismatched(mismatch) =
        compare_portable_domain_operations(&left, &right)
    else {
        panic!("sharing support drift must mismatch");
    };

    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableOperationDimension::Support(
            WorthQueryPortableOperationSupportDimension::Sharing
        )
    );
    assert_eq!(
        mismatch.category(),
        WorthQueryPortableOperationComparisonMismatchCategory::InstallationOwner
    );
}

#[test]
fn nested_conditional_drift_preserves_the_conditional_owner_dimension() {
    let mut left_semantics = semantics(1, "Entity");
    left_semantics.conditional_nodes = vec![conditional_node(
        WorthQueryComparatorRequirement::ExactCanonicalValue,
    )];
    let mut right_semantics = semantics(1, "Entity");
    right_semantics.conditional_nodes = vec![conditional_node(
        WorthQueryComparatorRequirement::FoundationalContractEquivalence,
    )];
    let left = operation("inspect", 1, left_semantics);
    let right = operation("inspect", 1, right_semantics);
    let WorthQueryPortableOperationComparisonOutcome::Mismatched(mismatch) =
        compare_portable_domain_operations(&left, &right)
    else {
        panic!("conditional comparator drift must mismatch");
    };

    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableOperationDimension::Conditional(
            WorthQueryOperationConditionalDimension::Declaration {
                location: WorthQueryConditionalNodeLocation::operation("gate").unwrap(),
                dimension: WorthQueryPortableConditionalDimension::DependencyComparator,
            }
        )
    );
    assert_eq!(
        mismatch.category(),
        WorthQueryPortableOperationComparisonMismatchCategory::Foundational
    );
    assert_eq!(mismatch.work().subject_conditional_nodes_submitted(), 1);
    assert_eq!(mismatch.work().candidate_conditional_nodes_submitted(), 1);
    assert!(
        mismatch
            .work()
            .delegated_conditional_foundational_comparison_requests()
            > 1
    );
}

#[test]
fn derived_canonical_identity_is_never_a_comparison_input() {
    let left = operation("inspect", 1, semantics(1, "Entity"));
    let right = operation("inspect-next", 1, semantics(1, "Entity"));
    assert_ne!(left.canonical_identity(), right.canonical_identity());
    let WorthQueryPortableOperationComparisonOutcome::Mismatched(mismatch) =
        compare_portable_domain_operations(&left, &right)
    else {
        panic!("typed identity drift must mismatch");
    };
    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableOperationDimension::IdentityName
    );
    assert_eq!(mismatch.work().owner_dimensions_inspected(), 1);
}

fn operation(
    name: &str,
    version: u32,
    semantics: WorthQueryDomainOperationSemanticClosure,
) -> WorthQueryPortableDomainOperationDefinition {
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(
        WorthQueryDomainOperationIdentity::new(name, version),
        semantics,
    )
    .into_portable()
}

fn semantics(native_revision: u64, root: &str) -> WorthQueryDomainOperationSemanticClosure {
    WorthQueryDomainOperationSemanticClosure {
        parameters: WorthQueryOperationParameterContract::NotRequired,
        native_projection: native_projection(native_revision),
        canonical_query: canonical_bundle(root),
        collection: WorthQueryOperationCollectionContract::NotCollection,
        required_capabilities: Vec::new(),
        required_domains: Vec::new(),
        workflow: WorthQueryOperationWorkflowContract::NotRequired,
        conditional_nodes: Vec::new(),
        graph_reads: WorthQueryOperationGraphReadContract::NotRequired,
        touches: WorthQueryOperationTouchContract::NotRequired,
        effects: WorthQueryOperationEffectContract::NotRequired,
        invariants: WorthQueryOperationInvariantContract::NotRequired,
        replay: WorthQueryOperationReplayContract::ReExecutable,
        reversal: WorthQueryOperationReversalContract::Irreversible,
        lineage: WorthQueryOperationLineageContract::NotRequired,
        promotion: WorthQueryOperationPromotionContract::NotRequired,
        publication: WorthQueryOperationPublicationContract::NotRequired,
        projection_consumption: WorthQueryOperationProjectionConsumptionContract::NotRequired,
        terminal: WorthQueryOperationTerminalContract {
            result_states: vec![WorthQueryOperationResultState::Ready],
            failure_classes: Vec::new(),
        },
        cost: WorthQueryOperationCostContract {
            lookup: WorthQueryOperationCostClass::Constant,
            execution: WorthQueryOperationCostClass::Constant,
            result_width: WorthQueryOperationCostClass::Constant,
        },
        support: no_support(),
        lowering: WorthQueryOperationLoweringContract {
            family: "owner-comparison-test".into(),
            deterministic: true,
        },
    }
}

fn no_support() -> WorthQueryOperationSupportRequirements {
    WorthQueryOperationSupportRequirements {
        live: WorthQuerySupportRequirement::NotRequired,
        continuation: WorthQuerySupportRequirement::NotRequired,
        async_result_state: WorthQuerySupportRequirement::NotRequired,
        recovery: WorthQuerySupportRequirement::NotRequired,
        inspection: WorthQuerySupportRequirement::NotRequired,
        projection_consumption: WorthQuerySupportRequirement::NotRequired,
        dependency_impact: WorthQuerySupportRequirement::NotRequired,
        sharing: WorthQuerySupportRequirement::NotRequired,
        invalidation: WorthQuerySupportRequirement::NotRequired,
        collection_delivery: WorthQuerySupportRequirement::NotRequired,
        conditional_evaluation: WorthQuerySupportRequirement::NotRequired,
        conditional_comparator: WorthQuerySupportRequirement::NotRequired,
        conditional_trigger: WorthQuerySupportRequirement::NotRequired,
        conditional_temporal_or_on_demand: WorthQuerySupportRequirement::NotRequired,
    }
}

struct TestTrigger;

impl WorthQueryOnDemandTriggerFamily for TestTrigger {
    const PORTABLE_IDENTITY: &'static str = "test.operation-comparison.trigger";
}

fn conditional_node(
    comparator: WorthQueryComparatorRequirement,
) -> WorthQueryPortableConditionalNodeDeclaration {
    WorthQueryPortableConditionalNodeDeclaration::declare(
        "gate",
        WorthQueryConditionalNodeRole::OperationGate,
    )
    .dependencies([])
    .outputs([WorthQueryConditionalNodeOutput::OperationOutput {
        projection_role: WorthQueryOperationProjectionRole::new("profile").unwrap(),
    }])
    .required_context([WorthQueryConditionalNodeContext::OperationInput])
    .evaluation(
        WorthQueryConditionalEvaluationCondition::on_demand(),
        WorthQueryConditionalTrigger::on_demand::<TestTrigger>(),
    )
    .comparison(
        comparator,
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
    )
    .artifact_policy(
        WorthQueryArtifactReuseEquivalence::NotReusable,
        WorthQueryMaintenancePosture::OnDemandOnly,
        WorthQueryArtifactPosture::Ephemeral,
    )
    .output_relationship(WorthQueryOutputRelationship::IsOperationOutput)
    .finish()
    .unwrap()
}

fn native_projection(revision: u64) -> WorthQueryOperationNativeProjectionContract {
    let field = FieldDeclaration::new(
        FieldKey::new("name").unwrap(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .unwrap();
    let contract = AspectContract::struct_aspect(
        AspectKey::new("profile").unwrap(),
        AspectIdentity(1602),
        AspectContractRevision(revision),
        StructAspectShape::new([field]).unwrap(),
    );
    WorthQueryOperationNativeProjectionContract::new(
        contract,
        AspectMask::<ProjectionMask>::whole_aspect(),
    )
    .unwrap()
}

fn canonical_bundle(
    root: &str,
) -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    canonical_bundle_with_bindings(root, QueryBindingDescriptor::new())
}

fn canonical_bundle_with_route_metadata(
    root: &str,
) -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    canonical_bundle_with_bindings(
        root,
        QueryBindingDescriptor::new()
            .with_non_identity(NonIdentityBindingMetadata::new("route", "entity.inspect").unwrap()),
    )
}

fn canonical_bundle_with_bindings(
    root: &str,
    bindings: QueryBindingDescriptor,
) -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    let selector = AspectFieldSelector::new("profile", "name").unwrap();
    let query = DetailQueryBuilder::new(RootEntityKey::new(root).unwrap())
        .project(selector)
        .build()
        .unwrap()
        .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("profile", "name", "name").unwrap())
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, bindings).unwrap(),
    )
    .unwrap()
}
