use std::sync::OnceLock;

use topology::touched_graph_parity_closeout::{
    current_topology_family_contributor_catalog as current_topology_catalog,
    TopologyContributorCatalogRowKind, TopologyContributorCoverageAuthority,
    TopologyContributorLocalLanguagePosture, TopologyFamilyContributorCatalog as TopologyCatalog,
    TopologyFamilyContributorCatalogRow as TopologyCatalogRow,
};

use super::KernelTouchedGraphParityCoverageError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyFamilyContributorCatalogErrorKind {
    CurrentSurfaceUnavailable,
    MissingRequiredFamily,
    OperatorLocalRoutingStillAuthoritative,
    EntityFallbackStillAuthoritative,
    MissingDeclareOnceBreadth,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyFamilyContributorCatalogError {
    kind: TopologyFamilyContributorCatalogErrorKind,
    detail: String,
}

pub fn current_topology_family_contributor_catalog(
) -> Result<TopologyCatalog, TopologyFamilyContributorCatalogError> {
    static CACHE: OnceLock<TopologyCatalog> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let catalog = current_topology_catalog().map_err(|error| {
        TopologyFamilyContributorCatalogError::new(
            TopologyFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            error.detail(),
        )
    })?;
    validate_catalog(&catalog)?;
    let _ = CACHE.set(catalog.clone());
    Ok(catalog)
}

pub(crate) fn validate_topology_family_contributor_catalog(
    catalog: &TopologyCatalog,
) -> Result<(), TopologyFamilyContributorCatalogError> {
    validate_catalog(catalog)
}

pub(crate) fn topology_coverage_contributor_rows(
) -> Result<Vec<TopologyCatalogRow>, KernelTouchedGraphParityCoverageError> {
    current_topology_family_contributor_catalog()
        .map(|catalog| catalog.rows().to_vec())
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

fn validate_catalog(
    catalog: &TopologyCatalog,
) -> Result<(), TopologyFamilyContributorCatalogError> {
    let mut has_read = false;
    let mut has_validator = false;
    let mut has_invalidation = false;
    let mut validator_has_multi_match = false;

    for row in catalog.rows() {
        match row.kind() {
            TopologyContributorCatalogRowKind::ReadFamily => {
                has_read = true;
                require_coverage_authority(
                    row,
                    "read family",
                    matches!(
                        row.coverage_authority(),
                        TopologyContributorCoverageAuthority::ReadRequestFamilies(_)
                    ),
                )?;
            }
            TopologyContributorCatalogRowKind::ValidatorInvariantFamily => {
                has_validator = true;
                require_coverage_authority(
                    row,
                    "validator/invariant family",
                    matches!(
                        row.coverage_authority(),
                        TopologyContributorCoverageAuthority::ValidatorRuleIdentities(_)
                    ),
                )?;
                require_current_surface(
                    row,
                    "validator/invariant family",
                    "current_topology_validator_invariant_milestone_nine_closeout",
                )?;
                if row.operator_or_stage_coverage().len() > 1 {
                    validator_has_multi_match = true;
                }
            }
            TopologyContributorCatalogRowKind::InvalidationFamily => {
                has_invalidation = true;
                require_coverage_authority(
                    row,
                    "invalidation family",
                    matches!(
                        row.coverage_authority(),
                        TopologyContributorCoverageAuthority::InvalidationStageIdentities(_)
                    ),
                )?;
            }
        }
        if let TopologyContributorLocalLanguagePosture::AuthoritativeLocalHelper {
            legacy_surface,
        } = row.local_language_posture()
        {
            return Err(TopologyFamilyContributorCatalogError::new(
                TopologyFamilyContributorCatalogErrorKind::OperatorLocalRoutingStillAuthoritative,
                format!(
                    "topology contributor catalog still marks operator-local routing language as authoritative: {legacy_surface}"
                ),
            ));
        }
    }

    if !(has_read && has_validator && has_invalidation) {
        return Err(TopologyFamilyContributorCatalogError::new(
            TopologyFamilyContributorCatalogErrorKind::MissingRequiredFamily,
            "topology contributor catalog requires read, validator/invariant, and invalidation rows",
        ));
    }
    if !validator_has_multi_match {
        return Err(TopologyFamilyContributorCatalogError::new(
            TopologyFamilyContributorCatalogErrorKind::MissingDeclareOnceBreadth,
            "validator/invariant contributor row must cover multiple matching rule identities from the live selected plan",
        ));
    }
    Ok(())
}

fn require_coverage_authority(
    row: &TopologyCatalogRow,
    family_label: &str,
    valid: bool,
) -> Result<(), TopologyFamilyContributorCatalogError> {
    if valid {
        return Ok(());
    }
    let kind = if matches!(
        row.coverage_authority(),
        TopologyContributorCoverageAuthority::RegistrationEntityFallback(_)
    ) {
        TopologyFamilyContributorCatalogErrorKind::EntityFallbackStillAuthoritative
    } else {
        TopologyFamilyContributorCatalogErrorKind::MissingDeclareOnceBreadth
    };
    Err(TopologyFamilyContributorCatalogError::new(
        kind,
        format!(
            "{family_label} contributor row carries {} instead of family-specific operator or stage authority",
            row.coverage_authority().kind()
        ),
    ))
}

fn require_current_surface(
    row: &TopologyCatalogRow,
    family_label: &str,
    expected: &str,
) -> Result<(), TopologyFamilyContributorCatalogError> {
    if row.current_packet_or_function() == expected {
        return Ok(());
    }
    Err(TopologyFamilyContributorCatalogError::new(
        TopologyFamilyContributorCatalogErrorKind::MissingDeclareOnceBreadth,
        format!(
            "{family_label} contributor row must be sourced from {expected} rather than {}",
            row.current_packet_or_function()
        ),
    ))
}

impl TopologyFamilyContributorCatalogError {
    fn new(kind: TopologyFamilyContributorCatalogErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> TopologyFamilyContributorCatalogErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
