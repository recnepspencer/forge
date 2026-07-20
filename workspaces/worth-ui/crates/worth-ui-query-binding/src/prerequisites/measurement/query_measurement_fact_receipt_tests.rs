mod installed_projection_fixture;

use worth_foundational::facade::FieldKey;
use worth_query::facade::{domain, runtime};

use crate::{WorthUiQueryBindingPlan, WorthUiQueryMeasurementFactFamily, WorthUiQueryWorkspaceExt};
use installed_projection_fixture::{
    installed_measurement_projection, measurement_value_path, measurement_workspace,
    native_struct_contract, project_view, projection_with_extra_aspect,
};

#[test]
fn installed_projection_preserves_authority_native_value_and_settlement_posture() {
    let (mut binding, outcome) = installed_measurement_projection("settled", 240.0);
    let settlement = binding
        .admit(outcome)
        .expect("installed projection should settle");

    assert!(!settlement.is_partial());
    assert_eq!(
        settlement.receipt().consumed_families(),
        [WorthUiQueryMeasurementFactFamily::ScrollContentExtent,]
    );
    assert_eq!(
        settlement.receipt().observations()[0].extent(),
        worth_foundational::CanonicalF32::from_f32(240.0),
    );
    assert_eq!(settlement.allocation_source_generation().as_u64(), 1);
    assert_eq!(settlement.allocation_source_order().as_u64(), 1);
    let counters = settlement.receipt().refinement_counters();
    assert_eq!(counters.declared_measurement_fact_count(), 1);
    assert_eq!(counters.projected_measurement_fact_count(), 1);
    assert_eq!(counters.refinement_attempt_count(), 1);
    assert_eq!(counters.admitted_observation_count(), 1);
}

#[test]
fn installed_projection_preserves_foundational_canonical_float_semantics() {
    let noncanonical_nan = f32::from_bits(0x7fc0_0042);
    let (mut binding, outcome) =
        installed_measurement_projection("canonical-nan", noncanonical_nan);
    let settlement = binding.admit(outcome).expect("native NaN should settle");

    assert_eq!(
        settlement.receipt().observations()[0].extent(),
        worth_foundational::CanonicalF32::from_f32(noncanonical_nan),
    );
    assert_eq!(
        settlement.receipt().observations()[0].extent().bits(),
        f32::NAN.to_bits(),
    );
}

#[test]
fn installed_projection_preserves_query_denial_for_structured_measurement() {
    use worth_foundational::facade::{AspectValue, StructAspectValue};

    let contract = native_struct_contract(
        "summary",
        0x5755_49f1,
        "label",
        worth_foundational::facade::FieldRequirement::Required,
        worth_foundational::facade::AbsenceLaw::Required,
    );
    let summary = StructAspectValue::new([(
        FieldKey::new("label").expect("field"),
        AspectValue::String("native".into()),
    )])
    .expect("native struct");
    let (mut binding, outcome) = projection_with_extra_aspect(
        "native-struct",
        contract,
        "summary",
        Some(runtime::WorthQueryAuthoredAspectValue::struct_value(
            summary,
        )),
    );

    assert_eq!(
        binding.admit(outcome),
        Err(super::WorthUiQueryMeasurementFactSettlementDenial::Denied),
    );
}

#[test]
fn installed_projection_preserves_query_denial_for_absent_measurement() {
    let contract = native_struct_contract(
        "optional_measurement",
        0x5755_49f2,
        "value",
        worth_foundational::facade::FieldRequirement::Optional,
        worth_foundational::facade::AbsenceLaw::Optional,
    );
    let (mut binding, outcome) = projection_with_extra_aspect(
        "native-absence",
        contract,
        "optional_measurement.value",
        None,
    );

    assert_eq!(
        binding.admit(outcome),
        Err(super::WorthUiQueryMeasurementFactSettlementDenial::Denied),
    );
}

#[test]
fn query_free_runtime_denies_before_frame_ingress() {
    let (_, outcome) = installed_measurement_projection("query-free", 240.0);
    let denial = WorthUiQueryBindingPlan::default()
        .activate()
        .admit(outcome)
        .expect_err("query-free runtime cannot consume Query work");
    assert_eq!(
        denial,
        super::WorthUiQueryMeasurementFactSettlementDenial::QueryNotInstalled
    );
}

#[test]
fn foreign_installed_authority_cannot_activate_registered_view() {
    let (mut left, _) = installed_measurement_projection("left", 240.0);
    let (_, foreign_outcome) = installed_measurement_projection("right", 241.0);
    let denial = left
        .admit(foreign_outcome)
        .expect_err("foreign installed authority must deny before settlement");
    assert_eq!(
        denial,
        super::WorthUiQueryMeasurementFactSettlementDenial::InstalledAuthorityMismatch,
    );
}

