use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::contributor::TopologyTouchedGraphParityCoverageContributor;
use super::invalidation_family::current_topology_invalidation_declaration_row;
use super::read_family::current_topology_read_declaration_row;
use super::validator_invariant_family::current_topology_validator_invariant_declaration_row;
use super::TopologyTouchedGraphParityCoverageError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyContributorCatalogRowKind {
    ReadFamily,
    ValidatorInvariantFamily,
    InvalidationFamily,
}

impl TopologyContributorCatalogRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadFamily => "read_family",
            Self::ValidatorInvariantFamily => "validator_invariant_family",
            Self::InvalidationFamily => "invalidation_family",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyContributorLocalLanguagePosture {
    None,
    ExplicitlyBlocked {
        legacy_surface: &'static str,
        blocking_surface: &'static str,
    },
    AuthoritativeLocalHelper {
        legacy_surface: &'static str,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyContributorCoverageAuthority {
    ReadRequestFamilies(Vec<String>),
    ValidatorRuleIdentities(Vec<String>),
    InvalidationStageIdentities(Vec<String>),
    RegistrationEntityFallback(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyFamilyContributorCatalogRow {
    kind: TopologyContributorCatalogRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_function: &'static str,
    selected_identity_fields_produced: &'static [&'static str],
    coverage_authority: TopologyContributorCoverageAuthority,
    local_language_posture: TopologyContributorLocalLanguagePosture,
    coverage_contributor: TopologyTouchedGraphParityCoverageContributor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyFamilyContributorCatalog {
    rows: Vec<TopologyFamilyContributorCatalogRow>,
    catalog_digest: String,
}

pub fn current_topology_family_contributor_catalog(
) -> Result<TopologyFamilyContributorCatalog, TopologyTouchedGraphParityCoverageError> {
    TopologyFamilyContributorCatalog::new(vec![
        current_topology_read_declaration_row().map_err(|error| {
            TopologyTouchedGraphParityCoverageError::new(format!(
                "read family declaration row failed: {}",
                error.detail()
            ))
        })?,
        current_topology_validator_invariant_declaration_row().map_err(|error| {
            TopologyTouchedGraphParityCoverageError::new(format!(
                "validator/invariant declaration row failed: {}",
                error.detail()
            ))
        })?,
        current_topology_invalidation_declaration_row().map_err(|error| {
            TopologyTouchedGraphParityCoverageError::new(format!(
                "invalidation declaration row failed: {}",
                error.detail()
            ))
        })?,
    ])
}

impl TopologyFamilyContributorCatalogRow {
    pub(crate) fn new(
        kind: TopologyContributorCatalogRowKind,
        family_kind: TouchedGraphParityFamilyKind,
        current_packet_or_function: &'static str,
        selected_identity_fields_produced: &'static [&'static str],
        coverage_authority: TopologyContributorCoverageAuthority,
        local_language_posture: TopologyContributorLocalLanguagePosture,
        coverage_contributor: TopologyTouchedGraphParityCoverageContributor,
    ) -> Result<Self, TopologyTouchedGraphParityCoverageError> {
        if coverage_authority.values().is_empty() {
            return Err(TopologyTouchedGraphParityCoverageError::new(
                "topology contributor catalog row requires non-empty operator or stage coverage",
            ));
        }
        if selected_identity_fields_produced.is_empty() {
            return Err(TopologyTouchedGraphParityCoverageError::new(
                "topology contributor catalog row requires non-empty selected identity fields",
            ));
        }
        Ok(Self {
            kind,
            family_kind,
            current_packet_or_function,
            selected_identity_fields_produced,
            coverage_authority,
            local_language_posture,
            coverage_contributor,
        })
    }

    pub const fn kind(&self) -> TopologyContributorCatalogRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_function(&self) -> &'static str {
        self.current_packet_or_function
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }

    pub fn operator_or_stage_coverage(&self) -> &[String] {
        self.coverage_authority.values()
    }

    pub const fn coverage_authority(&self) -> &TopologyContributorCoverageAuthority {
        &self.coverage_authority
    }

    pub const fn local_language_posture(&self) -> &TopologyContributorLocalLanguagePosture {
        &self.local_language_posture
    }

    pub const fn coverage_contributor(&self) -> &TopologyTouchedGraphParityCoverageContributor {
        &self.coverage_contributor
    }

    #[doc(hidden)]
    pub fn with_local_language_posture_for_testing(
        mut self,
        posture: TopologyContributorLocalLanguagePosture,
    ) -> Self {
        self.local_language_posture = posture;
        self
    }

    #[doc(hidden)]
    pub fn with_coverage_authority_for_testing(
        mut self,
        coverage_authority: TopologyContributorCoverageAuthority,
    ) -> Self {
        self.coverage_authority = coverage_authority;
        self
    }
}

impl TopologyFamilyContributorCatalog {
    pub fn new(
        rows: Vec<TopologyFamilyContributorCatalogRow>,
    ) -> Result<Self, TopologyTouchedGraphParityCoverageError> {
        if rows.len() != 3 {
            return Err(TopologyTouchedGraphParityCoverageError::new(
                "topology contributor catalog requires exactly read, validator/invariant, and invalidation rows",
            ));
        }
        let catalog_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &rows
                .iter()
                .flat_map(|row| {
                    let local_language = match row.local_language_posture() {
                        TopologyContributorLocalLanguagePosture::None => {
                            "local-language:none".to_string()
                        }
                        TopologyContributorLocalLanguagePosture::ExplicitlyBlocked {
                            legacy_surface,
                            blocking_surface,
                        } => format!("local-language:blocked:{legacy_surface}:{blocking_surface}"),
                        TopologyContributorLocalLanguagePosture::AuthoritativeLocalHelper {
                            legacy_surface,
                        } => format!("local-language:authoritative:{legacy_surface}"),
                    };
                    [
                        format!("kind:{}", row.kind().as_str()),
                        format!("family:{}", row.family_kind().as_str()),
                        format!("current:{}", row.current_packet_or_function()),
                        format!(
                            "produced:{}",
                            row.selected_identity_fields_produced().join(",")
                        ),
                        format!("coverage-authority:{}", row.coverage_authority().kind()),
                        format!("coverage:{}", row.operator_or_stage_coverage().join(",")),
                        local_language,
                    ]
                })
                .collect::<Vec<_>>(),
        );
        Ok(Self {
            rows,
            catalog_digest,
        })
    }

    pub fn rows(&self) -> &[TopologyFamilyContributorCatalogRow] {
        &self.rows
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

impl TopologyContributorCoverageAuthority {
    pub fn values(&self) -> &[String] {
        match self {
            Self::ReadRequestFamilies(values)
            | Self::ValidatorRuleIdentities(values)
            | Self::InvalidationStageIdentities(values)
            | Self::RegistrationEntityFallback(values) => values,
        }
    }

    pub const fn kind(&self) -> &'static str {
        match self {
            Self::ReadRequestFamilies(_) => "read_request_families",
            Self::ValidatorRuleIdentities(_) => "validator_rule_identities",
            Self::InvalidationStageIdentities(_) => "invalidation_stage_identities",
            Self::RegistrationEntityFallback(_) => "registration_entity_fallback",
        }
    }
}
