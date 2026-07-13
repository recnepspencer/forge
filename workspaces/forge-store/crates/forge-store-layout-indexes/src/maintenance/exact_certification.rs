use super::{ExactMaintenanceProtocol, LayoutMaintenanceFacade, LiveExactMaintenanceWitness};

impl LayoutMaintenanceFacade {
    pub fn certify_live_exact(
        &self,
        lowered: &ExactMaintenanceProtocol,
    ) -> Option<LiveExactMaintenanceWitness> {
        let plan = lowered.plan();
        let coverage = plan.exact_coverage()?;
        let publication_authority = plan.exact_publication_authority()?;
        if !plan.maintenance_mode().permits_exact_answers()
            || plan.lag_witness().is_some()
            || !publication_authority.supports_exact_coverage(coverage)
        {
            return None;
        }

        Some(LiveExactMaintenanceWitness::new(
            plan.admitted_strategy().lifecycle().declaration().family(),
            coverage.clone(),
            plan.maintenance_mode(),
            publication_authority,
        ))
    }
}
