use crate::capability::CapabilitySnapshot;
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiIdentityStateCertification, WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason,
    WorthUiIdentityStateQueryCertificationScenario,
};

impl WorthUiRuntime {
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
        WorthUiIdentityStateCertification::certify(scenario, active)
    }
}
