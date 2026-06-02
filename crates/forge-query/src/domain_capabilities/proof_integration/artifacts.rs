use forge_proof::{
    Artifact, AssumptionBasis, CurrentValidity, FreshnessScopedBasis, ProofSet,
    ProofSetAuthorizedBy,
};

use crate::domain_capabilities::payloads::{
    ForgeQueryAdmissionContributionPayload, ForgeQueryAftermathContributionPayload,
    ForgeQueryContinuityContributionPayload, ForgeQueryDomainCapabilityPayload,
    ForgeQueryExplanationContributionPayload, ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQuerySupportContributionPayload, ForgeQueryWorkflowContributionPayload,
};
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryDomainCapabilityTargetBinding, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::identity::hash_parts;

use super::phases::{
    ContributionAdmittedPhase, ContributionEligiblePhase, ContributionMaterializationReadyPhase,
    ContributionRequestedPhase,
};
use super::proofs::{
    admitted_contribution_proof, eligible_contribution_proof,
    materialization_ready_contribution_proof, requested_contribution_proof,
    AdmittedContributionProof, DomainCapabilityContributionAuthority, EligibleContributionProof,
    MaterializationReadyContributionProof, RequestedContributionProof,
};

type DomainCapabilityBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<String>>;

mod allowed_bindings {
    use super::*;

    pub trait Sealed {}

    pub trait AllowedContributionBinding<P, T>: Sealed
    where
        P: ForgeQueryDomainCapabilityPayload,
        T: ForgeQueryDomainCapabilityTargetBinding,
    {
    }

    macro_rules! allow {
        ($payload:ty => [$($target:ty),+ $(,)?]) => {
            $(impl Sealed for ($payload, $target) {}
            impl AllowedContributionBinding<$payload, $target> for ($payload, $target) {})+
        };
    }

