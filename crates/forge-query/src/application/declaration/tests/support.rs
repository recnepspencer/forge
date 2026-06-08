use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySingleOnlyGrouping, ForgeQueryTemporalDeclarationClause,
    ForgeQueryTemporalDeclarationSupport,
};

const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::QueryComposition];
const OPERATING_CAPABILITIES: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::HistoricalEvaluation];
const OPERATING_SECTIONS: &[ForgeQueryConfigSectionFamily] = &[
    ForgeQueryConfigSectionFamily::Query,
    ForgeQueryConfigSectionFamily::Relational,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryOperatingContext {
    regime: &'static str,
}

impl GeometryOperatingContext {
    pub fn collaborative() -> Self {
        Self {
            regime: "collaborative-authoritative",
        }
    }

    pub fn restricted() -> Self {
        Self {
            regime: "restricted-authoritative",
        }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        OPERATING_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        OPERATING_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry-regime:{}", self.regime)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitEdgeDeclaration {
    edge_ref: &'static str,
    parameter: &'static str,
    temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
}

impl SplitEdgeDeclaration {
    pub fn midpoint(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            parameter: "midpoint",
            temporal_clauses: Vec::new(),
        }
    }

    pub fn midpoint_builder(edge_ref: &'static str) -> Self {
        Self::midpoint(edge_ref)
    }

    pub fn at_parameter(edge_ref: &'static str, parameter: &'static str) -> Self {
        Self {
            edge_ref,
            parameter,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SplitEdgeSingleOnlyFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeSingleOnlyFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText("split-edge".to_string()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.parameter",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.parameter.to_string()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.edge_ref",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
            ),
        ]
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitEdgeSingleOnlyDeclaration {
    pub edge_ref: &'static str,
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeSingleOnlyDeclaration {
    type Family = SplitEdgeSingleOnlyFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "split_edge.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyDomain;

impl ForgeQueryDomainEntryMarker for TopologyDomain {
    fn domain_key(&self) -> &'static str {
        "test.topology"
    }

    fn display_name(&self) -> &'static str {
        "TopologyDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

impl ForgeQueryDomainOperatingContext<TopologyDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        OPERATING_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        OPERATING_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        format!("topology-regime:{}", self.regime)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SplitEdgeTopologyFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyDomain> for SplitEdgeTopologyFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologySplitEdgeDeclaration {
    pub edge_ref: &'static str,
}

impl ForgeQueryDeclarationInput<TopologyDomain> for TopologySplitEdgeDeclaration {
    type Family = SplitEdgeTopologyFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "split_edge.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalReadFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalReadFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-read"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeferredTemporalReadFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DeferredTemporalReadFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "deferred-temporal-read"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::DeferredDebt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemporalReadDeclaration {
    edge_ref: &'static str,
    temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
}

impl TemporalReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            temporal_clauses,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for TemporalReadDeclaration {
    type Family = TemporalReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "temporal_read.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredTemporalReadDeclaration {
    edge_ref: &'static str,
    temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
}

impl DeferredTemporalReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            temporal_clauses,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for DeferredTemporalReadDeclaration {
    type Family = DeferredTemporalReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "deferred_temporal_read.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

pub fn admitted_handle(
    regime: GeometryOperatingContext,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    GeometryDomain,
    GeometryOperatingContext,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(regime)
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}

pub fn admitted_topology_handle() -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    TopologyDomain,
    GeometryOperatingContext,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(TopologyDomain)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .expect("topology context should validate")
        .admit()
        .expect("topology context should admit")
}
