use worth_query::facade::{read, runtime};

use crate::{
    WorthUiInstalledQueryDomain, WorthUiInstalledQueryView, WorthUiQueryExt,
    WorthUiQueryLiveOpenError, WorthUiQueryLiveOpenOutcome, WorthUiQueryViewDefinition,
    WorthUiQueryViewLifecycle,
};

/// Installed live view. Query-owned managed-resource operations are added on
/// this lifecycle type rather than on the registration envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledLiveQueryView {
    registration: WorthUiInstalledQueryView,
}

impl WorthUiInstalledLiveQueryView {
    pub(super) fn from_registration(registration: WorthUiInstalledQueryView) -> Self {
        debug_assert_eq!(
            registration.definition().lifecycle(),
            WorthUiQueryViewLifecycle::Live
        );
        Self { registration }
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        self.registration.definition()
    }

    pub fn installed_domain(&self) -> &WorthUiInstalledQueryDomain {
        self.registration.installed_domain()
    }

    pub fn open_using(
        &self,
        context: impl Into<read::WorthQueryReadContextDeclaration>,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<WorthUiQueryLiveOpenOutcome, WorthUiQueryLiveOpenError> {
        let declaration = self
            .installed_domain()
            .handle()
            .live_measurements(query_live_resource_name(
                self.installed_domain(),
                self.definition().identity(),
            ))
            .map_err(WorthUiQueryLiveOpenError::Declaration)?;
        declaration
            .using(context)
            .open(workspace)
            .map(|outcome| {
                WorthUiQueryLiveOpenOutcome::from_query(self.definition().clone(), outcome)
            })
            .map_err(Box::new)
            .map_err(WorthUiQueryLiveOpenError::InstalledAuthority)
    }
}

pub(super) fn query_live_resource_name(
    installed_domain: &WorthUiInstalledQueryDomain,
    identity: &crate::WorthUiQueryViewIdentity,
) -> String {
    format!(
        "worth-ui.view.{}.{}",
        installed_domain.handle().installation_identity(),
        identity.as_str()
    )
}

impl From<WorthUiInstalledLiveQueryView> for WorthUiInstalledQueryView {
    fn from(view: WorthUiInstalledLiveQueryView) -> Self {
        view.registration
    }
}

#[cfg(test)]
mod tests {
    use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

    use crate::{worth_ui_domain_package, WorthUiQueryWorkspaceExt};

    #[test]
    fn live_resource_names_distinguish_equal_views_from_foreign_installations() {
        let first = installed_live_view("live-resource-name-first");
        let second = installed_live_view("live-resource-name-second");

        assert_ne!(
            super::query_live_resource_name(
                first.installed_domain(),
                first.definition().identity()
            ),
            super::query_live_resource_name(
                second.installed_domain(),
                second.definition().identity()
            )
        );
    }

    fn installed_live_view(name: &str) -> super::WorthUiInstalledLiveQueryView {
        let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
            .aspect_contracts(crate::worth_ui_native_aspect_contracts())
            .expect("native aspect contracts")
            .aspect("identity.id", "identity.id")
            .expect("identity aspect")
            .aspect("measurement.value", "measurement.value")
            .expect("measurement aspect");
        let workspace = in_memory_test_runtime()
            .with_schema(schema)
            .domain_package(worth_ui_domain_package())
            .workspace(name)
            .expect("installed Query workspace");
        workspace
            .worth_ui()
            .expect("Worth UI domain installed")
            .live_measurement_view("inspector.measurements")
            .expect("live measurement view")
    }
}
