//! Current capability re-admission at privileged transitions.

use worth_query_installation::facade::ApplicationSchema;

use super::capability_observation::observe_capability;
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
    pub(in crate::domain_computation) fn refresh_capability_authorization(
        &self,
        authorization: &mut WorthQueryRetainedCapabilityAuthorization,
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
            let snapshot = runtime.snapshots().snapshot();
            let principal_current = authorization
                .principal()
                .remains_current_in(runtime, &snapshot);
            let decision_current = authorization.decision().remains_current_in(
                runtime,
                &snapshot,
                self.authorization.bridge(),
            );
            let result = if !principal_current {
                Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    installed.contract.name(),
                ))
            } else if !decision_current {
                Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                    installed.contract.name(),
                ))
            } else {
                observe_capability(
                    runtime,
                    snapshot.clone(),
                    self.authorization.bridge(),
                    installed,
                    request,
                    &sample,
                    Some(authorization.grant()),
                )
            };
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
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
