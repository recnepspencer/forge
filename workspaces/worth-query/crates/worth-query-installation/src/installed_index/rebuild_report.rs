#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryInstalledPackageIndexCounters {
    pub package_rows_examined: usize,
    pub definition_rows_examined: usize,
    pub domain_operation_rows_examined: usize,
    pub artifact_contract_rows_examined: usize,
    pub application_schema_rows_examined: usize,
    pub conditional_application_operation_rows_examined: usize,
    pub equivalent_packages_converged: usize,
    pub installed_package_count: usize,
    pub installed_definition_count: usize,
    pub installed_domain_operation_count: usize,
    pub installed_artifact_contract_count: usize,
    pub installed_application_schema_count: usize,
    pub installed_conditional_application_operation_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledPackageIndexRebuildReport {
    prior_identity: WorthQueryInstalledPackageIndexIdentity,
    rebuilt_identity: WorthQueryInstalledPackageIndexIdentity,
    counters: WorthQueryInstalledPackageIndexCounters,
}

impl WorthQueryInstalledPackageIndexRebuildReport {
    pub(crate) fn new(
        prior_identity: WorthQueryInstalledPackageIndexIdentity,
        rebuilt_identity: WorthQueryInstalledPackageIndexIdentity,
        counters: WorthQueryInstalledPackageIndexCounters,
    ) -> Self {
        Self {
            prior_identity,
            rebuilt_identity,
            counters,
        }
    }

    pub fn prior_identity(&self) -> &WorthQueryInstalledPackageIndexIdentity {
        &self.prior_identity
    }

    pub fn rebuilt_identity(&self) -> &WorthQueryInstalledPackageIndexIdentity {
        &self.rebuilt_identity
    }

    pub fn counters(&self) -> WorthQueryInstalledPackageIndexCounters {
        self.counters
    }
}
use super::WorthQueryInstalledPackageIndexIdentity;
