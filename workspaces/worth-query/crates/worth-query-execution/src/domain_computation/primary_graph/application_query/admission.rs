use worth_query_admission::facade::{
    application_query::{
        derive_graph_read_access_requirements_for_contract,
        WorthQueryAdmittedApplicationQueryParameters,
    },
    graph_read_access::review_graph_read_access,
};
use worth_query_declaration::facade::{
    application_query::ApplicationQueryParameterSet, application_schema::ApplicationSchema,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkPhases;
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use crate::domain_computation::execution_runtime::WorthQueryApplicationQueryResourceProfile;

use super::{
    admission_preparation::validate_admission_request, basis::admit_application_query_basis,
    execution_shape::validate_one_shot_shape, runtime_support::primary_graph_support_inventory,
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
    WorthQueryApplicationQueryControls,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

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
        let (parameters, controls, authorization, authorization_work) =
            self.prepare_application_query_admission(query, access, parameters, controls)?;
        self.finish_application_query_admission(
            query,
            access,
            parameters,
            controls,
            authorization,
            authorization_work,
        )
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
        authorization: crate::domain_computation::primary_graph::authorization::WorthQueryRetainedAuthorizationDecisionFacts,
        authorization_work: super::WorthQueryApplicationAuthorizationWorkEvidence,
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
        let graph_read_plan = review_graph_read_access(
            requirements,
            inventory,
            application_query_graph_read_budget(
                self.runtime.application_query_resource_profile(),
                &controls,
            ),
        );
        let canonical_work = WorthQueryCanonicalWorkPhases::new(
            query.installation_canonical_work(),
            parameters
                .canonical_basis()
                .work()
                .combine(graph_read_plan.requirements().canonical_work()),
        );
        validate_work_limit(&graph_read_plan, &controls, query.name())?;
        if let Some(denial) = graph_read_plan.denial() {
            return Err(WorthQueryApplicationQueryAdmissionDenial::new(
                WorthQueryApplicationQueryAdmissionDenialKind::GraphReadPlan(denial.kind()),
                query.name(),
            ));
        }
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
            graph_read_plan,
            canonical_work,
            continuation_index_id,
            continuation_state: None,
            basis,
            authorization,
            authorization_work,
        })
    }
}

fn validate_work_limit<Schema>(
    plan: &worth_query_admission::facade::graph_read_access::WorthQueryGraphReadPlanReview,
    controls: &WorthQueryApplicationQueryControls<'_, Schema>,
    subject: &str,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
    let intrinsic = plan.cost_estimate().intrinsic();
    let estimated_work = intrinsic
        .candidate_roots()
        .saturating_add(intrinsic.edge_touches())
        .saturating_add(intrinsic.intermediate_set_size());
    if estimated_work > controls.maximum_work().get() {
        return Err(denial(
            WorthQueryApplicationQueryAdmissionDenialKind::WorkLimitExceeded,
            subject,
        ));
    }
    Ok(())
}

fn application_query_graph_read_budget<Schema>(
    profile: WorthQueryApplicationQueryResourceProfile,
    controls: &WorthQueryApplicationQueryControls<'_, Schema>,
) -> worth_query_admission::facade::graph_read_access::WorthQueryGraphReadBudget {
    profile.admission_budget(controls.maximum_result_count(), controls.maximum_work())
}

fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
