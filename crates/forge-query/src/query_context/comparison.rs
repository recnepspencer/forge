use crate::identity::ResultDigest;

use super::basis::{
    AdmittedQueryBasisContext, ComparisonBasisFamily, QueryContextAdmissionError,
    QueryContextAdmissionFailureClass, QueryContextDriftOutcome, QueryContextFamily,
};
use super::execution::QueryContextExecutionArtifact;
use super::performance::QueryContextCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedDiffQueryContext {
    left: AdmittedQueryBasisContext,
    right: AdmittedQueryBasisContext,
    family: ComparisonBasisFamily,
    drift_outcome: QueryContextDriftOutcome,
    counters: QueryContextCounters,
}

impl AdmittedDiffQueryContext {
    pub fn left(&self) -> &AdmittedQueryBasisContext {
        &self.left
    }

    pub fn right(&self) -> &AdmittedQueryBasisContext {
        &self.right
    }

    pub fn family(&self) -> &ComparisonBasisFamily {
        &self.family
    }

    pub fn drift_outcome(&self) -> &QueryContextDriftOutcome {
        &self.drift_outcome
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

    pub fn rows(&self) -> &[QueryDiffChangeRow] {
        &self.rows
    }
}

pub fn bind_diff_query_context(
    left: &AdmittedQueryBasisContext,
    right: &AdmittedQueryBasisContext,
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
                QueryContextCounters::for_diff_denial(false),
            ));
        }
    };

    if left.query_digest() != right.query_digest() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::DiffScopeMismatch,
            "diff comparison forbids query reinterpretation across basis contexts",
            QueryContextCounters::for_diff_denial(true),
        ));
    }

    if left.basis_digest() == right.basis_digest() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::BroadComparisonForbidden,
            "diff comparison requires two distinct admitted bases",
            QueryContextCounters::for_diff_denial(false),
        ));
    }

    Ok(AdmittedDiffQueryContext {
        left: left.clone(),
        right: right.clone(),
        family,
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        counters: QueryContextCounters::for_diff(),
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
            QueryContextCounters::for_diff_denial(true),
        ));
    }

    let width = left_result
        .payload()
        .len()
        .max(right_result.payload().len());
    let mut rows = Vec::with_capacity(width);
    for ordinal in 0..width {
        let left_value = left_result.payload().get(ordinal).cloned();
        let right_value = right_result.payload().get(ordinal).cloned();
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
        rows,
    })
}
