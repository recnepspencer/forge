use crate::basis::{BasisAuthorityFamily, ExecutionPreflightBundle};
use crate::historical::{
    HistoricalCapabilityDescriptor, HistoricalEvaluationAdmission,
    HistoricalMaterializationPathMetadata, RequestedHistoricalPathClass,
    ResolvedHistoricalPathClass,
};
use crate::identity::hash_parts;
use crate::preview::{AdmittedPreviewWorkflowFoundation, PreviewWorkflowFoundationRequest};

use super::historical::{
    drift_outcome_for_historical, historical_admission_class, materialization_path_cost_class,
    materialization_path_identity,
};
use super::performance::{
    HistoricalMaterializationCostClass, QueryContextBudgetClass, QueryContextCostClass,
    QueryContextCounters, QueryContextPredictionDriftOutcome, QueryContextPredictionReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextFamily {
    CurrentBranchHead,
    BranchHead,
    HistoricalSnapshot,
    HistoricalCommit,
    PreviewDerivedHistorical,
    DiffComparison,
}

impl QueryContextFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentBranchHead => "current_branch_head",
            Self::BranchHead => "branch_head",
            Self::HistoricalSnapshot => "historical_snapshot",
            Self::HistoricalCommit => "historical_commit",
            Self::PreviewDerivedHistorical => "preview_derived_historical",
            Self::DiffComparison => "diff_comparison",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComparisonBasisFamily {
    BranchToBranch,
    CurrentToHistorical,
    HistoricalToHistorical,
    PreviewToAuthoritative,
}

impl ComparisonBasisFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BranchToBranch => "branch_to_branch",
            Self::CurrentToHistorical => "current_to_historical",
            Self::HistoricalToHistorical => "historical_to_historical",
            Self::PreviewToAuthoritative => "preview_to_authoritative",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoricalAdmissionClass {
    RuntimeRetained,
    RuntimeReplay,
    RuntimeReconstruction,
    StoreDeferredDebt,
}

impl HistoricalAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeRetained => "runtime_retained",
            Self::RuntimeReplay => "runtime_replay",
            Self::RuntimeReconstruction => "runtime_reconstruction",
            Self::StoreDeferredDebt => "store_deferred_debt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextDriftOutcome {
    BasisExact,
    ExplicitHistoricalDenial,
    ExplicitComparisonDenial,
}

