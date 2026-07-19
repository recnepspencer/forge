use crate::application::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture, WorthQueryTemporalDeclarationClause,
    WorthQueryTemporalDeclarationSupport,
};

use super::support::{GeometryDomain, SplitEdgeFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncResourceReadFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncResourceReadFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-resource-read"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeferredAsyncResourceReadFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for DeferredAsyncResourceReadFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "deferred-async-resource-read"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::DeferredDebt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncResourceReadDeclaration {
    edge_ref: &'static str,
    async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
}

impl AsyncResourceReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses,
            temporal_clauses: Vec::new(),
        }
    }

    pub fn with_temporal(
        mut self,
        temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
    ) -> Self {
        self.temporal_clauses = temporal_clauses;
        self
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for AsyncResourceReadDeclaration {
    type Family = AsyncResourceReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::new(
            "async_resource_read.edge_ref",
            WorthQueryDeclarationCanonicalEntryKind::Identity,
            WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
        self.async_clauses.clone()
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredAsyncResourceReadDeclaration {
    edge_ref: &'static str,
    async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
}

impl DeferredAsyncResourceReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses,
        }
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for DeferredAsyncResourceReadDeclaration {
    type Family = DeferredAsyncResourceReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::new(
            "deferred_async_resource_read.edge_ref",
            WorthQueryDeclarationCanonicalEntryKind::Identity,
            WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
        self.async_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncUnsupportedSplitEdgeDeclaration {
    edge_ref: &'static str,
    async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
}

impl AsyncUnsupportedSplitEdgeDeclaration {
    pub fn new(
        edge_ref: &'static str,
        async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses,
        }
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for AsyncUnsupportedSplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::new(
            "split_edge.edge_ref",
            WorthQueryDeclarationCanonicalEntryKind::Identity,
            WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
        self.async_clauses.clone()
    }
}

pub fn request_identity(parts: &[(&str, &str)]) -> Vec<WorthQueryAsyncRequestIdentityPart> {
    parts
        .iter()
        .map(|(key, value)| WorthQueryAsyncRequestIdentityPart::text(*key, *value))
        .collect()
}

pub fn resource_request(
    source_family: WorthQueryAsyncSourceFamily,
    loading_posture: WorthQueryAsyncLoadingPosture,
    failure_posture: WorthQueryAsyncFailurePosture,
    parts: &[(&str, &str)],
) -> WorthQueryAsyncDeclarationClause {
    WorthQueryAsyncDeclarationClause::resource_request(
        source_family,
        loading_posture,
        failure_posture,
        request_identity(parts),
    )
}
