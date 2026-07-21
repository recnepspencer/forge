use worth_foundational::facade::{AspectValue, CanonicalF32, CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::ProjectionFactFieldPath;
use worth_query::facade::runtime;
use worth_ui::facade::query_binding::{worth_ui_domain_package, worth_ui_native_aspect_contracts};

pub(super) fn installed_measurement_workspace(label: &str) -> runtime::WorthQueryWorkspace {
    installed_measurement_workspace_with(label, false)
}

pub(super) fn partial_measurement_workspace(label: &str) -> runtime::WorthQueryWorkspace {
    installed_measurement_workspace_with(label, true)
}

pub(super) fn unsupported_measurement_workspace(label: &str) -> runtime::WorthQueryWorkspace {
    worth_ui_query_binding::install_worth_ui_test_operation_executors(
        measurement_runtime_builder().consumer_support_posture(
            worth_query::facade::domain::WorthQueryConsumerSupportDimension::ProjectionConsumption,
            worth_query::facade::domain::WorthQueryConsumerSupportPosture::Unsupported,
        ),
    )
    .workspace(label)
    .expect("unsupported Query workspace")
}

fn installed_measurement_workspace_with(
    label: &str,
    partial: bool,
) -> runtime::WorthQueryWorkspace {
    let builder = measurement_runtime_builder();
    let builder = if partial {
        worth_ui_query_binding::install_worth_ui_partial_test_operation_executors(builder)
    } else {
        worth_ui_query_binding::install_worth_ui_test_operation_executors(builder)
    };
    let mut workspace = builder.workspace(label).expect("installed Query workspace");
    insert_measurement(&mut workspace);
    workspace
}

fn measurement_runtime_builder(
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(worth_ui_native_aspect_contracts())
        .expect("native contracts")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect");
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
}

fn insert_measurement(workspace: &mut runtime::WorthQueryWorkspace) {
    workspace
        .insert("WorthUiMeasurement", |measurement| {
            measurement
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
                    runtime::WorthQueryAuthoredAspectValue::native(AspectValue::Float32(
                        CanonicalF32::from_f32(240.0),
                    )),
                )
        })
        .expect("measurement insertion");
}

pub(super) fn observation_basis() -> worth_query::facade::foundation::AdmittedBasisCapability<
    worth_query::facade::foundation::ObservationLaneWitness,
> {
    worth_query::facade::foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .expect("current head should admit observation preparation")
        .admit()
        .expect("observation preparation should admit")
        .capability()
        .clone()
}

pub(super) fn measurement_value_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new([
            FieldKey::new("measurement").expect("aspect path"),
            FieldKey::new("value").expect("field path"),
        ])
        .expect("measurement path"),
    )
}
