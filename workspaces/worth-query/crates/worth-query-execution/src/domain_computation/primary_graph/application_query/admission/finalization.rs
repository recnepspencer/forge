use worth_query_admission::facade::{
    application_query::{
        derive_graph_read_access_requirements_for_contract,
        WorthQueryAdmittedApplicationQueryParameters, WorthQueryApplicationQueryLane,
    },
    graph_obligation::WorthQueryGraphWorkIntent,
    graph_read_access::{WorthQueryGraphReadBudget, WorthQueryGraphReadPlanReview},
};
use worth_query_admission::integration::{
    admit_application_query_graph_work, require_selected_graph_work,
    review_application_query_graph_work, select_installed_graph_obligations,
    WorthQueryRequiredGraphWork, WorthQueryReviewedApplicationQueryGraphWork,
};
use worth_query_declaration::facade::application_schema::ApplicationSchema;
use worth_query_installation::facade::{
    WorthQueryCanonicalWorkPhases, WorthQueryInstalledApplicationQuery,
};
use worth_relational::facade::indexes::DerivedIndexId;

use super::super::{
    admission_preparation::validate_admission_request,
    basis::admit_application_query_basis,
    disclosure::{
        admit_application_query_governance, compile_disclosure_contract,
        WorthQueryApplicationGovernanceBinding, WorthQueryApplicationQueryGovernanceDenialKind,
        WorthQueryPendingApplicationQueryGovernance,
    },
    execution_shape::validate_one_shot_shape,
    runtime_support::primary_graph_support_inventory,
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
    WorthQueryApplicationQueryControls,
};
use super::governed_access::WorthQueryGovernanceAdmission;
use super::obligation_progression::validate_installed_obligation_progression;
use crate::domain_computation::{
    execution_runtime::WorthQueryApplicationQueryResourceProfile,
    primary_graph::WorthQueryPrimaryGraphApplicationRuntime,
    provider_session::{
        start_read_graph_work_session, WorthQueryGraphWorkAccessContextAffinity,
        WorthQueryGraphWorkBasisAffinity, WorthQueryGraphWorkSessionAffinity,
    },
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub(in crate::domain_computation::primary_graph::application_query) fn finish_application_query_admission<
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
        governance_admission: Option<
            WorthQueryGovernanceAdmission<Schema, Principal, PrincipalIdentity, Scope>,
        >,
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
        self.validate_application_query_access(query, access)?;
        validate_installed_obligation_progression(query)?;
        let selected = select_installed_graph_obligations(
            query.obligations(),
            WorthQueryGraphWorkIntent::application_query_read(),
        )
        .map_err(|_| graph_work_denial(query.name()))?;
        let required = require_selected_graph_work(selected, self.graph_admission_authority())
            .map_err(|_| graph_work_denial(query.name()))?;
        let reviewed_graph_work =
            review_application_graph_read(self, query, &parameters, &controls, required)?;
        let graph_read_plan = reviewed_graph_work.review().clone();
        validate_graph_read_plan(&graph_read_plan, &controls, query.name())?;
        validate_one_shot_shape(query)?;
        let continuation_index_id = continuation_index_id(self, query, &controls)?;
        validate_admission_request(controls.request_scope(), query.name())?;
        let (basis_selection, controls) = controls.into_admission_parts();
        let basis = admit_application_query_basis(self, basis_selection)?;
        validate_admission_request(controls.request_scope(), query.name())?;
        let support = self.graph_work_resource_support();
        let admitted_graph_work = admit_application_query_graph_work(reviewed_graph_work, &support)
            .map_err(|_| graph_work_denial(query.name()))?;
        let branch = self.branch_affinity().clone();
        let basis_affinity = WorthQueryGraphWorkBasisAffinity::query(basis.identity(), &branch)
            .map_err(|_| graph_work_denial(query.name()))?;
        let session_affinity = WorthQueryGraphWorkSessionAffinity::new(
            &admitted_graph_work,
            self.runtime.authority_identity(),
            query.obligations().identity(),
            query.authority_identity(),
            access.principal().principal_entity_id(),
            WorthQueryGraphWorkAccessContextAffinity::entity(access.scope().entity_id()),
            branch,
            basis_affinity,
            self.graph_work_provider_authority(),
        )
        .map_err(|_| graph_work_denial(query.name()))?;
        let mut graph_work_session =
            start_read_graph_work_session(admitted_graph_work, basis, session_affinity)
                .map_err(|_| graph_work_denial(query.name()))?;
        let pending_governance = match governance_admission {
            Some(admit) => Some(admit(self, access, &mut graph_work_session)?),
            None => None,
        };
        let governance =
            admit_disclosure_governance(self, query, access, &parameters, pending_governance)?;
        let (authorization, ordinary_authorization_work) = self.validate_access_in_session(
            query,
            access,
            controls.request_scope(),
            &mut graph_work_session,
        )?;
        let governance_authorization_work = governance
            .authorization()
            .map(|authorization| {
                super::super::WorthQueryApplicationAuthorizationWorkEvidence::from_dependencies(
                    std::slice::from_ref(authorization.decision()),
                )
            })
            .unwrap_or_default();
        let authorization_work = ordinary_authorization_work.combine(governance_authorization_work);
        let canonical_work = WorthQueryCanonicalWorkPhases::new(
            query.installation_canonical_work(),
            parameters
                .canonical_basis()
                .work()
                .combine(graph_read_plan.requirements().canonical_work())
                .combine(graph_work_session.canonical_work()),
        );
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
            obligations: query.obligations(),
            canonical_work,
            continuation_index_id,
            continuation_state: None,
            graph_work_session: Some(graph_work_session),
            authorization,
            authorization_work,
            governance,
        })
    }
}

