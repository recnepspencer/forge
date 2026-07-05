use std::sync::OnceLock;

use worth_spatial::touched_graph_parity_closeout::{
    current_spatial_family_contributor_catalog as current_spatial_catalog,
    SpatialContributorCatalogRowKind, SpatialContributorLocalLanguagePosture,
    SpatialContributorQueryBoundaryAuthority, SpatialContributorQueryInputKind,
    SpatialFamilyContributorCatalog as SpatialCatalog,
    SpatialFamilyContributorCatalogRow as SpatialCatalogRow,
};

use super::KernelTouchedGraphParityCoverageError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialFamilyContributorCatalogErrorKind {
    CurrentSurfaceUnavailable,
    MissingRequiredFamily,
    MissingQueryOwnedSupportSource,
    MissingWorthLocalResidueSource,
    LocalQueryGapFabricationStillAuthoritative,
    OperatorLocalLanguageStillAuthoritative,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialFamilyContributorCatalogError {
    kind: SpatialFamilyContributorCatalogErrorKind,
    detail: String,
}

pub fn current_spatial_family_contributor_catalog(
) -> Result<SpatialCatalog, SpatialFamilyContributorCatalogError> {
    static CACHE: OnceLock<SpatialCatalog> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let catalog = current_spatial_catalog().map_err(|error| {
        SpatialFamilyContributorCatalogError::new(
            SpatialFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            error.detail(),
        )
    })?;
    validate_catalog(&catalog)?;
    let _ = CACHE.set(catalog.clone());
    Ok(catalog)
}

pub(crate) fn validate_spatial_family_contributor_catalog(
    catalog: &SpatialCatalog,
) -> Result<(), SpatialFamilyContributorCatalogError> {
    validate_catalog(catalog)
}

pub(crate) fn spatial_coverage_contributor_rows(
) -> Result<Vec<SpatialCatalogRow>, KernelTouchedGraphParityCoverageError> {
    current_spatial_family_contributor_catalog()
        .map(|catalog| catalog.rows().to_vec())
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

fn validate_catalog(catalog: &SpatialCatalog) -> Result<(), SpatialFamilyContributorCatalogError> {
    let mut has_evidence = false;
    let mut has_retained = false;

    for row in catalog.rows() {
        if !matches!(
            row.query_input_kind(),
            SpatialContributorQueryInputKind::SupportPostureAndConsumerResidue
        ) {
            return Err(SpatialFamilyContributorCatalogError::new(
                SpatialFamilyContributorCatalogErrorKind::MissingQueryOwnedSupportSource,
                "spatial contributor catalog row must preserve separate Query support-posture and consumer-residue inputs",
            ));
        }
        if row.support_posture_source().is_empty() || row.consumer_residue_source().is_empty() {
            return Err(SpatialFamilyContributorCatalogError::new(
                SpatialFamilyContributorCatalogErrorKind::MissingQueryOwnedSupportSource,
                "spatial contributor catalog row requires explicit Query support-posture and consumer-residue sources",
            ));
        }
        match row.query_boundary_authority() {
            SpatialContributorQueryBoundaryAuthority::QueryOwnedSupportAndResidue => {}
            SpatialContributorQueryBoundaryAuthority::FabricatedLocalQueryGap {
                fabricated_surface,
            } => {
                return Err(SpatialFamilyContributorCatalogError::new(
                    SpatialFamilyContributorCatalogErrorKind::LocalQueryGapFabricationStillAuthoritative,
                    format!(
                        "spatial contributor catalog still fabricates a local Query-gap surface: {fabricated_surface}"
                    ),
                ));
            }
        }
        if let SpatialContributorLocalLanguagePosture::AuthoritativeLocalHelper { legacy_surface } =
            row.local_language_posture()
        {
            return Err(SpatialFamilyContributorCatalogError::new(
                SpatialFamilyContributorCatalogErrorKind::OperatorLocalLanguageStillAuthoritative,
                format!(
                    "spatial contributor catalog still marks local helper language as authoritative: {legacy_surface}"
                ),
            ));
        }
        match row.kind() {
            SpatialContributorCatalogRowKind::EvidenceLookupFamily => {
                has_evidence = true;
                require_current_source(
                    row,
                    "evidence lookup family",
                    "current_evidence_lookup_route_packet",
                )?;
            }
            SpatialContributorCatalogRowKind::RetainedSurfaceFamily => {
                has_retained = true;
                require_current_source(
                    row,
                    "retained surface family",
                    "current_evidence_lookup_public_closeout",
                )?;
                if row.worth_local_residue_source().is_none() {
                    return Err(SpatialFamilyContributorCatalogError::new(
                        SpatialFamilyContributorCatalogErrorKind::MissingWorthLocalResidueSource,
                        "retained surface family row must keep Worth-local residue separate from Query support posture and Query consumer residue",
                    ));
                }
            }
        }
    }

    if !(has_evidence && has_retained) {
        return Err(SpatialFamilyContributorCatalogError::new(
            SpatialFamilyContributorCatalogErrorKind::MissingRequiredFamily,
            "spatial contributor catalog requires evidence-lookup and retained-surface rows",
        ));
    }
    Ok(())
}

fn require_current_source(
    row: &SpatialCatalogRow,
    family_label: &str,
    expected: &str,
) -> Result<(), SpatialFamilyContributorCatalogError> {
    if row.current_packet_or_digest_source() == expected {
        return Ok(());
    }
    Err(SpatialFamilyContributorCatalogError::new(
        SpatialFamilyContributorCatalogErrorKind::MissingRequiredFamily,
        format!(
            "{family_label} contributor row must be sourced from {expected} rather than {}",
            row.current_packet_or_digest_source()
        ),
    ))
}

impl SpatialFamilyContributorCatalogError {
    fn new(kind: SpatialFamilyContributorCatalogErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> SpatialFamilyContributorCatalogErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
