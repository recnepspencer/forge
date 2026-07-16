use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::ProjectionFactFieldPath;
use worth_query::facade::{domain, runtime};

use crate::{
    worth_ui_domain_package, WorthUiDomainEntry, WorthUiQueryBindingPlan,
    WorthUiQueryMeasurementFactFamily, WorthUiQueryWorkspaceExt,
};

#[test]
fn installed_projection_preserves_authority_native_value_and_settlement_posture() {
    let (mut binding, outcome) = installed_measurement_projection("settled", 240.0);
    let settlement = binding.admit(outcome).expect("installed projection should settle");

    assert!(!settlement.is_partial());
    assert_eq!(settlement.receipt().consumed_families(), [
        WorthUiQueryMeasurementFactFamily::ScrollContentExtent,
    ]);
    assert_eq!(
        settlement.receipt().observations()[0].extent(),
        worth_foundational::CanonicalF32::from_f32(240.0),
    );
    assert_eq!(settlement.allocation_source_generation().as_u64(), 1);
    assert_eq!(settlement.allocation_source_order().as_u64(), 1);
    let counters = settlement.receipt().refinement_counters();
    assert_eq!(counters.declared_measurement_family_count(), 1);
    assert_eq!(counters.projected_measurement_fact_count(), 1);
    assert_eq!(counters.refinement_attempt_count(), 1);
    assert_eq!(counters.admitted_observation_count(), 1);
}

#[test]
fn installed_projection_preserves_foundational_canonical_float_semantics() {
    let noncanonical_nan = f32::from_bits(0x7fc0_0042);
    let (mut binding, outcome) = installed_measurement_projection("canonical-nan", noncanonical_nan);
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
        Some(runtime::WorthQueryAuthoredAspectValue::struct_value(summary)),
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
    assert_eq!(denial, super::WorthUiQueryMeasurementFactSettlementDenial::QueryNotInstalled);
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
fn repeated_installed_settlements_reuse_source_identity_but_advance_order() {
    let mut workspace = measurement_workspace("repeated", 240.0);
    let installed = workspace.worth_ui().expect("Worth UI domain should be installed");
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

    assert_eq!(first.allocation_source_identity(), second.allocation_source_identity());
    assert_eq!(first.allocation_source_generation().as_u64(), 1);
    assert_eq!(second.allocation_source_generation().as_u64(), 1);
    assert_eq!(first.allocation_source_order().as_u64(), 1);
    assert_eq!(second.allocation_source_order().as_u64(), 2);
}

fn installed_measurement_projection(
    name: &str,
    extent: f32,
) -> (
    crate::WorthUiRuntimeQueryBinding,
    crate::WorthUiQueryProjectionOutcome,
) {
    let mut workspace = measurement_workspace(name, extent);
    let installed = workspace.worth_ui().expect("Worth UI domain should be installed");
    let view = installed
        .measurement_view("inspector.measurements")
        .expect("measurement view should admit");
    let binding = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("installed view should register")
        .activate();
    let outcome = project_view(&mut workspace, &view);
    (binding, outcome)
}

fn project_view(
    workspace: &mut runtime::WorthQueryWorkspace,
    view: &crate::WorthUiInstalledQueryView,
) -> crate::WorthUiQueryProjectionOutcome {
    let completion = view
        .read()
        .expect("installed read should declare")
        .using(domain::current())
        .run(workspace)
        .expect("installed authority should match workspace")
        .into_result()
        .expect("installed read should complete");
    view.project(
        &completion,
        domain::project_facts().display_field(measurement_value_path()),
    )
    .expect("completion should retain view installation")
}

fn measurement_workspace(name: &str, extent: f32) -> runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(crate::worth_ui_native_aspect_contracts())
        .expect("native aspect contracts should admit")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
        .workspace(name)
        .expect("installed-domain workspace should build");
    workspace
        .insert("WorthUiMeasurement", |measurement| {
            measurement
                .set_aspect(
                    runtime::WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")
                        .expect("identity touch should admit"),
                    runtime::WorthQueryAuthoredAspectValue::string("measurement"),
                )
                .set_aspect(
                    runtime::WorthQueryAspectTouch::from_authoring_ingress_text(
                        "measurement.value",
                    )
                    .expect("measurement touch should admit"),
                    runtime::WorthQueryAuthoredAspectValue::native(
                        worth_foundational::AspectValue::Float32(
                            worth_foundational::CanonicalF32::from_f32(extent),
                        ),
                    ),
                )
        })
        .expect("measurement fixture should insert");
    assert!(workspace.domain(WorthUiDomainEntry).is_ok());
    workspace
}

