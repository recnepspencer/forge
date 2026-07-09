use worth_proof::TransitionOutcome;
use worth_query::facade::runtime::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    materialize_graph_composition_domain_invariant_denial,
    prepare_admitted_domain_capability_contribution_for_materialization,
    CustomInvariantRegistrationError, WORTHQueryDomainCapabilityProgressionDenial,
    WORTHQueryDomainCapabilityProgressionFailure, WORTHQueryDomainCapabilityRebindRequired,
    WORTHQueryDomainCapabilityStale, WORTHQueryDomainCapabilityTransitionOutcome,
    WORTHQueryInvariantCapabilityContributionAuthoring,
};

use crate::discovery_loop::{DiscoveryFrontier, ResearchEvidenceCorpus};
use crate::domain_artifacts::{HadwigerArtifactShapeError, HadwigerCanonicalArtifact};
use crate::query_entry::HadwigerResearchHandle;

use super::catalog::{
    HadwigerResearchInvariantCatalog, ResearchGraphInvariantFamily,
    ResearchGraphInvariantRegistrationPlan, ResearchGraphInvariantRule,
    ResearchGraphInvariantScope,
};
use super::denials::ResearchGraphInvariantDenial;
use super::requests::{ResearchGraphInvariantCheckRequest, ResearchGraphInvariantDenialRequest};
use super::runtime_projection::ResearchGraphInvariantRuntimeProjection;
use super::runtime_registration::{
    registrations_for_catalog, HadwigerResearchInvariantRegistrationChecked,
};
use super::violations::{ResearchGraphInvariantViolation, ResearchGraphInvariantViolationKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResearchGraphInvariantError {
    Shape(HadwigerArtifactShapeError),
    MissingCorpus,
    MissingLowerRuntimeBoundaryEnvelope,
    NoViolationDetected,
    QueryInvariantContributionDenied(WORTHQueryDomainCapabilityProgressionDenial),
    QueryInvariantContributionStale(WORTHQueryDomainCapabilityStale),
    QueryInvariantContributionRebindRequired(WORTHQueryDomainCapabilityRebindRequired),
    QueryInvariantContributionFailed(WORTHQueryDomainCapabilityProgressionFailure),
    CustomInvariantRegistration(CustomInvariantRegistrationError),
}

impl From<HadwigerArtifactShapeError> for ResearchGraphInvariantError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Shape(value)
    }
}

impl From<CustomInvariantRegistrationError> for ResearchGraphInvariantError {
    fn from(value: CustomInvariantRegistrationError) -> Self {
        Self::CustomInvariantRegistration(value)
    }
}

pub fn draft_research_graph_invariant_catalog(
    _handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
) -> Result<HadwigerResearchInvariantCatalog, ResearchGraphInvariantError> {
    if frontier.admits_theorem_authority() || frontier.registers_query_invariant_authority() {
        return Err(ResearchGraphInvariantError::Shape(
            HadwigerArtifactShapeError::EmptyField {
                field: "discovery_frontier_authority_claim",
            },
        ));
    }
    let parents = vec![corpus.reference(), frontier.reference()];
    let rules = ResearchGraphInvariantFamily::all()
        .into_iter()
        .map(|family| {
            ResearchGraphInvariantRule::new(family, scope_for_family(family), parents.clone())
        })
        .collect::<Result<Vec<_>, _>>()?;
    HadwigerResearchInvariantCatalog::new(corpus, frontier, rules).map_err(Into::into)
}

pub fn certify_research_graph_invariant_violation(
    _handle: &HadwigerResearchHandle,
    request: ResearchGraphInvariantCheckRequest,
) -> Result<ResearchGraphInvariantViolation, ResearchGraphInvariantError> {
    let corpus = request
        .corpus()
        .ok_or(ResearchGraphInvariantError::MissingCorpus)?;
    let batch = request.experiment_batch();
    let breadth = corpus.evidence_references().len() + batch.experiment_plans().len();

    if corpus.rejected_evidence_available() && corpus.reusable_negative_evidence().is_empty() {
        return violation(
            request.catalog(),
            ResearchGraphInvariantViolationKind::MissingRetainedNegativeEvidence,
            ResearchGraphInvariantFamily::FailureResidency,
            ResearchGraphInvariantScope::EvidenceCorpus,
            "rejected evidence must retain reusable negative evidence",
            vec![corpus.reference()],
            breadth,
        );
    }

    if corpus.rejected_evidence_available()
        && batch
            .experiment_plans()
            .iter()
            .any(|plan| !plan.is_suppressed())
    {
        return violation(
            request.catalog(),
            ResearchGraphInvariantViolationKind::SuppressedExperimentMissingProof,
            ResearchGraphInvariantFamily::SuppressionRelation,
            ResearchGraphInvariantScope::ExperimentBatch,
            "rejected evidence requires equivalent experiment suppression proof",
            vec![corpus.reference(), batch.reference()],
            breadth,
        );
    }

    if corpus.has_query_recovery_evidence() && batch.query_readiness_checks() == 0 {
        return violation(
            request.catalog(),
            ResearchGraphInvariantViolationKind::ExecutableExperimentReadinessDrift,
            ResearchGraphInvariantFamily::ExecutableExperimentAdmission,
            ResearchGraphInvariantScope::ExperimentBatch,
            "Query-owned recovery evidence requires retained readiness check evidence",
            vec![corpus.reference(), batch.reference()],
            breadth,
        );
    }

    Err(ResearchGraphInvariantError::NoViolationDetected)
}