impl QueryContextDriftOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BasisExact => "basis_exact",
            Self::ExplicitHistoricalDenial => "explicit_historical_denial",
            Self::ExplicitComparisonDenial => "explicit_comparison_denial",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextAdmissionFailureClass {
    UnsupportedHistoricalBasis,
    InvalidBasisPairing,
    PreviewProvenanceRequired,
    DiffScopeMismatch,
    AmbiguousComparisonBasis,
    StoreBackedHistoricalDeferred,
    BroadComparisonForbidden,
    ComparisonShapeMismatch,
    ComparisonBroadeningRequired,
    HistoricalPathTooBroadDenied,
    RawStorageDeltaLeakageForbidden,
    BasisSubstitutionForbidden,
    NonQueryOwnedHistoricalArtifact,
    UnsupportedHistoricalMaterializationPathClass,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryContextAdmissionError {
    failure_class: QueryContextAdmissionFailureClass,
    message: &'static str,
    counters: QueryContextCounters,
}

impl QueryContextAdmissionError {
    pub fn failure_class(&self) -> &QueryContextAdmissionFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &QueryContextCounters {
        &self.counters
    }

    pub(crate) fn new(
        failure_class: QueryContextAdmissionFailureClass,
        message: &'static str,
        counters: QueryContextCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBasisContextRequest {
    family: QueryContextFamily,
    declared_basis_label: String,
}

impl QueryBasisContextRequest {
    pub fn current_branch_head() -> Self {
        Self {
            family: QueryContextFamily::CurrentBranchHead,
            declared_basis_label: "current".to_string(),
        }
    }

    pub fn branch_head(branch_identity: impl Into<String>) -> Self {
        Self {
            family: QueryContextFamily::BranchHead,
            declared_basis_label: branch_identity.into(),
        }
    }

    pub fn historical_snapshot(basis_identity: impl Into<String>) -> Self {
        Self {
            family: QueryContextFamily::HistoricalSnapshot,
            declared_basis_label: basis_identity.into(),
        }
    }

    pub fn historical_commit(basis_identity: impl Into<String>) -> Self {
        Self {
            family: QueryContextFamily::HistoricalCommit,
            declared_basis_label: basis_identity.into(),
        }
    }

    pub fn preview_derived_historical(preview_identity: impl Into<String>) -> Self {
        Self {
            family: QueryContextFamily::PreviewDerivedHistorical,
            declared_basis_label: preview_identity.into(),
        }
    }

    pub fn family(&self) -> &QueryContextFamily {
        &self.family
    }

    pub fn declared_basis_label(&self) -> &str {
        &self.declared_basis_label
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum QueryBasisBindingEvidence {
    Runtime {
        preflight: ExecutionPreflightBundle,
    },
    Historical {
        query_preflight: ExecutionPreflightBundle,
        admission: HistoricalEvaluationAdmission,
        metadata: HistoricalMaterializationPathMetadata,
    },
    PreviewDerived {
        foundation: AdmittedPreviewWorkflowFoundation,
    },
}

pub(crate) enum QueryBasisBindingEvidenceView<'a> {
    Runtime {
        preflight: &'a ExecutionPreflightBundle,
    },
    Historical {
        query_preflight: &'a ExecutionPreflightBundle,
        admission: &'a HistoricalEvaluationAdmission,
        metadata: &'a HistoricalMaterializationPathMetadata,
    },
    PreviewDerived {
        foundation: &'a AdmittedPreviewWorkflowFoundation,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBasisContextBinding {
    request: QueryBasisContextRequest,
    query_digest: String,
    basis_digest: String,
    basis_authority_family: BasisAuthorityFamily,
    drift_outcome: QueryContextDriftOutcome,
    cost_class: QueryContextCostClass,
    budget_class: QueryContextBudgetClass,
    historical_admission_class: Option<HistoricalAdmissionClass>,
    historical_materialization_cost_class: Option<HistoricalMaterializationCostClass>,
    materialization_path_identity_source: Option<String>,
    preview_provenance_identity_source: Option<String>,
    prediction_report: Option<QueryContextPredictionReport>,
    prediction_drift_outcome: Option<QueryContextPredictionDriftOutcome>,
    evidence: QueryBasisBindingEvidence,
    counters: QueryContextCounters,
}

impl QueryBasisContextBinding {
    pub fn request(&self) -> &QueryBasisContextRequest {
        &self.request
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn basis_authority_family(&self) -> &BasisAuthorityFamily {
        &self.basis_authority_family
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

    pub fn historical_admission_class(&self) -> Option<&HistoricalAdmissionClass> {
        self.historical_admission_class.as_ref()
    }

    pub fn historical_materialization_cost_class(
        &self,
    ) -> Option<&HistoricalMaterializationCostClass> {
        self.historical_materialization_cost_class.as_ref()
    }

    pub fn materialization_path_identity_source(&self) -> Option<&str> {
        self.materialization_path_identity_source.as_deref()
    }

    pub fn preview_provenance_identity_source(&self) -> Option<&str> {
        self.preview_provenance_identity_source.as_deref()
    }

    pub fn prediction_report(&self) -> Option<&QueryContextPredictionReport> {
        self.prediction_report.as_ref()
    }

    pub fn prediction_drift_outcome(&self) -> Option<&QueryContextPredictionDriftOutcome> {
        self.prediction_drift_outcome.as_ref()
    }

    pub fn counters(&self) -> &QueryContextCounters {
        &self.counters
    }

    pub(crate) fn evidence(&self) -> QueryBasisBindingEvidenceView<'_> {
        match &self.evidence {
            QueryBasisBindingEvidence::Runtime { preflight } => {
                QueryBasisBindingEvidenceView::Runtime { preflight }
            }
            QueryBasisBindingEvidence::Historical {
                query_preflight,
                admission,
                metadata,
            } => QueryBasisBindingEvidenceView::Historical {
                query_preflight,
                admission,
                metadata,
            },
            QueryBasisBindingEvidence::PreviewDerived { foundation } => {
                QueryBasisBindingEvidenceView::PreviewDerived { foundation }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedQueryBasisContext {
    binding: QueryBasisContextBinding,
}

impl AdmittedQueryBasisContext {
    pub fn family(&self) -> &QueryContextFamily {
        self.binding.request.family()
    }

    pub fn query_digest(&self) -> &str {
        self.binding.query_digest()
    }

    pub fn basis_digest(&self) -> &str {
        self.binding.basis_digest()
    }

    pub fn basis_authority_family(&self) -> &BasisAuthorityFamily {
        self.binding.basis_authority_family()
    }

    pub fn cost_class(&self) -> &QueryContextCostClass {
        self.binding.cost_class()
    }

    pub fn budget_class(&self) -> &QueryContextBudgetClass {
        self.binding.budget_class()
    }

    pub fn historical_admission_class(&self) -> Option<&HistoricalAdmissionClass> {
        self.binding.historical_admission_class()
    }

    pub fn historical_materialization_cost_class(
        &self,
    ) -> Option<&HistoricalMaterializationCostClass> {
        self.binding.historical_materialization_cost_class()
    }

    pub fn materialization_path_identity_source(&self) -> Option<&str> {
        self.binding.materialization_path_identity_source()
    }

    pub fn preview_provenance_identity_source(&self) -> Option<&str> {
        self.binding.preview_provenance_identity_source()
    }

    pub fn prediction_report(&self) -> Option<&QueryContextPredictionReport> {
        self.binding.prediction_report()
    }

    pub fn prediction_drift_outcome(&self) -> Option<&QueryContextPredictionDriftOutcome> {
        self.binding.prediction_drift_outcome()
    }

    pub fn counters(&self) -> &QueryContextCounters {
        self.binding.counters()
    }

    pub fn drift_outcome(&self) -> &QueryContextDriftOutcome {
        self.binding.drift_outcome()
    }

    pub(crate) fn predicted_result_shape_width(&self) -> usize {
        match self.binding.evidence() {
            QueryBasisBindingEvidenceView::Runtime { preflight } => {
                preflight.plan().result_shape().binding_count()
            }
            QueryBasisBindingEvidenceView::Historical {
                query_preflight, ..
            } => query_preflight.plan().result_shape().binding_count(),
            QueryBasisBindingEvidenceView::PreviewDerived { foundation } => {
                foundation.shape_check_width()
            }
        }
    }

    pub(crate) fn binding(&self) -> &QueryBasisContextBinding {
        &self.binding
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextBindingSource<'a> {
    RuntimeCurrent(&'a ExecutionPreflightBundle),
    RuntimeBranch(&'a ExecutionPreflightBundle),
    Historical {
        query_preflight: &'a ExecutionPreflightBundle,
        admission: &'a HistoricalEvaluationAdmission,
        metadata: &'a HistoricalMaterializationPathMetadata,
    },
    HistoricalCapability(&'a HistoricalCapabilityDescriptor),
    PreviewDerivedHistorical(&'a AdmittedPreviewWorkflowFoundation),
}

pub fn bind_query_basis_context(
    request: QueryBasisContextRequest,
    source: QueryContextBindingSource<'_>,
) -> Result<QueryBasisContextBinding, QueryContextAdmissionError> {
    match source {
        QueryContextBindingSource::RuntimeCurrent(preflight) => {
            bind_runtime_context(request, preflight, true)
        }
        QueryContextBindingSource::RuntimeBranch(preflight) => {
            bind_runtime_context(request, preflight, false)
        }
        QueryContextBindingSource::Historical {
            query_preflight,
            admission,
            metadata,
        } => bind_historical_context(request, query_preflight, admission, metadata),
        QueryContextBindingSource::HistoricalCapability(capability) => {
            if capability.admitted_path_class().is_none() {
                return Err(QueryContextAdmissionError::new(
                    QueryContextAdmissionFailureClass::StoreBackedHistoricalDeferred,
                    "store-backed historical context remains explicit deferred debt",
                    QueryContextCounters::for_denial(true, false),
                ));
            }

            Err(QueryContextAdmissionError::new(
                QueryContextAdmissionFailureClass::NonQueryOwnedHistoricalArtifact,
                "historical capability descriptors are not admitted query-owned basis artifacts",
                QueryContextCounters::for_denial(true, false),
            ))
        }
        QueryContextBindingSource::PreviewDerivedHistorical(foundation) => {
            bind_preview_context(request, foundation)
        }
    }
}

pub fn admit_query_basis_context(
    binding: QueryBasisContextBinding,
) -> Result<AdmittedQueryBasisContext, QueryContextAdmissionError> {
    if matches!(
        binding.drift_outcome(),
        QueryContextDriftOutcome::ExplicitHistoricalDenial
    ) {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::UnsupportedHistoricalBasis,
            "denied historical basis cannot be admitted",
            binding.counters().clone(),
        ));
    }

    Ok(AdmittedQueryBasisContext { binding })
}

pub(crate) fn bind_runtime_context(
    request: QueryBasisContextRequest,
    preflight: &ExecutionPreflightBundle,
    current: bool,
) -> Result<QueryBasisContextBinding, QueryContextAdmissionError> {
    if preflight.basis().identity().authority_family() != &BasisAuthorityFamily::Runtime {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::InvalidBasisPairing,
            "runtime query contexts require runtime-authoritative preflight bundles",
            QueryContextCounters::for_denial(false, false),
        ));
    }

    let family_matches = match (request.family(), current) {
        (QueryContextFamily::CurrentBranchHead, true) => true,
        (QueryContextFamily::BranchHead, false) => true,
        (QueryContextFamily::CurrentBranchHead, false) => false,
        (QueryContextFamily::BranchHead, true) => false,
        (QueryContextFamily::HistoricalSnapshot, _) => false,
        (QueryContextFamily::HistoricalCommit, _) => false,
        (QueryContextFamily::PreviewDerivedHistorical, _) => false,
        (QueryContextFamily::DiffComparison, _) => false,
    };
    if !family_matches {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::InvalidBasisPairing,
            "runtime source does not match the declared query context family",
            QueryContextCounters::for_denial(false, false),
        ));
    }

    let cost_class = if current {
        QueryContextCostClass::CurrentHeadNarrow
    } else {
        QueryContextCostClass::BranchHeadNarrow
    };

    Ok(QueryBasisContextBinding {
        query_digest: preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        basis_digest: preflight.basis().proof().digest().as_str().to_string(),
        basis_authority_family: preflight.basis().identity().authority_family().clone(),
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        cost_class,
        budget_class: QueryContextBudgetClass::NarrowSingleBasis,
        historical_admission_class: None,
        historical_materialization_cost_class: None,
        materialization_path_identity_source: None,
        preview_provenance_identity_source: None,
        prediction_report: Some(QueryContextPredictionReport::for_runtime_binding()),
        prediction_drift_outcome: Some(QueryContextPredictionDriftOutcome::PendingExecution),
        evidence: QueryBasisBindingEvidence::Runtime {
            preflight: preflight.clone(),
        },
        request,
        counters: QueryContextCounters::for_runtime_basis_binding(),
    })
}

fn bind_historical_context(
    request: QueryBasisContextRequest,
    query_preflight: &ExecutionPreflightBundle,
    admission: &HistoricalEvaluationAdmission,
    metadata: &HistoricalMaterializationPathMetadata,
) -> Result<QueryBasisContextBinding, QueryContextAdmissionError> {
    let historical_family = match request.family() {
        QueryContextFamily::HistoricalSnapshot => true,
        QueryContextFamily::HistoricalCommit => true,
        QueryContextFamily::CurrentBranchHead => false,
        QueryContextFamily::BranchHead => false,
        QueryContextFamily::PreviewDerivedHistorical => false,
        QueryContextFamily::DiffComparison => false,
    };
    if !historical_family {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::InvalidBasisPairing,
            "historical evidence can only bind historical query context families",
            QueryContextCounters::for_denial(true, false),
        ));
    }

    if request.declared_basis_label() != admission.requested_path().basis_identity() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::BasisSubstitutionForbidden,
            "historical context binding forbids basis substitution after admission",
            QueryContextCounters::for_denial(true, true),
        ));
    }

    let admission_class = historical_admission_class(admission);
    let store_authority =
        query_preflight.basis().identity().authority_family() == &BasisAuthorityFamily::Store;
    if store_authority
        && (admission_class != HistoricalAdmissionClass::RuntimeRetained
            || metadata.requested_path_class()
                != &RequestedHistoricalPathClass::RequestedRetainedSnapshotPath
            || metadata.resolved_path_class()
                != &ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath)
    {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::StoreBackedHistoricalDeferred,
            "store-backed historical query contexts are only admitted for the retained-snapshot slice proven in Milestone 10",
            QueryContextCounters::for_denial(true, false),
        ));
    }
    let materialization_identity = materialization_path_identity(metadata);
    let materialization_cost_class = materialization_path_cost_class(admission);
    let cost_class = match admission_class {
        HistoricalAdmissionClass::RuntimeRetained => {
            QueryContextCostClass::HistoricalRetainedBounded
        }
        HistoricalAdmissionClass::RuntimeReplay => QueryContextCostClass::HistoricalReplayBounded,
        HistoricalAdmissionClass::RuntimeReconstruction => {
            QueryContextCostClass::HistoricalReconstructionBounded
        }
        HistoricalAdmissionClass::StoreDeferredDebt => {
            QueryContextCostClass::HistoricalReconstructionBounded
        }
    };

    Ok(QueryBasisContextBinding {
        query_digest: query_preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        basis_digest: hash_parts(&[
            format!(
                "basis_identity:{}",
                admission.requested_path().basis_identity()
            ),
            format!(
                "requested_path:{}",
                admission.requested_path().requested_path_class().as_str()
            ),
            format!("materialization:{}", materialization_identity),
        ]),
        basis_authority_family: query_preflight
            .basis()
            .identity()
            .authority_family()
            .clone(),
        drift_outcome: drift_outcome_for_historical(admission),
        cost_class,
        budget_class: QueryContextBudgetClass::HistoricalBounded,
        historical_admission_class: Some(admission_class),
        historical_materialization_cost_class: Some(materialization_cost_class),
        materialization_path_identity_source: Some(materialization_identity),
        preview_provenance_identity_source: None,
        prediction_report: Some(QueryContextPredictionReport::for_historical_binding()),
        prediction_drift_outcome: Some(QueryContextPredictionDriftOutcome::PendingExecution),
        evidence: QueryBasisBindingEvidence::Historical {
            query_preflight: query_preflight.clone(),
            admission: admission.clone(),
            metadata: metadata.clone(),
        },
        request,
        counters: QueryContextCounters::for_historical_basis_binding(),
    })
}

fn bind_preview_context(
    request: QueryBasisContextRequest,
    foundation: &AdmittedPreviewWorkflowFoundation,
) -> Result<QueryBasisContextBinding, QueryContextAdmissionError> {
    let preview_family = match request.family() {
        QueryContextFamily::PreviewDerivedHistorical => true,
        QueryContextFamily::CurrentBranchHead => false,
        QueryContextFamily::BranchHead => false,
        QueryContextFamily::HistoricalSnapshot => false,
        QueryContextFamily::HistoricalCommit => false,
        QueryContextFamily::DiffComparison => false,
    };
    if !preview_family {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::InvalidBasisPairing,
            "preview-derived evidence can only bind preview-derived historical contexts",
            QueryContextCounters::for_denial(false, false),
        ));
    }

    if foundation.request_family() != &PreviewWorkflowFoundationRequest::compare_basis_pair() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::PreviewProvenanceRequired,
            "preview-derived contexts require compare-basis provenance",
            QueryContextCounters::for_denial(false, false),
        ));
    }

    Ok(QueryBasisContextBinding {
        query_digest: foundation.validated_query_digest().as_str().to_string(),
        basis_digest: foundation.digest().to_string(),
        basis_authority_family: BasisAuthorityFamily::Runtime,
        drift_outcome: QueryContextDriftOutcome::BasisExact,
        cost_class: QueryContextCostClass::PreviewDerivedHistoricalBounded,
        budget_class: QueryContextBudgetClass::PreviewDerivedBounded,
        historical_admission_class: None,
        historical_materialization_cost_class: None,
        materialization_path_identity_source: None,
        preview_provenance_identity_source: Some(
            foundation.preview_session_identity().as_str().to_string(),
        ),
        prediction_report: Some(QueryContextPredictionReport::for_preview_binding()),
        prediction_drift_outcome: Some(QueryContextPredictionDriftOutcome::PendingExecution),
        evidence: QueryBasisBindingEvidence::PreviewDerived {
            foundation: foundation.clone(),
        },
        request,
        counters: QueryContextCounters::for_preview_basis_binding(),
    })
}

pub(crate) fn historical_admission_of(
    context: &AdmittedQueryBasisContext,
) -> Option<HistoricalAdmissionClass> {
    context.historical_admission_class().cloned()
}

pub(crate) fn materialization_identity_of(context: &AdmittedQueryBasisContext) -> Option<String> {
    context
        .materialization_path_identity_source()
        .map(ToString::to_string)
}

pub(crate) fn preview_identity_of(context: &AdmittedQueryBasisContext) -> Option<String> {
    context
        .preview_provenance_identity_source()
        .map(ToString::to_string)
}
