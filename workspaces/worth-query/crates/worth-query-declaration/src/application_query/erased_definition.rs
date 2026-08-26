use super::{
    ApplicationQueryAuthorizationRequirement, ApplicationQueryBasisSupport,
    ApplicationQueryCanonicalArtifact, ApplicationQueryCardinality,
    ApplicationQueryContinuationTarget, ApplicationQueryDefinition,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryLaneEligibility, ApplicationQueryLiveCauseContract,
    ApplicationQueryOrderingTerm, ApplicationQueryParameterDefinition, ApplicationQueryPredicate,
    ApplicationQueryReference, ApplicationQueryResultShape, ApplicationQueryRootPathMeaning,
};
use crate::portable_identity::WorthQueryPortableTypeIdentity;

/// Descriptive, authority-free application-query meaning retained by a
/// domain package.
///
/// Installation authority comes from resolving a typed reference against this
/// exact member of an installed application schema.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ErasedApplicationQueryDefinition {
    name: String,
    query_type: WorthQueryPortableTypeIdentity,
    parameter_type: WorthQueryPortableTypeIdentity,
    result_type: WorthQueryPortableTypeIdentity,
    scope_type: WorthQueryPortableTypeIdentity,
    root_entity: String,
    scope_entity: String,
    parameters: Vec<ApplicationQueryParameterDefinition>,
    result_shape: ApplicationQueryResultShape,
    root_paths: Vec<ApplicationQueryRootPathMeaning>,
    cardinality: ApplicationQueryCardinality,
    predicates: Vec<ApplicationQueryPredicate>,
    ordering: Vec<ApplicationQueryOrderingTerm>,
    continuation: Option<ApplicationQueryContinuationTarget>,
    live_cause: Option<ApplicationQueryLiveCauseContract>,
    dependency_ceiling: ApplicationQueryDependencyCeiling,
    disclosure: ApplicationQueryDisclosureContract,
    authorization: ApplicationQueryAuthorizationRequirement,
    basis_support: ApplicationQueryBasisSupport,
    lanes: ApplicationQueryLaneEligibility,
    canonical: ApplicationQueryCanonicalArtifact,
}

impl ErasedApplicationQueryDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn query_type(&self) -> &str {
        self.query_type.as_str()
    }

    pub const fn query_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.query_type
    }

    pub fn parameter_type(&self) -> &str {
        self.parameter_type.as_str()
    }

    pub const fn parameter_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.parameter_type
    }

    pub fn result_type(&self) -> &str {
        self.result_type.as_str()
    }

    pub const fn result_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.result_type
    }

    pub fn scope_type(&self) -> &str {
        self.scope_type.as_str()
    }

    pub const fn scope_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.scope_type
    }

    pub fn root_entity(&self) -> &str {
        &self.root_entity
    }

    pub fn scope_entity(&self) -> &str {
        &self.scope_entity
    }

    pub fn parameters(&self) -> &[ApplicationQueryParameterDefinition] {
        &self.parameters
    }

    pub fn result_shape(&self) -> &ApplicationQueryResultShape {
        &self.result_shape
    }

    pub fn root_paths(&self) -> &[ApplicationQueryRootPathMeaning] {
        &self.root_paths
    }

    pub const fn cardinality(&self) -> ApplicationQueryCardinality {
        self.cardinality
    }

    pub fn predicates(&self) -> &[ApplicationQueryPredicate] {
        &self.predicates
    }

    pub fn ordering(&self) -> &[ApplicationQueryOrderingTerm] {
        &self.ordering
    }

    pub const fn continuation(&self) -> Option<&ApplicationQueryContinuationTarget> {
        self.continuation.as_ref()
    }

    pub const fn live_cause(&self) -> Option<&ApplicationQueryLiveCauseContract> {
        self.live_cause.as_ref()
    }

    pub const fn dependency_ceiling(&self) -> ApplicationQueryDependencyCeiling {
        self.dependency_ceiling
    }

    pub fn disclosure(&self) -> &ApplicationQueryDisclosureContract {
        &self.disclosure
    }

    pub const fn authorization(&self) -> &ApplicationQueryAuthorizationRequirement {
        &self.authorization
    }

    pub const fn basis_support(&self) -> ApplicationQueryBasisSupport {
        self.basis_support
    }

    pub const fn lanes(&self) -> ApplicationQueryLaneEligibility {
        self.lanes
    }

    pub fn canonical_basis(&self) -> &ApplicationQueryCanonicalArtifact {
        &self.canonical
    }

    pub fn matches_reference<Schema, Query, Parameters, QueryResult, Scope>(
        &self,
        reference: ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>,
    ) -> bool {
        self.name == reference.name()
            && self.query_type == reference.query_type()
            && self.parameter_type == reference.parameter_type()
            && self.result_type == reference.result_type()
            && self.scope_type == reference.scope_type()
    }

    pub(super) fn from_typed<Schema, Query, Parameters, QueryResult, Scope>(
        definition: ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
    ) -> Self {
        let canonical = super::canonical_basis::prepare_definition_basis(&definition);
        Self {
            name: definition.name().to_string(),
            query_type: definition.query_identity(),
            parameter_type: definition.parameter_identity(),
            result_type: definition.result_identity(),
            scope_type: definition.scope_identity(),
            root_entity: definition.root_entity().to_string(),
            scope_entity: definition.scope_entity().to_string(),
            parameters: definition.parameters,
            result_shape: definition.result_shape,
            root_paths: definition.root_paths,
            cardinality: definition.cardinality,
            predicates: definition.predicates,
            ordering: definition.ordering,
            continuation: definition.continuation,
            live_cause: definition.live_cause,
            dependency_ceiling: definition.dependency_ceiling,
            disclosure: definition.disclosure,
            authorization: definition.authorization,
            basis_support: definition.basis_support,
            lanes: definition.lanes,
            canonical,
        }
    }
}
