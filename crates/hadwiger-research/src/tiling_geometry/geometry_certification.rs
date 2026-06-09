use crate::domain_artifacts::core_artifact::{
    canonical_digest_token, impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, PeriodicQuotientCellDeclaration,
};

use super::cell_artifacts::TilingCell;
use super::contact_facts::TilingGeometryCounters;
use super::tiling_geometry_errors::TilingGeometryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingGeometryCertification {
    core: HadwigerArtifactCore,
    cell: TilingCell,
    query_declaration_digest: String,
    counters: TilingGeometryCounters,
}

impl TilingGeometryCertification {
    pub(crate) fn checked(
        cell: TilingCell,
        query_declaration_digest: String,
    ) -> Result<Self, TilingGeometryError> {
        let counters = TilingGeometryCounters::new(cell.tile_count(), cell.tile_count(), 0, 1, 0);
        let core = artifact_core(
            HadwigerArtifactKind::TilingGeometryCertification,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "rectangular_tiling_cell_geometry_certification".to_string(),
            },
            vec![cell.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "schema",
                    "forge.hadwiger.tiling_geometry_certification.v1",
                ),
                HadwigerArtifactPayloadEntry::text("cell", cell.reference().stable_token()),
                HadwigerArtifactPayloadEntry::text(
                    "query_declaration_digest",
                    &query_declaration_digest,
                ),
                HadwigerArtifactPayloadEntry::unsigned("tile_count", cell.tile_count() as u128),
            ],
        )?;
        Ok(Self {
            core,
            cell,
            query_declaration_digest,
            counters,
        })
    }

    pub fn cell(&self) -> &TilingCell {
        &self.cell
    }

    pub fn tile_count(&self) -> usize {
        self.cell.tile_count()
    }

    pub fn counters(&self) -> &TilingGeometryCounters {
        &self.counters
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingGeometryCertification, core);

pub fn certify_rectangular_tiling_cell_geometry_checked(
    handle: &crate::query_entry::HadwigerResearchHandle,
    cell: TilingCell,
) -> Result<TilingGeometryCertification, TilingGeometryError> {
    let declaration = PeriodicQuotientCellDeclaration::new(cell.cell_id())
        .with_boundary_ownership_ref(cell.reference().stable_token());
    let checked = declare_research_request_checked(handle, declaration);
    let Some(admitted) = checked.admitted() else {
        return Err(TilingGeometryError::QueryCellDeclarationNotAdmitted);
    };
    TilingGeometryCertification::checked(
        cell,
        canonical_digest_token(admitted.declaration_digest()),
    )
}
