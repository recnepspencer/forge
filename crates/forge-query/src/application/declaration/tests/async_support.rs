use crate::application::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQueryTemporalDeclarationClause,
    ForgeQueryTemporalDeclarationSupport,
};

use super::support::{GeometryDomain, SplitEdgeFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncResourceReadFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncResourceReadFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-resource-read"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeferredAsyncResourceReadFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DeferredAsyncResourceReadFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "deferred-async-resource-read"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::DeferredDebt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncResourceReadDeclaration {
    edge_ref: &'static str,
    async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
    temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
}

impl AsyncResourceReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses,
            temporal_clauses: Vec::new(),
        }
    }

    pub fn with_temporal(
        mut self,
        temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
    ) -> Self {
        self.temporal_clauses = temporal_clauses;
        self
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for AsyncResourceReadDeclaration {
    type Family = AsyncResourceReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "async_resource_read.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
        self.async_clauses.clone()
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredAsyncResourceReadDeclaration {
    edge_ref: &'static str,
    async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
}

impl DeferredAsyncResourceReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for DeferredAsyncResourceReadDeclaration {
    type Family = DeferredAsyncResourceReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "deferred_async_resource_read.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
        self.async_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncUnsupportedSplitEdgeDeclaration {
    edge_ref: &'static str,
    async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
}

impl AsyncUnsupportedSplitEdgeDeclaration {
    pub fn new(
        edge_ref: &'static str,
        async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for AsyncUnsupportedSplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "split_edge.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
        self.async_clauses.clone()
    }
}

pub fn request_identity(parts: &[(&str, &str)]) -> Vec<ForgeQueryAsyncRequestIdentityPart> {
    parts
        .iter()
        .map(|(key, value)| ForgeQueryAsyncRequestIdentityPart::text(*key, *value))
        .collect()
}

pub fn resource_request(
    source_family: ForgeQueryAsyncSourceFamily,
    loading_posture: ForgeQueryAsyncLoadingPosture,
    failure_posture: ForgeQueryAsyncFailurePosture,
    parts: &[(&str, &str)],
) -> ForgeQueryAsyncDeclarationClause {
    ForgeQueryAsyncDeclarationClause::resource_request(
        source_family,
        loading_posture,
        failure_posture,
        request_identity(parts),
    )
}
