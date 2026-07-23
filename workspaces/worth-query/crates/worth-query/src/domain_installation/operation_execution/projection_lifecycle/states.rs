use std::sync::Arc;

use crate::basis_lifecycle::{AdmittedBasisCapability, BasisOperationLane};
use crate::domain_installation::{
    WorthQueryInstalledDomainAuthority, WorthQuerySettledDomainProjection,
};
use crate::runtime::WorthQueryRuntimeAuthorityIdentity;
use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityRevalidationRequiredBasis,
    AuthorityWitness, CurrentValidity, FreshnessScopedBasis, NoProofs, PhaseMarker,
    RebindRequiredBasis, StaleReadableBasis,
};

use super::{WorthQueryLiveProjectionReceipt, WorthQueryProjectionPromotionCounters};

pub(super) struct WorthQueryCurrentProjectionPhase;
pub(in crate::domain_installation::operation_execution) struct WorthQueryLiveProjectionPhase;
pub(super) struct WorthQueryProjectionLifecycleAuthority {
    _private: (),
}

impl AuthorityMarker for WorthQueryProjectionLifecycleAuthority {}
impl PhaseMarker for WorthQueryCurrentProjectionPhase {}
impl PhaseMarker for WorthQueryLiveProjectionPhase {}

pub(in crate::domain_installation::operation_execution) struct WorthQueryProjectionLifecycleEvidence
{
    pub(super) identity: String,
    pub(super) predecessor_identity: String,
}

impl WorthQueryProjectionLifecycleEvidence {
    pub(in crate::domain_installation::operation_execution) fn identity(&self) -> &str {
        &self.identity
    }
}

#[derive(Clone)]
pub(in crate::domain_installation::operation_execution) struct WorthQueryProjectionLifecycleBasis<
    L: BasisOperationLane,
> {
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    installation_generation: super::super::super::WorthQueryDomainInstallationGeneration,
    domain_authority: Arc<WorthQueryInstalledDomainAuthority>,
    binding_identity: String,
    capability_identity: u64,
    capability_generation: crate::domain_installation::WorthQueryBoundCapabilityGeneration,
    basis: AdmittedBasisCapability<L>,
}

pub(super) type CurrentProjectionProof<L> = Artifact<
    WorthQueryCurrentProjectionPhase,
    WorthQueryProjectionLifecycleEvidence,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryProjectionLifecycleBasis<L>>>,
>;
pub(super) type StaleProjectionProof<L> = Artifact<
    WorthQueryCurrentProjectionPhase,
    WorthQueryProjectionLifecycleEvidence,
    NoProofs,
    StaleReadableBasis<WorthQueryProjectionLifecycleBasis<L>>,
>;
pub(super) type RevalidationProjectionProof<L> = Artifact<
    WorthQueryCurrentProjectionPhase,
    WorthQueryProjectionLifecycleEvidence,
    NoProofs,
    AuthorityRevalidationRequiredBasis<WorthQueryProjectionLifecycleBasis<L>>,
>;
pub(super) type RebindProjectionProof<L> = Artifact<
    WorthQueryCurrentProjectionPhase,
    WorthQueryProjectionLifecycleEvidence,
    NoProofs,
    RebindRequiredBasis<WorthQueryProjectionLifecycleBasis<L>>,
>;
pub struct WorthQueryCurrentDomainProjection<D, O, F, L: BasisOperationLane> {
    pub(super) settled: WorthQuerySettledDomainProjection<D, O, F, L>,
    proof: CurrentProjectionProof<L>,
}

pub struct WorthQueryStaleReadableDomainProjection<D, O, F, L: BasisOperationLane> {
    settled: WorthQuerySettledDomainProjection<D, O, F, L>,
    proof: StaleProjectionProof<L>,
    counters: WorthQueryProjectionPromotionCounters,
}

pub struct WorthQueryAuthorityRevalidationDomainProjection<D, O, F, L: BasisOperationLane> {
    settled: WorthQuerySettledDomainProjection<D, O, F, L>,
    proof: RevalidationProjectionProof<L>,
    counters: WorthQueryProjectionPromotionCounters,
}

pub struct WorthQueryRebindRequiredDomainProjection<D, O, F, L: BasisOperationLane> {
    settled: WorthQuerySettledDomainProjection<D, O, F, L>,
    proof: RebindProjectionProof<L>,
    counters: WorthQueryProjectionPromotionCounters,
}

#[must_use = "live projection capabilities own a managed Query resource until transitioned or dropped"]
pub struct WorthQueryLiveBoundDomainProjection<D, O, F, L: BasisOperationLane> {
    pub(super) owner: super::operational_owner::WorthQueryOperationalProjection<
        WorthQuerySettledDomainProjection<D, O, F, L>,
        L,
        WorthQueryLiveProjectionPhase,
    >,
}

