//! Tests for vertex placement operations.

#[cfg(test)]
mod tests {
    use crate::context::ModelingContext;
    use crate::operations::shared_operations::facade::{place_vertex, PlacementRegistry};
    use forge_topo::provenance::{
        LineageMode, LineageRecorder, OperationLineageContext, FEATURE_ID_SYSTEM,
    };
    use forge_topo::transactions::TopologyState;

    fn fresh_draft() -> forge_topo::transactions::MutableDraft {
        TopologyState::empty().into_mutation()
    }

    fn test_recorder() -> LineageRecorder {
        LineageRecorder::new(
            OperationLineageContext {
                feature_id: FEATURE_ID_SYSTEM,
                op_name: "test_placement",
                mode: LineageMode::Root,
            },
            1,
        )
    }

    #[test]
    fn first_vertex_no_decision_recorded() {
        let mut draft = fresh_draft();
        let mut registry = PlacementRegistry::new();
        let mut ctx = ModelingContext::new();
        let mut recorder = test_recorder();
        place_vertex(
            &mut draft,
            &mut registry,
            [0.0, 0.0, 0.0],
            1e-6,
            &mut ctx,
            &mut recorder,
        );
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn coincident_vertex_reuses_existing_id() {
        let mut draft = fresh_draft();
        let mut registry = PlacementRegistry::new();
        let mut ctx = ModelingContext::new();
        let mut recorder = test_recorder();
        let v1 = place_vertex(
            &mut draft,
            &mut registry,
            [0.0, 0.0, 0.0],
            1e-3,
            &mut ctx,
            &mut recorder,
        );
        let v2 = place_vertex(
            &mut draft,
            &mut registry,
            [0.0, 0.0, 5e-4],
            1e-3,
            &mut ctx,
            &mut recorder,
        );
        assert_eq!(v1, v2, "Coincident vertex must reuse existing VertexId");
        assert_eq!(registry.len(), 1, "Only 1 entry in registry after merge");
    }

    #[test]
    fn distinct_vertex_gets_new_id() {
        let mut draft = fresh_draft();
        let mut registry = PlacementRegistry::new();
        let mut ctx = ModelingContext::new();
        let mut recorder = test_recorder();
        let v1 = place_vertex(
            &mut draft,
            &mut registry,
            [0.0, 0.0, 0.0],
            1e-6,
            &mut ctx,
            &mut recorder,
        );
        let v2 = place_vertex(
            &mut draft,
            &mut registry,
            [1.0, 0.0, 0.0],
            1e-6,
            &mut ctx,
            &mut recorder,
        );
        assert_ne!(v1, v2, "Distinct positions must produce distinct VertexIds");
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn decisions_recorded_from_second_vertex_onward() {
        use forge_core::DecisionKind;

        let mut draft = fresh_draft();
        let mut registry = PlacementRegistry::new();
        let mut ctx = ModelingContext::new();
        let mut recorder = test_recorder();

        place_vertex(
            &mut draft,
            &mut registry,
            [0.0, 0.0, 0.0],
            1e-6,
            &mut ctx,
            &mut recorder,
        );
        assert_eq!(
            ctx.get_decision_count(),
            0,
            "First vertex must not record a decision"
        );

        place_vertex(
            &mut draft,
            &mut registry,
            [10.0, 0.0, 0.0],
            1e-6,
            &mut ctx,
            &mut recorder,
        );
        assert_eq!(
            ctx.get_decision_count(),
            1,
            "Second vertex must record one NearBoundary decision"
        );

        let log = ctx.get_decision_log();
        let decisions: Vec<_> = log.decisions().collect();
        assert!(
            matches!(decisions[0].get_kind(), DecisionKind::NearBoundary { .. }),
            "Decision kind must be NearBoundary"
        );
    }
}
