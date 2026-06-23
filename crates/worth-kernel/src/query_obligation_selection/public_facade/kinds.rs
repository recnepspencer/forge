use super::super::selection_substrate::{
    QueryObligationSelectionAuthorityKind, QueryObligationSelectionErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryGraphObligationSelectionAuthorityKind {
    TopologyTouchedBasis,
    SpatialQueryDescriptor,
}

impl From<QueryObligationSelectionAuthorityKind> for QueryGraphObligationSelectionAuthorityKind {
    fn from(kind: QueryObligationSelectionAuthorityKind) -> Self {
        match kind {
            QueryObligationSelectionAuthorityKind::TopologyTouchedBasis => {
                Self::TopologyTouchedBasis
            }
            QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor => {
                Self::SpatialQueryDescriptor
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryGraphObligationSelectionFacadeErrorKind {
    MissingAuthorityDigest,
    MissingSpatialDescriptorAuthority,
    QueryConsumerKit,
    SpatialConsumerKit,
    EmptyExecutionProof,
    CopiedSelectionPartsDenied,
    LocalSelectorAuthorityDenied,
    BroadCollectionSelectorAuthorityDenied,
    LifecycleOnlySelectorAuthorityDenied,
    LocalSupportRowAuthorityDenied,
    InMemorySelectionAuthorityDenied,
    RawDescriptorAuthorityDenied,
    TopologySpatialSubstitutionAuthorityDenied,
    SourceGrepAuditAuthorityDenied,
    WorkloadAuthorityMismatch,
}

impl From<QueryObligationSelectionErrorKind> for QueryGraphObligationSelectionFacadeErrorKind {
    fn from(kind: QueryObligationSelectionErrorKind) -> Self {
        match kind {
            QueryObligationSelectionErrorKind::MissingAuthorityDigest => {
                Self::MissingAuthorityDigest
            }
            QueryObligationSelectionErrorKind::MissingSpatialDescriptorAuthority => {
                Self::MissingSpatialDescriptorAuthority
            }
            QueryObligationSelectionErrorKind::QueryConsumerKit => Self::QueryConsumerKit,
            QueryObligationSelectionErrorKind::SpatialConsumerKit => Self::SpatialConsumerKit,
            QueryObligationSelectionErrorKind::EmptyExecutionProof => Self::EmptyExecutionProof,
            QueryObligationSelectionErrorKind::CopiedSelectionPartsDenied => {
                Self::CopiedSelectionPartsDenied
            }
            QueryObligationSelectionErrorKind::LocalSelectorAuthorityDenied => {
                Self::LocalSelectorAuthorityDenied
            }
            QueryObligationSelectionErrorKind::BroadCollectionSelectorAuthorityDenied => {
                Self::BroadCollectionSelectorAuthorityDenied
            }
            QueryObligationSelectionErrorKind::LifecycleOnlySelectorAuthorityDenied => {
                Self::LifecycleOnlySelectorAuthorityDenied
            }
            QueryObligationSelectionErrorKind::LocalSupportRowAuthorityDenied => {
                Self::LocalSupportRowAuthorityDenied
            }
            QueryObligationSelectionErrorKind::InMemorySelectionAuthorityDenied => {
                Self::InMemorySelectionAuthorityDenied
            }
            QueryObligationSelectionErrorKind::RawDescriptorAuthorityDenied => {
                Self::RawDescriptorAuthorityDenied
            }
            QueryObligationSelectionErrorKind::TopologySpatialSubstitutionAuthorityDenied => {
                Self::TopologySpatialSubstitutionAuthorityDenied
            }
            QueryObligationSelectionErrorKind::SourceGrepAuditAuthorityDenied => {
                Self::SourceGrepAuditAuthorityDenied
            }
        }
    }
}
