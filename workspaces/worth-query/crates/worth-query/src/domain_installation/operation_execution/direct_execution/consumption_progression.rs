use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryConsumedOperationPhase,
    WorthQueryOperationPhaseProof, WorthQueryPublishedOperationPhase,
    WorthQuerySettledOperationPhase,
};
use crate::domain_installation::{
    WorthQueryBoundProjectionRequest, WorthQueryConsumerProjectionContract,
    WorthQueryNativeAccessDenial, WorthQueryNativeAccessKey, WorthQueryNativeAccessLayout,
    WorthQueryNativeFieldAccess, WorthQueryOperationResultState,
};
use crate::identity::hash_parts;
use crate::ordinary::read::{
    WorthQueryProjectionDeclaration, WorthQueryProjectionOutcome, WorthQueryReadCompletion,
};
use crate::projection_consumption::{
    ProjectionConsumptionWarnings, WorthQueryConsumedProjectionAuthority,
};
use worth_proof::TransitionOutcome;

use super::{
    WorthQueryBoundExecutionReceipt, WorthQueryDerivedPublicationReceipt,
    WorthQueryExecutedDomainOperation, WorthQueryOperationExecutionCounters,
    WorthQueryOperationExecutionWarning,
};

#[path = "consumption_progression/collection_delivery_contract.rs"]
mod collection_delivery_contract;

