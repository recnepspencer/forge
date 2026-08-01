//! Re-admission of retained authorization at privileged transitions.

use worth_query_installation::facade::ApplicationSchema;

use super::capability_observation::observe_capability;
use super::decision_facts::WorthQueryObservedCommitBasis;
use super::{
    WorthQueryApplicationCommitAuthorization, WorthQueryCapabilityCommitBasis,
    WorthQueryCommitAuthorizationBasis, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryOperationGraphWorkSession,
    WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryApplicationCommitSerialization,
    WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub(in crate::domain_computation) fn readmit_retained_authorization(
        &self,
        authorization: &mut WorthQueryRetainedAuthorizationDecisionFacts,
        session: &WorthQueryOperationGraphWorkSession,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        validate_retained_affinity(authorization, session)?;
        current_basis::with_current_authorization_basis(self, session, |runtime, snapshot| {
            if let Some(capability) = authorization.capability_authorization_mut() {
                self.refresh_capability_authorization_in_operation_session(
                    capability, session, runtime, snapshot,
                )
            } else {
                validate_retained_currentness(
                    authorization,
                    runtime,
                    snapshot,
                    self.authorization.bridge(),
                )
            }
        })
    }

    pub(in crate::domain_computation) fn authorize_retained_idempotency<
        'serialization,
        'admission,
        Operation,
        Input,
        Scope,
    >(
        &self,
        admission: &'admission mut WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            Scope,
        >,
        session: &WorthQueryOperationGraphWorkSession,
        serialization: &'serialization WorthQueryApplicationCommitSerialization<'_>,
    ) -> Result<
        WorthQueryApplicationCommitAuthorization<
            'serialization,
            'admission,
            Schema,
            Operation,
            Input,
            Scope,
        >,
        WorthQueryOperationAuthorizationDenial,
    > {
        let operation = admission.operation().to_owned();
        let Some(authorization) = admission.authorization_mut() else {
            return Err(WorthQueryOperationAuthorizationDenial::inconsistent(
                operation,
            ));
        };
        self.readmit_retained_authorization(authorization, session)?;
        Ok(WorthQueryApplicationCommitAuthorization::mint(
            serialization,
            admission,
        ))
    }

    pub(in crate::domain_computation) fn authorize_idempotency_inspection<
        'serialization,
        'admission,
        Operation,
        Input,
        Scope,
    >(
        &self,
        admission: &'admission WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            Scope,
        >,
        serialization: &'serialization WorthQueryApplicationCommitSerialization<'_>,
    ) -> Result<
        WorthQueryApplicationCommitAuthorization<
            'serialization,
            'admission,
            Schema,
            Operation,
            Input,
            Scope,
        >,
        WorthQueryOperationAuthorizationDenial,
    > {
        let authorization = admission.authorization().ok_or_else(|| {
            WorthQueryOperationAuthorizationDenial::inconsistent(admission.operation())
        })?;
        self.validate_retained_authorization(authorization, admission.graph_work_session())?;
        Ok(WorthQueryApplicationCommitAuthorization::mint(
            serialization,
            admission,
        ))
    }

    pub(in crate::domain_computation) fn authorize_application_commit<
        'serialization,
        'admission,
        Operation,
        Input,
        Scope,
    >(
        &self,
        admission: &'admission WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            Scope,
        >,
        session: &WorthQueryOperationGraphWorkSession,
        basis: &WorthQueryCommitAuthorizationBasis,
        serialization: &'serialization WorthQueryApplicationCommitSerialization<'_>,
    ) -> Result<
        WorthQueryApplicationCommitAuthorization<
            'serialization,
            'admission,
            Schema,
            Operation,
            Input,
            Scope,
        >,
        WorthQueryOperationAuthorizationDenial,
    > {
        if basis.admission_identity() != admission.admission_identity()
            || !basis.belongs_to_session(session.identity())
            || !basis.belongs_to_branch(session.branch_affinity().relational_branch())
        {
            return Err(WorthQueryOperationAuthorizationDenial::inconsistent(
                admission.operation(),
            ));
        }
        self.readmit_commit_basis(basis, session)?;
        Ok(WorthQueryApplicationCommitAuthorization::mint(
            serialization,
            admission,
        ))
    }

    fn readmit_commit_basis(
        &self,
        basis: &WorthQueryCommitAuthorizationBasis,
        session: &WorthQueryOperationGraphWorkSession,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        current_basis::with_current_authorization_basis(self, session, |runtime, snapshot| {
            match basis {
                WorthQueryCommitAuthorizationBasis::Observed {
                    authorization: observed,
                    ..
                } => self.readmit_observed_commit_basis(observed, runtime, snapshot),
                WorthQueryCommitAuthorizationBasis::Capability {
                    authorization: capability,
                    ..
                } => self.readmit_capability_commit_basis(capability, session, runtime, snapshot),
            }
        })
    }

    fn readmit_observed_commit_basis(
        &self,
        observed: &WorthQueryObservedCommitBasis,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        validate_observed_currentness(observed, runtime, snapshot, self.authorization.bridge())
    }

    fn readmit_capability_commit_basis(
        &self,
        capability: &WorthQueryCapabilityCommitBasis,
        session: &WorthQueryOperationGraphWorkSession,
        runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        let installed = self.installed_capability_plan(capability.request())?;
        if capability.capability_authority_identity()
            != installed.capability_authority_identity.as_ref()
        {
            return Err(stale_authorization());
        }
        let sample = self.sample_capability_time(installed)?;
        if !capability.principal().remains_current_in(runtime, snapshot) {
            return Err(stale_principal());
        }
        if !capability
            .decision()
            .remains_current_in(runtime, snapshot, self.authorization.bridge())
        {
            return Err(stale_authorization());
        }
        observe_capability(
            *session.identity(),
            runtime,
            snapshot.clone(),
            self.authorization.bridge(),
            installed,
            capability.request(),
            &sample,
            Some(capability.grant()),
        )
        .map(drop)
    }

    fn validate_retained_authorization(
        &self,
        authorization: &WorthQueryRetainedAuthorizationDecisionFacts,
        session: &WorthQueryOperationGraphWorkSession,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        validate_retained_affinity(authorization, session)?;
        current_basis::with_current_authorization_basis(self, session, |runtime, snapshot| {
            let Some(capability) = authorization.capability_authorization() else {
                return validate_retained_currentness(
                    authorization,
                    runtime,
                    snapshot,
                    self.authorization.bridge(),
                );
            };
            let installed = self.installed_capability_plan(capability.request())?;
            if capability.capability_authority_identity()
                != installed.capability_authority_identity.as_ref()
            {
                return Err(stale_authorization());
            }
            let sample = self.sample_capability_time(installed)?;
            if !capability.principal().remains_current_in(runtime, snapshot) {
                return Err(stale_principal());
            }
            if !capability.decision().remains_current_in(
                runtime,
                snapshot,
                self.authorization.bridge(),
            ) {
                return Err(stale_authorization());
            }
            observe_capability(
                *session.identity(),
                runtime,
                snapshot.clone(),
                self.authorization.bridge(),
                installed,
                capability.request(),
                &sample,
                Some(capability.grant()),
            )
            .map(drop)
        })
    }
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
    authorization
        .validate_currentness_in(runtime, snapshot, bridge)
        .map_err(|kind| {
            WorthQueryOperationAuthorizationDenial::new(kind, "application-authorization")
        })
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

fn validate_retained_affinity(
    authorization: &WorthQueryRetainedAuthorizationDecisionFacts,
    session: &WorthQueryOperationGraphWorkSession,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    if authorization.belongs_to_session(session.identity())
        && authorization.belongs_to_branch(session.branch_affinity().relational_branch())
    {
        Ok(())
    } else {
        Err(WorthQueryOperationAuthorizationDenial::inconsistent(
            "application-authorization",
        ))
    }
}

#[path = "authorization_revalidation/current_basis.rs"]
mod current_basis;