#[test]
fn execution_evidence_rejects_an_equal_definition_from_a_foreign_installation() {
    let mut left_workspace = measurement_workspace("left-reference", 240.0);
    let left_view = left_workspace
        .worth_ui()
        .expect("left domain")
        .measurement_view("inspector.measurements")
        .expect("left view");
    let left_plan = WorthUiQueryBindingPlan::default()
        .register_view(left_view.clone())
        .expect("left plan");
    let mut left_binding = left_plan.activate();
    left_binding
        .admit(project_view(&mut left_workspace, &left_view))
        .expect("left settlement");

    let foreign_workspace = measurement_workspace("foreign-reference", 240.0);
    let foreign_view = foreign_workspace
        .worth_ui()
        .expect("foreign domain")
        .measurement_view("inspector.measurements")
        .expect("foreign view");
    let foreign_plan = WorthUiQueryBindingPlan::default()
        .register_view(foreign_view)
        .expect("foreign plan");
    let foreign_reference = foreign_plan
        .resolve_definition(
            left_view.definition().identity(),
            left_view.definition().shape(),
        )
        .expect("equal foreign definition resolves inside its own plan");

    assert_eq!(
        left_binding.execution_evidence_for(&foreign_reference),
        Err(crate::WorthUiQueryViewExecutionEvidenceDenial::ForeignInstalledReference)
    );
}

#[test]
fn repeated_installed_settlements_reuse_source_identity_but_advance_order() {
    let mut workspace = measurement_workspace("repeated", 240.0);
    let installed = workspace
        .worth_ui()
        .expect("Worth UI domain should be installed");
    let view = installed
        .measurement_view("inspector.measurements")
        .expect("measurement view should admit");
    let mut binding = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("installed view should register")
        .activate();
    let first = binding
        .admit(project_view(&mut workspace, &view))
        .expect("first projection should settle");
    let second = binding
        .admit(project_view(&mut workspace, &view))
        .expect("second projection should settle");

    assert_eq!(
        first.allocation_source_identity(),
        second.allocation_source_identity()
    );
    assert_eq!(first.allocation_source_generation().as_u64(), 1);
    assert_eq!(second.allocation_source_generation().as_u64(), 1);
    assert_eq!(first.allocation_source_order().as_u64(), 1);
    assert_eq!(second.allocation_source_order().as_u64(), 2);
}

#[test]
fn equivalent_projection_keys_do_not_collapse_distinct_authority_allocations() {
    let mut workspace = measurement_workspace("authority-allocation", 240.0);
    let installed = workspace
        .worth_ui()
        .expect("Worth UI domain should be installed");
    let view = installed
        .measurement_view("inspector.measurements")
        .expect("measurement view should admit");
    let (_, first_outcome, _) = project_view(&mut workspace, &view)
        .into_transfer()
        .into_parts();
    let (_, second_outcome, _) = project_view(&mut workspace, &view)
        .into_transfer()
        .into_parts();
    let (first, _) = super::WorthUiQueryAuthorityHandle::from_outcome(first_outcome)
        .expect("first projection authority should admit");
    let (second, _) = super::WorthUiQueryAuthorityHandle::from_outcome(second_outcome)
        .expect("second projection authority should admit");

    assert!(first
        .authority()
        .structurally_equivalent(second.authority()));
    assert_eq!(
        first.authority_index_key().expect("first typed index key"),
        second
            .authority_index_key()
            .expect("second typed index key")
    );
    assert!(!first.shares_authority_with(&second));
    assert_ne!(first, second);
}

#[test]
fn refinement_counters_use_exact_declared_and_projected_fact_width() {
    let mut workspace = measurement_workspace("refinement-width", 240.0);
    let installed = workspace
        .worth_ui()
        .expect("Worth UI domain should be installed");
    let view = installed
        .measurement_view("inspector.measurements")
        .expect("measurement view should admit");
    let mut binding = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("installed view should register")
        .activate();
    let completion = view
        .read()
        .expect("installed read should declare")
        .using(domain::current())
        .run(&mut workspace)
        .expect("installed authority should match workspace")
        .into_result()
        .expect("installed read should complete");
    let field = measurement_value_path();
    let outcome = view
        .project(
            &completion,
            domain::project_facts()
                .display_field(field.clone())
                .derived_field(field),
        )
        .expect("view projection retains installed authority");
    let settlement = binding.admit(outcome).expect("projection should settle");

    let counters = settlement.receipt().refinement_counters();
    assert_eq!(counters.declared_measurement_fact_count(), 2);
    assert_eq!(counters.projected_measurement_fact_count(), 2);
    assert_eq!(counters.refinement_attempt_count(), 2);
    assert_eq!(counters.admitted_observation_count(), 1);
}
