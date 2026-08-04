use worth_query_admission::facade::{
    application_query::WorthQueryAdmittedApplicationQueryParameters,
    graph_obligation::WorthQueryGraphWorkIntent,
};
use worth_query_admission::integration::{
    admit_application_query_graph_work, derive_graph_read_access_requirements_for_contract,
    review_application_query_graph_work, select_installed_graph_obligations,
};
use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest,
    application_query::ApplicationQueryParameterSet, application_schema::ApplicationSchema,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkPhases;
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use super::{
    admission_preparation::validate_admission_request,
    basis::admit_application_query_basis,
    disclosure::{
        admit_application_query_governance, compile_disclosure_contract,
        WorthQueryApplicationGovernanceBinding, WorthQueryPendingApplicationQueryGovernance,
    },
    execution_shape::validate_one_shot_shape,
    runtime_support::primary_graph_support_inventory,
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
    WorthQueryApplicationQueryControls,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::provider_session::{
    WorthQueryGraphWorkAccessContextAffinity, WorthQueryManagedGraphWorkSession,
};

mod denial;
mod governed_access;
mod live_readmission;
mod work_limit;
use denial::{denial, graph_work_denial};
use governed_access::governance_denial;
pub(in crate::domain_computation::primary_graph::application_query) use governed_access::prepare_governed_access;
use work_limit::{application_query_graph_read_budget, validate_work_limit};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn admit_application_query<
        'a,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        access: &WorthQueryApplicationQueryAccessContext<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
    ) -> Result<
        WorthQueryAdmittedApplicationQueryPlan<
            'a,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        let (parameters, controls) =
            self.prepare_application_query_admission(query, access, parameters, controls)?;
        self.finish_application_query_admission(query, access, parameters, controls, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn admit_governed_application_query<
        'a,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Capability,
        Operation,
        Input,
    >(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        access: &WorthQueryApplicationQueryAccessContext<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        capability: crate::domain_computation::authorization::WorthQueryAdmittedApplicationCapabilityAccess<
            Schema,
            Capability,
            Operation,
            Input,
        >,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
    ) -> Result<
        WorthQueryAdmittedApplicationQueryPlan<
            'a,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        WorthQueryApplicationQueryAdmissionDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability, Scope = Scope>,
    {
        let pending = prepare_governed_access(self, query, access, capability, &controls)?;
        let (parameters, controls) =
            self.prepare_application_query_admission(query, access, parameters, controls)?;
        self.finish_application_query_admission(query, access, parameters, controls, Some(pending))
    }

    pub(super) fn finish_application_query_admission<
        'a,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        access: &WorthQueryApplicationQueryAccessContext<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        parameters: WorthQueryAdmittedApplicationQueryParameters,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
        pending_governance: Option<WorthQueryPendingApplicationQueryGovernance>,
    ) -> Result<
        WorthQueryAdmittedApplicationQueryPlan<
            'a,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        let requirements = derive_graph_read_access_requirements_for_contract(
            query.read_family_binding().planning_contract(),
            controls.lane(),
            controls.maximum_result_count().get(),
            parameters.identity(),
            query.canonical_work_policy().admission_planning(),
        )
        .map_err(|_| {
            denial(
                WorthQueryApplicationQueryAdmissionDenialKind::CanonicalWorkDenied,
                query.name(),
            )
        })?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryApplicationQueryAdmissionDenialKind::StaleScope,
                query.name(),
            )
        })?;
        let inventory = primary_graph_support_inventory(
            &graph.layout,
            query.continuation(),
            query.live(),
            &requirements,
        );
        let disclosure = compile_disclosure_contract(query, &graph.layout).map_err(|denial| {
            WorthQueryApplicationQueryAdmissionDenial::new(
                WorthQueryApplicationQueryAdmissionDenialKind::DisclosureContractInvalid,
                denial.subject(),
            )
        })?;
        let obligations = query.retain_graph_obligations_for_admission();
        let obligation_identity = obligations.identity().clone();
        let selected = select_installed_graph_obligations(
            obligations,
            WorthQueryGraphWorkIntent::application_query_read(),
        )
        .map_err(|_| graph_work_denial(query.name()))?;
        let reviewed = review_application_query_graph_work(
            selected,
            requirements,
            inventory,
            application_query_graph_read_budget(
                self.runtime.application_query_resource_profile(),
                &controls,
            ),
        )
        .map_err(|_| graph_work_denial(query.name()))?;
        let canonical_work = WorthQueryCanonicalWorkPhases::new(
            query.installation_canonical_work(),
            parameters
                .canonical_basis()
                .work()
                .combine(reviewed.review().requirements().canonical_work()),
        );
        validate_work_limit(reviewed.review(), &controls, query.name())?;
        if let Some(denial) = reviewed.review().denial() {
            return Err(WorthQueryApplicationQueryAdmissionDenial::new(
                WorthQueryApplicationQueryAdmissionDenialKind::GraphReadPlan(denial.kind()),
                query.name(),
            ));
        }
        let admitted_graph_work =
            admit_application_query_graph_work(reviewed, &self.graph_work_resource_support())
                .map_err(|_| graph_work_denial(query.name()))?;
        validate_one_shot_shape(query)?;
        let continuation_index_id = if controls.lane()
            == worth_query_admission::facade::application_query::WorthQueryApplicationQueryLane::Continuation
        {
            query
                .continuation()
                .and_then(|contract| graph.layout.continuation_ordering_index_id(contract))
                .ok_or_else(|| {
                    denial(
                        WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
                        query.name(),
                    )
                })
                .map(Some)?
        } else {
            None
        };
        validate_admission_request(controls.request_scope(), query.name())?;
        let (basis_selection, controls) = controls.into_admission_parts();
        let basis = admit_application_query_basis(self, basis_selection)?;
        validate_admission_request(controls.request_scope(), query.name())?;
        let capability_identity = pending_governance
            .as_ref()
            .map(WorthQueryPendingApplicationQueryGovernance::installed_capability_identity);
        let mut graph_work = WorthQueryManagedGraphWorkSession::start_query(
            admitted_graph_work,
            self.runtime.authority_identity(),
            query.binding_identity(),
            &obligation_identity,
            query.authority_identity(),
            access.principal().principal_entity_id(),
            capability_identity.map_or_else(
                || WorthQueryGraphWorkAccessContextAffinity::entity(access.scope().entity_id()),
                |identity| {
                    WorthQueryGraphWorkAccessContextAffinity::governed_entity(
                        access.scope().entity_id(),
                        identity,
                    )
                },
            ),
            basis.identity(),
            self.graph_work_provider_identity(),
            graph.query_session_port(),
        )
        .map_err(|_| graph_work_denial(query.name()))?;
        let mut pending_governance = pending_governance;
        if let Some(pending) = pending_governance.as_mut() {
            self.refresh_capability_authorization_for_graph_work(
                pending.authorization_mut(),
                &graph_work,
            )
            .map_err(|denial| {
                WorthQueryApplicationQueryAdmissionDenial::new(
                    WorthQueryApplicationQueryAdmissionDenialKind::Authorization(denial.kind()),
                    denial.subject(),
                )
            })?;
        }
        let governance = admit_application_query_governance(
            disclosure,
            pending_governance,
            WorthQueryApplicationGovernanceBinding::from_session(
                &graph_work,
                query.identity().clone(),
                *parameters.identity(),
                access.principal().principal_entity_id(),
                access.scope().entity_id(),
            ),
        )
        .map_err(|kind| governance_denial(kind, query.name()))?;
        let (authorization, authorization_work) =
            self.observe_application_query_access(&mut graph_work, query, access)?;
        if !authorization.belongs_to_session(graph_work.identity()) {
            return Err(graph_work_denial(query.name()));
        }
        if !governance.authorization_belongs_to_session(graph_work.identity()) {
            return Err(graph_work_denial(query.name()));
        }
        let retained_fact_count = authorization.exact_fact_count()
            + governance
                .authorization()
                .map_or(0, |capability| capability.exact_fact_count());
        graph_work.set_retained_decision_facts(retained_fact_count);
        Ok(WorthQueryAdmittedApplicationQueryPlan {
            runtime_authority: self.runtime.authority_identity(),
            graph_authority_identity: self
                .primary_graph_authority
                .authority_identity()
                .to_string(),
            provider_identity: self.primary_graph_authority.provider_identity().to_string(),
            query,
            principal: access.principal(),
            scope: access.scope(),
            parameters,
            controls,
            canonical_work,
            continuation_index_id,
            continuation_state: None,
            basis,
            graph_work,
            authorization,
            authorization_work,
            governance,
        })
    }
}
