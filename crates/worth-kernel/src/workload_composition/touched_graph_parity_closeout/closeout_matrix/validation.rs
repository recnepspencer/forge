use std::collections::{BTreeMap, BTreeSet};

use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use super::matrix::WorthTouchedGraphCrossFamilyCloseoutMatrix;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CloseoutMatrixAuthority {
    pub representative_path_coverage: BTreeSet<TouchedGraphParityFamilyKind>,
    pub family_parity_coverage: BTreeSet<TouchedGraphParityFamilyKind>,
    pub readiness_coverage: BTreeSet<TouchedGraphParityFamilyKind>,
    pub public_proof_parity_passed: bool,
    pub diagnostic_parity_passed: bool,
    pub covered_counts: BTreeMap<TouchedGraphParityFamilyKind, usize>,
    pub deleted_counts: BTreeMap<TouchedGraphParityFamilyKind, usize>,
    pub capped_residue_counts: BTreeMap<TouchedGraphParityFamilyKind, usize>,
    pub query_gap_counts: BTreeMap<TouchedGraphParityFamilyKind, usize>,
    pub blocked_counts: BTreeMap<TouchedGraphParityFamilyKind, usize>,
    pub closeout_architecture_claim_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind {
    CurrentCoverageInventoryUnavailable,
    CurrentLiveCoverageLedgerUnavailable,
    CurrentRepresentativePathUnavailable,
    CurrentReadinessHandoffUnavailable,
    CurrentTopologyParityUnavailable,
    CurrentSpatialParityUnavailable,
    CurrentReplayUndoParityUnavailable,
    CurrentConflictParityUnavailable,
    CurrentReuseParityUnavailable,
    CurrentPublicProjectionParityUnavailable,
    CurrentPublicCloseoutUnavailable,
    MissingCertifiedFamilyRow,
    DuplicateCertifiedFamilyRow,
    MissingRepresentativePathCoverage,
    MissingFamilyParity,
    MissingReadinessCoverage,
    MissingPublicProjectionParity,
    CountMismatch,
    UnclassifiedDeletedSurface,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphCrossFamilyCloseoutMatrixError {
    kind: WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind,
    detail: String,
}

pub(crate) fn validate_closeout_matrix(
    matrix: &WorthTouchedGraphCrossFamilyCloseoutMatrix,
    authority: &CloseoutMatrixAuthority,
) -> Result<(), WorthTouchedGraphCrossFamilyCloseoutMatrixError> {
    let mut seen = BTreeSet::new();
    for row in matrix.rows() {
        if !seen.insert(row.family_kind()) {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::DuplicateCertifiedFamilyRow,
                format!(
                    "closeout matrix duplicated family kind `{}`",
                    row.family_kind().as_str()
                ),
            ));
        }
        if !authority
            .family_parity_coverage
            .contains(&row.family_kind())
        {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingFamilyParity,
                format!(
                    "closeout matrix family `{}` is missing family parity certification",
                    row.family_kind().as_str()
                ),
            ));
        }
        if !authority.readiness_coverage.contains(&row.family_kind()) {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingReadinessCoverage,
                format!(
                    "closeout matrix family `{}` is missing readiness coverage",
                    row.family_kind().as_str()
                ),
            ));
        }
        assert_family_count(
            row.family_kind(),
            "covered_surface_count",
            row.covered_surface_count(),
            authority
                .covered_counts
                .get(&row.family_kind())
                .copied()
                .unwrap_or(0),
        )?;
        assert_family_count(
            row.family_kind(),
            "deleted_count",
            row.deleted_count(),
            authority
                .deleted_counts
                .get(&row.family_kind())
                .copied()
                .unwrap_or(0),
        )?;
        assert_family_count(
            row.family_kind(),
            "capped_residue_count",
            row.capped_residue_count(),
            authority
                .capped_residue_counts
                .get(&row.family_kind())
                .copied()
                .unwrap_or(0),
        )?;
        assert_family_count(
            row.family_kind(),
            "query_gap_count",
            row.query_gap_count(),
            authority
                .query_gap_counts
                .get(&row.family_kind())
                .copied()
                .unwrap_or(0),
        )?;
        assert_family_count(
            row.family_kind(),
            "blocked_outside_roadmap_count",
            row.blocked_outside_roadmap_count(),
            authority
                .blocked_counts
                .get(&row.family_kind())
                .copied()
                .unwrap_or(0),
        )?;
        if row.representative_path_covered()
            != authority
                .representative_path_coverage
                .contains(&row.family_kind())
        {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingRepresentativePathCoverage,
                format!(
                    "closeout matrix family `{}` drifted from the actual representative-path coverage set",
                    row.family_kind().as_str()
                ),
            ));
        }
        if !row.declare_once_parity_passed() {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingFamilyParity,
                format!(
                    "closeout matrix family `{}` must carry family-parity coverage",
                    row.family_kind().as_str()
                ),
            ));
        }
        if !authority.public_proof_parity_passed || !authority.diagnostic_parity_passed {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingPublicProjectionParity,
                "closeout matrix requires both public-proof and derived-diagnostic parity certification".to_string(),
            ));
        }
        if row.public_proof_parity_passed() != authority.public_proof_parity_passed
            || row.diagnostic_parity_passed() != authority.diagnostic_parity_passed
        {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingPublicProjectionParity,
                format!(
                    "closeout matrix family `{}` drifted from the current public-projection parity posture",
                    row.family_kind().as_str()
                ),
            ));
        }
        if !row.readiness_handoff_passed() {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingReadinessCoverage,
                format!(
                    "closeout matrix family `{}` must publish readiness handoff certification",
                    row.family_kind().as_str()
                ),
            ));
        }
    }

    for family_kind in TouchedGraphParityFamilyKind::ALL {
        if !seen.contains(&family_kind) {
            return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
                WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingCertifiedFamilyRow,
                format!(
                    "closeout matrix is missing family kind `{}`",
                    family_kind.as_str()
                ),
            ));
        }
    }
    if matrix.closeout_architecture_claim_digest() != authority.closeout_architecture_claim_digest {
        return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CountMismatch,
            "closeout matrix architecture claim digest drifted from the live ledger".to_string(),
        ));
    }

    Ok(())
}

fn assert_family_count(
    family_kind: TouchedGraphParityFamilyKind,
    column: &str,
    actual: usize,
    expected: usize,
) -> Result<(), WorthTouchedGraphCrossFamilyCloseoutMatrixError> {
    if actual != expected {
        return Err(WorthTouchedGraphCrossFamilyCloseoutMatrixError::new(
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::CountMismatch,
            format!(
                "closeout matrix family `{}` column `{column}` drifted: expected {expected}, found {actual}",
                family_kind.as_str()
            ),
        ));
    }
    Ok(())
}

impl WorthTouchedGraphCrossFamilyCloseoutMatrixError {
    pub(crate) fn new(
        kind: WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
