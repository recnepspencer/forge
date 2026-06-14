use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalAftermathArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyAftermathContribution,
};
use crate::projection_consumption::{
    declare_projection_consumption, discover_projection_consumption_support,
    evaluate_projection_consumption_eligibility, AdmittedProjectionConsumption,
    MaterializedProjectionContract, ProjectionConsumptionDeclaration,
    ProjectionConsumptionEligibility, ProjectionConsumptionSupportReport,
};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAftermathProjectionConsumptionReview {
    semantic_code: String,
    request_identity: ForgeQueryEvidenceIdentity,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    declaration: ProjectionConsumptionDeclaration,
    support_report: ProjectionConsumptionSupportReport,
    eligibility: ProjectionConsumptionEligibility,
}

impl ForgeQueryAftermathProjectionConsumptionReview {
    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_for_reporting(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn target_kind(&self) -> crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind {
        self.target_kind
    }

    pub fn declaration(&self) -> &ProjectionConsumptionDeclaration {
        &self.declaration
    }

    pub fn support_report(&self) -> &ProjectionConsumptionSupportReport {
        &self.support_report
    }

    pub fn eligibility(&self) -> &ProjectionConsumptionEligibility {
        &self.eligibility
    }
}

pub fn materialize_canonical_aftermath_artifact<T>(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<T>,
) -> ForgeQueryCanonicalAftermathArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

pub fn materialize_projection_consumption_eligibility(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ProjectionConsumptionEligibility> {
    match materialize_projection_consumption_review(contribution) {
        TransitionOutcome::Success(review) => {
            TransitionOutcome::Success(review.eligibility().clone())
        }
        TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    }
}

pub fn materialize_projection_consumption_support_report(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ProjectionConsumptionSupportReport> {
    match materialize_projection_consumption_review(contribution) {
        TransitionOutcome::Success(review) => {
            TransitionOutcome::Success(review.support_report().clone())
        }
        TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    }
}

pub fn materialize_projection_consumption_review(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryAftermathProjectionConsumptionReview> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload.semantic_code(),
            domain_contribution.request_identity().clone(),
        ));
    };

    let declaration = match declare_projection_consumption(
        runtime_semantics.source().clone(),
        runtime_semantics.binding().clone(),
        runtime_semantics.requested_facts().clone(),
    ) {
        Ok(declaration) => declaration,
        Err(error) => {
            return TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                "consequence-aftermath",
                domain_contribution.target().kind(),
                domain_contribution.request_identity().clone(),
                format!(
                    "projection consumption declaration denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error
                ),
            ));
        }
    };

    let support_report = discover_projection_consumption_support(runtime_semantics.source());
    let eligibility = evaluate_projection_consumption_eligibility(&declaration);

    TransitionOutcome::Success(ForgeQueryAftermathProjectionConsumptionReview {
        semantic_code: payload.semantic_code().to_string(),
        request_identity: domain_contribution.request_identity().clone(),
        target_kind: domain_contribution.target().kind(),
        declaration,
        support_report,
        eligibility,
    })
}

pub fn materialize_admitted_projection_consumption(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<AdmittedProjectionConsumption> {
    match materialize_projection_consumption_review(contribution) {
        TransitionOutcome::Success(review) => match review.eligibility() {
            ProjectionConsumptionEligibility::Admitted(admitted) => {
                TransitionOutcome::Success(admitted.clone())
            }
            ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, _) => {
                TransitionOutcome::Success(admitted.clone())
            }
            ProjectionConsumptionEligibility::Denied(denied) => {
                TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                    ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                    "consequence-aftermath",
                    review.target_kind(),
                    review.request_identity().clone(),
                    format!(
                        "projection consumption eligibility denied for `{}` with `{:?}`",
                        review.semantic_code(),
                        denied.reason()
                    ),
                ))
            }
            ProjectionConsumptionEligibility::Deferred(deferred) => {
                TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                    ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                    "consequence-aftermath",
                    review.target_kind(),
                    review.request_identity().clone(),
                    format!(
                        "projection consumption eligibility deferred for `{}` with `{:?}`",
                        review.semantic_code(),
                        deferred.reason()
                    ),
                ))
            }
            ProjectionConsumptionEligibility::SourceMismatch(mismatch) => {
                TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                    ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
                    "consequence-aftermath",
                    review.target_kind(),
                    review.request_identity().clone(),
                    format!(
                        "projection consumption source mismatch for `{}` with `{:?}` / `{:?}`",
                        review.semantic_code(),
                        mismatch.source_family(),
                        mismatch.requested_fact_kind()
                    ),
                ))
            }
        },
        TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    }
}

pub fn materialize_projection_consumption_contract(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<MaterializedProjectionContract> {
    match materialize_admitted_projection_consumption(contribution) {
        TransitionOutcome::Success(admitted) => {
            TransitionOutcome::Success(admitted.bind_contract())
        }
        TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    }
}

fn missing_runtime_semantics_denial(
    semantic_code: &str,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "consequence-aftermath",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "projection consumption materialization requires runtime aftermath semantics for `{semantic_code}`"
        ),
    )
}
