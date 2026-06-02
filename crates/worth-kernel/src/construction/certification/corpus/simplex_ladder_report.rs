use forge_query::facade::ForgeQueryWorkspace;
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

use super::cases::{
    simplex_world_collapsed_admitted_local_or_exact_intent,
    simplex_world_collapsed_explicit_exhaustion_intent,
    simplex_world_collapsed_threshold_rejected_intent,
};
use super::{
    prepare_primitive_construction_corpus_replay_siege, PrimitiveConstructionCorpusParameterRole,
    PrimitiveConstructionCorpusReplaySiegeError, PrimitiveConstructionCorpusReplaySiegeRow,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::request::PrimitiveConstructionGeometryError;
use crate::construction::{
    prepare_primitive_construction_query_inspection_parity_report,
    prepare_primitive_construction_query_projection_consumption_receipt_report,
    prepare_primitive_construction_realization_exhaustion_report,
    prepare_primitive_construction_realization_strategy_report, PrimitiveConstructionFamily,
    PrimitiveConstructionIntent, PrimitiveConstructionPhaseError,
    PrimitiveConstructionQueryEntryError, PrimitiveConstructionQueryInspectionParityError,
    PrimitiveConstructionQueryProjectionConsumptionReceiptError,
    PrimitiveConstructionRealizationExhaustionStatus, PrimitiveConstructionResultError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionSimplexQuerySurfaceStatus {
    Available,
    UnavailableByTypedAdmissionRejection,
    UnavailableByRealizationExhaustion,
}

impl PrimitiveConstructionSimplexQuerySurfaceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::UnavailableByTypedAdmissionRejection => {
                "unavailable_by_typed_admission_rejection"
            }
            Self::UnavailableByRealizationExhaustion => "unavailable_by_realization_exhaustion",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionSimplexRealizationLadderRow {
    scenario_id: String,
    parameter_role: PrimitiveConstructionCorpusParameterRole,
    direct_selected_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    normalization_disposition: Option<PrimitiveNormalizationDisposition>,
    exhaustion_status: PrimitiveConstructionRealizationExhaustionStatus,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    direct_strategy_digest: String,
    direct_exhaustion_digest: String,
    query_surface_status: PrimitiveConstructionSimplexQuerySurfaceStatus,
    inspection_digest: Option<String>,
    projection_consumption_digest: Option<String>,
    corpus_row_digest: String,
    replay_digest: String,
    branch_local_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionSimplexRealizationLadderRow {
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn parameter_role(&self) -> PrimitiveConstructionCorpusParameterRole {
        self.parameter_role
    }

    pub fn direct_selected_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.direct_selected_strategy
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.normalization_disposition
    }

    pub fn exhaustion_status(&self) -> PrimitiveConstructionRealizationExhaustionStatus {
        self.exhaustion_status
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub fn direct_strategy_digest(&self) -> &str {
        &self.direct_strategy_digest
    }

    pub fn direct_exhaustion_digest(&self) -> &str {
        &self.direct_exhaustion_digest
    }

    pub fn query_surface_status(&self) -> PrimitiveConstructionSimplexQuerySurfaceStatus {
        self.query_surface_status
    }

    pub fn inspection_digest(&self) -> Option<&str> {
        self.inspection_digest.as_deref()
    }

    pub fn projection_consumption_digest(&self) -> Option<&str> {
        self.projection_consumption_digest.as_deref()
    }

    pub fn corpus_row_digest(&self) -> &str {
        &self.corpus_row_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn branch_local_digest(&self) -> &str {
        &self.branch_local_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionSimplexRealizationStrategyLadderReport {
    rows: Vec<PrimitiveConstructionSimplexRealizationLadderRow>,
    report_digest: String,
}

impl PrimitiveConstructionSimplexRealizationStrategyLadderReport {
    pub fn rows(&self) -> &[PrimitiveConstructionSimplexRealizationLadderRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        scenario_id: &str,
    ) -> Option<&PrimitiveConstructionSimplexRealizationLadderRow> {
        self.rows
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionSimplexRealizationLadderReportError {
    Corpus(PrimitiveConstructionCorpusReplaySiegeError),
    Inspection(PrimitiveConstructionQueryInspectionParityError),
    Projection(PrimitiveConstructionQueryProjectionConsumptionReceiptError),
    MissingCorpusRow(PrimitiveConstructionCorpusParameterRole),
}

impl std::fmt::Display for PrimitiveConstructionSimplexRealizationLadderReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Corpus(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::MissingCorpusRow(role) => {
                write!(f, "missing simplex corpus row for {}", role.as_str())
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionSimplexRealizationLadderReportError {}

pub fn prepare_primitive_construction_simplex_realization_strategy_ladder_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionSimplexRealizationStrategyLadderReport,
    PrimitiveConstructionSimplexRealizationLadderReportError,
> {
    let corpus = prepare_primitive_construction_corpus_replay_siege(workspace)
        .map_err(PrimitiveConstructionSimplexRealizationLadderReportError::Corpus)?;
    let rows = [
        (
            "simplex_world_collapsed_admitted_local_or_exact",
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            simplex_world_collapsed_admitted_local_or_exact_intent(),
        ),
        (
            "simplex_world_collapsed_threshold_rejected",
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
            simplex_world_collapsed_threshold_rejected_intent(),
        ),
        (
            "simplex_world_collapsed_explicit_exhaustion",
            PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion,
            simplex_world_collapsed_explicit_exhaustion_intent(),
        ),
    ]
    .into_iter()
    .map(|(scenario_id, role, intent)| {
        simplex_ladder_row(workspace, &corpus, scenario_id, role, intent)
    })
    .collect::<Result<Vec<_>, _>>()?;
    let report_digest = digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    Ok(
        PrimitiveConstructionSimplexRealizationStrategyLadderReport {
            rows,
            report_digest,
        },
    )
}

fn simplex_ladder_row(
    workspace: &mut ForgeQueryWorkspace,
    corpus: &super::PrimitiveConstructionCorpusReplaySiegeReport,
    scenario_id: &str,
    role: PrimitiveConstructionCorpusParameterRole,
    intent: PrimitiveConstructionIntent,
) -> Result<
    PrimitiveConstructionSimplexRealizationLadderRow,
    PrimitiveConstructionSimplexRealizationLadderReportError,
> {
    let direct_strategy =
        prepare_primitive_construction_realization_strategy_report(intent.clone());
    let direct_exhaustion =
        prepare_primitive_construction_realization_exhaustion_report(intent.clone());
    let query_surface = simplex_query_surface(workspace, intent)?;
    let corpus_row = simplex_corpus_row(corpus, role)
        .ok_or(PrimitiveConstructionSimplexRealizationLadderReportError::MissingCorpusRow(role))?;
    let normalization_disposition = direct_exhaustion
        .conditioning_witness()
        .map(|witness| witness.normalization_disposition());
    let inspection_digest = query_surface
        .inspection_digest
        .as_deref()
        .unwrap_or_default()
        .to_string();
    let projection_digest = query_surface
        .projection_consumption_digest
        .as_deref()
        .unwrap_or_default()
        .to_string();
    let row_digest = digest_owned_parts(&[
        scenario_id.to_string(),
        role.as_str().to_string(),
        direct_strategy.report_digest().to_string(),
        direct_exhaustion.report_digest().to_string(),
        query_surface.status.as_str().to_string(),
        inspection_digest,
        projection_digest,
        corpus_row.row_digest().to_string(),
    ]);

    Ok(PrimitiveConstructionSimplexRealizationLadderRow {
        scenario_id: scenario_id.to_string(),
        parameter_role: role,
        direct_selected_strategy: direct_strategy.selected_strategy(),
        attempted_strategies: direct_strategy.attempted_strategies().to_vec(),
        stability_class: direct_strategy.stability_class(),
        normalization_disposition,
        exhaustion_status: direct_exhaustion.status(),
        exhaustion_reason: direct_exhaustion.exhaustion_reason(),
        direct_strategy_digest: direct_strategy.report_digest().to_string(),
        direct_exhaustion_digest: direct_exhaustion.report_digest().to_string(),
        query_surface_status: query_surface.status,
        inspection_digest: query_surface.inspection_digest,
        projection_consumption_digest: query_surface.projection_consumption_digest,
        corpus_row_digest: corpus_row.row_digest().to_string(),
        replay_digest: corpus_row.replay_digest().to_string(),
        branch_local_digest: corpus_row.branch_local_digest().to_string(),
        row_digest,
    })
}

fn simplex_corpus_row(
    corpus: &super::PrimitiveConstructionCorpusReplaySiegeReport,
    role: PrimitiveConstructionCorpusParameterRole,
) -> Option<&PrimitiveConstructionCorpusReplaySiegeRow> {
    corpus.row_for(PrimitiveConstructionFamily::SimplexSolid, role)
}

struct PrimitiveConstructionSimplexQuerySurface {
    status: PrimitiveConstructionSimplexQuerySurfaceStatus,
    inspection_digest: Option<String>,
    projection_consumption_digest: Option<String>,
}

fn simplex_query_surface(
    workspace: &mut ForgeQueryWorkspace,
    intent: PrimitiveConstructionIntent,
) -> Result<
    PrimitiveConstructionSimplexQuerySurface,
    PrimitiveConstructionSimplexRealizationLadderReportError,
> {
    match prepare_primitive_construction_query_inspection_parity_report(workspace, intent.clone()) {
        Ok(inspection) => {
            let projection =
                prepare_primitive_construction_query_projection_consumption_receipt_report(
                    workspace, intent,
                )
                .map_err(PrimitiveConstructionSimplexRealizationLadderReportError::Projection)?;
            Ok(PrimitiveConstructionSimplexQuerySurface {
                status: PrimitiveConstructionSimplexQuerySurfaceStatus::Available,
                inspection_digest: Some(inspection.report_digest().to_string()),
                projection_consumption_digest: Some(projection.report_digest().to_string()),
            })
        }
        Err(PrimitiveConstructionQueryInspectionParityError::QueryEntry(
            PrimitiveConstructionQueryEntryError::Result(PrimitiveConstructionResultError::Phase(
                PrimitiveConstructionPhaseError::InvalidRequest {
                    family: PrimitiveConstructionFamily::SimplexSolid,
                    ..
                },
            )),
        )) => Ok(PrimitiveConstructionSimplexQuerySurface {
            status:
                PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByTypedAdmissionRejection,
            inspection_digest: None,
            projection_consumption_digest: None,
        }),
        Err(PrimitiveConstructionQueryInspectionParityError::QueryEntry(
            PrimitiveConstructionQueryEntryError::Result(PrimitiveConstructionResultError::Phase(
                PrimitiveConstructionPhaseError::Geometry(
                    PrimitiveConstructionGeometryError::RealizationExhausted(_),
                ),
            )),
        )) => Ok(PrimitiveConstructionSimplexQuerySurface {
            status:
                PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByRealizationExhaustion,
            inspection_digest: None,
            projection_consumption_digest: None,
        }),
        Err(error) => {
            Err(PrimitiveConstructionSimplexRealizationLadderReportError::Inspection(error))
        }
    }
}