    allow!(
        ForgeQueryAdmissionContributionPayload => [
            ForgeQueryDeclarationBoundContributionTarget,
            ForgeQueryAdmittedPlanBoundContributionTarget
        ]
    );
    allow!(
        ForgeQuerySupportContributionPayload => [
            ForgeQueryDeclarationBoundContributionTarget,
            ForgeQueryAdmittedPlanBoundContributionTarget,
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );
    allow!(
        ForgeQueryInvariantCapabilityContributionPayload => [
            ForgeQueryDeclarationBoundContributionTarget,
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );
    allow!(
        ForgeQueryWorkflowContributionPayload => [
            ForgeQueryDeclarationBoundContributionTarget,
            ForgeQueryAdmittedPlanBoundContributionTarget
        ]
    );
    allow!(
        ForgeQueryContinuityContributionPayload => [
            ForgeQueryDeclarationBoundContributionTarget,
            ForgeQueryAdmittedPlanBoundContributionTarget
        ]
    );
    allow!(
        ForgeQueryAftermathContributionPayload => [
            ForgeQueryAdmittedPlanBoundContributionTarget,
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );
    allow!(
        ForgeQueryExplanationContributionPayload => [
            ForgeQueryDeclarationBoundContributionTarget,
            ForgeQueryAdmittedPlanBoundContributionTarget,
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );
}

pub(crate) use allowed_bindings::AllowedContributionBinding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    target: T,
    payload: P,
    request_digest: String,
}

impl<P, T> ForgeQueryDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn category(&self) -> crate::domain_capabilities::ForgeQueryDomainCapabilityCategory {
        self.payload.category()
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    fn new(target: T, payload: P) -> Self {
        let request_digest = hash_parts(&[
            "forge_query_domain_capability_request_v1".to_string(),
            format!("category:{}", payload.category().as_str()),
            format!("target:{}", target.binding_digest()),
            format!("payload:{}", payload.payload_digest()),
        ]);
        Self {
            target,
            payload,
            request_digest,
        }
    }
}

type RequestedContributionArtifact<P, T> = Artifact<
    ContributionRequestedPhase,
    ForgeQueryDomainCapabilityContribution<P, T>,
    RequestedContributionProof,
    DomainCapabilityBasis,
>;
type EligibleContributionArtifact<P, T> = Artifact<
    ContributionEligiblePhase,
    ForgeQueryDomainCapabilityContribution<P, T>,
    EligibleContributionProof,
    DomainCapabilityBasis,
>;
type AdmittedContributionArtifact<P, T> = Artifact<
    ContributionAdmittedPhase,
    ForgeQueryDomainCapabilityContribution<P, T>,
    AdmittedContributionProof,
    DomainCapabilityBasis,
>;
type MaterializationReadyContributionArtifact<P, T> = Artifact<
    ContributionMaterializationReadyPhase,
    ForgeQueryDomainCapabilityContribution<P, T>,
    MaterializationReadyContributionProof,
    DomainCapabilityBasis,
>;

pub struct ForgeQueryRequestedDomainCapabilityContribution<P, T>(
    pub(crate) RequestedContributionArtifact<P, T>,
)
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding;

pub struct ForgeQueryEligibleDomainCapabilityContribution<P, T>(
    pub(crate) EligibleContributionArtifact<P, T>,
)
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding;

pub struct ForgeQueryAdmittedDomainCapabilityContribution<P, T>(
    pub(crate) AdmittedContributionArtifact<P, T>,
)
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding;

pub struct ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>(
    pub(crate) MaterializationReadyContributionArtifact<P, T>,
)
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding;

macro_rules! impl_wrapper_accessors {
    ($name:ident, $inner:ident) => {
        impl<P, T> $name<P, T>
        where
            P: ForgeQueryDomainCapabilityPayload,
            T: ForgeQueryDomainCapabilityTargetBinding,
        {
            pub fn payload(&self) -> &ForgeQueryDomainCapabilityContribution<P, T> {
                self.0.payload()
            }

            #[allow(dead_code)]
            pub(crate) fn into_inner(self) -> $inner<P, T> {
                self.0
            }
        }
    };
}

fn phase_digest<Phase, P, T, S>(
    phase: &'static str,
    artifact: &Artifact<
        Phase,
        ForgeQueryDomainCapabilityContribution<P, T>,
        S,
        DomainCapabilityBasis,
    >,
) -> String
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    hash_parts(&[
        "forge_query_domain_capability_phase_digest_v1".to_string(),
        format!("phase:{phase}"),
        format!("request:{}", artifact.payload().request_digest()),
        format!("basis:{}", contribution_basis(artifact)),
    ])
}

impl_wrapper_accessors!(
    ForgeQueryRequestedDomainCapabilityContribution,
    RequestedContributionArtifact
);
impl_wrapper_accessors!(
    ForgeQueryEligibleDomainCapabilityContribution,
    EligibleContributionArtifact
);
impl_wrapper_accessors!(
    ForgeQueryAdmittedDomainCapabilityContribution,
    AdmittedContributionArtifact
);
impl_wrapper_accessors!(
    ForgeQueryMaterializationReadyDomainCapabilityContribution,
    MaterializationReadyContributionArtifact
);

impl<P, T> ForgeQueryRequestedDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn requested_digest(&self) -> String {
        phase_digest("requested", &self.0)
    }
}

impl<P, T> ForgeQueryEligibleDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn eligibility_digest(&self) -> String {
        phase_digest("eligible", &self.0)
    }
}

impl<P, T> ForgeQueryAdmittedDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn admitted_digest(&self) -> String {
        phase_digest("admitted", &self.0)
    }
}

impl<P, T> ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn materialization_ready_digest(&self) -> String {
        phase_digest("materialization-ready", &self.0)
    }
}

pub type ForgeQueryRequestedAdmissionContribution<T> =
    ForgeQueryRequestedDomainCapabilityContribution<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQueryEligibleAdmissionContribution<T> =
    ForgeQueryEligibleDomainCapabilityContribution<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQueryAdmittedAdmissionContribution<T> =
    ForgeQueryAdmittedDomainCapabilityContribution<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQueryMaterializationReadyAdmissionContribution<T> =
    ForgeQueryMaterializationReadyDomainCapabilityContribution<
        ForgeQueryAdmissionContributionPayload,
        T,
    >;

pub type ForgeQueryRequestedSupportContribution<T> =
    ForgeQueryRequestedDomainCapabilityContribution<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryEligibleSupportContribution<T> =
    ForgeQueryEligibleDomainCapabilityContribution<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryAdmittedSupportContribution<T> =
    ForgeQueryAdmittedDomainCapabilityContribution<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryMaterializationReadySupportContribution<T> =
    ForgeQueryMaterializationReadyDomainCapabilityContribution<
        ForgeQuerySupportContributionPayload,
        T,
    >;

pub type ForgeQueryRequestedInvariantCapabilityContribution<T> =
    ForgeQueryRequestedDomainCapabilityContribution<
        ForgeQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type ForgeQueryEligibleInvariantCapabilityContribution<T> =
    ForgeQueryEligibleDomainCapabilityContribution<
        ForgeQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type ForgeQueryAdmittedInvariantCapabilityContribution<T> =
    ForgeQueryAdmittedDomainCapabilityContribution<
        ForgeQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type ForgeQueryMaterializationReadyInvariantCapabilityContribution<T> =
    ForgeQueryMaterializationReadyDomainCapabilityContribution<
        ForgeQueryInvariantCapabilityContributionPayload,
        T,
    >;

