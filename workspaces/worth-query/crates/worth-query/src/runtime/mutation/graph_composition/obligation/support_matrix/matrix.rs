use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportStatus,
};

use super::authority_certification::authority_certification_surface_rows;
use super::row::WorthQueryGraphObligationSupportMatrixRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSupportMatrix {
    rows: Vec<WorthQueryGraphObligationSupportMatrixRow>,
    matrix_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationSupportMatrix {
    pub const MILESTONE_9_9_AUTHORITY_CERTIFICATION_MATRIX_NAME: &'static str =
        "Milestone 9.9 Graph Touch Obligation Authority Hostile Certification Matrix";

    pub fn assembly_selection_foundation() -> Self {
        Self::new(assembly_selection_foundation_rows())
    }

    pub fn milestone_9_9_authority_surface() -> Self {
        Self::new(authority_certification_surface_rows())
    }

    pub fn new(mut rows: Vec<WorthQueryGraphObligationSupportMatrixRow>) -> Self {
        rows.sort_by(|left, right| left.row_digest().cmp(right.row_digest()));
        let row_digests = rows
            .iter()
            .map(WorthQueryGraphObligationSupportMatrixRow::row_evidence_digest)
            .collect::<Vec<_>>();
        let matrix_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationSupportMatrix)
                .field_usize(WorthQueryEvidenceTag::new("rows"), rows.len())
                .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("row"), row_digests)
                .seal();
        Self {
            rows,
            matrix_digest,
        }
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationSupportMatrixRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        self.matrix_digest.as_str()
    }

    pub fn rows_for_kind(
        &self,
        kind: WorthQueryGraphObligationKind,
    ) -> impl Iterator<Item = &WorthQueryGraphObligationSupportMatrixRow> {
        self.rows
            .iter()
            .filter(move |row| row.obligation_kind() == kind)
    }

    pub fn supported_lane_count_for_kind(&self, kind: WorthQueryGraphObligationKind) -> usize {
        self.rows_for_kind(kind)
            .filter(|row| row.status() == WorthQueryGraphObligationSupportStatus::Supported)
            .count()
    }

    pub fn rows_for_lane(
        &self,
        lane: WorthQueryGraphObligationSupportLane,
    ) -> impl Iterator<Item = &WorthQueryGraphObligationSupportMatrixRow> {
        self.rows
            .iter()
            .filter(move |row| row.support_lane() == lane)
    }
}

fn assembly_selection_foundation_rows() -> Vec<WorthQueryGraphObligationSupportMatrixRow> {
    let mut rows = Vec::new();
    for kind in WorthQueryGraphObligationKind::ALL {
        rows.push(WorthQueryGraphObligationSupportMatrixRow::new(
            kind,
            WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
            WorthQueryGraphObligationSupportStatus::Supported,
        ));
        rows.push(WorthQueryGraphObligationSupportMatrixRow::new(
            kind,
            WorthQueryGraphObligationSupportLane::GraphComposition,
            WorthQueryGraphObligationSupportStatus::DeferredToBackstop,
        ));
        rows.push(WorthQueryGraphObligationSupportMatrixRow::new(
            kind,
            WorthQueryGraphObligationSupportLane::ReadFamily,
            WorthQueryGraphObligationSupportStatus::DiagnosticOnly,
        ));
        rows.push(WorthQueryGraphObligationSupportMatrixRow::new(
            kind,
            WorthQueryGraphObligationSupportLane::LiveRead,
            WorthQueryGraphObligationSupportStatus::DiagnosticOnly,
        ));
    }
    rows
}
