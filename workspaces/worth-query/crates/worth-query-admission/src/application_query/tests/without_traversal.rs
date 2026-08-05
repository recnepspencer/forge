use worth_foundational::facade::CanonicalDigestId;
use worth_query_declaration::facade::application_query::ApplicationQueryCardinality;
use worth_query_installation::facade::{
    prepare_canonical_read_graph_planning_basis, WorthQueryApplicationCanonicalArtifact,
    WorthQueryInstalledGraphReadContract, WorthQueryPreparedReadGraphPlanningContract,
    WorthQueryReadGraphOrderingView, WorthQueryReadGraphPlanningContract,
    WorthQueryReadGraphPredicateView, WorthQueryReadGraphProjectionView,
    WorthQueryReadGraphRelationView,
};

/// Hostile one-axis planning contract that removes only traversal meaning from
/// a real installed application graph.
pub(super) struct WithoutTraversalGraph<'a> {
    graph: &'a WorthQueryInstalledGraphReadContract,
    canonical: WorthQueryApplicationCanonicalArtifact,
}

impl<'a> WithoutTraversalGraph<'a> {
    pub(super) fn new(graph: &'a WorthQueryInstalledGraphReadContract) -> Self {
        let mut prepared = Self {
            graph,
            canonical: graph.canonical_planning_basis().clone(),
        };
        prepared.canonical =
            prepare_canonical_read_graph_planning_basis(&prepared, super::test_canonical_budget())
                .expect("the hostile planning graph fits its canonical budget");
        prepared
    }
}

impl WorthQueryReadGraphPlanningContract for WithoutTraversalGraph<'_> {
    fn schema_basis_digest(&self) -> &CanonicalDigestId {
        self.graph.schema_basis_digest()
    }

    fn root_entity(&self) -> &str {
        self.graph.root_entity()
    }

    fn cardinality(&self) -> ApplicationQueryCardinality {
        self.graph.cardinality()
    }

    fn projection_count(&self) -> usize {
        self.graph.projection_count()
    }

    fn projection(&self, index: usize) -> Option<WorthQueryReadGraphProjectionView<'_>> {
        self.graph.projection(index)
    }

    fn relation_count(&self) -> usize {
        0
    }

    fn relation(&self, _index: usize) -> Option<WorthQueryReadGraphRelationView<'_>> {
        None
    }

    fn predicate_count(&self) -> usize {
        self.graph.predicate_count()
    }

    fn predicate(&self, index: usize) -> Option<WorthQueryReadGraphPredicateView<'_>> {
        self.graph.predicate(index)
    }

    fn ordering_count(&self) -> usize {
        self.graph.ordering_count()
    }

    fn ordering(&self, index: usize) -> Option<WorthQueryReadGraphOrderingView<'_>> {
        WorthQueryReadGraphPlanningContract::ordering(self.graph, index)
    }

    fn maximum_traversal_depth(&self) -> usize {
        0
    }
}

impl WorthQueryPreparedReadGraphPlanningContract for WithoutTraversalGraph<'_> {
    fn canonical_planning_basis(&self) -> &WorthQueryApplicationCanonicalArtifact {
        &self.canonical
    }
}
