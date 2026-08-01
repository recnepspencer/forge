use super::{WorthQueryExecutionRuntime, WorthQueryRuntimeAuthorityIdentity};

/// Opaque authority retained by the composition root that installed one
/// execution runtime.
///
/// Runtime identities are descriptive. Possession of this value is what
/// authorizes installed provider closure and provider-call binding.
pub struct WorthQueryExecutionInstallationAuthority {
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    installation_runtime: worth_query_installation::facade::WorthQueryInstallationRuntimeIdentity,
    graph_admission_authority:
        worth_query_installation::facade::WorthQueryInstalledGraphAdmissionAuthority,
}

impl WorthQueryExecutionInstallationAuthority {
    pub(crate) fn new(
        runtime_authority: WorthQueryRuntimeAuthorityIdentity,
        installation_runtime: worth_query_installation::facade::WorthQueryInstallationRuntimeIdentity,
        graph_admission_authority: worth_query_installation::facade::WorthQueryInstalledGraphAdmissionAuthority,
    ) -> Self {
        Self {
            runtime_authority,
            installation_runtime,
            graph_admission_authority,
        }
    }

    pub(crate) fn belongs_to(&self, runtime: &WorthQueryExecutionRuntime) -> bool {
        self.runtime_authority == runtime.authority_identity()
    }

    pub(crate) fn installation_runtime(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryInstallationRuntimeIdentity {
        &self.installation_runtime
    }

    pub(crate) fn into_graph_admission_authority(
        self,
    ) -> worth_query_installation::facade::WorthQueryInstalledGraphAdmissionAuthority {
        self.graph_admission_authority
    }
}

/// Compiler-total result of installing one execution runtime.
pub struct WorthQueryExecutionRuntimeInstallation {
    runtime: WorthQueryExecutionRuntime,
    authority: WorthQueryExecutionInstallationAuthority,
}

impl WorthQueryExecutionRuntimeInstallation {
    pub(crate) fn new(
        runtime: WorthQueryExecutionRuntime,
        installation_runtime: worth_query_installation::facade::WorthQueryInstallationRuntimeIdentity,
        graph_admission_authority: worth_query_installation::facade::WorthQueryInstalledGraphAdmissionAuthority,
    ) -> Self {
        let authority = WorthQueryExecutionInstallationAuthority::new(
            runtime.authority_identity(),
            installation_runtime,
            graph_admission_authority,
        );
        Self { runtime, authority }
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryExecutionRuntime,
        WorthQueryExecutionInstallationAuthority,
    ) {
        (self.runtime, self.authority)
    }
}
