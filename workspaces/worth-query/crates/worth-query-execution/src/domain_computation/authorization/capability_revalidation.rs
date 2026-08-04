//! Current capability re-admission at privileged transitions.

use worth_query_installation::facade::ApplicationSchema;

use super::delegation_admission::{observe_capability, observe_elevation_upper_bound};
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub(in crate::domain_computation) fn refresh_capability_authorization_for_graph_work(
        &self,
        authorization: &mut WorthQueryRetainedCapabilityAuthorization,
        graph_work: &crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        self.refresh_capability_authorization_for_session(
            authorization,
            graph_work.identity(),
            graph_work.branch().relational(),
        )
    }

    pub(in crate::domain_computation) fn refresh_capability_authorization_for_session(
        &self,
        authorization: &mut WorthQueryRetainedCapabilityAuthorization,
        session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
        branch: &worth_relational::facade::history::BranchId,
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
        let sample = self.sample_capability_time(installed)?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                installed.contract.name(),
            )
        })?;
        let observed = graph.integration_handle().with_runtime_mut(|runtime| {
            let Some(snapshot) = runtime.snapshots().snapshot_for_branch(branch) else {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    installed.contract.name(),
                ));
            };
            let result = if snapshot.branch_id != *branch {
                Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    installed.contract.name(),
                ))
            } else if !authorization
                .principal()
                .remains_current_in(runtime, &snapshot)
            {
                Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    installed.contract.name(),
                ))
            } else if !authorization.decision().remains_current_in(
                runtime,
                &snapshot,
                self.authorization.bridge(),
            ) {
                Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                    installed.contract.name(),
                ))
            } else {
                observe_capability(
                    session_identity,
                    runtime,
                    snapshot.clone(),
                    self.authorization.bridge(),
                    installed,
                    request,
                    &sample,
                    Some(authorization.grant()),
                    Some(authorization.decision()),
                )
            };
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        let (fact, grant) = observed.into_parts();
        let supporting_sample = sample.clone();
        authorization
            .replace_current_session_decision(
                session_identity,
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
        self.refresh_supporting_authorization(
            authorization,
            session_identity,
            branch,
            &supporting_sample,
        )
    }

    fn refresh_supporting_authorization(
        &self,
        authorization: &mut WorthQueryRetainedCapabilityAuthorization,
        session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
        branch: &worth_relational::facade::history::BranchId,
        primary_sample: &super::WorthQueryAuthorizationTimeSample,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        let Some(supporting) = authorization.supporting_mut() else {
            return Ok(());
        };
        let installed = self.installed_capability_plan(supporting.request())?;
        if supporting.capability_authority_identity()
            != installed.capability_authority_identity.as_ref()
        {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                installed.contract.name(),
            ));
        }
        let sample = if installed.request.timeline == primary_sample.timeline() {
            primary_sample.clone()
        } else {
            self.sample_capability_time(installed)?
        };
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                installed.contract.name(),
            )
        })?;
        let observed = graph.integration_handle().with_runtime_mut(|runtime| {
            let Some(snapshot) = runtime.snapshots().snapshot_for_branch(branch) else {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    installed.contract.name(),
                ));
            };
            let result = observe_elevation_upper_bound(
                session_identity,
                runtime,
                snapshot.clone(),
                self.authorization.bridge(),
                installed,
                supporting.request(),
                &sample,
                supporting.grant(),
                Some(supporting.decision()),
            );
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        let (decision, grant) = observed.into_parts();
        if grant != supporting.grant() {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                installed.contract.name(),
            ));
        }
        supporting.replace_current(sample, decision).map_err(|()| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                installed.contract.name(),
            )
        })
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

    pub(super) fn sample_capability_time(
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
