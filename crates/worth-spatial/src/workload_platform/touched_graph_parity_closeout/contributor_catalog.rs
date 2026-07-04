use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::contributor::SpatialTouchedGraphParityCoverageContributor;
use super::evidence_lookup_family::current_spatial_evidence_lookup_declaration_row;
use super::retained_surface_family::current_spatial_retained_surface_declaration_row;
use super::SpatialTouchedGraphParityCoverageError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialContributorCatalogRowKind {
    EvidenceLookupFamily,
    RetainedSurfaceFamily,
}

impl SpatialContributorCatalogRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceLookupFamily => "evidence_lookup_family",
            Self::RetainedSurfaceFamily => "retained_surface_family",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialContributorQueryInputKind {
    SupportPostureAndConsumerResidue,
}

impl SpatialContributorQueryInputKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportPostureAndConsumerResidue => "support_posture_and_consumer_residue",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialContributorQueryBoundaryAuthority {
    QueryOwnedSupportAndResidue,
    FabricatedLocalQueryGap { fabricated_surface: &'static str },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialContributorLocalLanguagePosture {
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
pub struct SpatialFamilyContributorCatalogRow {
    kind: SpatialContributorCatalogRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_digest_source: &'static str,
    public_closeout_digest_source: &'static str,
    support_posture_source: &'static str,
    consumer_residue_source: &'static str,
    worth_local_residue_source: Option<&'static str>,
    selected_identity_fields_produced: &'static [&'static str],
    query_input_kind: SpatialContributorQueryInputKind,
    query_boundary_authority: SpatialContributorQueryBoundaryAuthority,
    local_language_posture: SpatialContributorLocalLanguagePosture,
    coverage_contributor: SpatialTouchedGraphParityCoverageContributor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialFamilyContributorCatalog {
    rows: Vec<SpatialFamilyContributorCatalogRow>,
    catalog_digest: String,
}

pub fn current_spatial_family_contributor_catalog(
) -> Result<SpatialFamilyContributorCatalog, SpatialTouchedGraphParityCoverageError> {
    SpatialFamilyContributorCatalog::new(vec![
        current_spatial_evidence_lookup_declaration_row().map_err(|error| {
            SpatialTouchedGraphParityCoverageError::new(format!(
                "evidence lookup declaration row failed: {}",
                error.detail()
            ))
        })?,
        current_spatial_retained_surface_declaration_row().map_err(|error| {
            SpatialTouchedGraphParityCoverageError::new(format!(
                "retained surface declaration row failed: {}",
                error.detail()
            ))
        })?,
    ])
}

impl SpatialFamilyContributorCatalogRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        kind: SpatialContributorCatalogRowKind,
        family_kind: TouchedGraphParityFamilyKind,
        current_packet_or_digest_source: &'static str,
        public_closeout_digest_source: &'static str,
        support_posture_source: &'static str,
        consumer_residue_source: &'static str,
        worth_local_residue_source: Option<&'static str>,
        selected_identity_fields_produced: &'static [&'static str],
        query_input_kind: SpatialContributorQueryInputKind,
        query_boundary_authority: SpatialContributorQueryBoundaryAuthority,
        local_language_posture: SpatialContributorLocalLanguagePosture,
        coverage_contributor: SpatialTouchedGraphParityCoverageContributor,
    ) -> Result<Self, SpatialTouchedGraphParityCoverageError> {
        if current_packet_or_digest_source.is_empty()
            || public_closeout_digest_source.is_empty()
            || support_posture_source.is_empty()
            || consumer_residue_source.is_empty()
        {
            return Err(SpatialTouchedGraphParityCoverageError::new(
                "spatial contributor catalog row requires exact route, public-closeout, support-posture, and consumer-residue sources",
            ));
        }
        if selected_identity_fields_produced.is_empty() {
            return Err(SpatialTouchedGraphParityCoverageError::new(
                "spatial contributor catalog row requires non-empty selected identity fields",
            ));
        }
        Ok(Self {
            kind,
            family_kind,
            current_packet_or_digest_source,
            public_closeout_digest_source,
            support_posture_source,
            consumer_residue_source,
            worth_local_residue_source,
            selected_identity_fields_produced,
            query_input_kind,
            query_boundary_authority,
            local_language_posture,
            coverage_contributor,
        })
    }

    pub const fn kind(&self) -> SpatialContributorCatalogRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_digest_source(&self) -> &'static str {
        self.current_packet_or_digest_source
    }

    pub const fn public_closeout_digest_source(&self) -> &'static str {
        self.public_closeout_digest_source
    }

    pub const fn support_posture_source(&self) -> &'static str {
        self.support_posture_source
    }

    pub const fn consumer_residue_source(&self) -> &'static str {
        self.consumer_residue_source
    }

    pub const fn worth_local_residue_source(&self) -> Option<&'static str> {
        self.worth_local_residue_source
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }

    pub const fn query_input_kind(&self) -> SpatialContributorQueryInputKind {
        self.query_input_kind
    }

    pub const fn query_boundary_authority(&self) -> &SpatialContributorQueryBoundaryAuthority {
        &self.query_boundary_authority
    }

    pub const fn local_language_posture(&self) -> &SpatialContributorLocalLanguagePosture {
        &self.local_language_posture
    }

    pub const fn coverage_contributor(&self) -> &SpatialTouchedGraphParityCoverageContributor {
        &self.coverage_contributor
    }

    #[doc(hidden)]
    pub fn with_query_boundary_authority_for_testing(
        mut self,
        query_boundary_authority: SpatialContributorQueryBoundaryAuthority,
    ) -> Self {
        self.query_boundary_authority = query_boundary_authority;
        self
    }
}