fn review_application_graph_read<Schema, Query, Parameters, QueryResult, Scope>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    parameters: &WorthQueryAdmittedApplicationQueryParameters,
    controls: &WorthQueryApplicationQueryControls<'_, Schema>,
    required: WorthQueryRequiredGraphWork,
) -> Result<WorthQueryReviewedApplicationQueryGraphWork, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
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
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
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
    review_application_query_graph_work(
        required,
        requirements,
        inventory,
        application_query_graph_read_budget(
            runtime.runtime.application_query_resource_profile(),
            controls,
        ),
    )
    .map_err(|_| graph_work_denial(query.name()))
}

fn admit_disclosure_governance<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    access: &WorthQueryApplicationQueryAccessContext<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    parameters: &WorthQueryAdmittedApplicationQueryParameters,
    pending: Option<WorthQueryPendingApplicationQueryGovernance>,
) -> Result<
    super::super::disclosure::WorthQueryApplicationQueryGovernance,
    WorthQueryApplicationQueryAdmissionDenial,
>
where
    Schema: ApplicationSchema,
{
    if query.disclosure().posture()
        == worth_query_declaration::facade::application_query::ApplicationQueryDisclosurePosture::Governed
        && pending.is_none()
    {
        return Err(governance_denial(
            WorthQueryApplicationQueryGovernanceDenialKind::Required,
            query.name(),
        ));
    }
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryApplicationQueryAdmissionDenialKind::StaleScope,
            query.name(),
        )
    })?;
    let disclosure = compile_disclosure_contract(query, &graph.layout).map_err(|denial| {
        WorthQueryApplicationQueryAdmissionDenial::new(
            WorthQueryApplicationQueryAdmissionDenialKind::DisclosureContractInvalid,
            denial.subject(),
        )
    })?;
    admit_application_query_governance(
        disclosure,
        pending,
        WorthQueryApplicationGovernanceBinding::new(
            runtime.runtime.authority_identity(),
            query.identity().clone(),
            *parameters.identity(),
            access.principal().principal_entity_id(),
            access.scope().entity_id(),
        ),
    )
    .map_err(|kind| governance_denial(kind, query.name()))
}

fn continuation_index_id<Schema, Query, Parameters, QueryResult, Scope>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    controls: &WorthQueryApplicationQueryControls<'_, Schema>,
) -> Result<Option<DerivedIndexId>, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    if controls.lane() != WorthQueryApplicationQueryLane::Continuation {
        return Ok(None);
    }
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryApplicationQueryAdmissionDenialKind::StaleScope,
            query.name(),
        )
    })?;
    query
        .continuation()
        .and_then(|contract| graph.layout.continuation_ordering_index_id(contract))
        .ok_or_else(|| {
            denial(
                WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
                query.name(),
            )
        })
        .map(Some)
}

fn governance_denial(
    kind: WorthQueryApplicationQueryGovernanceDenialKind,
    subject: &str,
) -> WorthQueryApplicationQueryAdmissionDenial {
    let kind = match kind {
        WorthQueryApplicationQueryGovernanceDenialKind::Required => {
            WorthQueryApplicationQueryAdmissionDenialKind::DisclosureGovernanceRequired
        }
        WorthQueryApplicationQueryGovernanceDenialKind::CapabilityMismatch => {
            WorthQueryApplicationQueryAdmissionDenialKind::DisclosureAuthorizationMismatch
        }
        WorthQueryApplicationQueryGovernanceDenialKind::InternalComputationDenied => {
            WorthQueryApplicationQueryAdmissionDenialKind::InternalComputationDenied
        }
    };
    denial(kind, subject)
}

fn graph_work_denial(subject: &str) -> WorthQueryApplicationQueryAdmissionDenial {
    denial(
        WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
        subject,
    )
}

fn validate_graph_read_plan<Schema>(
    plan: &WorthQueryGraphReadPlanReview,
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
    if let Some(plan_denial) = plan.denial() {
        return Err(WorthQueryApplicationQueryAdmissionDenial::new(
            WorthQueryApplicationQueryAdmissionDenialKind::GraphReadPlan(plan_denial.kind()),
            subject,
        ));
    }
    Ok(())
}

fn application_query_graph_read_budget<Schema>(
    profile: WorthQueryApplicationQueryResourceProfile,
    controls: &WorthQueryApplicationQueryControls<'_, Schema>,
) -> WorthQueryGraphReadBudget {
    profile.admission_budget(controls.maximum_result_count(), controls.maximum_work())
}

fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
