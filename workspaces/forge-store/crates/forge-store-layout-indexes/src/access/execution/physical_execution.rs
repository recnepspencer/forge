use super::{AccessLoweringFacade, DegradedScanReady, PhysicalDegradedExecutionDenial};
use forge_store_physical_format::PlatformPhysicalFacade;

impl AccessLoweringFacade {
    pub fn execute_physical_degraded_exact_scan(
        &self,
        ready: DegradedScanReady,
        physical: &mut PlatformPhysicalFacade,
    ) -> Result<super::DegradedScanExecution, PhysicalDegradedExecutionDenial> {
        let selected = ready.selected();
        let expected = selected.admitted_family().authority_identity();
        let actual = physical.store_identity().authority_identity();
        if actual != expected {
            return Err(PhysicalDegradedExecutionDenial::StoreAuthorityMismatch {
                expected,
                actual,
            });
        }
        let contract = physical
            .admit_degraded_exact_scan(
                selected.intent().budget_rows().unwrap_or(0),
                selected.budget_receipt(),
            )
            .map_err(PhysicalDegradedExecutionDenial::Admission)?;
        let observation = physical
            .execute_admitted_degraded_exact_scan(contract)
            .map_err(PhysicalDegradedExecutionDenial::Physical)?;
        Ok(super::degraded_scan::executed(ready, observation))
    }
}
