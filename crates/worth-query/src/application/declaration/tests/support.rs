use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
    WorthQuerySingleOnlyGrouping, WorthQueryTemporalDeclarationClause,
    WorthQueryTemporalDeclarationSupport,
};

const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] =
    &[WorthQueryCapabilityFamily::QueryComposition];
const OPERATING_CAPABILITIES: &[WorthQueryCapabilityFamily] =
    &[WorthQueryCapabilityFamily::HistoricalEvaluation];
const OPERATING_SECTIONS: &[WorthQueryConfigSectionFamily] = &[
    WorthQueryConfigSectionFamily::Query,
    WorthQueryConfigSectionFamily::Relational,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
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

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        OPERATING_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
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
    temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
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
        temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
    ) -> Self {
        self.temporal_clauses = temporal_clauses;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SplitEdgeFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SplitEdgeSingleOnlyFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeSingleOnlyFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::new(
                "family.operation",
                WorthQueryDeclarationCanonicalEntryKind::Header,
                WorthQueryDeclarationCanonicalValue::ExactText("split-edge".to_string()),
            ),
            WorthQueryDeclarationCanonicalEntry::new(
                "split_edge.parameter",
                WorthQueryDeclarationCanonicalEntryKind::Field,
                WorthQueryDeclarationCanonicalValue::ExactText(self.parameter.to_string()),
            ),
            WorthQueryDeclarationCanonicalEntry::new(
                "split_edge.edge_ref",
                WorthQueryDeclarationCanonicalEntryKind::Identity,
                WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
            ),
        ]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitEdgeSingleOnlyDeclaration {
    pub edge_ref: &'static str,
}

impl WorthQueryDeclarationInput<GeometryDomain> for SplitEdgeSingleOnlyDeclaration {
    type Family = SplitEdgeSingleOnlyFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::new(
            "split_edge.edge_ref",
            WorthQueryDeclarationCanonicalEntryKind::Identity,
            WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyDomain;

impl WorthQueryDomainEntryMarker for TopologyDomain {
    fn domain_key(&self) -> &'static str {
        "test.topology"
    }

    fn display_name(&self) -> &'static str {
        "TopologyDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

impl WorthQueryDomainOperatingContext<TopologyDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        OPERATING_CAPABILITIES
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        OPERATING_SECTIONS
    }

    fn context_identity_digest(&self) -> String {
        format!("topology-regime:{}", self.regime)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SplitEdgeTopologyFamily;

impl WorthQueryDeclarationFamilyMarker<TopologyDomain> for SplitEdgeTopologyFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologySplitEdgeDeclaration {
    pub edge_ref: &'static str,
}

impl WorthQueryDeclarationInput<TopologyDomain> for TopologySplitEdgeDeclaration {
    type Family = SplitEdgeTopologyFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::new(
            "split_edge.edge_ref",
            WorthQueryDeclarationCanonicalEntryKind::Identity,
            WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalReadFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalReadFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-read"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeferredTemporalReadFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for DeferredTemporalReadFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "deferred-temporal-read"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::DeferredDebt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemporalReadDeclaration {
    edge_ref: &'static str,
    temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
}

impl TemporalReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            temporal_clauses,
        }
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for TemporalReadDeclaration {
    type Family = TemporalReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::new(
            "temporal_read.edge_ref",
            WorthQueryDeclarationCanonicalEntryKind::Identity,
            WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredTemporalReadDeclaration {
    edge_ref: &'static str,
    temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
}

impl DeferredTemporalReadDeclaration {
    pub fn new(
        edge_ref: &'static str,
        temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
    ) -> Self {
        Self {
            edge_ref,
            temporal_clauses,
        }
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for DeferredTemporalReadDeclaration {
    type Family = DeferredTemporalReadFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::new(
            "deferred_temporal_read.edge_ref",
            WorthQueryDeclarationCanonicalEntryKind::Identity,
            WorthQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        self.temporal_clauses.clone()
    }
}

pub fn admitted_handle(
    regime: GeometryOperatingContext,
) -> crate::application::WorthQueryAdmittedConfiguredDomainHandle<
    GeometryDomain,
    GeometryOperatingContext,
> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(regime)
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}

pub fn admitted_topology_handle() -> crate::application::WorthQueryAdmittedConfiguredDomainHandle<
    TopologyDomain,
    GeometryOperatingContext,
> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(TopologyDomain)
        .with_operating_context(GeometryOperatingContext::collaborative())
        .validate()
        .expect("topology context should validate")
        .admit()
        .expect("topology context should admit")
}