impl SpatialFamilyContributorCatalog {
    pub fn new(
        rows: Vec<SpatialFamilyContributorCatalogRow>,
    ) -> Result<Self, SpatialTouchedGraphParityCoverageError> {
        if rows.len() != 2 {
            return Err(SpatialTouchedGraphParityCoverageError::new(
                "spatial contributor catalog requires exactly evidence-lookup and retained-surface rows",
            ));
        }
        let catalog_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &rows
                .iter()
                .flat_map(|row| {
                    let query_boundary = match row.query_boundary_authority() {
                        SpatialContributorQueryBoundaryAuthority::QueryOwnedSupportAndResidue => {
                            "query-boundary:query-owned".to_string()
                        }
                        SpatialContributorQueryBoundaryAuthority::FabricatedLocalQueryGap {
                            fabricated_surface,
                        } => format!("query-boundary:fabricated:{fabricated_surface}"),
                    };
                    let local_language = match row.local_language_posture() {
                        SpatialContributorLocalLanguagePosture::None => {
                            "local-language:none".to_string()
                        }
                        SpatialContributorLocalLanguagePosture::ExplicitlyBlocked {
                            legacy_surface,
                            blocking_surface,
                        } => format!("local-language:blocked:{legacy_surface}:{blocking_surface}"),
                        SpatialContributorLocalLanguagePosture::AuthoritativeLocalHelper {
                            legacy_surface,
                        } => format!("local-language:authoritative:{legacy_surface}"),
                    };
                    [
                        format!("kind:{}", row.kind().as_str()),
                        format!("family:{}", row.family_kind().as_str()),
                        format!("current:{}", row.current_packet_or_digest_source()),
                        format!("closeout:{}", row.public_closeout_digest_source()),
                        format!("support:{}", row.support_posture_source()),
                        format!("consumer-residue:{}", row.consumer_residue_source()),
                        format!(
                            "worth-residue:{}",
                            row.worth_local_residue_source().unwrap_or("none")
                        ),
                        format!("query-input:{}", row.query_input_kind().as_str()),
                        format!(
                            "produced:{}",
                            row.selected_identity_fields_produced().join(",")
                        ),
                        query_boundary,
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

    pub fn rows(&self) -> &[SpatialFamilyContributorCatalogRow] {
        &self.rows
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}
