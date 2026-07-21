use worth_query::facade::{domain, foundation, runtime};

use crate::{
    application_binding::WorthUiInstalledSnapshotOperationReference,
    WorthUiInstalledQueryBindingReference, WorthUiQueryInstallationDenial,
    WorthUiQueryWorkspaceExt, WorthUiSnapshotMeasurement, WorthUiSnapshotMeasurementFamily,
};

pub type WorthUiBoundSnapshotMeasurement<L> = domain::WorthQueryBoundDomainOperation<
    crate::WorthUiDomainEntry,
    WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
    L,
>;

#[derive(Debug)]
pub enum WorthUiQueryOperationAttemptDenial {
    Installation(WorthUiQueryInstallationDenial),
    InstalledDomainAuthorityMismatch,
    SnapshotOperationRequired,
}

/// One attempt-scoped entry into Query's installed operating world.
///
/// Construction verifies that the application-owned reference and the
/// workspace resolve the same exact installed-domain authority. Consuming the
/// gateway binds one operation; neither the world nor the move-only bound
/// operation can be retained in an application plan or steady frame.
pub struct WorthUiQueryOperatingWorldGateway<'runtime, L: foundation::BasisOperationLane> {
    world: domain::WorthQueryInstalledOperatingWorld<'runtime, L>,
    pub(super) reference: WorthUiInstalledQueryBindingReference,
    operation: WorthUiInstalledSnapshotOperationReference,
}

impl WorthUiInstalledQueryBindingReference {
    pub fn enter_snapshot_attempt<'runtime, L: foundation::BasisOperationLane>(
        &self,
        workspace: &'runtime runtime::WorthQueryWorkspace,
        basis: foundation::AdmittedBasisCapability<L>,
    ) -> Result<WorthUiQueryOperatingWorldGateway<'runtime, L>, WorthUiQueryOperationAttemptDenial>
    {
        let operation = self
            .snapshot_operation()
            .ok_or(WorthUiQueryOperationAttemptDenial::SnapshotOperationRequired)?;
        let current = workspace
            .worth_ui()
            .map_err(WorthUiQueryOperationAttemptDenial::Installation)?;
        if !current.shares_authority_with(self.installed_domain()) {
            return Err(WorthUiQueryOperationAttemptDenial::InstalledDomainAuthorityMismatch);
        }
        Ok(WorthUiQueryOperatingWorldGateway {
            world: workspace.operating_world(basis),
            reference: self.clone(),
            operation,
        })
    }
}

impl<L: foundation::BasisOperationLane> WorthUiQueryOperatingWorldGateway<'_, L> {
    pub fn bind_snapshot(
        self,
    ) -> Result<WorthUiBoundSnapshotMeasurement<L>, domain::WorthQueryOperationBindingDenial> {
        self.world.family(self.operation.family).bind(
            self.reference.installed_domain().handle(),
            self.operation.operation,
        )
    }
}
