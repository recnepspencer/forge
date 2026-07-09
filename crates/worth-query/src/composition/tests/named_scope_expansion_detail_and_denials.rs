use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, IntegerComparisonPredicate,
    QueryFamily, RootEntityKey, TraversalSelector,
};
use crate::composition::{
    BasisScopeEvidence, GuidedCompositionPath, QueryCompositionAdmissionFailureClass,
    QueryScopeDescriptor, ScopeFamily,
};
use crate::harness::fixtures::execution_preflights;
use crate::query_context::{
    admit_query_basis_context, bind_query_basis_context, QueryBasisContextRequest,
    QueryContextBindingSource,
};

fn direct_detail_query() -> crate::authoring::DetailAuthoredQuery {
    crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .build()
        .unwrap()
}

fn direct_detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

fn direct_collection_query() -> crate::authoring::CollectionAuthoredQuery {
    crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .build()
        .unwrap()
}

fn direct_collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

#[test]
fn scope_expansion_preserves_parity_and_emits_lineage() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 21).unwrap(),
            )
            .build()
            .unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_detail(direct_query, direct_detail_shape()).unwrap();

    let scope = QueryScopeDescriptor::predicate(
        "adults_only",
        [crate::authoring::PredicateSelector::IntegerComparison(
            IntegerComparisonPredicate::greater_than("profile", "age", 21).unwrap(),
        )],
    );
    let (artifact, expanded) = GuidedCompositionPath::expand_detail_scopes(
        direct_detail_query(),
        direct_detail_shape(),
        [scope],
    )
    .unwrap();
    let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();

    assert_eq!(artifact.query_family(), QueryFamily::Detail);
    assert_eq!(
        direct.query().digest(),
        composed.canonical().query().digest()
    );
    assert_ne!(
        composed
            .composition()
            .scope_lineage_digest()
            .expect("scope lineage should be explicit")
            .as_str(),
        ""
    );
    assert_eq!(composed.composition().counters().scope_expansion_count(), 1);
    assert_eq!(
        composed.composition().counters().scope_rediscovery_count(),
        0
    );
}

#[test]
fn basis_aware_scope_preserves_query_meaning_and_emits_basis_metadata() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_detail(direct_detail_query(), direct_detail_shape())
            .unwrap();
    let evidence = BasisScopeEvidence::from_admitted_context_for_canonical_query(
        &admitted,
        direct.query().digest(),
    );
    let scope = QueryScopeDescriptor::basis_aware("current_basis", evidence.clone());
    let (artifact, expanded) = GuidedCompositionPath::expand_detail_scopes(
        direct_detail_query(),
        direct_detail_shape(),
        [scope],
    )
    .unwrap();
    let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();

    assert_eq!(
        direct.query().digest(),
        composed.canonical().query().digest()
    );
    assert_eq!(
        artifact
            .basis_evidence()
            .expect("basis scope should retain evidence")
            .basis_digest(),
        admitted.basis_digest()
    );
    assert_eq!(
        composed.composition().basis_digest(),
        Some(admitted.basis_digest())
    );
}

#[test]
fn basis_aware_scope_denies_mismatched_query_binding() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let wrong_direct = GuidedAuthoringPath::canonicalize_detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("account").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::detail_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap();
    let evidence = BasisScopeEvidence::from_admitted_context_for_canonical_query(
        &admitted,
        wrong_direct.query().digest(),
    );

    let error = GuidedCompositionPath::expand_detail_scopes(
        direct_detail_query(),
        direct_detail_shape(),
        [QueryScopeDescriptor::basis_aware("current_basis", evidence)],
    )
    .expect_err("basis evidence should be query-bound");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::BasisEvidenceQueryMismatch
    );
}

#[test]
fn duplicate_basis_aware_scope_fails_with_exact_failure_class_and_counters() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let direct =
        GuidedAuthoringPath::canonicalize_detail(direct_detail_query(), direct_detail_shape())
            .unwrap();
    let evidence = BasisScopeEvidence::from_admitted_context_for_canonical_query(
        &admitted,
        direct.query().digest(),
    );

    let error = GuidedCompositionPath::expand_detail_scopes(
        direct_detail_query(),
        direct_detail_shape(),
        [
            QueryScopeDescriptor::basis_aware("current_basis", evidence.clone()),
            QueryScopeDescriptor::basis_aware("current_basis_again", evidence),
        ],
    )
    .expect_err("multiple basis-aware scopes should fail before lowering");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::DuplicateBasisAwareScope
    );
    assert_eq!(error.scope_family(), Some(ScopeFamily::BasisAwareScope));
    assert_eq!(error.counters().scope_expansion_count(), 2);
    assert_eq!(error.counters().scope_expansion_width(), 2);
}

#[test]
fn unsupported_scope_fails_typed_and_early() {
    let error = GuidedCompositionPath::expand_detail_scopes(
        direct_detail_query(),
        direct_detail_shape(),
        [QueryScopeDescriptor::unsupported_for_test("nope")],
    )
    .expect_err("unsupported scope should deny before authored request creation");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::UnsupportedScopeFamily
    );
    assert_eq!(error.scope_family(), Some(ScopeFamily::UnsupportedScope));
}

#[test]
fn traversal_bound_scope_denies_illegal_widening_before_canonicalization() {
    let scope = QueryScopeDescriptor::traversal_bound(
        "manager_depth",
        1,
        [TraversalSelector::bounded("manager", 2).unwrap()],
    );
    let error = GuidedCompositionPath::expand_collection_scopes(
        direct_collection_query(),
        direct_collection_shape(),
        [scope],
    )
    .expect_err("traversal widening should deny before canonicalization");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::IllegalScopeWidening
    );
}
