use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationConsumerKitErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryObligationSelectionErrorKind {
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
}

#[derive(Debug)]
pub struct QueryObligationSelectionError {
    kind: QueryObligationSelectionErrorKind,
    detail: String,
    query_consumer_kit_kind: Option<ForgeQueryGraphObligationConsumerKitErrorKind>,
}

impl QueryObligationSelectionError {
    pub(crate) fn missing_authority_digest() -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::MissingAuthorityDigest,
            "query obligation selection requires a non-empty authority digest",
        )
    }

    pub(crate) fn query_consumer_kit(error: ForgeQueryGraphObligationConsumerKitError) -> Self {
        Self {
            kind: QueryObligationSelectionErrorKind::QueryConsumerKit,
            detail: error.to_string(),
            query_consumer_kit_kind: Some(error.kind()),
        }
    }

    pub(crate) fn missing_spatial_descriptor_authority() -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::MissingSpatialDescriptorAuthority,
            "spatial query obligation selection requires the spatial descriptor authority",
        )
    }

    pub(crate) fn spatial_consumer_kit(detail: impl Into<String>) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::SpatialConsumerKit,
            detail,
        )
    }

    pub(crate) fn empty_execution_proof() -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::EmptyExecutionProof,
            "query obligation selection requires real executor rows",
        )
    }

    pub(crate) fn copied_selection_parts_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::CopiedSelectionPartsDenied,
            format!("copied selection parts cannot enter query obligation selection: {surface}"),
        )
    }

    pub(crate) fn local_selector_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::LocalSelectorAuthorityDenied,
            format!("local selector authority cannot enter query obligation selection: {surface}"),
        )
    }

    pub(crate) fn broad_collection_selector_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::BroadCollectionSelectorAuthorityDenied,
            format!(
                "broad collection selector authority cannot close query obligation selection: {surface}"
            ),
        )
    }

    pub(crate) fn lifecycle_only_selector_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::LifecycleOnlySelectorAuthorityDenied,
            format!(
                "lifecycle-only selector authority cannot close query obligation selection: {surface}"
            ),
        )
    }

    pub(crate) fn local_support_row_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::LocalSupportRowAuthorityDenied,
            format!("local support rows cannot enter query obligation selection: {surface}"),
        )
    }

    pub(crate) fn in_memory_selection_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::InMemorySelectionAuthorityDenied,
            format!(
                "in-memory selection cannot enter as final query obligation authority: {surface}"
            ),
        )
    }

    pub(crate) fn raw_descriptor_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::RawDescriptorAuthorityDenied,
            format!("raw descriptor authority cannot enter query obligation selection: {surface}"),
        )
    }

    pub(crate) fn topology_spatial_substitution_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::TopologySpatialSubstitutionAuthorityDenied,
            format!(
                "topology/spatial substitution cannot enter query obligation selection: {surface}"
            ),
        )
    }

    pub(crate) fn source_grep_audit_authority_denied(surface: &str) -> Self {
        Self::new(
            QueryObligationSelectionErrorKind::SourceGrepAuditAuthorityDenied,
            format!("source-grep audit cannot enter query obligation selection: {surface}"),
        )
    }

    fn new(kind: QueryObligationSelectionErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
            query_consumer_kit_kind: None,
        }
    }

    pub const fn kind(&self) -> QueryObligationSelectionErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn query_consumer_kit_kind(
        &self,
    ) -> Option<ForgeQueryGraphObligationConsumerKitErrorKind> {
        self.query_consumer_kit_kind
    }
}

impl std::fmt::Display for QueryObligationSelectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.detail)
    }
}

impl std::error::Error for QueryObligationSelectionError {}
