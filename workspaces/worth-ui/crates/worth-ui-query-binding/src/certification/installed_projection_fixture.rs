use worth_foundational::{CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::ProjectionFactFieldPath;
use worth_query::facade::runtime::{
    WorthQueryAspectTouch, WorthQueryAuthoredAspectValue, WorthQueryWorkspace,
};

use crate::{
    worth_ui_domain_package, worth_ui_native_aspect_contracts, WorthUiInstalledQueryDomain,
    WorthUiInstalledQueryView, WorthUiQueryBindingPlan, WorthUiQueryProjectionOutcome,
    WorthUiQueryWorkspaceExt,
};

pub fn worth_ui_installed_test_domain(label: &str) -> WorthUiInstalledQueryDomain {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(worth_ui_native_aspect_contracts())
        .expect("Worth UI aspect contracts")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect");
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
        .workspace(label)
        .expect("installed Worth UI Query workspace")
        .worth_ui()
        .expect("Worth UI domain installed")
}

/// Hostile integration fixture owned by the only crate allowed to translate
/// Query execution into Worth UI binding artifacts.
pub struct WorthUiInstalledQueryTestFixture {
    workspace: WorthQueryWorkspace,
    view: WorthUiInstalledQueryView,
}

impl WorthUiInstalledQueryTestFixture {
    pub fn new(label: &str) -> Self {
        let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
            .aspect_contracts(worth_ui_native_aspect_contracts())
            .expect("Worth UI aspect contracts")
            .aspect("identity.id", "identity.id")
            .expect("identity aspect")
            .aspect("measurement.value", "measurement.value")
            .expect("measurement aspect");
        let mut workspace = in_memory_test_runtime()
            .with_schema(schema)
            .domain_package(worth_ui_domain_package())
            .workspace(label)
            .expect("installed Worth UI Query workspace");
        workspace
            .insert("WorthUiMeasurement", |measurement| {
                measurement
                    .set_aspect(
                        WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")
                            .expect("identity touch"),
                        WorthQueryAuthoredAspectValue::string("measurement"),
                    )
                    .set_aspect(
                        WorthQueryAspectTouch::from_authoring_ingress_text("measurement.value")
                            .expect("measurement touch"),
                        WorthQueryAuthoredAspectValue::native(
                            worth_foundational::AspectValue::Float32(
                                worth_foundational::CanonicalF32::from_f32(240.0),
                            ),
                        ),
                    )
            })
            .expect("measurement insertion");
        let installed = workspace.worth_ui().expect("Worth UI domain installed");
        let view = installed
            .measurement_view("inspector.measurements")
            .expect("measurement view");
        Self { workspace, view }
    }

    pub fn binding_plan(&self) -> WorthUiQueryBindingPlan {
        WorthUiQueryBindingPlan::default()
            .register_view(self.view.clone())
            .expect("installed view registration")
    }

    /// Clone the installed view as input to a production registration facade.
    /// The fixture retains its workspace so later projections carry the same
    /// installed Query authority as the registered view.
    pub fn installed_view(&self) -> WorthUiInstalledQueryView {
        self.view.clone()
    }

    pub fn project(&mut self) -> WorthUiQueryProjectionOutcome {
        let completion = self
            .view
            .read()
            .expect("installed measurement read")
            .using(worth_query::facade::domain::current())
            .run(&mut self.workspace)
            .expect("installed authority matches workspace")
            .into_result()
            .expect("measurement read completes");
        self.view
            .project(
                &completion,
                worth_query::facade::domain::project_facts().display_field(
                    ProjectionFactFieldPath::from_canonical_field_path(
                        CanonicalFieldPath::new(vec![
                            FieldKey::new("measurement").expect("aspect path"),
                            FieldKey::new("value").expect("field path"),
                        ])
                        .expect("measurement path"),
                    ),
                ),
            )
            .expect("view projection retains installed authority")
    }
}