impl<D, O, F, L: BasisOperationLane> WorthQuerySettledDomainProjection<D, O, F, L> {
    pub fn into_lifecycle(self) -> WorthQueryCurrentDomainProjection<D, O, F, L> {
        let basis = WorthQueryProjectionLifecycleBasis::from_source(&self);
        let identity = crate::identity::hash_parts(&[
            "worth_query_current_projection_lifecycle_v1".into(),
            format!("settled:{}", self.identity()),
            format!("binding:{}", self.execution_receipt().binding_identity()),
            format!(
                "basis:{}",
                self.bound_operation().basis().capability_digest()
            ),
        ]);
        let proof = mint_current_proof(
            WorthQueryProjectionLifecycleEvidence {
                identity,
                predecessor_identity: self.identity().to_string(),
            },
            basis,
        );
        WorthQueryCurrentDomainProjection {
            settled: self,
            proof,
        }
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCurrentDomainProjection<D, O, F, L> {
    pub fn identity(&self) -> &str {
        &self.proof.payload().identity
    }

    pub fn snapshot(&self) -> &WorthQuerySettledDomainProjection<D, O, F, L> {
        &self.settled
    }

    pub(in crate::domain_installation::operation_execution) fn lifecycle_basis(
        &self,
    ) -> &WorthQueryProjectionLifecycleBasis<L> {
        self.proof.strong_basis().value()
    }

    pub(in crate::domain_installation::operation_execution) fn into_live_parts(
        self,
    ) -> (
        WorthQuerySettledDomainProjection<D, O, F, L>,
        WorthQueryProjectionLifecycleBasis<L>,
        String,
    ) {
        let basis = self.proof.strong_basis().value().clone();
        let identity = self.proof.payload().identity.clone();
        (self.settled, basis, identity)
    }

    pub(super) fn into_stale(
        self,
        counters: WorthQueryProjectionPromotionCounters,
    ) -> WorthQueryStaleReadableDomainProjection<D, O, F, L> {
        WorthQueryStaleReadableDomainProjection {
            settled: self.settled,
            proof: self.proof.downgrade_to_stale_readable(),
            counters,
        }
    }

    pub(super) fn into_authority_revalidation(
        self,
        counters: WorthQueryProjectionPromotionCounters,
    ) -> WorthQueryAuthorityRevalidationDomainProjection<D, O, F, L> {
        WorthQueryAuthorityRevalidationDomainProjection {
            settled: self.settled,
            proof: self.proof.downgrade_to_authority_revalidation_required(),
            counters,
        }
    }

    pub(super) fn into_rebind_required(
        self,
        counters: WorthQueryProjectionPromotionCounters,
    ) -> WorthQueryRebindRequiredDomainProjection<D, O, F, L> {
        WorthQueryRebindRequiredDomainProjection {
            settled: self.settled,
            proof: self.proof.downgrade_to_rebind_required(),
            counters,
        }
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryStaleReadableDomainProjection<D, O, F, L> {
    pub fn snapshot(&self) -> &WorthQuerySettledDomainProjection<D, O, F, L> {
        &self.settled
    }

    pub fn counters(&self) -> WorthQueryProjectionPromotionCounters {
        self.counters
    }

    pub fn identity(&self) -> &str {
        &self.proof.payload().identity
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryAuthorityRevalidationDomainProjection<D, O, F, L> {
    pub fn snapshot(&self) -> &WorthQuerySettledDomainProjection<D, O, F, L> {
        &self.settled
    }

    pub fn counters(&self) -> WorthQueryProjectionPromotionCounters {
        self.counters
    }

    pub fn identity(&self) -> &str {
        &self.proof.payload().identity
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryRebindRequiredDomainProjection<D, O, F, L> {
    pub fn snapshot(&self) -> &WorthQuerySettledDomainProjection<D, O, F, L> {
        &self.settled
    }

    pub fn counters(&self) -> WorthQueryProjectionPromotionCounters {
        self.counters
    }

    pub fn identity(&self) -> &str {
        &self.proof.payload().identity
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryLiveBoundDomainProjection<D, O, F, L> {
    pub(super) fn mint(
        settled: WorthQuerySettledDomainProjection<D, O, F, L>,
        basis: WorthQueryProjectionLifecycleBasis<L>,
        predecessor_identity: String,
        handle: crate::ordinary::live::WorthQueryManagedLiveHandle,
        receipt: WorthQueryLiveProjectionReceipt,
        conditional_provenance: Vec<super::super::super::WorthQueryConditionalProvenance>,
    ) -> Self {
        Self {
            owner: super::operational_owner::WorthQueryOperationalProjection::mint(
                settled,
                basis,
                predecessor_identity,
                receipt.operational_identity().to_string(),
                handle,
                receipt,
                conditional_provenance,
            ),
        }
    }

    pub fn identity(&self) -> &str {
        debug_assert_eq!(
            self.owner.proof().payload().identity,
            self.owner.receipt().operational_identity()
        );
        &self.owner.proof().payload().identity
    }

    pub fn predecessor_identity(&self) -> &str {
        &self.owner.proof().payload().predecessor_identity
    }

    pub fn resource_name(&self) -> &str {
        self.owner.handle().name()
    }

    pub fn receipt(&self) -> &WorthQueryLiveProjectionReceipt {
        self.owner.receipt()
    }

    pub fn snapshot(&self) -> &WorthQuerySettledDomainProjection<D, O, F, L> {
        self.owner.source()
    }

    pub fn conditional_provenance(
        &self,
    ) -> &[super::super::super::WorthQueryConditionalProvenance] {
        self.owner.conditional_provenance()
    }

    pub(in crate::domain_installation::operation_execution) fn lifecycle_basis(
        &self,
    ) -> &WorthQueryProjectionLifecycleBasis<L> {
        self.owner.proof().strong_basis().value()
    }

    pub(super) fn managed_handle(&self) -> &crate::ordinary::live::WorthQueryManagedLiveHandle {
        self.owner.handle()
    }

    pub(in crate::domain_installation::operation_execution) fn from_owner(
        owner: super::operational_owner::WorthQueryOperationalProjection<
            WorthQuerySettledDomainProjection<D, O, F, L>,
            L,
            WorthQueryLiveProjectionPhase,
        >,
    ) -> Self {
        Self { owner }
    }

    pub(in crate::domain_installation::operation_execution) fn into_owner(
        self,
    ) -> super::operational_owner::WorthQueryOperationalProjection<
        WorthQuerySettledDomainProjection<D, O, F, L>,
        L,
        WorthQueryLiveProjectionPhase,
    > {
        self.owner
    }
}

impl<L: BasisOperationLane> WorthQueryProjectionLifecycleBasis<L> {
    pub(in crate::domain_installation::operation_execution) const fn capability_generation(
        &self,
    ) -> crate::domain_installation::WorthQueryBoundCapabilityGeneration {
        self.capability_generation
    }

    pub(super) fn from_source<D, O, F, S>(source: &S) -> Self
    where
        S: super::source::WorthQueryProjectionLifecycleSource<D, O, F, L>,
    {
        let bound = source.bound_operation();
        Self {
            runtime_authority: bound.operation().domain_authority().runtime_authority(),
            installation_generation: bound.operation().installation_generation(),
            domain_authority: Arc::clone(bound.operation().domain_authority()),
            binding_identity: bound.binding_identity().to_string(),
            capability_identity: bound.capability_identity(),
            capability_generation: super::WorthQueryBoundCapabilityGeneration::mint(),
            basis: bound.basis().clone(),
        }
    }

    pub(super) fn binds<D, O, F, S>(&self, source: &S, checks: &mut usize) -> bool
    where
        S: super::source::WorthQueryProjectionLifecycleSource<D, O, F, L>,
    {
        let bound = source.bound_operation();
        exact(
            checks,
            self.runtime_authority == bound.operation().domain_authority().runtime_authority(),
        ) && exact(
            checks,
            self.installation_generation == bound.operation().installation_generation(),
        ) && exact(
            checks,
            Arc::ptr_eq(&self.domain_authority, bound.operation().domain_authority()),
        ) && exact(checks, self.binding_identity == bound.binding_identity())
            && exact(
                checks,
                self.capability_identity == bound.capability_identity(),
            )
            && exact(checks, &self.basis == bound.basis())
    }
}

fn exact(checks: &mut usize, matches: bool) -> bool {
    *checks += 1;
    matches
}

pub(super) fn mint_current_proof<L: BasisOperationLane>(
    evidence: WorthQueryProjectionLifecycleEvidence,
    basis: WorthQueryProjectionLifecycleBasis<L>,
) -> CurrentProjectionProof<L> {
    Artifact::with_current_basis(evidence, basis, lifecycle_authority())
}

pub(super) fn lifecycle_authority() -> AuthorityWitness<WorthQueryProjectionLifecycleAuthority> {
    AuthorityWitness::from_authority_marker(WorthQueryProjectionLifecycleAuthority { _private: () })
}
