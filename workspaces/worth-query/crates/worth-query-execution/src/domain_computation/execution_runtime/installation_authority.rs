use super::{WorthQueryExecutionRuntime, WorthQueryRuntimeAuthorityIdentity};

/// Opaque authority retained by the composition root that installed one
/// execution runtime.
///
/// Runtime identities are descriptive. Possession of this value is what
/// authorizes installed provider closure and provider-call binding.
pub struct WorthQueryExecutionInstallationAuthority {
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
}

impl WorthQueryExecutionInstallationAuthority {
    pub(crate) fn new(runtime_authority: WorthQueryRuntimeAuthorityIdentity) -> Self {
        Self { runtime_authority }
    }

    pub(crate) fn belongs_to(&self, runtime: &WorthQueryExecutionRuntime) -> bool {
        self.runtime_authority == runtime.authority_identity()
    }
}

/// Compiler-total result of installing one execution runtime.
pub struct WorthQueryExecutionRuntimeInstallation {
    runtime: WorthQueryExecutionRuntime,
    authority: WorthQueryExecutionInstallationAuthority,
}

impl WorthQueryExecutionRuntimeInstallation {
    pub(crate) fn new(runtime: WorthQueryExecutionRuntime) -> Self {
        let authority = WorthQueryExecutionInstallationAuthority::new(runtime.authority_identity());
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
