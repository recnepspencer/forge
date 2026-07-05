use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::WorthTouchedGraphCrossFamilyCloseoutMatrixRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphCrossFamilyCloseoutMatrix {
    rows: Vec<WorthTouchedGraphCrossFamilyCloseoutMatrixRow>,
    closeout_architecture_claim_digest: String,
    matrix_digest: String,
}

impl WorthTouchedGraphCrossFamilyCloseoutMatrix {
    pub(crate) fn new(
        mut rows: Vec<WorthTouchedGraphCrossFamilyCloseoutMatrixRow>,
        closeout_architecture_claim_digest: impl Into<String>,
    ) -> Self {
        rows.sort_by_key(|row| row.family_kind());
        let closeout_architecture_claim_digest = closeout_architecture_claim_digest.into();
        let matrix_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                        row.family_kind().as_str(),
                        row.covered_surface_count(),
                        row.representative_path_covered(),
                        row.declare_once_parity_passed(),
                        row.public_proof_parity_passed(),
                        row.diagnostic_parity_passed(),
                        row.readiness_handoff_passed(),
                        row.deleted_count(),
                        row.capped_residue_count(),
                        row.query_gap_count(),
                        row.blocked_outside_roadmap_count()
                    )
                })
                .chain(std::iter::once(format!(
                    "closeout-architecture-claim:{}",
                    closeout_architecture_claim_digest
                )))
                .chain(std::iter::once(
                    "worth-kernel:touched-graph-cross-family-closeout-matrix:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );

        Self {
            rows,
            closeout_architecture_claim_digest,
            matrix_digest,
        }
    }

    pub fn rows(&self) -> &[WorthTouchedGraphCrossFamilyCloseoutMatrixRow] {
        &self.rows
    }

    pub fn row(
        &self,
        family_kind: schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind,
    ) -> Option<&WorthTouchedGraphCrossFamilyCloseoutMatrixRow> {
        self.rows
            .iter()
            .find(|row| row.family_kind() == family_kind)
    }

    pub fn closeout_architecture_claim_digest(&self) -> &str {
        &self.closeout_architecture_claim_digest
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn covered_surface_count(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.covered_surface_count())
            .sum()
    }

    pub fn deleted_count(&self) -> usize {
        self.rows.iter().map(|row| row.deleted_count()).sum()
    }

    pub fn capped_residue_count(&self) -> usize {
        self.rows.iter().map(|row| row.capped_residue_count()).sum()
    }

    pub fn query_gap_count(&self) -> usize {
        self.rows.iter().map(|row| row.query_gap_count()).sum()
    }

    pub fn blocked_outside_roadmap_count(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.blocked_outside_roadmap_count())
            .sum()
    }

    pub fn total_certified_rows(&self) -> usize {
        self.rows.iter().map(|row| row.total_certified_rows()).sum()
    }
}
