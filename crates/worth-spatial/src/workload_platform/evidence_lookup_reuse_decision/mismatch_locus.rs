#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupReuseMismatchLocus {
    SpatialTouchAuthorityDigest,
    StageReceiptDigest,
    EvidenceLedgerBasisDigest,
    TopologySupportDigest,
    QuerySupportDigest,
    EquivalencePolicyIdentity,
    SelectedEquivalenceFamilyIdentity,
    SelectedEquivalenceBasisIdentity,
    SelectedCompatibilityBasisIdentity,
    SelectedReuseBasisIdentity,
}
