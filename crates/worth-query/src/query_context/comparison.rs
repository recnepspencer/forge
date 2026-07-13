use crate::identity::ResultDigest;

use super::basis::{
    ComparisonBasisFamily, QueryContextAdmissionError, QueryContextAdmissionFailureClass,
    QueryContextDriftOutcome, QueryContextFamily,
};
use super::execution::QueryContextExecutionArtifact;
use super::performance::{
    QueryContextBudgetClass, QueryContextCostClass, QueryContextCounters,
    QueryContextPredictionDriftOutcome, QueryContextPredictionReport,
};
use super::scoped::ScopedQueryBasisContext;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedDiffQueryContext {
    left: ScopedQueryBasisContext,
    right: ScopedQueryBasisContext,
    family: ComparisonBasisFamily,
    drift_outcome: QueryContextDriftOutcome,
    cost_class: QueryContextCostClass,
    budget_class: QueryContextBudgetClass,
    prediction_report: QueryContextPredictionReport,
    prediction_drift_outcome: QueryContextPredictionDriftOutcome,
    counters: QueryContextCounters,
}

impl AdmittedDiffQueryContext {
    pub fn left(&self) -> &ScopedQueryBasisContext {
        &self.left
    }

    pub fn right(&self) -> &ScopedQueryBasisContext {
        &self.right
    }

    pub fn family(&self) -> &ComparisonBasisFamily {
        &self.family
    }

    pub fn drift_outcome(&self) -> &QueryContextDriftOutcome {
        &self.drift_outcome
    }

    pub fn cost_class(&self) -> &QueryContextCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &QueryContextBudgetClass {
        &self.budget_class
    }

    pub fn prediction_report(&self) -> &QueryContextPredictionReport {
        &self.prediction_report
    }

    pub fn prediction_drift_outcome(&self) -> &QueryContextPredictionDriftOutcome {
        &self.prediction_drift_outcome
    }

    pub fn counters(&self) -> &QueryContextCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryDiffChangeFamily {
    Unchanged,
    Added,
    Removed,
    Modified,
}

impl QueryDiffChangeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Modified => "modified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDiffChangeRow {
    ordinal: usize,
    family: QueryDiffChangeFamily,
    left_value: Option<String>,
    right_value: Option<String>,
}

impl QueryDiffChangeRow {
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub fn family(&self) -> &QueryDiffChangeFamily {
        &self.family
    }

    pub fn left_value(&self) -> Option<&str> {
        self.left_value.as_deref()
    }

    pub fn right_value(&self) -> Option<&str> {
        self.right_value.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDiffChangeSetArtifact {
    query_digest: String,
    comparison_basis_family: ComparisonBasisFamily,
    left_basis_digest: String,
    right_basis_digest: String,
    result_digest: String,
    prediction_drift_outcome: QueryContextPredictionDriftOutcome,
    rows: Vec<QueryDiffChangeRow>,
}

impl QueryDiffChangeSetArtifact {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn comparison_basis_family(&self) -> &ComparisonBasisFamily {
        &self.comparison_basis_family
    }

    pub fn left_basis_digest(&self) -> &str {
        &self.left_basis_digest
    }

    pub fn right_basis_digest(&self) -> &str {
        &self.right_basis_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn prediction_drift_outcome(&self) -> &QueryContextPredictionDriftOutcome {
        &self.prediction_drift_outcome
    }

    pub fn rows(&self) -> &[QueryDiffChangeRow] {
        &self.rows
    }
}

#[cfg(test)]
pub(crate) fn reject_raw_storage_delta_access() -> QueryContextAdmissionError {
    QueryContextAdmissionError::new(
        QueryContextAdmissionFailureClass::RawStorageDeltaLeakageForbidden,
        "diff comparison exposes query-shaped change sets only and forbids raw storage delta access",
        QueryContextCounters::for_diff_denial(false, true),
    )
}

pub fn bind_diff_query_context(
    left: &ScopedQueryBasisContext,
    right: &ScopedQueryBasisContext,
) -> Result<AdmittedDiffQueryContext, QueryContextAdmissionError> {
    let family = match (left.family(), right.family()) {
        (QueryContextFamily::CurrentBranchHead, QueryContextFamily::BranchHead)
        | (QueryContextFamily::BranchHead, QueryContextFamily::CurrentBranchHead)
        | (QueryContextFamily::BranchHead, QueryContextFamily::BranchHead) => {
            ComparisonBasisFamily::BranchToBranch
        }
        (QueryContextFamily::CurrentBranchHead, QueryContextFamily::HistoricalSnapshot)
        | (QueryContextFamily::CurrentBranchHead, QueryContextFamily::HistoricalCommit)
        | (QueryContextFamily::HistoricalSnapshot, QueryContextFamily::CurrentBranchHead)
        | (QueryContextFamily::HistoricalCommit, QueryContextFamily::CurrentBranchHead) => {
            ComparisonBasisFamily::CurrentToHistorical
        }
        (QueryContextFamily::HistoricalSnapshot, QueryContextFamily::HistoricalSnapshot)
        | (QueryContextFamily::HistoricalSnapshot, QueryContextFamily::HistoricalCommit)
        | (QueryContextFamily::HistoricalCommit, QueryContextFamily::HistoricalSnapshot)
        | (QueryContextFamily::HistoricalCommit, QueryContextFamily::HistoricalCommit) => {
            ComparisonBasisFamily::HistoricalToHistorical
        }
        (QueryContextFamily::PreviewDerivedHistorical, QueryContextFamily::CurrentBranchHead)
        | (QueryContextFamily::PreviewDerivedHistorical, QueryContextFamily::BranchHead)
        | (QueryContextFamily::CurrentBranchHead, QueryContextFamily::PreviewDerivedHistorical)
        | (QueryContextFamily::BranchHead, QueryContextFamily::PreviewDerivedHistorical) => {
            ComparisonBasisFamily::PreviewToAuthoritative
        }
        _ => {
            return Err(QueryContextAdmissionError::new(
                QueryContextAdmissionFailureClass::AmbiguousComparisonBasis,
                "diff comparison requires an explicit supported basis pairing",
                QueryContextCounters::for_diff_denial(false, false),
            ));
        }
    };

    if left.query_digest() != right.query_digest() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::DiffScopeMismatch,
            "diff comparison forbids query reinterpretation across basis contexts",
            QueryContextCounters::for_diff_denial(true, false),
        ));
    }

