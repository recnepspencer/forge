use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest,
    application_query::{ApplicationQueryLiveCauseBinding, ApplicationQueryParameterSet},
    application_schema::ApplicationSchema,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use super::super::{
    controls::WorthQueryApplicationLiveControls,
    outcome::{WorthQueryApplicationLiveOpenDenial, WorthQueryApplicationLiveOpenDenialKind},
    scope_identity::read_scope_identity,
};
use super::validation::{
    open_admission_denial, open_denial, validate_live_binding, validate_live_resource_controls,
};
use super::WorthQueryApplicationLiveLease;
use crate::domain_computation::{
    managed_run::{
        admit_managed_lower_execution_basis, WorthQueryManagedLowerBinding,
        WorthQueryManagedTruthReadRequest,
    },
    primary_graph::{
        application_query::{
            admission::{prepare_governed_access, WorthQueryGovernanceAdmission},
            authorized_read::execute_authorized_read,
            WorthQueryApplicationProjection, WorthQueryApplicationQueryAccessContext,
            WorthQueryApplicationQueryControls,
        },
        live_delivery::WorthQueryLiveCauseQueue,
        WorthQueryApplicationEntityIdentity, WorthQueryAuthenticatedPrincipal,
        WorthQueryPrimaryGraphApplicationRuntime,
    },
};

