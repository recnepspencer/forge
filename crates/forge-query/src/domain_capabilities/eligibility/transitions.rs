use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
    ForgeQueryDomainCapabilityRebindRequired, ForgeQueryDomainCapabilityStale,
    ForgeQueryDomainCapabilityTransitionOutcome,
};
use crate::domain_capabilities::payloads::ForgeQueryDomainCapabilityPayload;
use crate::domain_capabilities::proof_integration::{
    admitted_proof, contribution_basis, eligible_proof, materialization_ready_proof,
    remint_with_phase, ForgeQueryAdmittedDomainCapabilityContribution,
    ForgeQueryDomainCapabilityContribution, ForgeQueryEligibleDomainCapabilityContribution,
    ForgeQueryMaterializationReadyDomainCapabilityContribution,
    ForgeQueryRequestedDomainCapabilityContribution,
};
use crate::domain_capabilities::targets::{
    ForgeQueryDomainCapabilityTargetBinding, ForgeQueryDomainCapabilityTargetKind,
};

pub fn evaluate_requested_domain_capability_contribution<P, T>(
    requested: ForgeQueryRequestedDomainCapabilityContribution<P, T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryEligibleDomainCapabilityContribution<P, T>>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    if requested
        .payload()
        .payload()
        .semantic_code()
        .trim()
        .is_empty()
    {
        return TransitionOutcome::Denied(denial_for(
            requested.payload(),
            ForgeQueryDomainCapabilityProgressionDenialKind::EmptySemanticCode,
            "domain capability contributions require a non-empty semantic code",
        ));
    }
    if requested.payload().payload().detail().trim().is_empty() {
        return TransitionOutcome::Denied(denial_for(
            requested.payload(),
            ForgeQueryDomainCapabilityProgressionDenialKind::EmptyDetail,
            "domain capability contributions require non-empty detail text",
        ));
    }
    let basis = contribution_basis(&requested.0);
    let payload = requested.into_inner().into_parts().into_parts().0;
    TransitionOutcome::Success(ForgeQueryEligibleDomainCapabilityContribution(
        remint_with_phase(payload, basis, eligible_proof()),
    ))
}

pub fn admit_eligible_domain_capability_contribution<P, T>(
    eligible: ForgeQueryEligibleDomainCapabilityContribution<P, T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryAdmittedDomainCapabilityContribution<P, T>>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let basis = contribution_basis(&eligible.0);
    let payload = eligible.into_inner().into_parts().into_parts().0;
    TransitionOutcome::Success(ForgeQueryAdmittedDomainCapabilityContribution(
        remint_with_phase(payload, basis, admitted_proof()),
    ))
}

pub fn prepare_admitted_domain_capability_contribution_for_materialization<P, T>(
    admitted: ForgeQueryAdmittedDomainCapabilityContribution<P, T>,
    current_target: T,
) -> ForgeQueryDomainCapabilityTransitionOutcome<
    ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let basis = contribution_basis(&admitted.0);
    let payload = admitted.into_inner().into_parts().into_parts().0;
    let category = payload.category().as_str();
    let bound_target = payload.target();
    if bound_target.binding_identity() != current_target.binding_identity() {
        return match current_target.kind() {
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope => {
                TransitionOutcome::Stale(ForgeQueryDomainCapabilityStale::new(
                    category,
                    bound_target.target_identity(),
                    current_target.target_identity(),
                ))
            }
            _ => TransitionOutcome::RebindRequired(ForgeQueryDomainCapabilityRebindRequired::new(
                category,
                bound_target.target_identity(),
                current_target.target_identity(),
            )),
        };
    }

    TransitionOutcome::Success(ForgeQueryMaterializationReadyDomainCapabilityContribution(
        remint_with_phase(payload, basis, materialization_ready_proof()),
    ))
}

fn denial_for<P, T>(
    contribution: &ForgeQueryDomainCapabilityContribution<P, T>,
    kind: ForgeQueryDomainCapabilityProgressionDenialKind,
    message: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    ForgeQueryDomainCapabilityProgressionDenial::new(
        kind,
        contribution.category().as_str(),
        contribution.target().kind(),
        contribution.request_identity().clone(),
        message,
    )
}
