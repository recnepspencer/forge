use super::*;

impl RuntimeBridge {
    /// Identifies the authoritative adapter installed into this Bridge runtime.
    ///
    /// This is correlation evidence, not execution authority. Consumers must
    /// still obtain an owner-minted execution basis before doing managed work.
    pub fn authoritative_source_profile(
        &self,
    ) -> Option<&crate::input::envelope::BridgeAuthoritativeSourceProfile> {
        self.authoritative_source_profile.as_ref()
    }
}
