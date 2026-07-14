use worth_proof::{
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
use crate::domain_capabilities::identity::domain_capability_scope_encoder;
use crate::domain_capabilities::payloads::{
    WorthQueryAdmissionContributionPayload, WorthQueryAftermathContributionPayload,
    WorthQueryContinuityContributionPayload, WorthQueryDomainCapabilityPayload,
    WorthQueryExplanationContributionPayload, WorthQueryInvariantCapabilityContributionPayload,
    WorthQuerySupportContributionPayload, WorthQueryWorkflowContributionPayload,
};
use crate::domain_capabilities::targets::{
    WorthQueryAdmittedPlanBoundContributionTarget, WorthQueryDeclarationBoundContributionTarget,
    WorthQueryDomainCapabilityTargetBinding, WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::evidence_identity::{WorthQueryEvidenceIdentity, WorthQueryEvidenceTag};

type DomainCapabilityBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<String>>;

mod allowed_bindings {
    use super::*;

    pub trait Sealed {}

    pub trait AllowedContributionBinding<P, T>: Sealed
    where
        P: WorthQueryDomainCapabilityPayload,
        T: WorthQueryDomainCapabilityTargetBinding,
    {
    }

    macro_rules! allow {
        ($payload:ty => [$($target:ty),+ $(,)?]) => {
            $(impl Sealed for ($payload, $target) {}
            impl AllowedContributionBinding<$payload, $target> for ($payload, $target) {})+
        };
    }

    allow!(
        WorthQueryAdmissionContributionPayload => [
            WorthQueryDeclarationBoundContributionTarget,
            WorthQueryAdmittedPlanBoundContributionTarget
        ]
    );
    allow!(
        WorthQuerySupportContributionPayload => [
            WorthQueryDeclarationBoundContributionTarget,
            WorthQueryAdmittedPlanBoundContributionTarget,
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );
    allow!(
        WorthQueryInvariantCapabilityContributionPayload => [
            WorthQueryDeclarationBoundContributionTarget,
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );
    allow!(
        WorthQueryWorkflowContributionPayload => [
            WorthQueryDeclarationBoundContributionTarget,
            WorthQueryAdmittedPlanBoundContributionTarget
        ]
    );
    allow!(
        WorthQueryContinuityContributionPayload => [
            WorthQueryDeclarationBoundContributionTarget,
            WorthQueryAdmittedPlanBoundContributionTarget
        ]
    );
    allow!(
        WorthQueryAftermathContributionPayload => [
            WorthQueryAdmittedPlanBoundContributionTarget,
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );
    allow!(
        WorthQueryExplanationContributionPayload => [
            WorthQueryDeclarationBoundContributionTarget,
            WorthQueryAdmittedPlanBoundContributionTarget,
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget
        ]
    );

    impl<P, T> Sealed
        for (
            P,
            crate::domain_capabilities::WorthQueryInstalledDomainContributionTarget<T>,
        )
    where
        P: WorthQueryDomainCapabilityPayload,
        T: WorthQueryDomainCapabilityTargetBinding,
        (P, T): AllowedContributionBinding<P, T>,
    {
    }

    impl<P, T>
        AllowedContributionBinding<
            P,
            crate::domain_capabilities::WorthQueryInstalledDomainContributionTarget<T>,
        >
        for (
            P,
            crate::domain_capabilities::WorthQueryInstalledDomainContributionTarget<T>,
        )
    where
        P: WorthQueryDomainCapabilityPayload,
        T: WorthQueryDomainCapabilityTargetBinding,
        (P, T): AllowedContributionBinding<P, T>,
    {
    }
}

pub(crate) use allowed_bindings::AllowedContributionBinding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    target: T,
    payload: P,
    request_identity: WorthQueryEvidenceIdentity,
}

impl<P, T> WorthQueryDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn category(&self) -> crate::domain_capabilities::WorthQueryDomainCapabilityCategory {
        self.payload.category()
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn request_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_digest(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn installed_authority(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryInstalledDomainAuthority> {
        self.target.installed_authority()
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
) -> WorthQueryEvidenceIdentity
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    let binding_identity = target.binding_identity();
    let mut identity = domain_capability_scope_encoder("worth_query_domain_capability_request_v2")
        .field_shape(
            WorthQueryEvidenceTag::new("category"),
            payload.category().as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("binding"), &binding_identity)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("payload"),
            payload.payload_identity(),
        );
    if let Some(authority) = target.installed_authority() {
        identity = identity
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("installed_authority"),
                authority.authority_identity(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("installed_world"),
                authority.world_identity(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("installed_package"),
                authority.package_identity().evidence_identity(),
            )
            .field_usize(
                WorthQueryEvidenceTag::new("installed_generation"),
                authority.installation_generation().ordinal() as usize,
            );
    }
    identity.seal()
}

type RequestedContributionArtifact<P, T> = Artifact<
    ContributionRequestedPhase,
    WorthQueryDomainCapabilityContribution<P, T>,
    RequestedContributionProof,
    DomainCapabilityBasis,
>;
type EligibleContributionArtifact<P, T> = Artifact<
    ContributionEligiblePhase,
    WorthQueryDomainCapabilityContribution<P, T>,
    EligibleContributionProof,
    DomainCapabilityBasis,
>;
type AdmittedContributionArtifact<P, T> = Artifact<
    ContributionAdmittedPhase,
    WorthQueryDomainCapabilityContribution<P, T>,
    AdmittedContributionProof,
    DomainCapabilityBasis,
>;
type MaterializationReadyContributionArtifact<P, T> = Artifact<
    ContributionMaterializationReadyPhase,
    WorthQueryDomainCapabilityContribution<P, T>,
    MaterializationReadyContributionProof,
    DomainCapabilityBasis,
>;

pub struct WorthQueryRequestedDomainCapabilityContribution<P, T>(
    pub(crate) RequestedContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryEligibleDomainCapabilityContribution<P, T>(
    pub(crate) EligibleContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryAdmittedDomainCapabilityContribution<P, T>(
    pub(crate) AdmittedContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>(
    pub(crate) MaterializationReadyContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

macro_rules! impl_wrapper_accessors {
    ($name:ident, $inner:ident) => {
        impl<P, T> $name<P, T>
        where
            P: WorthQueryDomainCapabilityPayload,
            T: WorthQueryDomainCapabilityTargetBinding,
        {
            pub fn payload(&self) -> &WorthQueryDomainCapabilityContribution<P, T> {
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
        WorthQueryDomainCapabilityContribution<P, T>,
        S,
        DomainCapabilityBasis,
    >,
) -> WorthQueryEvidenceIdentity
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    domain_capability_scope_encoder("worth_query_domain_capability_phase_v1")
        .field_shape(WorthQueryEvidenceTag::new("phase"), phase)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("request"),
            artifact.payload().request_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("basis"),
            &contribution_basis_identity(artifact),
        )
        .seal()
}

pub(crate) fn contribution_basis_identity<Phase, P, T, S>(
    artifact: &Artifact<
        Phase,
        WorthQueryDomainCapabilityContribution<P, T>,
        S,
        DomainCapabilityBasis,
    >,
) -> WorthQueryEvidenceIdentity
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    domain_capability_scope_encoder("worth_query_domain_capability_basis_v1")
        .field_shape(
            WorthQueryEvidenceTag::new("assumption"),
            contribution_basis(artifact).as_str(),
        )
        .seal()
}

impl_wrapper_accessors!(
    WorthQueryRequestedDomainCapabilityContribution,
    RequestedContributionArtifact
);
impl_wrapper_accessors!(
    WorthQueryEligibleDomainCapabilityContribution,
    EligibleContributionArtifact
);
impl_wrapper_accessors!(
    WorthQueryAdmittedDomainCapabilityContribution,
    AdmittedContributionArtifact
);
impl_wrapper_accessors!(
    WorthQueryMaterializationReadyDomainCapabilityContribution,
    MaterializationReadyContributionArtifact
);

impl<P, T> WorthQueryRequestedDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn requested_identity(&self) -> WorthQueryEvidenceIdentity {
        phase_identity("requested", &self.0)
    }

    pub fn requested_for_reporting(&self) -> String {
        self.requested_identity().as_str().to_string()
    }
}

impl<P, T> WorthQueryEligibleDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn eligibility_identity(&self) -> WorthQueryEvidenceIdentity {
        phase_identity("eligible", &self.0)
    }

    pub fn eligibility_for_reporting(&self) -> String {
        self.eligibility_identity().as_str().to_string()
    }
}

impl<P, T> WorthQueryAdmittedDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn admitted_identity(&self) -> WorthQueryEvidenceIdentity {
        phase_identity("admitted", &self.0)
    }

    pub fn admitted_for_reporting(&self) -> String {
        self.admitted_identity().as_str().to_string()
    }
}

impl<P, T> WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn materialization_ready_identity(&self) -> WorthQueryEvidenceIdentity {
        phase_identity("materialization-ready", &self.0)
    }

    pub fn materialization_ready_for_reporting(&self) -> String {
        self.materialization_ready_identity().as_str().to_string()
    }
}

pub type WorthQueryRequestedAdmissionContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQueryEligibleAdmissionContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQueryAdmittedAdmissionContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQueryMaterializationReadyAdmissionContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryAdmissionContributionPayload,
        T,
    >;

pub type WorthQueryRequestedSupportContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryEligibleSupportContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryAdmittedSupportContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryMaterializationReadySupportContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQuerySupportContributionPayload,
        T,
    >;

pub type WorthQueryRequestedInvariantCapabilityContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryEligibleInvariantCapabilityContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryAdmittedInvariantCapabilityContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryMaterializationReadyInvariantCapabilityContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;

pub type WorthQueryRequestedWorkflowContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryEligibleWorkflowContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryAdmittedWorkflowContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryMaterializationReadyWorkflowContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryWorkflowContributionPayload,
        T,
    >;

pub type WorthQueryRequestedContinuityContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryEligibleContinuityContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryAdmittedContinuityContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryMaterializationReadyContinuityContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryContinuityContributionPayload,
        T,
    >;

pub type WorthQueryRequestedAftermathContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryEligibleAftermathContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryAdmittedAftermathContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryMaterializationReadyAftermathContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryAftermathContributionPayload,
        T,
    >;

pub type WorthQueryRequestedExplanationContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryExplanationContributionPayload, T>;
pub type WorthQueryEligibleExplanationContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryExplanationContributionPayload, T>;
pub type WorthQueryAdmittedExplanationContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryExplanationContributionPayload, T>;
pub type WorthQueryMaterializationReadyExplanationContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryExplanationContributionPayload,
        T,
    >;

pub(crate) fn create_requested_domain_capability_contribution<P, T>(
    target: T,
    payload: P,
) -> WorthQueryRequestedDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
    (P, T): AllowedContributionBinding<P, T>,
{
    let binding_basis = target.binding_identity().as_str().to_string();
    WorthQueryRequestedDomainCapabilityContribution(remint_with_phase(
        WorthQueryDomainCapabilityContribution::new(target, payload),
        binding_basis,
        requested_contribution_proof(),
    ))
}

pub(crate) fn contribution_basis<Phase, P, T, S>(
    artifact: &Artifact<
        Phase,
        WorthQueryDomainCapabilityContribution<P, T>,
        S,
        DomainCapabilityBasis,
    >,
) -> String
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    artifact.basis().basis().value().clone()
}

pub(crate) fn remint_with_phase<Phase, P, T, Proofs>(
    payload: WorthQueryDomainCapabilityContribution<P, T>,
    basis: String,
    proofs: Proofs,
) -> Artifact<Phase, WorthQueryDomainCapabilityContribution<P, T>, Proofs, DomainCapabilityBasis>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
    Proofs: ProofSet + ProofSetAuthorizedBy<DomainCapabilityContributionAuthority>,
{
    Artifact::with_proofs_and_current_basis(
        payload,
        proofs,
        basis,
        worth_proof::AuthorityWitness::from_authority_marker(DomainCapabilityContributionAuthority),
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
