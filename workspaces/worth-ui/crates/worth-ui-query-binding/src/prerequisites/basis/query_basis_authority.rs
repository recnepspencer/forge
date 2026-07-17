use super::WorthUiQueryAuthorityHandle;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryBasisIdentity(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBasisAuthority {
    query_authority: WorthUiQueryAuthorityHandle,
    identity: WorthUiQueryBasisIdentity,
}

impl WorthUiQueryBasisAuthority {
    pub(crate) fn from_execution(
        query_authority: WorthUiQueryAuthorityHandle,
        basis_identity: &worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    ) -> Self {
        let bytes = basis_identity.canonical_digest().value().bytes();
        let identity = bytes
            .iter()
            .take(8)
            .enumerate()
            .fold(0_u64, |value, (index, byte)| {
                value | (u64::from(*byte) << (index * 8))
            });
        Self {
            query_authority,
            identity: WorthUiQueryBasisIdentity(identity),
        }
    }

    pub fn identity(&self) -> WorthUiQueryBasisIdentity {
        self.identity
    }

    pub fn query_authority(&self) -> &WorthUiQueryAuthorityHandle {
        &self.query_authority
    }

    pub fn shares_authority_with(&self, other: &Self) -> bool {
        self.identity == other.identity
            && self
                .query_authority
                .shares_authority_with(&other.query_authority)
    }
}

impl WorthUiQueryBasisIdentity {
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}
