use worth_foundational::facade::CanonicalDigestId;
use worth_query_admission::facade::{
    application_query::WorthQueryAdmittedApplicationQueryParameters,
    graph_read_access::WorthQueryGraphReadPlanReview,
};
use worth_query_installation::facade::{
    WorthQueryCanonicalWorkPhases, WorthQueryInstalledApplicationQuery,
    WorthQueryInstalledApplicationQueryIdentity,
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
    pub(super) canonical_work: WorthQueryCanonicalWorkPhases,
    pub(super) continuation_index_id: Option<DerivedIndexId>,
    pub(super) continuation_state: Option<WorthQueryAdmittedContinuationState>,
    pub(super) basis: WorthQueryApplicationBasisLease,
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

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.canonical_work
    }

    pub fn continuation_index_id(&self) -> Option<DerivedIndexId> {
        self.continuation_index_id
    }

    pub fn basis_identity(&self) -> &RelationalExecutionBasisIdentity {
        self.basis.identity()
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
}
