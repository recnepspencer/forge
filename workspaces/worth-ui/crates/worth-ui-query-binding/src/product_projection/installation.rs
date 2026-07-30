use worth_query::facade::{domain, runtime};

use super::{
    platform_pulse_bridge, shared_source_state, SharedSourceState,
    WorthUiExternalScalarSourceBackend,
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
    pub fn prepare() -> Result<Self, WorthUiScalarProjectionInstallationError> {
        let source = shared_source_state();
        let bridge = platform_pulse_bridge()
            .map_err(WorthUiScalarProjectionInstallationError::Bridge)?;
        let builder = projection_runtime_builder(source.clone())?;
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

fn projection_runtime_builder(
    source: SharedSourceState,
) -> Result<runtime::WorthQueryRuntimeBuilder, WorthUiScalarProjectionInstallationError> {
    let builder = runtime::WorthQueryRuntime::builder()
        .backend(WorthUiExternalScalarSourceBackend::new(source))
        .domain_package(crate::worth_ui_domain_package())
        .map_err(WorthUiScalarProjectionInstallationError::DomainPackage)?;
    let builder = crate::install_worth_ui_operation_executors(builder)
        .aspect_contracts(crate::worth_ui_native_aspect_contracts())
        .map_err(WorthUiScalarProjectionInstallationError::AspectContract)?;
    Ok(projection_consumer_support(builder))
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
