use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, OrderingSelector, RootEntityKey,
};
use crate::composition::{
    BasisScopeEvidence, GuidedCompositionPath, QueryCompositionAdmissionFailureClass,
    QueryTemplateDescriptor, TemplateBindingSet, TemplateFamily, TemplateParameterSlot,
};
use crate::harness::fixtures::execution_preflights;
use crate::query_context::{
    admit_query_basis_context, bind_query_basis_context, QueryBasisContextRequest,
    QueryContextBindingSource,
};

use super::template_instantiation_support::{
    equality_binding, focused_inspector_deferred_template, observed_inspector_deferred_template,
    template_detail_query, template_detail_shape,
};

#[test]
fn observed_inspector_template_family_remains_deferred_typed_and_early() {
    let slot = TemplateParameterSlot::predicate("name_filter");
    let template = observed_inspector_deferred_template().with_slot(slot.clone());

    let error =
        GuidedCompositionPath::instantiate_detail_template(template, equality_binding(&slot))
            .expect_err("observed inspector template family should remain deferred");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::DeferredTemplateFamily
    );
}

#[test]
fn focused_inspector_template_family_remains_deferred_typed_and_early() {
    let slot = TemplateParameterSlot::predicate("name_filter");
    let template = focused_inspector_deferred_template().with_slot(slot.clone());

    let error =
        GuidedCompositionPath::instantiate_detail_template(template, equality_binding(&slot))
            .expect_err("focused inspector template family should remain deferred");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::DeferredTemplateFamily
    );
}

#[test]
fn template_instantiation_denies_missing_binding_with_exact_failure_class_and_counters() {
    let slot = TemplateParameterSlot::predicate("name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(slot);

    let error =
        GuidedCompositionPath::instantiate_detail_template(template, TemplateBindingSet::new())
            .expect_err("missing template binding should fail before lowering");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::MissingTemplateBinding
    );
    assert_eq!(
        error.template_family(),
        Some(TemplateFamily::DetailTemplate)
    );
    assert_eq!(error.counters().template_slot_count(), 1);
    assert_eq!(error.counters().template_binding_width(), 0);
}

#[test]
fn template_instantiation_denies_duplicate_binding_with_exact_failure_class_and_counters() {
    let slot = TemplateParameterSlot::predicate("name_filter");
    let predicate = crate::authoring::PredicateSelector::Equality(
        crate::authoring::EqualityPredicate::new(
            "profile",
            "display_name",
            crate::authoring::ScalarPredicateValue::String("Alice".to_string()),
        )
        .unwrap(),
    );
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(slot.clone());
    let bindings = TemplateBindingSet::new()
        .bind_predicate(&slot, predicate.clone())
        .bind_predicate(&slot, predicate);

    let error = GuidedCompositionPath::instantiate_detail_template(template, bindings)
        .expect_err("duplicate template binding should fail before lowering");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::DuplicateTemplateBinding
    );
    assert_eq!(
        error.template_family(),
        Some(TemplateFamily::DetailTemplate)
    );
    assert_eq!(error.counters().template_slot_count(), 1);
    assert_eq!(error.counters().template_binding_width(), 2);
}

#[test]
fn template_instantiation_denies_binding_for_undeclared_slot_with_exact_failure_class() {
    let declared_slot = TemplateParameterSlot::predicate("declared_name_filter");
    let undeclared_slot = TemplateParameterSlot::predicate("undeclared_name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(declared_slot);
    let bindings = equality_binding(&undeclared_slot);

    let error = GuidedCompositionPath::instantiate_detail_template(template, bindings)
        .expect_err("undeclared slot binding should fail before lowering");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::TemplateBindingMismatch
    );
    assert_eq!(
        error.template_family(),
        Some(TemplateFamily::DetailTemplate)
    );
}

#[test]
fn template_instantiation_denies_binding_kind_mismatch_with_exact_failure_class() {
    let slot = TemplateParameterSlot::predicate("name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(slot.clone());
    let bindings = TemplateBindingSet::new().bind_ordering(
        &slot,
        OrderingSelector::ascending("profile", "display_name").unwrap(),
    );

    let error = GuidedCompositionPath::instantiate_detail_template(template, bindings)
        .expect_err("binding kind mismatch should fail before lowering");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::TemplateBindingMismatch
    );
    assert_eq!(
        error.template_family(),
        Some(TemplateFamily::DetailTemplate)
    );
    assert_eq!(error.counters().template_slot_count(), 1);
    assert_eq!(error.counters().template_binding_width(), 1);
}

#[test]
fn template_instantiation_denies_duplicate_slot_declaration_with_exact_failure_class() {
    let slot = TemplateParameterSlot::predicate("name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(slot.clone())
            .with_slot(slot.clone());

    let error =
        GuidedCompositionPath::instantiate_detail_template(template, equality_binding(&slot))
            .expect_err("duplicate slot declaration should fail before lowering");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::TemplateBindingMismatch
    );
    assert_eq!(
        error.template_family(),
        Some(TemplateFamily::DetailTemplate)
    );
    assert_eq!(error.counters().template_slot_count(), 2);
    assert_eq!(error.counters().template_binding_width(), 0);
}

#[test]
fn template_instantiation_denies_basis_evidence_bound_to_different_canonical_query() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let wrong_direct = crate::authoring::GuidedAuthoringPath::canonicalize_detail(
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

    let slot = TemplateParameterSlot::predicate("name_filter");
    let template =
        QueryTemplateDescriptor::detail(template_detail_query(), template_detail_shape())
            .with_slot(slot.clone())
            .with_basis_evidence(evidence);

    let error =
        GuidedCompositionPath::instantiate_detail_template(template, equality_binding(&slot))
            .expect_err("mismatched basis evidence should fail on the template lane");

    assert_eq!(
        error.failure_class(),
        &QueryCompositionAdmissionFailureClass::BasisEvidenceQueryMismatch
    );
    assert_eq!(error.counters().template_slot_count(), 1);
    assert_eq!(error.counters().template_binding_width(), 1);
}
