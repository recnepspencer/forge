use forge_proof::{
    Artifact, AssumptionBasis, CurrentValidity, FreshnessScopedBasis, ProofSet,
    ProofSetAuthorizedBy,
};

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
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
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
    request_identity: ForgeQueryEvidenceIdentity,
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

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_digest(&self) -> &str {
        self.request_identity.as_str()
    }

    fn new(target: T, payload: P) -> Self {
        let request_identity = compose_domain_capability_request_identity(&target, &payload);
        Self {
            target,
            payload,
            request_identity,
        }
    }
}

fn compose_domain_capability_request_identity<P, T>(
    target: &T,
    payload: &P,
) -> ForgeQueryEvidenceIdentity
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let binding_identity = target.binding_identity();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_request_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("category"),
            payload.category().as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("binding"), &binding_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("payload"),
            payload.payload_identity(),
        )
        .seal()
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

fn phase_identity<Phase, P, T, S>(
    phase: &'static str,
    artifact: &Artifact<
        Phase,
        ForgeQueryDomainCapabilityContribution<P, T>,
        S,
        DomainCapabilityBasis,
    >,
) -> ForgeQueryEvidenceIdentity
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_phase_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("phase"), phase)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("request"),
            artifact.payload().request_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            &contribution_basis_identity(artifact),
        )
        .seal()
}

pub(crate) fn contribution_basis_identity<Phase, P, T, S>(
    artifact: &Artifact<
        Phase,
        ForgeQueryDomainCapabilityContribution<P, T>,
        S,
        DomainCapabilityBasis,
    >,
) -> ForgeQueryEvidenceIdentity
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_basis_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("assumption"),
            contribution_basis(artifact).as_str(),
        )
        .seal()
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
    pub fn requested_identity(&self) -> ForgeQueryEvidenceIdentity {
        phase_identity("requested", &self.0)
    }

    pub fn requested_for_reporting(&self) -> String {
        self.requested_identity().as_str().to_string()
    }
}

impl<P, T> ForgeQueryEligibleDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn eligibility_identity(&self) -> ForgeQueryEvidenceIdentity {
        phase_identity("eligible", &self.0)
    }

    pub fn eligibility_for_reporting(&self) -> String {
        self.eligibility_identity().as_str().to_string()
    }
}

impl<P, T> ForgeQueryAdmittedDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn admitted_identity(&self) -> ForgeQueryEvidenceIdentity {
        phase_identity("admitted", &self.0)
    }

    pub fn admitted_for_reporting(&self) -> String {
        self.admitted_identity().as_str().to_string()
    }
}

impl<P, T> ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub fn materialization_ready_identity(&self) -> ForgeQueryEvidenceIdentity {
        phase_identity("materialization-ready", &self.0)
    }

    pub fn materialization_ready_for_reporting(&self) -> String {
        self.materialization_ready_identity().as_str().to_string()
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
