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
    declare_projection_consumption, evaluate_projection_consumption_eligibility,
    AdmittedProjectionConsumption, MaterializedProjectionContract,
    ProjectionConsumptionEligibility,
};

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
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload.semantic_code(),
            domain_contribution.request_digest(),
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
                domain_contribution.request_digest(),
                format!(
                    "projection consumption declaration denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error
                ),
            ));
        }
    };

    TransitionOutcome::Success(evaluate_projection_consumption_eligibility(&declaration))
}

pub fn materialize_admitted_projection_consumption(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<AdmittedProjectionConsumption> {
    let domain_contribution = contribution.payload();
    let target_kind = domain_contribution.target().kind();
    let request_digest = domain_contribution.request_digest().to_string();
    let semantic_code = domain_contribution.payload().semantic_code().to_string();

    match materialize_projection_consumption_eligibility(contribution) {
        TransitionOutcome::Success(eligibility) => match eligibility {
            ProjectionConsumptionEligibility::Admitted(admitted) => {
                TransitionOutcome::Success(admitted)
            }
            ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, _) => {
                TransitionOutcome::Success(admitted)
            }
            ProjectionConsumptionEligibility::Denied(denied) => {
                TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                    ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                    "consequence-aftermath",
                    target_kind,
                    request_digest.clone(),
                    format!(
                        "projection consumption eligibility denied for `{}` with `{:?}`",
                        semantic_code,
                        denied.reason()
                    ),
                ))
            }
            ProjectionConsumptionEligibility::Deferred(deferred) => {
                TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                    ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                    "consequence-aftermath",
                    target_kind,
                    request_digest.clone(),
                    format!(
                        "projection consumption eligibility deferred for `{}` with `{:?}`",
                        semantic_code,
                        deferred.reason()
                    ),
                ))
            }
            ProjectionConsumptionEligibility::SourceMismatch(mismatch) => {
                TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                    ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
                    "consequence-aftermath",
                    target_kind,
                    request_digest,
                    format!(
                        "projection consumption source mismatch for `{}` with `{:?}` / `{:?}`",
                        semantic_code,
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
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "consequence-aftermath",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_digest,
        format!(
            "projection consumption materialization requires runtime aftermath semantics for `{semantic_code}`"
        ),
    )
}