static NEXT_APPLICATION_QUERY_LIVE_LEASE: AtomicU64 = AtomicU64::new(1);

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub fn open_application_query_live<
        'runtime,
        'principal,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    >(
        &'runtime self,
        query: WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
        principal: &'principal WorthQueryAuthenticatedPrincipal<
            Schema,
            Principal,
            PrincipalIdentity,
        >,
        scope: WorthQueryApplicationEntityIdentity<Schema, Scope>,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationLiveControls,
    ) -> Result<
        WorthQueryApplicationLiveLease<
            'runtime,
            'principal,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
            Target,
            Binding,
        >,
        WorthQueryApplicationLiveOpenDenial,
    >
    where
        QueryResult: WorthQueryApplicationProjection<Schema, Query>,
        Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
    {
        self.open_application_query_live_with_governance::<
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
            Target,
            Binding,
        >(query, principal, scope, parameters, controls, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn open_governed_application_query_live<
        'runtime,
        'principal,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
        Capability,
        Operation,
        Input,
    >(
        &'runtime self,
        query: WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
        principal: &'principal WorthQueryAuthenticatedPrincipal<
            Schema,
            Principal,
            PrincipalIdentity,
        >,
        scope: WorthQueryApplicationEntityIdentity<Schema, Scope>,
        capability: crate::domain_computation::authorization::WorthQueryPreparedApplicationCapabilityAccess<
            Schema,
            Capability,
            Operation,
            Input,
        >,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationLiveControls,
    ) -> Result<
        WorthQueryApplicationLiveLease<
            'runtime,
            'principal,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
            Target,
            Binding,
        >,
        WorthQueryApplicationLiveOpenDenial,
    >
    where
        QueryResult: WorthQueryApplicationProjection<Schema, Query>,
        Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
        Capability: 'static,
        Operation: 'static,
        Input: ApplicationCapabilityRequest<Schema, Capability, Scope = Scope> + 'static,
    {
        let access = WorthQueryApplicationQueryAccessContext::new(principal, &scope);
        let query_controls = WorthQueryApplicationQueryControls::current_live(
            controls.maximum_materialized_record_count(),
            controls.maximum_work_per_delivery(),
            controls.request(),
        );
        let pending = prepare_governed_access(self, &query, &access, capability, &query_controls)
            .map_err(open_admission_denial)?;
        self.open_application_query_live_with_governance::<
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
            Target,
            Binding,
        >(
            query,
            principal,
            scope,
            parameters,
            controls,
            Some(pending),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn open_application_query_live_with_governance<
        'runtime,
        'principal,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    >(
        &'runtime self,
        query: WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
        principal: &'principal WorthQueryAuthenticatedPrincipal<
            Schema,
            Principal,
            PrincipalIdentity,
        >,
        scope: WorthQueryApplicationEntityIdentity<Schema, Scope>,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationLiveControls,
        pending_governance: Option<
            WorthQueryGovernanceAdmission<Schema, Principal, PrincipalIdentity, Scope>,
        >,
    ) -> Result<
        WorthQueryApplicationLiveLease<
            'runtime,
            'principal,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
            Target,
            Binding,
        >,
        WorthQueryApplicationLiveOpenDenial,
    >
    where
        QueryResult: WorthQueryApplicationProjection<Schema, Query>,
        Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
    {
        let live = validate_live_binding::<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
            Target,
            Binding,
        >(&query)?;
        validate_live_resource_controls(live, &controls, query.name())?;
        let access = WorthQueryApplicationQueryAccessContext::new(principal, &scope);
        let query_controls = WorthQueryApplicationQueryControls::current_live(
            controls.maximum_materialized_record_count(),
            controls.maximum_work_per_delivery(),
            controls.request(),
        );
        let (admitted_parameters, query_controls) = self
            .prepare_application_query_admission(&query, parameters.clone(), query_controls)
            .map_err(open_admission_denial)?;
        let mut plan = self
            .finish_application_query_admission(
                &query,
                &access,
                admitted_parameters,
                query_controls,
                pending_governance,
            )
            .map_err(open_admission_denial)?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            open_denial(
                WorthQueryApplicationLiveOpenDenialKind::ScopeIdentityUnavailable,
                query.name(),
            )
        })?;
        let (scope_identity, _) =
            execute_authorized_read(self, graph, &mut plan, read_scope_identity).map_err(|_| {
                open_denial(
                    WorthQueryApplicationLiveOpenDenialKind::ScopeIdentityUnavailable,
                    query.name(),
                )
            })?;
        let governance = plan.take_governance();
        let release = plan
            .complete_graph_read()
            .map_err(|_| {
                open_denial(
                    WorthQueryApplicationLiveOpenDenialKind::BasisReleaseFailed,
                    query.name(),
                )
            })?
            .release;
        if !release.basis_released() {
            return Err(open_denial(
                WorthQueryApplicationLiveOpenDenialKind::BasisReleaseFailed,
                query.name(),
            ));
        }
        let version = self
            .primary_provider
            .graph
            .with_runtime(|runtime| {
                runtime
                    .history()
                    .branch_head(self.branch_affinity.relational_branch())
                    .map(|head| head.version_id)
            })
            .ok_or_else(|| {
                open_denial(
                    WorthQueryApplicationLiveOpenDenialKind::ProviderVersionUnavailable,
                    query.name(),
                )
            })?;
        let attempt = NEXT_APPLICATION_QUERY_LIVE_LEASE.fetch_add(1, Ordering::Relaxed);
        let attempt_identity = format!("application-query-live:{attempt}");
        let binding = WorthQueryManagedLowerBinding::new(
            query.name(),
            &attempt_identity,
            live.resource_envelope(),
        );
        let request = WorthQueryManagedTruthReadRequest::new(
            version,
            self.branch_affinity.truth_branch().clone(),
            worth_runtime_bridge::facade::SnapshotReadPacket::new(Vec::new()),
        );
        let request_bridge = self.bridge.fork_managed_request_lane();
        let basis = admit_managed_lower_execution_basis(
            &request_bridge,
            &self.relational_source,
            binding,
            request,
        )
        .map_err(|failure| {
            open_denial(
                WorthQueryApplicationLiveOpenDenialKind::BridgeBasisRejected,
                failure.detail.as_ref(),
            )
        })?;
        Ok(WorthQueryApplicationLiveLease {
            runtime: self,
            query,
            principal,
            scope,
            parameters,
            controls,
            governance,
            scope_identity,
            basis: Some(basis),
            queue: WorthQueryLiveCauseQueue::open(&self.primary_provider.live_delivery),
            _target: PhantomData,
            _thread_affinity: PhantomData,
        })
    }
}
