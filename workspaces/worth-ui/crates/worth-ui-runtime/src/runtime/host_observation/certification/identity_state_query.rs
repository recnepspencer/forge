use crate::capability::CapabilitySnapshot;
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiIdentityStateCertification, WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason,
    WorthUiIdentityStateQueryCertificationScenario, WorthUiStateQueryResidueScan,
};

impl WorthUiRuntime {
    pub fn inspect_query_state_residue(&self) -> WorthUiStateQueryResidueScan {
        self.query_state_residue_scan(0)
    }

    pub fn certify_identity_state_and_query_drift_against_snapshot(
        &self,
        scenario: WorthUiIdentityStateQueryCertificationScenario,
        snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiIdentityStateCertification, WorthUiIdentityStateQueryCertificationDenial>
    {
        let active = self.inspect_active();
        if active.snapshot_digest() != snapshot.digest().as_u64() {
            return Err(WorthUiIdentityStateQueryCertificationDenial::new(
                WorthUiIdentityStateQueryCertificationDenialReason::SnapshotDigestMismatch {
                    active_snapshot_digest: active.snapshot_digest(),
                    provided_snapshot_digest: snapshot.digest().as_u64(),
                },
                Default::default(),
            ));
        }
        let residue_scan = self.query_state_residue_scan(scenario.state_steps().len());
        WorthUiIdentityStateCertification::certify(scenario, active, residue_scan)
    }

    fn query_state_residue_scan(
        &self,
        scanned_state_receipts: usize,
    ) -> WorthUiStateQueryResidueScan {
        let active = self.inspect_active();
        let plan = self.active.active_plan_ref();
        WorthUiStateQueryResidueScan::from_active_runtime(
            scanned_state_receipts,
            self.query_binding.state_observation(),
            plan.query_plan_state_observation(&self.query_binding),
            active.generation_identity() == plan.generation_identity(),
        )
    }
}
