use worth_query::facade::{domain, runtime};

use super::{
    configure_product_projection_backend, evaluate_product_projection_support,
    platform_pulse_bridge, shared_source_state, SharedSourceState,
};

pub struct WorthUiQueryHostInstallationRequest {
    inner: runtime::WorthQueryHostRuntimeInstallationRequest,
}

impl WorthUiQueryHostInstallationRequest {
    pub fn generation(&self) -> runtime::WorthQueryInstallationGeneration {
        self.inner.generation()
    }

    pub fn into_packages(self) -> Vec<runtime::WorthQueryAdmittedPortableDomainPackage> {
        self.inner.into_packages()
    }
}

pub struct WorthUiScalarProjectionHostCompletion {
    inner: runtime::WorthQueryHostRuntimeInstallationCompletion,
    bridge: worth_runtime_bridge::facade::RuntimeBridge,
    source: SharedSourceState,
}

pub struct WorthUiScalarProjectionHostPlan {
    request: WorthUiQueryHostInstallationRequest,
    completion: WorthUiScalarProjectionHostCompletion,
}

impl WorthUiScalarProjectionHostPlan {
    #[allow(
        clippy::result_large_err,
        reason = "cold host installation preserves exact Query failure topology"
    )]
    pub fn prepare() -> Result<Self, WorthUiScalarProjectionInstallationError> {
        let source = shared_source_state();
        let bridge =
            platform_pulse_bridge().map_err(WorthUiScalarProjectionInstallationError::Bridge)?;
        let builder = projection_runtime_builder(source.clone(), bridge.clone())?;
        let plan = builder.prepare_host_installation();
        let (request, completion) = plan.into_parts();
        Ok(Self {
            request: WorthUiQueryHostInstallationRequest { inner: request },
            completion: WorthUiScalarProjectionHostCompletion {
                inner: completion,
                bridge,
                source,
            },
        })
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthUiQueryHostInstallationRequest,
        WorthUiScalarProjectionHostCompletion,
    ) {
        (self.request, self.completion)
    }
}

impl WorthUiScalarProjectionHostCompletion {
    #[allow(
        clippy::result_large_err,
        reason = "cold host completion preserves exact Query failure topology"
    )]
    pub fn complete(
        self,
        installation: runtime::WorthQueryExecutionRuntimeInstallation,
    ) -> Result<super::WorthUiScalarProjectionInstallation, WorthUiScalarProjectionInstallationError>
    {
        let runtime = self
            .inner
            .complete(installation)
            .map_err(WorthUiScalarProjectionInstallationError::RuntimeCompletion)?;
        let workspace = runtime
            .workspace("worth-ui-platform-pulse")
            .map_err(WorthUiScalarProjectionInstallationError::Runtime)?;
        evaluate_product_projection_support(&workspace).map_err(|error| {
            WorthUiScalarProjectionInstallationError::SourceLifecycle(format!(
                "Query support pin denied product projection installation: {error}"
            ))
        })?;
        super::WorthUiScalarProjectionInstallation::open(workspace, self.bridge, self.source)
    }
}

#[derive(Debug)]
pub enum WorthUiScalarProjectionInstallationError {
    AspectContract(runtime::WorthQueryAspectContractRegistrationDenial),
    Bridge(String),
    DomainPackage(domain::WorthQueryDomainPackageInstallationError),
    Runtime(runtime::WorthQueryRuntimeError),
    RuntimeCompletion(runtime::WorthQueryHostRuntimeCompletionError),
    SourceLifecycle(String),
}

#[allow(
    clippy::result_large_err,
    reason = "cold runtime construction preserves exact Query failure topology"
)]
pub(crate) fn projection_runtime_builder(
    source: SharedSourceState,
    bridge: worth_runtime_bridge::facade::RuntimeBridge,
) -> Result<runtime::WorthQueryRuntimeBuilder, WorthUiScalarProjectionInstallationError> {
    let builder = runtime::WorthQueryRuntime::builder()
        .domain_package(crate::worth_ui_domain_package())
        .map_err(WorthUiScalarProjectionInstallationError::DomainPackage)?;
    let builder = crate::install_worth_ui_operation_executors(builder)
        .aspect_contracts(crate::worth_ui_native_aspect_contracts())
        .map_err(WorthUiScalarProjectionInstallationError::AspectContract)?;
    Ok(projection_consumer_support(
        configure_product_projection_backend(builder, bridge, source),
    ))
}

fn projection_consumer_support(
    builder: runtime::WorthQueryRuntimeBuilder,
) -> runtime::WorthQueryRuntimeBuilder {
    use domain::{
        WorthQueryConsumerSupportDimension as Dimension,
        WorthQueryConsumerSupportPosture as Posture,
    };

    [
        Dimension::Live,
        Dimension::ProjectionConsumption,
        Dimension::Sharing,
        Dimension::Invalidation,
        Dimension::AsyncResultState,
        Dimension::Recovery,
    ]
    .into_iter()
    .fold(builder, |builder, dimension| {
        builder.consumer_support_posture(dimension, Posture::Supported)
    })
}
