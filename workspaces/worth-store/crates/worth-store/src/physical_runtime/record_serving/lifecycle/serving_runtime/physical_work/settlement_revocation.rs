impl crate::physical_runtime::instance::PhysicalStoreWorkRuntime {
    pub(in crate::physical_runtime) fn consume_settlement_revocation(
        &self,
        settled: &crate::physical_runtime::SettledPhysicalWork,
        revocation: Option<crate::physical_runtime::PhysicalWorkHealthRevocation>,
    ) {
        let Some(revocation) = revocation else {
            return;
        };
        if self.signal.apply_projection_failure_delta(settled).is_err() {
            self.signal.revoke_derived_admission();
        }
        self.health.consume_physical_revocation(revocation);
    }
}
