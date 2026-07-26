use worth_runtime_bridge::facade::BridgeAuthoritativeSourceProfile;

use super::RuntimeBridgeRelationalSource;

impl RuntimeBridgeRelationalSource {
    /// Identifies the Relational adapter authority represented by this source.
    ///
    /// The profile is correlation evidence. Snapshot and execution authority
    /// still require owner-minted handles or leases.
    pub fn authoritative_source_profile(&self) -> BridgeAuthoritativeSourceProfile {
        BridgeAuthoritativeSourceProfile::new(
            self.runtime.runtime_instance_id(),
            super::super::identities::relational_bridge_adapter_identity(
                self.runtime.runtime_instance_id(),
            ),
        )
        .expect("Relational runtime authority always yields a valid Bridge source profile")
    }
}
