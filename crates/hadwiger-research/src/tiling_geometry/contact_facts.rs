use crate::candidate_screening::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use crate::domain_artifacts::core_artifact::{
    canonical_digest_token, impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::artifact_core;
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::canonical_geometry_digest::report_payload;
use super::cell_artifacts::TilingCell;
use super::tiling_geometry_errors::TilingGeometryError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TilingContactRole {
    SameColorConflictCandidate,
    BoundaryContact,
    DiameterSafety,
    MinkowskiUnitContact,
}

impl TilingContactRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SameColorConflictCandidate => "same_color_conflict_candidate",
            Self::BoundaryContact => "boundary_contact",
            Self::DiameterSafety => "diameter_safety",
            Self::MinkowskiUnitContact => "minkowski_unit_contact",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingContactFact {
    left_tile_id: String,
    right_tile_id: String,
    role: TilingContactRole,
}

impl TilingContactFact {
    pub(crate) fn exact_replay(
        left_tile_id: impl Into<String>,
        right_tile_id: impl Into<String>,
        role: TilingContactRole,
    ) -> Result<Self, TilingGeometryError> {
        let mut left_tile_id = left_tile_id.into();
        let mut right_tile_id = right_tile_id.into();
        if left_tile_id == right_tile_id {
            return Err(TilingGeometryError::SameTileContact {
                tile_id: left_tile_id,
            });
        }
        if right_tile_id < left_tile_id {
            std::mem::swap(&mut left_tile_id, &mut right_tile_id);
        }
        Ok(Self {
            left_tile_id,
            right_tile_id,
            role,
        })
    }

    pub fn left_tile_id(&self) -> &str {
        &self.left_tile_id
    }

    pub fn right_tile_id(&self) -> &str {
        &self.right_tile_id
    }

    pub fn role(&self) -> TilingContactRole {
        self.role
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.left_tile_id,
            self.right_tile_id,
            self.role.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingGeometryCounters {
    tile_count: usize,
    boundary_ownership_rows_checked: usize,
    contact_pairs_checked: usize,
    query_declarations_performed: usize,
    screening_evaluations_performed: usize,
}

impl TilingGeometryCounters {
    pub(crate) fn new(
        tile_count: usize,
        boundary_rows: usize,
        contact_pairs: usize,
        query_declarations: usize,
        screening_evaluations: usize,
    ) -> Self {
        Self {
            tile_count,
            boundary_ownership_rows_checked: boundary_rows,
            contact_pairs_checked: contact_pairs,
            query_declarations_performed: query_declarations,
            screening_evaluations_performed: screening_evaluations,
        }
    }

    pub fn tile_count(&self) -> usize {
        self.tile_count
    }

    pub fn boundary_ownership_rows_checked(&self) -> usize {
        self.boundary_ownership_rows_checked
    }

    pub fn contact_pairs_checked(&self) -> usize {
        self.contact_pairs_checked
    }

    pub fn query_declarations_performed(&self) -> usize {
        self.query_declarations_performed
    }

    pub fn screening_evaluations_performed(&self) -> usize {
        self.screening_evaluations_performed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingBoundaryOwnershipReport {
    core: HadwigerArtifactCore,
    evaluation: CandidateScreeningEvaluation,
    counters: TilingGeometryCounters,
}

impl TilingBoundaryOwnershipReport {
    pub(crate) fn checked(
        cell: &TilingCell,
        evaluation: CandidateScreeningEvaluation,
    ) -> Result<Self, TilingGeometryError> {
        let counters = TilingGeometryCounters::new(cell.tile_count(), cell.tile_count(), 0, 1, 1);
        let query_digest = query_digest_from_evidence(evaluation.evidence());
        let evaluation_digest = canonical_digest_token(evaluation.artifact_digest().canonical());
        let core = artifact_core(
            HadwigerArtifactKind::TilingBoundaryOwnershipReport,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "tiling_boundary_ownership_report".to_string(),
            },
            vec![cell.reference(), evaluation.reference()],
            report_payload(
                "forge.hadwiger.tiling_boundary_ownership_report.v1",
                &cell.reference().stable_token(),
                query_digest.as_deref(),
                Some(&evaluation_digest),
                evaluation.evidence(),
                &counters,
            ),
        )?;
        Ok(Self {
            core,
            evaluation,
            counters,
        })
    }

    pub fn evaluation(&self) -> &CandidateScreeningEvaluation {
        &self.evaluation
    }

    pub fn counters(&self) -> &TilingGeometryCounters {
        &self.counters
    }

    pub fn query_declaration_digest(&self) -> Option<String> {
        query_digest_from_evidence(self.evaluation.evidence())
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingBoundaryOwnershipReport, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingContactReplayReport {
    core: HadwigerArtifactCore,
    contact_fact: TilingContactFact,
    evaluation: CandidateScreeningEvaluation,
    contact_witness_declaration_digest: Option<String>,
    counters: TilingGeometryCounters,
}

impl TilingContactReplayReport {
    pub(crate) fn checked(
        cell: &TilingCell,
        contact_fact: TilingContactFact,
        evaluation: CandidateScreeningEvaluation,
        contact_witness_declaration_digest: Option<String>,
    ) -> Result<Self, TilingGeometryError> {
        let counters = TilingGeometryCounters::new(
            cell.tile_count(),
            0,
            1,
            1 + usize::from(contact_witness_declaration_digest.is_some()),
            1,
        );
        let query_digest = query_digest_from_evidence(evaluation.evidence());
        let evaluation_digest = canonical_digest_token(evaluation.artifact_digest().canonical());
        let evidence = format!(
            "contact={};contact_witness_declaration_digest={};verdict={};{}",
            contact_fact.stable_token(),
            contact_witness_declaration_digest
                .as_deref()
                .unwrap_or("none"),
            evaluation.verdict().as_str(),
            evaluation.evidence()
        );
        let core = artifact_core(
            HadwigerArtifactKind::TilingContactReplayReport,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "tiling_contact_replay_report".to_string(),
            },
            vec![cell.reference(), evaluation.reference()],
            report_payload(
                "forge.hadwiger.tiling_contact_replay_report.v1",
                &cell.reference().stable_token(),
                query_digest.as_deref(),
                Some(&evaluation_digest),
                &evidence,
                &counters,
            ),
        )?;
        Ok(Self {
            core,
            contact_fact,
            evaluation,
            contact_witness_declaration_digest,
            counters,
        })
    }

    pub fn contact_fact(&self) -> &TilingContactFact {
        &self.contact_fact
    }

    pub fn evaluation(&self) -> &CandidateScreeningEvaluation {
        &self.evaluation
    }

    pub fn counters(&self) -> &TilingGeometryCounters {
        &self.counters
    }

    pub fn is_exact_replay(&self) -> bool {
        matches!(
            self.evaluation.verdict(),
            CandidateScreeningVerdict::Rejected | CandidateScreeningVerdict::Passed
        )
    }

    pub fn query_declaration_digest(&self) -> Option<String> {
        query_digest_from_evidence(self.evaluation.evidence())
    }

    pub fn contact_witness_declaration_digest(&self) -> Option<&str> {
        self.contact_witness_declaration_digest.as_deref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingContactReplayReport, core);

fn query_digest_from_evidence(evidence: &str) -> Option<String> {
    evidence
        .split(';')
        .find_map(|part| part.strip_prefix("query_declaration_digest="))
        .map(str::to_string)
}
