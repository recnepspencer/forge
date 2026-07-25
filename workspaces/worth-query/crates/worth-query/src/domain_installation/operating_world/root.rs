use crate::basis_lifecycle::{AdmittedBasisCapability, BasisOperationLane};
use crate::runtime::WorthQueryRuntime;

pub struct WorthQueryInstalledOperatingWorld<'runtime, L: BasisOperationLane> {
    pub(super) runtime: &'runtime WorthQueryRuntime,
    pub(super) basis: AdmittedBasisCapability<L>,
}

impl<'runtime, L: BasisOperationLane> WorthQueryInstalledOperatingWorld<'runtime, L> {
    pub(crate) fn new(
        runtime: &'runtime WorthQueryRuntime,
        basis: AdmittedBasisCapability<L>,
    ) -> Self {
        Self { runtime, basis }
    }

    pub fn family<F>(
        &self,
        _family: F,
    ) -> super::WorthQueryOperationFamilyView<'_, 'runtime, F, L> {
        super::WorthQueryOperationFamilyView::new(self)
    }
}
