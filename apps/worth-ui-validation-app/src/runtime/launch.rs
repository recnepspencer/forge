use worth_ui::facade::{
    WorthUi, WorthUiApp, WorthUiLayoutTopologyCatalog, WorthUiRuntimeHost,
    WorthUiRuntimeLaunchDenial, WorthUiRuntimeLaunchPreparationDenial,
};

use crate::honesty::ValidationAppPublicFacadeLaunch;
use crate::runtime::ValidationWorkbenchSnapshot;
use crate::runtime::{
    ValidationLayoutMeasurementCatalog, ValidationLayoutMeasurementCatalogDenial,
};
use crate::sample::{ValidationAuthoringSample, VALIDATION_AUTHORING_SAMPLE};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValidationWorkbenchLaunch;

pub struct PreparedValidationWorkbenchLaunch {
    app: WorthUiApp,
    runtime: WorthUiRuntimeHost,
    layout_topology: WorthUiLayoutTopologyCatalog,
    layout_measurements: ValidationLayoutMeasurementCatalog,
    sample: ValidationAuthoringSample,
}

#[derive(Debug)]
pub enum ValidationWorkbenchLaunchError {
    LayoutMeasurements {
        page_name: String,
        token_name: String,
    },
    RuntimePreparation(WorthUiRuntimeLaunchPreparationDenial),
    RuntimeLaunch(WorthUiRuntimeLaunchDenial),
}

impl ValidationWorkbenchLaunch {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare(
        self,
    ) -> Result<PreparedValidationWorkbenchLaunch, ValidationWorkbenchLaunchError> {
        let app = WorthUi::app().freeze();
        let prepared_authoring = ValidationAppPublicFacadeLaunch::DEFAULT
            .prepare_authoring_for(&app)
            .map_err(ValidationWorkbenchLaunchError::RuntimePreparation)?;
        let (runtime_launch, layout_topology) = prepared_authoring.into_parts();
        let layout_measurements = ValidationLayoutMeasurementCatalog::shopify_admin_defaults();
        layout_measurements
            .validate_topology(&layout_topology)
            .map_err(map_layout_measurement_denial)?;
        let runtime = app
            .launch_runtime(runtime_launch)
            .map_err(ValidationWorkbenchLaunchError::RuntimeLaunch)?;
        Ok(PreparedValidationWorkbenchLaunch {
            app,
            runtime,
            layout_topology,
            layout_measurements,
            sample: VALIDATION_AUTHORING_SAMPLE,
        })
    }
}

fn map_layout_measurement_denial(
    denial: ValidationLayoutMeasurementCatalogDenial,
) -> ValidationWorkbenchLaunchError {
    match denial {
        ValidationLayoutMeasurementCatalogDenial::MissingNamedToken {
            page_name,
            token_name,
        } => ValidationWorkbenchLaunchError::LayoutMeasurements {
            page_name,
            token_name,
        },
    }
}

impl PreparedValidationWorkbenchLaunch {
    pub fn runtime(&self) -> &WorthUiRuntimeHost {
        &self.runtime
    }

    pub fn app(&self) -> &WorthUiApp {
        &self.app
    }

    pub(crate) fn layout_topology(&self) -> &WorthUiLayoutTopologyCatalog {
        &self.layout_topology
    }

    pub(crate) fn layout_measurements(&self) -> &ValidationLayoutMeasurementCatalog {
        &self.layout_measurements
    }

    pub fn sample(&self) -> ValidationAuthoringSample {
        self.sample
    }

    pub fn snapshot(&self) -> ValidationWorkbenchSnapshot {
        ValidationWorkbenchSnapshot::from_launch(self.sample, self.runtime.inspect_active())
    }
}
