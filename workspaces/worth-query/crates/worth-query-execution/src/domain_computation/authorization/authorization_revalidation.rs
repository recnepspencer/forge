//! Re-admission of retained authorization at privileged transitions.

use worth_query_installation::facade::ApplicationSchema;

use super::capability_observation::observe_capability;
use super::decision_facts::WorthQueryObservedCommitBasis;
use super::{
    WorthQueryApplicationCommitAuthorization, WorthQueryCommitAuthorizationBasis,
    WorthQueryOperationAdmissionIdentity, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitSerialization, WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub(in crate::domain_computation) fn readmit_retained_authorization(
        &self,
        authorization: &mut WorthQueryRetainedAuthorizationDecisionFacts,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        if let Some(capability) = authorization.capability_authorization_mut() {
            return self.refresh_capability_authorization(capability);
        }
        let graph = self.runtime.primary_graph().ok_or_else(foreign_runtime)?;
        graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let current = validate_retained_currentness(
                authorization,
                runtime,
                &snapshot,
                self.authorization.bridge(),
            );
            runtime.snapshots().release_snapshot(&snapshot);
            current
        })
    }

    pub(in crate::domain_computation) fn authorize_retained_idempotency<'serialization>(
        &self,
        authorization: &mut WorthQueryRetainedAuthorizationDecisionFacts,
        admission_identity: WorthQueryOperationAdmissionIdentity,
        serialization: &'serialization WorthQueryApplicationCommitSerialization<'_>,
    ) -> Result<
        WorthQueryApplicationCommitAuthorization<'serialization>,
        WorthQueryOperationAuthorizationDenial,
    > {
        self.readmit_retained_authorization(authorization)?;
        Ok(WorthQueryApplicationCommitAuthorization::mint(
            serialization,
            admission_identity,
        ))
    }

    pub(in crate::domain_computation) fn authorize_idempotency_inspection<'serialization>(
        &self,
        authorization: &WorthQueryRetainedAuthorizationDecisionFacts,
        admission_identity: WorthQueryOperationAdmissionIdentity,
        serialization: &'serialization WorthQueryApplicationCommitSerialization<'_>,
    ) -> Result<
        WorthQueryApplicationCommitAuthorization<'serialization>,
        WorthQueryOperationAuthorizationDenial,
    > {
        self.validate_retained_authorization(authorization)?;
        Ok(WorthQueryApplicationCommitAuthorization::mint(
            serialization,
            admission_identity,
        ))
    }

    pub(in crate::domain_computation) fn authorize_application_commit<'serialization>(
        &self,
        basis: &WorthQueryCommitAuthorizationBasis,
        admission_identity: WorthQueryOperationAdmissionIdentity,
        serialization: &'serialization WorthQueryApplicationCommitSerialization<'_>,
    ) -> Result<
        WorthQueryApplicationCommitAuthorization<'serialization>,
        WorthQueryOperationAuthorizationDenial,
    > {
        self.readmit_commit_basis(basis)?;
        Ok(WorthQueryApplicationCommitAuthorization::mint(
            serialization,
            admission_identity,
        ))
    }

    fn readmit_commit_basis(
        &self,
        basis: &WorthQueryCommitAuthorizationBasis,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        match basis {
            WorthQueryCommitAuthorizationBasis::Observed(observed) => {
                let graph = self.runtime.primary_graph().ok_or_else(foreign_runtime)?;
                graph.integration_handle().with_runtime_mut(|runtime| {
                    let snapshot = runtime.snapshots().snapshot();
                    let current = validate_observed_currentness(
                        observed,
                        runtime,
                        &snapshot,
                        self.authorization.bridge(),
                    );
                    runtime.snapshots().release_snapshot(&snapshot);
                    current
                })
            }
            WorthQueryCommitAuthorizationBasis::Capability(capability) => {
                let installed = self.installed_capability_plan(capability.request())?;
                if capability.capability_authority_identity()
                    != installed.capability_authority_identity.as_ref()
                {
                    return Err(stale_authorization());
                }
                let sample = self.sample_capability_time(installed)?;
                let graph = self.runtime.primary_graph().ok_or_else(foreign_runtime)?;
                graph.integration_handle().with_runtime_mut(|runtime| {
                    let snapshot = runtime.snapshots().snapshot();
                    let current = capability
                        .principal()
                        .remains_current_in(runtime, &snapshot);
                    let result = if current {
                        observe_capability(
                            runtime,
                            snapshot.clone(),
                            self.authorization.bridge(),
                            installed,
                            capability.request(),
                            &sample,
                            Some(capability.grant()),
                        )
                        .map(drop)
                    } else {
                        Err(stale_principal())
                    };
                    runtime.snapshots().release_snapshot(&snapshot);
                    result
                })
            }
        }
    }

    fn validate_retained_authorization(
        &self,
        authorization: &WorthQueryRetainedAuthorizationDecisionFacts,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        let Some(capability) = authorization.capability_authorization() else {
            let graph = self.runtime.primary_graph().ok_or_else(foreign_runtime)?;
            return graph.integration_handle().with_runtime_mut(|runtime| {
                let snapshot = runtime.snapshots().snapshot();
                let current = validate_retained_currentness(
                    authorization,
                    runtime,
                    &snapshot,
                    self.authorization.bridge(),
                );
                runtime.snapshots().release_snapshot(&snapshot);
                current
            });
        };
        let installed = self.installed_capability_plan(capability.request())?;
        if capability.capability_authority_identity()
            != installed.capability_authority_identity.as_ref()
        {
            return Err(stale_authorization());
        }
        let sample = self.sample_capability_time(installed)?;
        let graph = self.runtime.primary_graph().ok_or_else(foreign_runtime)?;
        graph.integration_handle().with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let principal_current = capability
                .principal()
                .remains_current_in(runtime, &snapshot);
            let decision_current = capability.decision().remains_current_in(
                runtime,
                &snapshot,
                self.authorization.bridge(),
            );
            let result = if !principal_current {
                Err(stale_principal())
            } else if !decision_current {
                Err(stale_authorization())
            } else {
                observe_capability(
                    runtime,
                    snapshot.clone(),
                    self.authorization.bridge(),
                    installed,
                    capability.request(),
                    &sample,
                    Some(capability.grant()),
                )
                .map(drop)
            };
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })
    }
}

fn foreign_runtime() -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
        "application-authorization",
    )
}

fn stale_authorization() -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
        "application-authorization",
    )
}

fn stale_principal() -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
        "application-authorization",
    )
}

fn validate_retained_currentness(
    authorization: &WorthQueryRetainedAuthorizationDecisionFacts,
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &worth_runtime_bridge::facade::BridgeAuthorizationRuntime,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    if !authorization.principal_remains_current_in(runtime, snapshot) {
        return Err(stale_principal());
    }
    authorization
        .decisions_remain_current_in(runtime, snapshot, bridge)
        .then_some(())
        .ok_or_else(stale_authorization)
}

fn validate_observed_currentness(
    authorization: &WorthQueryObservedCommitBasis,
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &worth_runtime_bridge::facade::BridgeAuthorizationRuntime,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    if !authorization.principal_remains_current_in(runtime, snapshot) {
        return Err(stale_principal());
    }
    authorization
        .decisions_remain_current_in(runtime, snapshot, bridge)
        .then_some(())
        .ok_or_else(stale_authorization)
}
