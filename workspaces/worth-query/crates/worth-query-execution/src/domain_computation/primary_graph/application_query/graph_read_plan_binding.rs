use worth_foundational::facade::CanonicalDigestId;
use worth_query_admission::facade::{
    application_query::WorthQueryAdmittedApplicationQueryParameters,
    graph_read_access::WorthQueryGraphReadPlanReview,
};
use worth_query_installation::facade::{
    WorthQueryCanonicalWorkPhases, WorthQueryInstalledApplicationQuery,
    WorthQueryInstalledApplicationQueryIdentity, WorthQueryInstalledGraphObligationSet,
};
use worth_relational::facade::indexes::DerivedIndexId;
use worth_relational::facade::indexes::{DerivedIndexGenerationId, RelatedEntityOrderingBoundary};
use worth_relational::facade::runtime::RelationalExecutionBasisIdentity;

use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;

use super::super::{WorthQueryApplicationEntityIdentity, WorthQueryAuthenticatedPrincipal};
use super::{
    resource_lifecycle::WorthQueryApplicationBasisLease, WorthQueryAdmittedApplicationQueryControls,
};

/// Sealed execution authority for one exact installed query admission.
///
/// Descriptive query identities, support inventories, and graph-plan reviews
/// cannot construct this value.
pub struct WorthQueryAdmittedApplicationQueryPlan<
    'a,
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
> {
    pub(super) runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    pub(super) graph_authority_identity: String,
    pub(super) provider_identity: String,
    pub(super) query:
        &'a WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    pub(super) principal:
        &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    pub(super) scope: &'a WorthQueryApplicationEntityIdentity<Schema, Scope>,
    pub(super) parameters: WorthQueryAdmittedApplicationQueryParameters,
    pub(super) controls: WorthQueryAdmittedApplicationQueryControls<'a>,
    pub(super) graph_read_plan: WorthQueryGraphReadPlanReview,
    pub(super) obligations: &'a WorthQueryInstalledGraphObligationSet,
    pub(super) canonical_work: WorthQueryCanonicalWorkPhases,
    pub(super) continuation_index_id: Option<DerivedIndexId>,
    pub(super) continuation_state: Option<WorthQueryAdmittedContinuationState>,
    pub(super) graph_work_session: Option<super::WorthQueryApplicationQueryGraphWorkSession>,
    pub(super) authorization:
        crate::domain_computation::authorization::WorthQueryRetainedAuthorizationDecisionFacts,
    pub(super) authorization_work: super::WorthQueryApplicationAuthorizationWorkEvidence,
    pub(super) governance: super::disclosure::WorthQueryApplicationQueryGovernance,
}

pub(super) struct WorthQueryCompletedApplicationQueryPlan<
    'a,
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
> {
    pub(super) plan: WorthQueryAdmittedApplicationQueryPlan<
        'a,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    pub(super) release:
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt,
}

pub(super) struct WorthQueryAdmittedContinuationState {
    pub(super) expected_generation: DerivedIndexGenerationId,
    pub(super) boundary: RelatedEntityOrderingBoundary,
    pub(super) page_ordinal: u64,
}

impl<'a, Schema, Query, Parameters, QueryResult, Principal, PrincipalIdentity, Scope>
    WorthQueryAdmittedApplicationQueryPlan<
        'a,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >
{
    pub fn query_identity(&self) -> &WorthQueryInstalledApplicationQueryIdentity {
        self.query.identity()
    }

    pub fn parameter_binding_identity(&self) -> &CanonicalDigestId {
        self.parameters.identity()
    }

    pub fn controls(&self) -> &WorthQueryAdmittedApplicationQueryControls<'a> {
        &self.controls
    }

    pub fn graph_read_plan(&self) -> &WorthQueryGraphReadPlanReview {
        &self.graph_read_plan
    }

    pub const fn obligations(&self) -> &WorthQueryInstalledGraphObligationSet {
        self.obligations
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.canonical_work
    }

    pub fn continuation_index_id(&self) -> Option<DerivedIndexId> {
        self.continuation_index_id
    }

    pub fn basis_identity(&self) -> &RelationalExecutionBasisIdentity {
        self.basis().identity()
    }

    pub(super) fn basis(&self) -> &WorthQueryApplicationBasisLease {
        self.graph_work_session
            .as_ref()
            .expect("an admitted application query owns one graph-work session")
            .basis()
    }

    pub(super) fn graph_work_session_mut(
        &mut self,
    ) -> &mut super::WorthQueryApplicationQueryGraphWorkSession {
        self.graph_work_session
            .as_mut()
            .expect("an admitted application query owns one graph-work session")
    }

    pub(super) fn graph_work_session(&self) -> &super::WorthQueryApplicationQueryGraphWorkSession {
        self.graph_work_session
            .as_ref()
            .expect("an admitted application query owns one graph-work session")
    }

    pub(super) fn take_graph_work_session(
        &mut self,
    ) -> super::WorthQueryApplicationQueryGraphWorkSession {
        self.graph_work_session
            .take()
            .expect("an admitted application query completes one graph-work session")
    }

    pub(super) fn complete_graph_read(
        mut self,
    ) -> Result<
        WorthQueryCompletedApplicationQueryPlan<
            'a,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        crate::domain_computation::provider_session::WorthQueryGraphWorkDecisionReadSetDenial,
    > {
        let release = self
            .take_graph_work_session()
            .complete_decision_read_set()?
            .finish_read();
        Ok(WorthQueryCompletedApplicationQueryPlan {
            plan: self,
            release,
        })
    }

    pub(super) fn abort_graph_read(
        mut self,
    ) -> crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt {
        self.take_graph_work_session().abort_read()
    }

    pub fn principal(
        &self,
    ) -> &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity> {
        self.principal
    }

    pub fn scope(&self) -> &WorthQueryApplicationEntityIdentity<Schema, Scope> {
        self.scope
    }

    pub fn runtime_authority_identity(&self) -> u64 {
        self.runtime_authority.as_u64()
    }

    pub fn authorization_decision_fact_count(&self) -> usize {
        self.authorization.exact_fact_count()
            + self
                .governance
                .authorization()
                .map_or(0, |authorization| authorization.exact_fact_count())
    }

    pub fn capability_time_sample(&self) -> Option<&worth_foundational::facade::AspectValue> {
        self.governance
            .authorization()
            .map(|authorization| authorization.sampled_value())
    }

    pub const fn authorization_work(
        &self,
    ) -> super::WorthQueryApplicationAuthorizationWorkEvidence {
        self.authorization_work
    }

    pub(super) fn take_governance(
        &mut self,
    ) -> super::disclosure::WorthQueryApplicationQueryGovernance {
        std::mem::replace(
            &mut self.governance,
            super::disclosure::WorthQueryApplicationQueryGovernance::Public,
        )
    }
}