pub fn materialize_research_graph_invariant_denial(
    _handle: &HadwigerResearchHandle,
    request: ResearchGraphInvariantDenialRequest,
) -> Result<ResearchGraphInvariantDenial, ResearchGraphInvariantError> {
    let source = request
        .lower_runtime_boundary_source()
        .ok_or(ResearchGraphInvariantError::MissingLowerRuntimeBoundaryEnvelope)?;
    let violation = request.violation();
    let family = violation.rule_family().query_invariant_family();
    let authoring = WORTHQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
        family,
        ["hadwiger_research_graph"],
        [violation.reference().stable_token()],
        [violation.scope().as_str()],
        [violation.violation_kind().as_str()],
        request.catalog().artifact_digest().stable_token(),
        violation.artifact_digest().stable_token(),
        violation.counters().stable_token(),
        family,
        violation.detail(),
    );
    let requested = authoring.for_lower_runtime_boundary_source(source.envelope());
    let eligible =
        transition_success(evaluate_requested_domain_capability_contribution(requested))?;
    let admitted = transition_success(admit_eligible_domain_capability_contribution(eligible))?;
    let target = admitted.payload().target().clone();
    let ready = transition_success(
        prepare_admitted_domain_capability_contribution_for_materialization(admitted, target),
    )?;
    let query_denial =
        transition_success(materialize_graph_composition_domain_invariant_denial(ready))?;
    ResearchGraphInvariantDenial::new(request.catalog(), violation, source, query_denial)
        .map_err(Into::into)
}

pub fn plan_research_graph_invariant_registration(
    _handle: &HadwigerResearchHandle,
    catalog: &HadwigerResearchInvariantCatalog,
) -> Result<ResearchGraphInvariantRegistrationPlan, ResearchGraphInvariantError> {
    ResearchGraphInvariantRegistrationPlan::custom_invariant_registrations_ready(catalog)
        .map_err(Into::into)
}

pub fn project_research_graph_for_invariant_registration_checked(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
) -> Result<ResearchGraphInvariantRuntimeProjection, ResearchGraphInvariantError> {
    let catalog = draft_research_graph_invariant_catalog(handle, corpus, frontier)?;
    ResearchGraphInvariantRuntimeProjection::new(corpus, frontier, catalog).map_err(Into::into)
}

pub fn register_research_graph_invariants_checked(
    _handle: &HadwigerResearchHandle,
    catalog: &HadwigerResearchInvariantCatalog,
) -> Result<HadwigerResearchInvariantRegistrationChecked, ResearchGraphInvariantError> {
    let registrations = registrations_for_catalog(catalog)?;
    HadwigerResearchInvariantRegistrationChecked::new(catalog, registrations).map_err(Into::into)
}

fn violation(
    catalog: &HadwigerResearchInvariantCatalog,
    violation_kind: ResearchGraphInvariantViolationKind,
    rule_family: ResearchGraphInvariantFamily,
    scope: ResearchGraphInvariantScope,
    detail: &'static str,
    parents: Vec<crate::domain_artifacts::HadwigerArtifactReference>,
    breadth_inspected: usize,
) -> Result<ResearchGraphInvariantViolation, ResearchGraphInvariantError> {
    ResearchGraphInvariantViolation::new(
        catalog,
        violation_kind,
        rule_family,
        scope,
        detail,
        parents,
        breadth_inspected,
    )
    .map_err(Into::into)
}

fn scope_for_family(family: ResearchGraphInvariantFamily) -> ResearchGraphInvariantScope {
    match family {
        ResearchGraphInvariantFamily::FailureResidency => {
            ResearchGraphInvariantScope::EvidenceCorpus
        }
        ResearchGraphInvariantFamily::SuppressionRelation
        | ResearchGraphInvariantFamily::ExecutableExperimentAdmission => {
            ResearchGraphInvariantScope::ExperimentBatch
        }
        ResearchGraphInvariantFamily::HypothesisLifecycle => {
            ResearchGraphInvariantScope::EvidenceCorpus
        }
        ResearchGraphInvariantFamily::BranchPromotion => {
            ResearchGraphInvariantScope::DiscoveryFrontier
        }
    }
}

fn transition_success<S>(
    outcome: WORTHQueryDomainCapabilityTransitionOutcome<S>,
) -> Result<S, ResearchGraphInvariantError> {
    match outcome {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => {
            Err(ResearchGraphInvariantError::QueryInvariantContributionDenied(denial))
        }
        TransitionOutcome::Stale(stale) => {
            Err(ResearchGraphInvariantError::QueryInvariantContributionStale(stale))
        }
        TransitionOutcome::RebindRequired(rebind) => {
            Err(ResearchGraphInvariantError::QueryInvariantContributionRebindRequired(rebind))
        }
        TransitionOutcome::Failed(failure) => {
            Err(ResearchGraphInvariantError::QueryInvariantContributionFailed(failure))
        }
        TransitionOutcome::Deferred(never) => match never {},
    }
}