pub struct WorthQueryPublishedDomainOperation<D, O, F, L: BasisOperationLane> {
    executed: WorthQueryExecutedDomainOperation<D, O, F, L, WorthQueryReadCompletion>,
    receipt: WorthQueryDerivedPublicationReceipt,
    phase_proof: WorthQueryOperationPhaseProof<WorthQueryPublishedOperationPhase>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryPublishedDomainOperation<D, O, F, L> {
    pub(super) fn mint(
        executed: WorthQueryExecutedDomainOperation<D, O, F, L, WorthQueryReadCompletion>,
        receipt: WorthQueryDerivedPublicationReceipt,
        phase_proof: WorthQueryOperationPhaseProof<WorthQueryPublishedOperationPhase>,
    ) -> Self {
        Self {
            executed,
            receipt,
            phase_proof,
        }
    }

    pub fn receipt(&self) -> &WorthQueryDerivedPublicationReceipt {
        &self.receipt
    }

    pub fn conditional_provenance(
        &self,
    ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
        self.executed.conditional_provenance()
    }

    pub fn consume(
        mut self,
        consumer: WorthQueryConsumerProjectionContract<D, O, F, L>,
        request: WorthQueryProjectionDeclaration,
    ) -> TransitionOutcome<
        WorthQueryConsumedDomainProjection<D, O, F, L>,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
    > {
        if !self.executed.bound.installation_is_current() {
            return TransitionOutcome::Stale(
                WorthQueryProgressionDenial::StaleInstallationGeneration,
            );
        }
        if !consumer.binds_capability(self.executed.bound.capability_identity())
            || consumer.binding_identity() != self.executed.bound.binding_identity()
            || consumer.operation_identity() != self.executed.bound.definition().identity()
            || consumer.installation_generation()
                != self.executed.bound.operation().installation_generation()
            || consumer.basis_identity() != self.executed.bound.basis().capability_digest()
        {
            return TransitionOutcome::Denied(
                WorthQueryProgressionDenial::ConsumerContractMismatch,
            );
        }
        self.executed.counters.consumption_contacts += 1;
        let outcome = self.executed.output.consume_projection(request);
        let (authority, warnings) = match outcome.into_admitted() {
            Ok(admitted) => admitted,
            Err(outcome @ WorthQueryProjectionOutcome::Violation(_)) => {
                return TransitionOutcome::Denied(WorthQueryProgressionDenial::Projection(
                    Box::new(outcome),
                ));
            }
            Err(outcome @ WorthQueryProjectionOutcome::Deferred(_)) => {
                return TransitionOutcome::Deferred(WorthQueryProgressionDenial::Projection(
                    Box::new(outcome),
                ));
            }
            Err(outcome @ WorthQueryProjectionOutcome::Unavailable(_)) => {
                return TransitionOutcome::Failed(WorthQueryProgressionDenial::Projection(
                    Box::new(outcome),
                ));
            }
            Err(outcome) => {
                return TransitionOutcome::Failed(WorthQueryProgressionDenial::Projection(
                    Box::new(outcome),
                ));
            }
        };
        let consumption_identity = hash_parts(&[
            "worth_query_consumed_operation_v1".into(),
            format!("publication:{}", self.receipt.identity()),
            format!("consumption:{}", authority.receipt().receipt_digest()),
        ]);
        let phase_proof = mint_operation_phase_proof(
            consumption_identity,
            Some(self.phase_proof.payload().identity()),
            operation_phase_basis(&self.phase_proof).clone(),
        );
        TransitionOutcome::Success(WorthQueryConsumedDomainProjection {
            published: self,
            consumer,
            authority,
            projection_warnings: warnings,
            native_access: None,
            phase_proof,
        })
    }

    pub fn consume_bound(
        self,
        request: WorthQueryBoundProjectionRequest<D, O, F, L>,
    ) -> TransitionOutcome<
        WorthQueryConsumedDomainProjection<D, O, F, L>,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
    > {
        let (consumer, declaration, plan) = request.into_parts();
        match self.consume(consumer, declaration) {
            TransitionOutcome::Success(mut consumed) => {
                match WorthQueryNativeAccessLayout::admit(
                    plan,
                    &consumed.consumer,
                    &consumed.authority,
                ) {
                    Ok(layout) => {
                        consumed.native_access = Some(layout);
                        TransitionOutcome::Success(consumed)
                    }
                    Err(denial) => {
                        TransitionOutcome::Denied(WorthQueryProgressionDenial::NativeAccess(denial))
                    }
                }
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(denial) => TransitionOutcome::Deferred(denial),
            TransitionOutcome::Stale(denial) => TransitionOutcome::Stale(denial),
            TransitionOutcome::RebindRequired(denial) => TransitionOutcome::RebindRequired(denial),
            TransitionOutcome::Failed(denial) => TransitionOutcome::Failed(denial),
        }
    }
}

#[derive(Debug)]
pub enum WorthQueryProgressionDenial {
    StaleInstallationGeneration,
    ConsumerContractMismatch,
    DependencyCompilation(
        crate::domain_installation::WorthQuerySemanticAspectDependencyCompilationDenial,
    ),
    NativeAccess(WorthQueryNativeAccessDenial),
    Projection(Box<WorthQueryProjectionOutcome>),
}

pub struct WorthQueryConsumedDomainProjection<D, O, F, L: BasisOperationLane> {
    published: WorthQueryPublishedDomainOperation<D, O, F, L>,
    consumer: WorthQueryConsumerProjectionContract<D, O, F, L>,
    authority: Box<WorthQueryConsumedProjectionAuthority>,
    projection_warnings: Option<ProjectionConsumptionWarnings>,
    native_access: Option<WorthQueryNativeAccessLayout>,
    phase_proof: WorthQueryOperationPhaseProof<WorthQueryConsumedOperationPhase>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryConsumedDomainProjection<D, O, F, L> {
    pub fn authority(&self) -> &WorthQueryConsumedProjectionAuthority {
        &self.authority
    }

    pub fn projection_warnings(&self) -> Option<&ProjectionConsumptionWarnings> {
        self.projection_warnings.as_ref()
    }

    pub fn conditional_provenance(
        &self,
    ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
        self.published.conditional_provenance()
    }

    pub fn settle(
        self,
    ) -> TransitionOutcome<
        WorthQuerySettledDomainProjection<D, O, F, L>,
        WorthQueryProgressionDenial,
        std::convert::Infallible,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
        WorthQueryProgressionDenial,
    > {
        if !self.published.executed.bound.installation_is_current() {
            return TransitionOutcome::Stale(
                WorthQueryProgressionDenial::StaleInstallationGeneration,
            );
        }
        let identity = hash_parts(&[
            "worth_query_installed_projection_settlement_v1".into(),
            format!("publication:{}", self.published.receipt.identity()),
            format!("consumption:{}", self.authority.receipt().receipt_digest()),
        ]);
        let phase_proof = mint_operation_phase_proof(
            identity.clone(),
            Some(self.phase_proof.payload().identity()),
            operation_phase_basis(&self.phase_proof).clone(),
        );
        let dependency_closure = match
            crate::domain_installation::dependency_impact::compile_direct_semantic_aspect_dependencies(
                &self.published.executed.bound,
                self.published.executed.graph_receipts(),
                self.conditional_provenance(),
                self.published.executed.receipt(),
                self.published.receipt(),
            ) {
                Ok(closure) => closure,
                Err(denial) => {
                    return TransitionOutcome::Denied(
                        WorthQueryProgressionDenial::DependencyCompilation(denial),
                    )
                }
            };
        TransitionOutcome::Success(WorthQuerySettledDomainProjection {
            consumed: self,
            identity,
            phase_proof,
            dependency_closure: std::sync::Arc::new(dependency_closure),
        })
    }
}

pub struct WorthQuerySettledDomainProjection<D, O, F, L: BasisOperationLane> {
    consumed: WorthQueryConsumedDomainProjection<D, O, F, L>,
    identity: String,
    phase_proof: WorthQueryOperationPhaseProof<WorthQuerySettledOperationPhase>,
    dependency_closure: std::sync::Arc<
        crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure,
    >,
}

impl<D, O, F, L: BasisOperationLane> WorthQuerySettledDomainProjection<D, O, F, L> {
    pub(crate) fn bound_operation(
        &self,
    ) -> &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L> {
        &self.consumed.published.executed.bound
    }

    pub(crate) fn consumer_contract(&self) -> &WorthQueryConsumerProjectionContract<D, O, F, L> {
        &self.consumed.consumer
    }

    pub(crate) fn native_access_layout(&self) -> Option<&WorthQueryNativeAccessLayout> {
        self.consumed.native_access.as_ref()
    }

    pub(crate) fn collection_execution_rows(&self) -> &[crate::memory_workspace::WorthQueryEntity] {
        self.consumed.published.executed.output.result().rows()
    }

    pub(crate) fn collection_declarative_request(
        &self,
    ) -> Option<&crate::declarative_live::DeclarativeLiveQueryRequest> {
        self.bound_operation()
            .direct_executor()
            .installed_read
            .as_ref()
            .map(crate::ordinary::read::WorthQueryReadDeclaration::declarative_request)
    }

    pub fn identity(&self) -> &str {
        debug_assert_eq!(self.phase_proof.payload().identity(), self.identity);
        debug_assert_eq!(
            self.phase_proof.payload().predecessor_identity(),
            Some(self.consumed.phase_proof.payload().identity())
        );
        &self.identity
    }

    pub fn authority(&self) -> &WorthQueryConsumedProjectionAuthority {
        self.consumed.authority()
    }

    pub fn execution_receipt(&self) -> &WorthQueryBoundExecutionReceipt {
        self.consumed.published.executed.receipt()
    }

    pub fn publication_receipt(&self) -> &WorthQueryDerivedPublicationReceipt {
        self.consumed.published.receipt()
    }

    pub fn warnings(&self) -> &[WorthQueryOperationExecutionWarning] {
        self.consumed.published.executed.warnings()
    }

    pub fn projection_warnings(&self) -> Option<&ProjectionConsumptionWarnings> {
        self.consumed.projection_warnings()
    }

    pub fn result_state(&self) -> WorthQueryOperationResultState {
        self.execution_receipt().result_state()
    }

    pub fn counters(&self) -> WorthQueryOperationExecutionCounters {
        self.consumed.published.executed.counters()
    }

    pub fn resources(
        &self,
    ) -> &crate::domain_installation::WorthQueryAdmittedExecutionResourcePlan {
        self.consumed.published.executed.resources()
    }

    pub fn conditional_provenance(
        &self,
    ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
        self.consumed.conditional_provenance()
    }

    pub fn semantic_aspect_dependency_closure(
        &self,
    ) -> &crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure {
        &self.dependency_closure
    }

    pub(crate) fn dependency_closure_arc(
        &self,
    ) -> std::sync::Arc<crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure>
    {
        std::sync::Arc::clone(&self.dependency_closure)
    }

    pub fn classify_authoritative_impact(
        &self,
        delivery: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        conditional: &crate::domain_installation::WorthQueryConditionalProvenance,
    ) -> Result<
        crate::domain_installation::WorthQueryImpactDecision,
        crate::domain_installation::WorthQueryImpactAdmissionDenial,
    > {
        crate::domain_installation::classify_owner_delivered_impact(
            &self.dependency_closure,
            delivery,
            conditional,
        )
    }

    pub fn native_value<'a>(
        &'a self,
        key: &WorthQueryNativeAccessKey,
        row: usize,
    ) -> Result<WorthQueryNativeFieldAccess<'a>, WorthQueryNativeAccessDenial> {
        let Some(layout) = &self.consumed.native_access else {
            return Err(WorthQueryNativeAccessLayout::unbound_denial(
                &self.consumed.authority,
                key,
            ));
        };
        layout.access(&self.consumed.authority, key, row)
    }

    pub fn native_access_binding_counters(
        &self,
    ) -> Option<crate::domain_installation::WorthQueryNativeAccessBindingCounters> {
        self.consumed
            .native_access
            .as_ref()
            .map(WorthQueryNativeAccessLayout::binding_counters)
    }
}
