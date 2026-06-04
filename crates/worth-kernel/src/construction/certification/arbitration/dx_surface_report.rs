use crate::construction::digest::digest_owned_parts;

use super::{
    prepare_primitive_intent_arbitration_policy_report,
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
};
use worth_spatial::facade::arbitration::{SpatialIntentCandidate, SpatialIntentEscalation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationDxSurface {
    CommonPath,
    AdvancedPath,
    HumanEscalation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationDxSurfaceRow {
    case: PrimitiveConstructionIntentArbitrationPolicyCase,
    dx_surface: PrimitiveConstructionIntentArbitrationDxSurface,
    conflict_class: PrimitiveConstructionIntentArbitrationConflictClass,
    escalation: SpatialIntentEscalation,
    candidate_count: usize,
    blocked_candidate_count: usize,
    chosen_candidate: Option<SpatialIntentCandidate>,
    row_digest: String,
}

impl PrimitiveConstructionIntentArbitrationDxSurfaceRow {
    pub fn case(&self) -> PrimitiveConstructionIntentArbitrationPolicyCase {
        self.case
    }

    pub fn dx_surface(&self) -> PrimitiveConstructionIntentArbitrationDxSurface {
        self.dx_surface
    }

    pub fn conflict_class(&self) -> PrimitiveConstructionIntentArbitrationConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    pub fn blocked_candidate_count(&self) -> usize {
        self.blocked_candidate_count
    }

    pub fn chosen_candidate(&self) -> Option<SpatialIntentCandidate> {
        self.chosen_candidate
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationDxSurfaceReport {
    rows: Vec<PrimitiveConstructionIntentArbitrationDxSurfaceRow>,
    report_digest: String,
}

impl PrimitiveConstructionIntentArbitrationDxSurfaceReport {
    pub fn rows(&self) -> &[PrimitiveConstructionIntentArbitrationDxSurfaceRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionIntentArbitrationPolicyCase,
    ) -> Option<&PrimitiveConstructionIntentArbitrationDxSurfaceRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_intent_conflict_dx_surface_report() -> Result<
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
> {
    let policy_report = prepare_primitive_intent_arbitration_policy_report()?;
    let rows = policy_report
        .rows()
        .iter()
        .map(|row| {
            let dx_surface = match row.escalation() {
                SpatialIntentEscalation::AutoResolve(_) => {
                    PrimitiveConstructionIntentArbitrationDxSurface::CommonPath
                }
                SpatialIntentEscalation::PreserveCandidates
                | SpatialIntentEscalation::BlockedByMissingCapability(_) => {
                    PrimitiveConstructionIntentArbitrationDxSurface::AdvancedPath
                }
                SpatialIntentEscalation::AskForClarification => {
                    PrimitiveConstructionIntentArbitrationDxSurface::HumanEscalation
                }
            };
            let row_digest = digest_owned_parts(&[
                format!("{:?}", row.case()),
                format!("{dx_surface:?}"),
                format!("{:?}", row.conflict_class()),
                format!("{:?}", row.escalation()),
                row.candidates().len().to_string(),
                row.blocked_candidates().len().to_string(),
                format!("{:?}", row.chosen_candidate()),
            ]);
            PrimitiveConstructionIntentArbitrationDxSurfaceRow {
                case: row.case(),
                dx_surface,
                conflict_class: row.conflict_class(),
                escalation: row.escalation(),
                candidate_count: row.candidates().len(),
                blocked_candidate_count: row.blocked_candidates().len(),
                chosen_candidate: row.chosen_candidate(),
                row_digest,
            }
        })
        .collect::<Vec<_>>();
    let report_digest = digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest.clone())
            .collect::<Vec<_>>(),
    );
    Ok(PrimitiveConstructionIntentArbitrationDxSurfaceReport {
        rows,
        report_digest,
    })
}
