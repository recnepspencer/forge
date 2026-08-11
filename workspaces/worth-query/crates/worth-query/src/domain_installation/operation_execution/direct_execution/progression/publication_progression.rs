//! Publication transition for one completed direct execution.

use crate::identity::hash_parts;
use crate::ordinary::read::WorthQueryReadCompletion;

use super::{
    mint_operation_phase_proof, operation_phase_basis, BasisOperationLane, TransitionOutcome,
    WorthQueryExecutableDomainOperation, WorthQueryExecutedDomainOperation,
};
use crate::domain_installation::operation_execution::{
    WorthQueryDerivedPublicationReceipt, WorthQueryPublishedDomainOperation,
    WorthQueryPublishingOperation,
};

impl<D, O, F, L: BasisOperationLane>
    WorthQueryExecutedDomainOperation<D, O, F, L, WorthQueryReadCompletion>
where
    O: WorthQueryExecutableDomainOperation<
        D,
        F,
        Output = WorthQueryReadCompletion,
        Publication = WorthQueryPublishingOperation,
    >,
{
    pub fn publish(
        mut self,
    ) -> TransitionOutcome<
        WorthQueryPublishedDomainOperation<D, O, F, L>,
        WorthQueryPublicationDenial,
        std::convert::Infallible,
        WorthQueryPublicationDenial,
        WorthQueryPublicationDenial,
        WorthQueryPublicationDenial,
    > {
        if !self.bound.installation_is_current() {
            return TransitionOutcome::Stale(
                WorthQueryPublicationDenial::StaleInstallationGeneration,
            );
        }
        self.counters.publication_checks += 1;
        let canonical = &self.bound.definition().semantics().canonical_query;
        if !self.output.validates_installed_publication(
            canonical,
            self.bound.basis().normalized().family(),
            &self.execution_snapshot,
            self.bound
                .operation()
                .domain_authority()
                .runtime_authority(),
        ) {
            return TransitionOutcome::Denied(
                WorthQueryPublicationDenial::ExecutionMaterialMismatch,
            );
        }
        let identity = hash_parts(&[
            "worth_query_derived_publication_v1".into(),
            format!("execution:{}", self.receipt.identity()),
            format!("query:{}", canonical.query().digest().as_str()),
            format!(
                "result_shape:{}",
                canonical.result_shape().digest().as_str()
            ),
        ]);
        let receipt = WorthQueryDerivedPublicationReceipt {
            identity,
            execution_identity: self.receipt.identity().into(),
        };
        let phase_proof = mint_operation_phase_proof(
            receipt.identity().to_string(),
            Some(self.phase_proof.payload().identity()),
            operation_phase_basis(&self.phase_proof).clone(),
        );
        TransitionOutcome::Success(WorthQueryPublishedDomainOperation::mint(
            self,
            receipt,
            phase_proof,
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublicationDenial {
    StaleInstallationGeneration,
    ExecutionMaterialMismatch,
}
