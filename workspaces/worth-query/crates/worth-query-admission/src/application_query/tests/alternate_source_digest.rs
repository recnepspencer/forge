use worth_foundational::facade::CanonicalDigestId;
use worth_query_declaration::facade::application_query::ApplicationQueryCardinality;
use worth_query_installation::facade::{
    WorthQueryApplicationCanonicalArtifact, WorthQueryInstalledGraphReadContract,
    WorthQueryPreparedReadGraphPlanningContract, WorthQueryReadGraphOrderingView,
    WorthQueryReadGraphPlanningContract, WorthQueryReadGraphPredicateView,
    WorthQueryReadGraphProjectionView, WorthQueryReadGraphRelationView,
};

/// The same installed semantic contract observed through another descriptive
/// source identity. This is deliberately not a mature `WorthQueryReadGraph`.
pub(super) struct AlternateSourceDigestGraph<'a>(
    pub(super) &'a WorthQueryInstalledGraphReadContract,
);

impl WorthQueryReadGraphPlanningContract for AlternateSourceDigestGraph<'_> {
    fn schema_basis_digest(&self) -> &CanonicalDigestId {
        self.0.schema_basis_digest()
    }

    fn root_entity(&self) -> &str {
        self.0.root_entity()
    }

    fn cardinality(&self) -> ApplicationQueryCardinality {
        self.0.cardinality()
    }

    fn projection_count(&self) -> usize {
        self.0.projection_count()
    }

    fn projection(&self, index: usize) -> Option<WorthQueryReadGraphProjectionView<'_>> {
        self.0.projection(index)
    }

    fn relation_count(&self) -> usize {
        self.0.relation_count()
    }

    fn relation(&self, index: usize) -> Option<WorthQueryReadGraphRelationView<'_>> {
        self.0.relation(index)
    }

    fn predicate_count(&self) -> usize {
        self.0.predicate_count()
    }

    fn predicate(&self, index: usize) -> Option<WorthQueryReadGraphPredicateView<'_>> {
        self.0.predicate(index)
    }

    fn ordering_count(&self) -> usize {
        self.0.ordering_count()
    }

    fn ordering(&self, index: usize) -> Option<WorthQueryReadGraphOrderingView<'_>> {
        WorthQueryReadGraphPlanningContract::ordering(self.0, index)
    }

    fn maximum_traversal_depth(&self) -> usize {
        self.0.maximum_traversal_depth()
    }
}

impl WorthQueryPreparedReadGraphPlanningContract for AlternateSourceDigestGraph<'_> {
    fn canonical_planning_basis(&self) -> &WorthQueryApplicationCanonicalArtifact {
        self.0.canonical_planning_basis()
    }
}