pub type ForgeQueryRequestedWorkflowContribution<T> =
    ForgeQueryRequestedDomainCapabilityContribution<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryEligibleWorkflowContribution<T> =
    ForgeQueryEligibleDomainCapabilityContribution<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryAdmittedWorkflowContribution<T> =
    ForgeQueryAdmittedDomainCapabilityContribution<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryMaterializationReadyWorkflowContribution<T> =
    ForgeQueryMaterializationReadyDomainCapabilityContribution<
        ForgeQueryWorkflowContributionPayload,
        T,
    >;

pub type ForgeQueryRequestedContinuityContribution<T> =
    ForgeQueryRequestedDomainCapabilityContribution<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryEligibleContinuityContribution<T> =
    ForgeQueryEligibleDomainCapabilityContribution<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryAdmittedContinuityContribution<T> =
    ForgeQueryAdmittedDomainCapabilityContribution<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryMaterializationReadyContinuityContribution<T> =
    ForgeQueryMaterializationReadyDomainCapabilityContribution<
        ForgeQueryContinuityContributionPayload,
        T,
    >;

pub type ForgeQueryRequestedAftermathContribution<T> =
    ForgeQueryRequestedDomainCapabilityContribution<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryEligibleAftermathContribution<T> =
    ForgeQueryEligibleDomainCapabilityContribution<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryAdmittedAftermathContribution<T> =
    ForgeQueryAdmittedDomainCapabilityContribution<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryMaterializationReadyAftermathContribution<T> =
    ForgeQueryMaterializationReadyDomainCapabilityContribution<
        ForgeQueryAftermathContributionPayload,
        T,
    >;

pub type ForgeQueryRequestedExplanationContribution<T> =
    ForgeQueryRequestedDomainCapabilityContribution<ForgeQueryExplanationContributionPayload, T>;
pub type ForgeQueryEligibleExplanationContribution<T> =
    ForgeQueryEligibleDomainCapabilityContribution<ForgeQueryExplanationContributionPayload, T>;
pub type ForgeQueryAdmittedExplanationContribution<T> =
    ForgeQueryAdmittedDomainCapabilityContribution<ForgeQueryExplanationContributionPayload, T>;
pub type ForgeQueryMaterializationReadyExplanationContribution<T> =
    ForgeQueryMaterializationReadyDomainCapabilityContribution<
        ForgeQueryExplanationContributionPayload,
        T,
    >;

pub(crate) fn create_requested_domain_capability_contribution<P, T>(
    target: T,
    payload: P,
) -> ForgeQueryRequestedDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
    (P, T): AllowedContributionBinding<P, T>,
{
    let binding_basis = target.binding_digest().to_string();
    ForgeQueryRequestedDomainCapabilityContribution(remint_with_phase(
        ForgeQueryDomainCapabilityContribution::new(target, payload),
        binding_basis,
        requested_contribution_proof(),
    ))
}

pub(crate) fn contribution_basis<Phase, P, T, S>(
    artifact: &Artifact<
        Phase,
        ForgeQueryDomainCapabilityContribution<P, T>,
        S,
        DomainCapabilityBasis,
    >,
) -> String
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    artifact.basis().basis().value().clone()
}

pub(crate) fn remint_with_phase<Phase, P, T, Proofs>(
    payload: ForgeQueryDomainCapabilityContribution<P, T>,
    basis: String,
    proofs: Proofs,
) -> Artifact<Phase, ForgeQueryDomainCapabilityContribution<P, T>, Proofs, DomainCapabilityBasis>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
    Proofs: ProofSet + ProofSetAuthorizedBy<DomainCapabilityContributionAuthority>,
{
    Artifact::with_proofs_and_current_basis(
        payload,
        proofs,
        basis,
        forge_proof::AuthorityWitness::from_authority_marker(DomainCapabilityContributionAuthority),
    )
}

pub(crate) fn eligible_proof() -> EligibleContributionProof {
    eligible_contribution_proof()
}

pub(crate) fn admitted_proof() -> AdmittedContributionProof {
    admitted_contribution_proof()
}

pub(crate) fn materialization_ready_proof() -> MaterializationReadyContributionProof {
    materialization_ready_contribution_proof()
}
