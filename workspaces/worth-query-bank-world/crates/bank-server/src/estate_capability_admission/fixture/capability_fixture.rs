use std::time::{SystemTime, UNIX_EPOCH};

use bank_domain::estate::{
    BankEstateOracles, BankEstateWorld, EstateAction, EstateActorContext, EstateCapabilityUse,
    EstateDecision, EstateMoment, EstateWorkflowStage,
};
use worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity;

use super::{
    block_on, request_scope, TestAuthenticationAdapter, TestCredential, ASSIGNMENT, GRANT,
    SPECIALIST,
};
use crate::{BankAuthenticatedPrincipal, BankAuthenticationBoundary, BankIdentityRuntime};

pub(crate) struct CapabilityFixture {
    pub(crate) runtime: BankIdentityRuntime,
    pub(super) estate_world: BankEstateWorld,
    pub(super) workflow_stage: EstateWorkflowStage,
    pub(super) authentication: BankAuthenticationBoundary<TestAuthenticationAdapter>,
    pub(super) specialist_identity: WorthQueryExternalPrincipalIdentity,
    pub(super) executor_identity: WorthQueryExternalPrincipalIdentity,
    pub(super) approver_identity: WorthQueryExternalPrincipalIdentity,
    pub(super) reviewer_identity: WorthQueryExternalPrincipalIdentity,
}

impl CapabilityFixture {
    pub(crate) fn authenticate(&self) -> BankAuthenticatedPrincipal {
        self.authenticate_identity(self.specialist_identity.clone())
    }

    pub(crate) fn authenticate_approver(&self) -> BankAuthenticatedPrincipal {
        self.authenticate_identity(self.approver_identity.clone())
    }

    pub(crate) fn authenticate_executor(&self) -> BankAuthenticatedPrincipal {
        self.authenticate_identity(self.executor_identity.clone())
    }

    pub(in crate::estate_capability_admission) fn authenticate_reviewer(
        &self,
    ) -> BankAuthenticatedPrincipal {
        self.authenticate_identity(self.reviewer_identity.clone())
    }

    fn authenticate_identity(
        &self,
        identity: WorthQueryExternalPrincipalIdentity,
    ) -> BankAuthenticatedPrincipal {
        let request = request_scope();
        block_on(self.runtime.authenticate_with(
            &self.authentication,
            TestCredential(identity),
            &request,
        ))
        .expect("the mapped employee should authenticate")
    }

    pub(in crate::estate_capability_admission) fn oracle_decision(
        &self,
        action: EstateAction,
    ) -> EstateDecision {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("the test clock is after the Unix epoch")
            .as_secs();
        BankEstateOracles::evaluate(
            &self.estate_world,
            EstateActorContext {
                principal: SPECIALIST,
                assignment: ASSIGNMENT,
            },
            action,
            EstateCapabilityUse {
                grant: GRANT,
                workflow_stage: self.workflow_stage,
                now: EstateMoment::from_epoch_seconds(now),
                emergency_access: None,
            },
        )
    }
}
