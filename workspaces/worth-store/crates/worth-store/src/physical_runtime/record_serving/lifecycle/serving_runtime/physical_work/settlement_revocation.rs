impl crate::physical_runtime::instance::PhysicalStoreWorkRuntime {
    pub(in crate::physical_runtime) fn consume_settlement_revocation(
        &self,
        settled: &crate::physical_runtime::SettledPhysicalWork,
        revocation: Option<crate::physical_runtime::PhysicalWorkHealthRevocation>,
    ) {
        if projection_failure(settled, revocation.is_some())
            && self.signal.apply_projection_failure_delta(settled).is_err()
        {
            self.signal.revoke_derived_admission();
            self.health.revoke();
        }
        let Some(revocation) = revocation else {
            return;
        };
        self.health.consume_physical_revocation(revocation);
    }
}

fn projection_failure(
    settled: &crate::physical_runtime::SettledPhysicalWork,
    health_revoked: bool,
) -> bool {
    health_revoked
        || (settled
            .intent()
            .semantic_basis()
            .projection_fact()
            .is_some()
            && matches!(
                settled.evidence(),
                crate::physical_runtime::PhysicalWorkSettlementEvidence::NoEffect(evidence)
                    if matches!(
                        evidence.failure().kind(),
                        worth_store_physical_backend::ArtifactTreeFailureKind::Absent
                            | worth_store_physical_backend::ArtifactTreeFailureKind::Damaged
                    )
            ))
}
