//! Current capability re-admission at privileged transitions.

use worth_query_installation::facade::ApplicationSchema;

use super::capability_currentness::WorthQueryCapabilityCurrentnessAuthority;
use super::capability_observation::observe_capability;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub(in crate::domain_computation) fn refresh_capability_authorization(
        &self,
        request: &WorthQueryRetainedCapabilityRequest,
        currentness: &mut WorthQueryCapabilityCurrentnessAuthority,
        authorization: &mut WorthQueryRetainedAuthorizationDecisionFacts,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        let installed = self.installed_capability_plan(request)?;
        if currentness.capability_authority_identity()
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
        let fact = graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let result = if authorization.principal_remains_current_in(runtime, &snapshot) {
                observe_capability(
                    runtime,
                    snapshot.clone(),
                    self.authorization.bridge(),
                    installed,
                    request,
                    &sample,
                )
            } else {
                Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    installed.contract.name(),
                ))
            };
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        authorization.replace_single_policy(fact).map_err(|()| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                installed.contract.name(),
            )
        })?;
        if !currentness.replace_sample(sample.timeline(), sample.value().clone()) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                installed.contract.name(),
            ));
        }
        Ok(())
    }

    pub(in crate::domain_computation) fn validate_capability_at_current_time(
        &self,
        request: &WorthQueryRetainedCapabilityRequest,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        let installed = self.installed_capability_plan(request)?;
        let sample = self.sample_capability_time(installed)?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                installed.contract.name(),
            )
        })?;
        graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let result = observe_capability(
                runtime,
                snapshot.clone(),
                self.authorization.bridge(),
                installed,
                request,
                &sample,
            )
            .map(drop);
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })
    }

    fn installed_capability_plan(
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

    fn sample_capability_time(
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