fn measurement_value_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("measurement").expect("aspect path should admit"),
            FieldKey::new("value").expect("field path should admit"),
        ])
        .expect("measurement path should admit"),
    )
}

fn projection_with_extra_aspect(
    name: &str,
    extra_contract: worth_foundational::facade::AspectContract,
    projection_path: &str,
    value: Option<runtime::WorthQueryAuthoredAspectValue>,
) -> (
    crate::WorthUiRuntimeQueryBinding,
    crate::WorthUiQueryProjectionOutcome,
) {
    let mut contracts = crate::worth_ui_native_aspect_contracts().to_vec();
    contracts.push(extra_contract);
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(contracts)
        .expect("native contracts")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect")
        .aspect(projection_path, projection_path)
        .expect("extra native aspect");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
        .workspace(name)
        .expect("installed workspace");
    workspace
        .insert("WorthUiMeasurement", |measurement| {
            let measurement = measurement
                .set_aspect(
                    runtime::WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")
                        .expect("identity touch"),
                    runtime::WorthQueryAuthoredAspectValue::string("measurement"),
                )
                .set_aspect(
                    runtime::WorthQueryAspectTouch::from_authoring_ingress_text(
                        "measurement.value",
                    )
                    .expect("measurement touch"),
                    runtime::WorthQueryAuthoredAspectValue::native(
                        worth_foundational::facade::AspectValue::Float32(
                            worth_foundational::CanonicalF32::from_f32(240.0),
                        ),
                    ),
                );
            match value {
                Some(value) => measurement.set_aspect(
                    runtime::WorthQueryAspectTouch::from_authoring_ingress_text(projection_path)
                        .expect("extra touch"),
                    value,
                ),
                None => measurement,
            }
        })
        .expect("native fixture insertion");
    let installed = workspace.worth_ui().expect("Worth UI installed");
    let view = installed
        .measurement_view("inspector.measurements")
        .expect("measurement view");
    let binding = WorthUiQueryBindingPlan::default()
        .register_view(view.clone())
        .expect("view registration")
        .activate();
    let completion = view
        .read()
        .expect("installed read")
        .using(domain::current())
        .run(&mut workspace)
        .expect("installed authority")
        .into_result()
        .expect("read completion");
    let projection_path = ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(
            projection_path
                .split('.')
                .map(|segment| FieldKey::new(segment).expect("projection segment"))
                .collect::<Vec<_>>(),
        )
        .expect("projection path"),
    );
    let outcome = view
        .project(
            &completion,
            domain::project_facts().display_field(projection_path),
        )
        .expect("installed projection");
    (binding, outcome)
}

fn native_struct_contract(
    aspect: &str,
    identity: u64,
    field: &str,
    requirement: worth_foundational::facade::FieldRequirement,
    absence: worth_foundational::facade::AbsenceLaw,
) -> worth_foundational::facade::AspectContract {
    use worth_foundational::facade::{
        AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity, AspectKey,
        FieldDeclaration, ScalarAspectType, StructAspectShape,
    };

    let declaration = FieldDeclaration::new(
        FieldKey::new(field).expect("field key"),
        ScalarAspectType::String,
        requirement,
        absence,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .expect("field law");
    AspectContract::struct_aspect(
        AspectKey::new(aspect).expect("aspect key"),
        AspectIdentity(identity),
        AspectContractRevision(1),
        StructAspectShape::new([declaration]).expect("struct shape"),
    )
}
