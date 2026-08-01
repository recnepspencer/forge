/// Opaque WORTH UI reference to one exact Query-owned evidence identity.
///
/// The reference supports equality, ordering, and indexing without exposing
/// Query's terminal text projection or any construction authority.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiQueryEvidenceReference {
    key: worth_query::facade::runtime::WorthQueryEvidenceIdentityKey,
}

impl UiQueryEvidenceReference {
    pub(crate) fn query_issued(
        identity: &worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            key: identity.operational_key(),
        }
    }

    /// Opaque correlation material for the pure mounted host contract.
    #[doc(hidden)]
    pub const fn host_correlation_digest(self) -> [u8; 32] {
        self.key.correlation_digest()
    }
}

impl std::fmt::Debug for UiQueryEvidenceReference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiQueryEvidenceReference")
            .field("authority", &"sealed Query evidence")
            .finish()
    }
}
