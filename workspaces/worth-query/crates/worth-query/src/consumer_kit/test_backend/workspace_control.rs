use std::ops::{Deref, DerefMut};

use crate::runtime::WorthQueryWorkspace;

/// In-memory-test-only control over lifecycle events that production callers
/// can observe but cannot author directly.
pub struct WorthQueryControlledTestWorkspace {
    workspace: WorthQueryWorkspace,
}

impl WorthQueryControlledTestWorkspace {
    pub(super) fn new(workspace: WorthQueryWorkspace) -> Self {
        Self { workspace }
    }

    pub fn from_runtime(
        name: impl Into<String>,
        runtime: crate::runtime::WorthQueryRuntime,
    ) -> Result<Self, crate::runtime::WorthQueryRuntimeError> {
        WorthQueryWorkspace::new(name, runtime).map(Self::new)
    }

    pub fn advance_domain_installation_generation(
        &mut self,
    ) -> Result<(), crate::runtime::WorthQueryRuntimeError> {
        self.workspace
            .replace_domain_installation_with_successor_generation()
    }

    /// Injects exact classified-delivery failures after owner impact has been
    /// proven, for retry transactionality tests.
    pub fn fail_next_classified_live_emissions(&mut self, count: usize) {
        self.workspace
            .inject_classified_live_emission_failures(count);
    }

    /// Test-only hostile control that substitutes owner-installed lowerings
    /// while retaining the recipient Query installation authority.
    pub fn replace_conditional_lowerings_from<D: 'static, O: 'static, F: 'static>(
        &mut self,
        donor: &WorthQueryWorkspace,
    ) -> Result<(), &'static str> {
        self.workspace
            .replace_conditional_lowerings_for_test_from::<D, O, F>(donor)
    }
}

impl Deref for WorthQueryControlledTestWorkspace {
    type Target = WorthQueryWorkspace;

    fn deref(&self) -> &Self::Target {
        &self.workspace
    }
}

impl DerefMut for WorthQueryControlledTestWorkspace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.workspace
    }
}