    if left.basis_digest() == right.basis_digest() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::BroadComparisonForbidden,
            "diff comparison requires two distinct admitted bases",
            QueryContextCounters::for_diff_denial(false, true),
        ));
    }

    let predicted_row_width = left
        .predicted_result_shape_width()
        .max(right.predicted_result_shape_width());

    Ok(AdmittedDiffQueryContext {
        left: left.clone(),
        right: right.clone(),
        family,
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        cost_class: QueryContextCostClass::DiffComparisonBounded,
        budget_class: QueryContextBudgetClass::ComparisonBounded,
        prediction_report: QueryContextPredictionReport::for_diff_binding(predicted_row_width),
        prediction_drift_outcome: QueryContextPredictionDriftOutcome::PendingComparison,
        counters: QueryContextCounters::for_diff(predicted_row_width),
    })
}

pub fn shape_query_diff_change_set(
    context: &AdmittedDiffQueryContext,
    left_result: &QueryContextExecutionArtifact,
    right_result: &QueryContextExecutionArtifact,
) -> Result<QueryDiffChangeSetArtifact, QueryContextAdmissionError> {
    if left_result.query_digest() != context.left().query_digest()
        || right_result.query_digest() != context.right().query_digest()
    {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::DiffScopeMismatch,
            "diff change-set shaping requires execution artifacts that match the admitted context pair",
            QueryContextCounters::for_diff_denial(true, false),
        ));
    }

    if left_result.basis_digest() != context.left().basis_digest()
        || right_result.basis_digest() != context.right().basis_digest()
    {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::ComparisonShapeMismatch,
            "diff change-set shaping requires execution artifacts bound to the admitted basis pair",
            QueryContextCounters::for_diff_denial(false, false),
        ));
    }

    if left_result.counters().result_shape_width() != right_result.counters().result_shape_width() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::ComparisonShapeMismatch,
            "diff change-set shaping requires both admitted bases to preserve one declared result-shape width",
            QueryContextCounters::for_diff_denial(false, false),
        ));
    }

    if context.prediction_report().comparison_row_width() > 1 {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::ComparisonBroadeningRequired,
            "phase-three diff shaping is intentionally narrow and denies multi-row comparisons that would require broader collection or lineage semantics",
            QueryContextCounters::for_diff_denial(false, true),
        ));
    }

    let width = left_result.rows().len().max(right_result.rows().len());

    if width > context.prediction_report().comparison_row_width() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::ComparisonBroadeningRequired,
            "diff change-set shaping denies when the admitted basis pair would require hidden broadening or reconstruction",
            QueryContextCounters::for_diff_denial(false, true),
        ));
    }

    let mut rows = Vec::with_capacity(width);
    for ordinal in 0..width {
        let left_value = left_result.rows().get(ordinal).cloned();
        let right_value = right_result.rows().get(ordinal).cloned();
        let family = match (&left_value, &right_value) {
            (Some(left), Some(right)) if left == right => QueryDiffChangeFamily::Unchanged,
            (Some(_), Some(_)) => QueryDiffChangeFamily::Modified,
            (None, Some(_)) => QueryDiffChangeFamily::Added,
            (Some(_), None) => QueryDiffChangeFamily::Removed,
            (None, None) => continue,
        };

        rows.push(QueryDiffChangeRow {
            ordinal,
            family,
            left_value,
            right_value,
        });
    }

    let result_digest = ResultDigest::from_parts(
        &rows
            .iter()
            .flat_map(|row| {
                [
                    format!("ordinal:{}", row.ordinal()),
                    format!("family:{}", row.family().as_str()),
                    format!("left:{}", row.left_value().unwrap_or("none")),
                    format!("right:{}", row.right_value().unwrap_or("none")),
                ]
            })
            .chain(std::iter::once(format!(
                "comparison_family:{}",
                context.family().as_str()
            )))
            .chain(std::iter::once(format!(
                "left_basis:{}",
                context.left().basis_digest()
            )))
            .chain(std::iter::once(format!(
                "right_basis:{}",
                context.right().basis_digest()
            )))
            .collect::<Vec<_>>(),
    );

    Ok(QueryDiffChangeSetArtifact {
        query_digest: context.left().query_digest().to_string(),
        comparison_basis_family: context.family().clone(),
        left_basis_digest: context.left().basis_digest().to_string(),
        right_basis_digest: context.right().basis_digest().to_string(),
        result_digest: result_digest.as_str().to_string(),
        prediction_drift_outcome: QueryContextPredictionDriftOutcome::WithinBudget,
        rows,
    })
}
