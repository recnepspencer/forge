use super::UiNativeTextAtlasGpuPages;
use crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis;
use crate::native::text_atlas::UiNativeTextAtlasDenial;

#[derive(Clone, Copy)]
pub(super) struct PendingAtlasTransactionCorrelation {
    pub(super) transaction: u64,
    pub(super) basis: UiNativePhysicalSignalExternalBasis,
}

impl UiNativeTextAtlasGpuPages {
    pub(crate) fn bind_transaction_correlation(
        &mut self,
        transaction: u64,
        basis: UiNativePhysicalSignalExternalBasis,
    ) -> Result<(), UiNativeTextAtlasDenial> {
        if self
            .correlations
            .iter()
            .any(|correlation| correlation.transaction == transaction)
        {
            return Err(UiNativeTextAtlasDenial::ReservationConflict);
        }
        self.correlations
            .push(PendingAtlasTransactionCorrelation { transaction, basis });
        Ok(())
    }

    pub(crate) fn release_transaction_correlation(&mut self, transaction: u64) {
        self.correlations
            .retain(|correlation| correlation.transaction != transaction);
    }

    pub(crate) fn rebind_transaction_correlation(
        &mut self,
        transaction: u64,
        basis: UiNativePhysicalSignalExternalBasis,
    ) -> bool {
        let Some(correlation) = self
            .correlations
            .iter_mut()
            .find(|correlation| correlation.transaction == transaction)
        else {
            return false;
        };
        correlation.basis = basis;
        true
    }

    #[cfg(test)]
    pub(crate) fn transaction_correlation_basis(
        &self,
        transaction: u64,
    ) -> Option<UiNativePhysicalSignalExternalBasis> {
        self.correlations
            .iter()
            .find(|correlation| correlation.transaction == transaction)
            .map(|correlation| correlation.basis)
    }
}
