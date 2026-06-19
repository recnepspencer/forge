use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus,
};

use super::authority_certification::authority_certification_surface_rows;
use super::row::ForgeQueryGraphObligationSupportMatrixRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationSupportMatrix {
    rows: Vec<ForgeQueryGraphObligationSupportMatrixRow>,
    matrix_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationSupportMatrix {
    pub const MILESTONE_9_9_AUTHORITY_CERTIFICATION_MATRIX_NAME: &'static str =
        "Milestone 9.9 Graph Touch Obligation Authority Hostile Certification Matrix";

    pub fn assembly_selection_foundation() -> Self {
        Self::new(assembly_selection_foundation_rows())
    }

    pub fn milestone_9_9_authority_surface() -> Self {
        Self::new(authority_certification_surface_rows())
    }

    pub fn new(mut rows: Vec<ForgeQueryGraphObligationSupportMatrixRow>) -> Self {
        rows.sort_by(|left, right| left.row_digest().cmp(right.row_digest()));
        let row_digests = rows
            .iter()
            .map(ForgeQueryGraphObligationSupportMatrixRow::row_evidence_digest)
            .collect::<Vec<_>>();
        let matrix_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationSupportMatrix)
                .field_usize(ForgeQueryEvidenceTag::new("rows"), rows.len())
                .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("row"), row_digests)
                .seal();
        Self {
            rows,
            matrix_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryGraphObligationSupportMatrixRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        self.matrix_digest.as_str()
    }

    pub fn rows_for_kind(
        &self,
        kind: ForgeQueryGraphObligationKind,
    ) -> impl Iterator<Item = &ForgeQueryGraphObligationSupportMatrixRow> {
        self.rows
            .iter()
            .filter(move |row| row.obligation_kind() == kind)
    }

    pub fn supported_lane_count_for_kind(&self, kind: ForgeQueryGraphObligationKind) -> usize {
        self.rows_for_kind(kind)
            .filter(|row| row.status() == ForgeQueryGraphObligationSupportStatus::Supported)
            .count()
    }

    pub fn rows_for_lane(
        &self,
        lane: ForgeQueryGraphObligationSupportLane,
    ) -> impl Iterator<Item = &ForgeQueryGraphObligationSupportMatrixRow> {
        self.rows
            .iter()
            .filter(move |row| row.support_lane() == lane)
    }
}

fn assembly_selection_foundation_rows() -> Vec<ForgeQueryGraphObligationSupportMatrixRow> {
    let mut rows = Vec::new();
    for kind in ForgeQueryGraphObligationKind::ALL {
        rows.push(ForgeQueryGraphObligationSupportMatrixRow::new(
            kind,
            ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
            ForgeQueryGraphObligationSupportStatus::Supported,
        ));
        rows.push(ForgeQueryGraphObligationSupportMatrixRow::new(
            kind,
            ForgeQueryGraphObligationSupportLane::GraphComposition,
            ForgeQueryGraphObligationSupportStatus::DeferredToBackstop,
        ));
        rows.push(ForgeQueryGraphObligationSupportMatrixRow::new(
            kind,
            ForgeQueryGraphObligationSupportLane::ReadFamily,
            ForgeQueryGraphObligationSupportStatus::DiagnosticOnly,
        ));
        rows.push(ForgeQueryGraphObligationSupportMatrixRow::new(
            kind,
            ForgeQueryGraphObligationSupportLane::LiveRead,
            ForgeQueryGraphObligationSupportStatus::DiagnosticOnly,
        ));
    }
    rows
}
