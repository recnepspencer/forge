use worth_relational::facade::snapshots::SnapshotHandle;
use worth_relational::facade::{
    history::BranchId,
    runtime::{RelationalExecutionBasisDenial, RelationalExecutionBasisLease},
};

use super::super::WorthQueryPrimaryGraphIntegrationHandle;

#[derive(Debug)]
pub(in crate::domain_computation) enum WorthQueryApplicationSnapshotLeaseAdmissionDenial {
    Indexes(&'static str),
    Basis(RelationalExecutionBasisDenial),
}

impl WorthQueryApplicationSnapshotLeaseAdmissionDenial {
    pub(in crate::domain_computation) fn detail(&self) -> &str {
        match self {
            Self::Indexes(detail) => detail,
            Self::Basis(denial) => denial.detail(),
        }
    }
}

pub(in crate::domain_computation) struct WorthQueryApplicationSnapshotLease {
    handle: WorthQueryPrimaryGraphIntegrationHandle,
    basis: Option<RelationalExecutionBasisLease>,
    pub(super) layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
}

impl WorthQueryApplicationSnapshotLease {
    pub(in crate::domain_computation) fn acquire(
        handle: WorthQueryPrimaryGraphIntegrationHandle,
        layout: std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
        branch: &BranchId,
    ) -> Result<Self, WorthQueryApplicationSnapshotLeaseAdmissionDenial> {
        let version = handle
            .with_runtime_mut(|runtime| handle.ensure_primary_indexes_current(runtime, branch))
            .map_err(WorthQueryApplicationSnapshotLeaseAdmissionDenial::Indexes)?;
        let basis = handle
            .with_runtime_mut(|runtime| runtime.snapshots().admit_execution_basis(branch, version))
            .map_err(WorthQueryApplicationSnapshotLeaseAdmissionDenial::Basis)?;
        Ok(Self {
            handle,
            basis: Some(basis),
            layout,
        })
    }

    pub(in crate::domain_computation) fn snapshot(&self) -> &SnapshotHandle {
        self.basis
            .as_ref()
            .expect("application snapshot lease remains live until consumed")
            .snapshot_handle()
    }

    pub(in crate::domain_computation) fn handle(&self) -> &WorthQueryPrimaryGraphIntegrationHandle {
        &self.handle
    }

    pub(in crate::domain_computation) fn layout(
        &self,
    ) -> &std::sync::Arc<super::super::schema_layout::WorthQueryPrimaryGraphLayout> {
        &self.layout
    }

    pub(in crate::domain_computation) fn take_relational_basis(
        &mut self,
    ) -> Option<RelationalExecutionBasisLease> {
        self.basis.take()
    }

    pub(in crate::domain_computation) fn restore_relational_basis(
        &mut self,
        basis: RelationalExecutionBasisLease,
    ) -> Result<(), RelationalExecutionBasisLease> {
        if self.basis.is_some() {
            Err(basis)
        } else {
            self.basis = Some(basis);
            Ok(())
        }
    }

    pub(in crate::domain_computation) fn release(mut self) -> bool {
        self.basis
            .take()
            .is_some_and(|basis| basis.release().released())
    }
}
