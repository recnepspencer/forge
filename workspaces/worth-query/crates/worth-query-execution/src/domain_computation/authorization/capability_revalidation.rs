//! Current capability re-admission at privileged transitions.

use worth_query_installation::facade::ApplicationSchema;

use super::capability_observation::observe_capability;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryOperationGraphWorkSession, WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub(in crate::domain_computation) fn readmit_capability_authorization_in_session(
        &self,
        authorization: WorthQueryRetainedCapabilityAuthorization,
        session_identity: worth_foundational::facade::CanonicalDigestId,
        branch_id: worth_relational::facade::history::BranchId,
        snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    ) -> Result<WorthQueryRetainedCapabilityAuthorization, WorthQueryOperationAuthorizationDenial>
    {
        if snapshot.branch_id != branch_id {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                "capability-session-branch",
            ));
        }
        let request = authorization.request();
        let installed = self.installed_capability_plan(request)?;
        let sample = self.sample_capability_time(installed)?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                installed.contract.name(),
            )
        })?;
        let observed = graph.integration_handle().with_runtime_mut(|runtime| {
            if !authorization
                .principal()
                .remains_current_in(runtime, &snapshot)
            {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    installed.contract.name(),
                ));
            }
            if !authorization.decision().remains_current_in(
                runtime,
                &snapshot,
                self.authorization.bridge(),
            ) {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                    installed.contract.name(),
                ));
            }
            observe_capability(
                session_identity,
                runtime,
                snapshot,
                self.authorization.bridge(),
                installed,
                authorization.request(),
                &sample,
                Some(authorization.grant()),
            )
        })?;
        let (decision, grant) = observed.into_parts();
        if grant != authorization.grant() {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                installed.contract.name(),
            ));
        }
        authorization
            .into_rebound_session(session_identity, branch_id, sample, decision)
            .map_err(|()| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    installed.contract.name(),
                )
            })
    }

    pub(in crate::domain_computation) fn refresh_capability_authorization_in_operation_session(
        &self,
        authorization: &mut WorthQueryRetainedCapabilityAuthorization,
        session: &WorthQueryOperationGraphWorkSession,
        runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        let request = authorization.request();
        let installed = self.installed_capability_plan(request)?;
        if authorization.capability_authority_identity()
            != installed.capability_authority_identity.as_ref()
        {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                installed.contract.name(),
            ));
        }
        let branch = session.branch_affinity().relational_branch();
        if snapshot.branch_id != *branch
            || !authorization.belongs_to_session(session.identity())
            || !authorization.belongs_to_branch(branch)
        {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                installed.contract.name(),
            ));
        }
        let sample = self.sample_capability_time(installed)?;
        if !authorization
            .principal()
            .remains_current_in(runtime, snapshot)
        {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                installed.contract.name(),
            ));
        }
        if !authorization.decision().remains_current_in(
            runtime,
            snapshot,
            self.authorization.bridge(),
        ) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                installed.contract.name(),
            ));
        }
        let observed = observe_capability(
            *session.identity(),
            runtime,
            snapshot.clone(),
            self.authorization.bridge(),
            installed,
            request,
            &sample,
            Some(authorization.grant()),
        )?;
        let (fact, grant) = observed.into_parts();
        authorization
            .replace_current_decision(
                installed.capability_authority_identity.as_ref(),
                grant,
                sample,
                fact,
            )
            .map_err(|()| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    installed.contract.name(),
                )
            })?;
        Ok(())
    }

    pub(super) fn installed_capability_plan(
        &self,
        request: &WorthQueryRetainedCapabilityRequest,
    ) -> Result<
        &super::capability_registry::WorthQueryInstalledCapabilityPlan,
        WorthQueryOperationAuthorizationDenial,
    > {
        self.authorization
            .capability_plan_by_identity(&request.capability_identity)
            .ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                    "retained-capability-request",
                )
            })
    }

    pub(in crate::domain_computation) fn sample_capability_time(
        &self,
        installed: &super::capability_registry::WorthQueryInstalledCapabilityPlan,
    ) -> Result<
        crate::domain_computation::authorization::WorthQueryAuthorizationTimeSample,
        WorthQueryOperationAuthorizationDenial,
    > {
        self.authorization_clock
            .sample(installed.request.timeline)
            .map_err(|_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
                    installed.contract.name(),
                )
            })
    }
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
