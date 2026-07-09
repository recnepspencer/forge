use crate::query_basis_lifecycle::{
    normalize_query_context_request, scope_materialization_basis_intent,
    scope_observation_basis_intent, try_raw_basis_intent_from_query_context_request,
    BasisIntentDenial, BasisOperationLaneRequest, BasisScopedAdmissionDenial,
    DeniedBasisCapability, NormalizedBasisFamily, RawBasisIntent, ScopedMaterializationBasis,
    ScopedObservationBasis,
};

use super::{
    admit_query_basis_context, bind_query_basis_context, build_query_basis_result_bundle,
    execute_query_basis_context, AdmittedQueryBasisContext, QueryBasisContextRequest,
    QueryBasisResultBundle, QueryContextAdmissionError, QueryContextBindingSource,
    QueryContextExecutionArtifact, QueryContextFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopedQueryContextAdmissionError {
    Intent(BasisIntentDenial),
    Eligibility(DeniedBasisCapability),
    Context(QueryContextAdmissionError),
}

impl ScopedQueryContextAdmissionError {
    pub fn intent_denial(&self) -> Option<&BasisIntentDenial> {
        match self {
            Self::Intent(denial) => Some(denial),
            _ => None,
        }
    }

    pub fn eligibility_denial(&self) -> Option<&DeniedBasisCapability> {
        match self {
            Self::Eligibility(denial) => Some(denial),
            _ => None,
        }
    }

    pub fn context_error(&self) -> Option<&QueryContextAdmissionError> {
        match self {
            Self::Context(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedObservationQueryBasisContext {
    scoped_basis: ScopedObservationBasis,
    context: AdmittedQueryBasisContext,
}

impl ScopedObservationQueryBasisContext {
    pub fn scoped_basis(&self) -> &ScopedObservationBasis {
        &self.scoped_basis
    }

    pub fn context(&self) -> &AdmittedQueryBasisContext {
        &self.context
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedMaterializationQueryBasisContext {
    scoped_basis: ScopedMaterializationBasis,
    context: AdmittedQueryBasisContext,
}

impl ScopedMaterializationQueryBasisContext {
    pub fn scoped_basis(&self) -> &ScopedMaterializationBasis {
        &self.scoped_basis
    }

    pub fn context(&self) -> &AdmittedQueryBasisContext {
        &self.context
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopedQueryBasisContext {
    Observation(ScopedObservationQueryBasisContext),
    Materialization(ScopedMaterializationQueryBasisContext),
}

impl ScopedQueryBasisContext {
    pub fn context(&self) -> &AdmittedQueryBasisContext {
        match self {
            Self::Observation(scoped) => scoped.context(),
            Self::Materialization(scoped) => scoped.context(),
        }
    }
}

pub fn admit_scoped_query_basis_context(
    request: QueryBasisContextRequest,
    source: QueryContextBindingSource<'_>,
) -> Result<ScopedQueryBasisContext, ScopedQueryContextAdmissionError> {
    let scoped_basis = scope_query_context_request(request.clone())?;
    let binding = bind_query_basis_context(request, source)
        .map_err(ScopedQueryContextAdmissionError::Context)?;
    let context =
        admit_query_basis_context(binding).map_err(ScopedQueryContextAdmissionError::Context)?;
    ensure_scoped_context_coherence(&scoped_basis, &context)
        .map_err(ScopedQueryContextAdmissionError::Context)?;

    Ok(match scoped_basis {
        QueryContextScopedBasis::Observation(scoped_basis) => {
            ScopedQueryBasisContext::Observation(ScopedObservationQueryBasisContext {
                scoped_basis,
                context,
            })
        }
        QueryContextScopedBasis::Materialization(scoped_basis) => {
            ScopedQueryBasisContext::Materialization(ScopedMaterializationQueryBasisContext {
                scoped_basis,
                context,
            })
        }
    })
}

pub fn execute_scoped_query_basis_context(
    context: &ScopedQueryBasisContext,
) -> Result<QueryContextExecutionArtifact, QueryContextAdmissionError> {
    execute_query_basis_context(context.context())
}

pub fn build_scoped_query_basis_result_bundle(
    context: &ScopedQueryBasisContext,
    execution: QueryContextExecutionArtifact,
) -> Result<QueryBasisResultBundle, QueryContextAdmissionError> {
    build_query_basis_result_bundle(context.context(), execution)
}

pub fn execute_and_build_scoped_query_basis_result_bundle(
    context: &ScopedQueryBasisContext,
) -> Result<QueryBasisResultBundle, QueryContextAdmissionError> {
    let execution = execute_scoped_query_basis_context(context)?;
    build_scoped_query_basis_result_bundle(context, execution)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum QueryContextScopedBasis {
    Observation(ScopedObservationBasis),
    Materialization(ScopedMaterializationBasis),
}

fn scope_query_context_request(
    request: QueryBasisContextRequest,
) -> Result<QueryContextScopedBasis, ScopedQueryContextAdmissionError> {
    let kind = scoped_request_kind(request.family());
    let raw_intent = raw_intent_for_scoped_request(&request, kind)?;
    match kind {
        QueryContextScopedRequestKind::Observation => scope_observation_basis_intent(raw_intent)
            .map(QueryContextScopedBasis::Observation)
            .map_err(map_scoped_admission_error),
        QueryContextScopedRequestKind::Materialization => {
            scope_materialization_basis_intent(raw_intent)
                .map(QueryContextScopedBasis::Materialization)
                .map_err(map_scoped_admission_error)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QueryContextScopedRequestKind {
    Observation,
    Materialization,
}

fn scoped_request_kind(family: &QueryContextFamily) -> QueryContextScopedRequestKind {
    match family {
        QueryContextFamily::CurrentBranchHead
        | QueryContextFamily::BranchHead
        | QueryContextFamily::PreviewDerivedHistorical
        | QueryContextFamily::DiffComparison => QueryContextScopedRequestKind::Observation,
        QueryContextFamily::HistoricalSnapshot | QueryContextFamily::HistoricalCommit => {
            QueryContextScopedRequestKind::Materialization
        }
    }
}

fn raw_intent_for_scoped_request(
    request: &QueryBasisContextRequest,
    kind: QueryContextScopedRequestKind,
) -> Result<RawBasisIntent, ScopedQueryContextAdmissionError> {
    let lane = match kind {
        QueryContextScopedRequestKind::Observation => BasisOperationLaneRequest::Observation,
        QueryContextScopedRequestKind::Materialization => {
            BasisOperationLaneRequest::Materialization
        }
    };
    try_raw_basis_intent_from_query_context_request(request, lane)
        .map_err(ScopedQueryContextAdmissionError::Intent)
}

fn map_scoped_admission_error(
    error: BasisScopedAdmissionDenial,
) -> ScopedQueryContextAdmissionError {
    match error {
        BasisScopedAdmissionDenial::Intent(denial) => {
            ScopedQueryContextAdmissionError::Intent(denial)
        }
        BasisScopedAdmissionDenial::Eligibility(denial) => {
            ScopedQueryContextAdmissionError::Eligibility(denial)
        }
    }
}

fn ensure_scoped_context_coherence(
    scoped_basis: &QueryContextScopedBasis,
    context: &AdmittedQueryBasisContext,
) -> Result<(), QueryContextAdmissionError> {
    let (observed_family, observed_scope_label) = match scoped_basis {
        QueryContextScopedBasis::Observation(scoped) => match scoped.admission() {
            crate::query_basis_lifecycle::BasisCapabilityAdmission::Admitted(capability) => {
                (capability.family(), capability.scope_label())
            }
            crate::query_basis_lifecycle::BasisCapabilityAdmission::Advisory(capability) => {
                (capability.family(), capability.scope_label())
            }
        },
        QueryContextScopedBasis::Materialization(scoped) => (
            scoped.capability().family(),
            scoped.capability().scope_label(),
        ),
    };

    if observed_family != expected_normalized_family(context.family()) {
        return Err(QueryContextAdmissionError::new(
            super::QueryContextAdmissionFailureClass::BasisSubstitutionForbidden,
            "scoped query-context admission requires lifecycle family parity with the admitted legacy query context",
            super::QueryContextCounters::for_denial(false, true),
        ));
    }

    let expected_scope_label = expected_scope_label(context);
    if observed_scope_label != expected_scope_label.as_str() {
        return Err(QueryContextAdmissionError::new(
            super::QueryContextAdmissionFailureClass::BasisSubstitutionForbidden,
            "scoped query-context admission requires semantic basis-label parity across lifecycle and legacy adapters",
            super::QueryContextCounters::for_denial(false, true),
        ));
    }

    Ok(())
}

fn expected_normalized_family(family: &QueryContextFamily) -> &NormalizedBasisFamily {
    match family {
        QueryContextFamily::CurrentBranchHead => &NormalizedBasisFamily::CurrentHead,
        QueryContextFamily::BranchHead => &NormalizedBasisFamily::BranchHead,
        QueryContextFamily::HistoricalSnapshot => &NormalizedBasisFamily::HistoricalSnapshot,
        QueryContextFamily::HistoricalCommit => &NormalizedBasisFamily::HistoricalCommit,
        QueryContextFamily::PreviewDerivedHistorical => {
            &NormalizedBasisFamily::PreviewDerivedHistorical
        }
        QueryContextFamily::DiffComparison => &NormalizedBasisFamily::CurrentHead,
    }
}

fn expected_scope_label(context: &AdmittedQueryBasisContext) -> String {
    match context.family() {
        QueryContextFamily::CurrentBranchHead => "current_head".to_string(),
        QueryContextFamily::BranchHead
        | QueryContextFamily::HistoricalSnapshot
        | QueryContextFamily::HistoricalCommit
        | QueryContextFamily::PreviewDerivedHistorical
        | QueryContextFamily::DiffComparison => expected_scoped_compatibility_scope_label(context)
            .unwrap_or_else(|| context.declared_basis_label().to_string()),
    }
}

fn expected_scoped_compatibility_scope_label(
    context: &AdmittedQueryBasisContext,
) -> Option<String> {
    let lane = match scoped_request_kind(context.family()) {
        QueryContextScopedRequestKind::Observation => BasisOperationLaneRequest::Observation,
        QueryContextScopedRequestKind::Materialization => {
            BasisOperationLaneRequest::Materialization
        }
    };
    normalize_query_context_request(context.binding().request(), lane)
        .ok()
        .map(|intent| intent.normalized_label().to_string())
}
