use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQuerySettledWorkflowProjection;

use super::states::{
    mint_current_proof, CurrentProjectionProof, RebindProjectionProof, RevalidationProjectionProof,
    StaleProjectionProof, WorthQueryProjectionLifecycleBasis,
    WorthQueryProjectionLifecycleEvidence,
};
use super::{WorthQueryLiveProjectionReceipt, WorthQueryProjectionPromotionCounters};

pub struct WorthQueryCurrentWorkflowProjection<D, O, F, L: BasisOperationLane> {
    pub(super) settled: WorthQuerySettledWorkflowProjection<D, O, F, L>,
    proof: CurrentProjectionProof<L>,
}

pub struct WorthQueryStaleReadableWorkflowProjection<D, O, F, L: BasisOperationLane> {
    settled: WorthQuerySettledWorkflowProjection<D, O, F, L>,
    proof: StaleProjectionProof<L>,
    counters: WorthQueryProjectionPromotionCounters,
}

pub struct WorthQueryAuthorityRevalidationWorkflowProjection<D, O, F, L: BasisOperationLane> {
    settled: WorthQuerySettledWorkflowProjection<D, O, F, L>,
    proof: RevalidationProjectionProof<L>,
    counters: WorthQueryProjectionPromotionCounters,
}

pub struct WorthQueryRebindRequiredWorkflowProjection<D, O, F, L: BasisOperationLane> {
    settled: WorthQuerySettledWorkflowProjection<D, O, F, L>,
    proof: RebindProjectionProof<L>,
    counters: WorthQueryProjectionPromotionCounters,
}

#[must_use = "live workflow projections own a managed Query resource until dropped"]
pub struct WorthQueryLiveBoundWorkflowProjection<D, O, F, L: BasisOperationLane> {
    pub(super) owner: super::operational_owner::WorthQueryOperationalProjection<
        WorthQuerySettledWorkflowProjection<D, O, F, L>,
        L,
        super::states::WorthQueryLiveProjectionPhase,
    >,
}

impl<D, O, F, L: BasisOperationLane> WorthQuerySettledWorkflowProjection<D, O, F, L> {
    pub fn into_lifecycle(self) -> WorthQueryCurrentWorkflowProjection<D, O, F, L> {
        let basis = WorthQueryProjectionLifecycleBasis::from_source(&self);
        let identity = crate::identity::hash_parts(&[
            "worth_query_current_workflow_projection_lifecycle_v1".into(),
            format!("settled:{}", self.identity()),
            format!("trace:{}", self.trace().identity()),
            format!("publication_stage:{}", self.publication_stage_identity()),
            format!("binding:{}", self.bound_operation().binding_identity()),
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
        WorthQueryCurrentWorkflowProjection {
            settled: self,
            proof,
        }
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCurrentWorkflowProjection<D, O, F, L> {
    pub fn identity(&self) -> &str {
        &self.proof.payload().identity
    }

    pub fn snapshot(&self) -> &WorthQuerySettledWorkflowProjection<D, O, F, L> {
        &self.settled
    }

    pub(super) fn lifecycle_basis(&self) -> &WorthQueryProjectionLifecycleBasis<L> {
        self.proof.strong_basis().value()
    }

    pub(super) fn into_live_parts(
        self,
    ) -> (
        WorthQuerySettledWorkflowProjection<D, O, F, L>,
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
    ) -> WorthQueryStaleReadableWorkflowProjection<D, O, F, L> {
        WorthQueryStaleReadableWorkflowProjection {
            settled: self.settled,
            proof: self.proof.downgrade_to_stale_readable(),
            counters,
        }
    }

    pub(super) fn into_authority_revalidation(
        self,
        counters: WorthQueryProjectionPromotionCounters,
    ) -> WorthQueryAuthorityRevalidationWorkflowProjection<D, O, F, L> {
        WorthQueryAuthorityRevalidationWorkflowProjection {
            settled: self.settled,
            proof: self.proof.downgrade_to_authority_revalidation_required(),
            counters,
        }
    }

    pub(super) fn into_rebind_required(
        self,
        counters: WorthQueryProjectionPromotionCounters,
    ) -> WorthQueryRebindRequiredWorkflowProjection<D, O, F, L> {
        WorthQueryRebindRequiredWorkflowProjection {
            settled: self.settled,
            proof: self.proof.downgrade_to_rebind_required(),
            counters,
        }
    }
}

macro_rules! inspection_state {
    ($name:ident) => {
        impl<D, O, F, L: BasisOperationLane> $name<D, O, F, L> {
            pub fn snapshot(&self) -> &WorthQuerySettledWorkflowProjection<D, O, F, L> {
                &self.settled
            }

            pub fn counters(&self) -> WorthQueryProjectionPromotionCounters {
                self.counters
            }

            pub fn identity(&self) -> &str {
                &self.proof.payload().identity
            }
        }
    };
}

inspection_state!(WorthQueryStaleReadableWorkflowProjection);
inspection_state!(WorthQueryAuthorityRevalidationWorkflowProjection);
inspection_state!(WorthQueryRebindRequiredWorkflowProjection);

impl<D, O, F, L: BasisOperationLane> WorthQueryLiveBoundWorkflowProjection<D, O, F, L> {
    pub(super) fn mint(
        settled: WorthQuerySettledWorkflowProjection<D, O, F, L>,
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

    pub fn snapshot(&self) -> &WorthQuerySettledWorkflowProjection<D, O, F, L> {
        self.owner.source()
    }

    pub fn conditional_provenance(
        &self,
    ) -> &[super::super::super::WorthQueryConditionalProvenance] {
        self.owner.conditional_provenance()
    }

    pub(super) fn managed_handle(&self) -> &crate::ordinary::live::WorthQueryManagedLiveHandle {
        self.owner.handle()
    }

    pub(super) fn from_owner(
        owner: super::operational_owner::WorthQueryOperationalProjection<
            WorthQuerySettledWorkflowProjection<D, O, F, L>,
            L,
            super::states::WorthQueryLiveProjectionPhase,
        >,
    ) -> Self {
        Self { owner }
    }

    pub(super) fn into_owner(
        self,
    ) -> super::operational_owner::WorthQueryOperationalProjection<
        WorthQuerySettledWorkflowProjection<D, O, F, L>,
        L,
        super::states::WorthQueryLiveProjectionPhase,
    > {
        self.owner
    }
}
