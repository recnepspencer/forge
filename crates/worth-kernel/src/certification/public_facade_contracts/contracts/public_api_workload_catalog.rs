#[cfg(test)]
mod tests {
    use worth_kernel::workload_composition::{
        WorkloadCatalog, WorkloadCatalogError, WorkloadCatalogRecipe, WorkloadCatalogSupportPosture,
    };
    use worth_spatial::facade::workload_vocabulary::{
        CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger, WorkloadEvidenceLedgerError,
        WorkloadEvidenceRow, WorkloadEvidenceStage,
    };

    #[test]
    fn workload_catalog_recipes_emit_complete_evidence_ledgers() {
        for recipe in admitted_catalog_recipes() {
            let built = recipe.build().expect("catalog recipe should build");
            let workload = built.workload();
            let ledger = workload.evidence_ledger();

            assert_eq!(ledger.counters().rows(), 8);
            assert_authority_stage(ledger, WorkloadEvidenceStage::Topology);
            assert_authority_stage(ledger, WorkloadEvidenceStage::GeometryBinding);
            assert_authority_stage(ledger, WorkloadEvidenceStage::SurfaceSupport);
            assert_authority_stage(ledger, WorkloadEvidenceStage::Projection);
            assert_authority_stage(ledger, WorkloadEvidenceStage::Transform);
            assert_authority_stage(ledger, WorkloadEvidenceStage::RetainedReplay);
            assert_authority_stage(ledger, WorkloadEvidenceStage::Diagnostics);
            assert_authority_stage(ledger, WorkloadEvidenceStage::Response);

            assert_eq!(
                built.support().posture(),
                WorkloadCatalogSupportPosture::Admitted
            );
            assert_catalog_query_receipt_is_digest(built.declaration().query_declaration_digest());
            assert_catalog_query_receipt_is_digest(built.support().query_support_digest());
            assert_real_geometry_breadth(ledger);
        }
    }

    #[test]
    fn workload_catalog_reports_unsupported_dirty_recipe_without_fake_workload() {
        let dirty_recipe = WorkloadCatalog::dirty_self_intersecting_loop();
        let support = dirty_recipe
            .inspect_support()
            .expect("dirty recipe support should be inspectable");
        assert_eq!(
            support.posture(),
            WorkloadCatalogSupportPosture::Unsupported
        );
        assert!(support.human_reason().contains("self-intersecting"));
        assert_catalog_query_receipt_is_digest(support.query_support_digest());

        let error = dirty_recipe
            .build()
            .expect_err("dirty catalog recipe must not synthesize an admitted workload");

        match error {
            WorkloadCatalogError::UnsupportedRecipe { recipe, reason } => {
                assert_eq!(
                    recipe.human_name(),
                    "dirty self-intersecting loop workload recipe"
                );
                assert!(reason.contains("not yet supported"));
                assert!(reason.contains("self-intersecting"));
            }
            other => panic!("expected unsupported dirty recipe, got {other:?}"),
        }
    }

    #[test]
    fn retained_catalog_recipe_proves_retained_artifact_and_replay_breadth() {
        let built = WorkloadCatalog::retained_cancellation_chain()
            .build()
            .expect("retained catalog recipe should build");
        let replay = built
            .workload()
            .evidence_ledger()
            .row_for_stage(WorkloadEvidenceStage::RetainedReplay)
            .expect("retained replay evidence row");

        assert!(replay.counters().retained_artifact_count() > 0);
        assert!(replay.counters().replay_checkpoint_count() > 0);
    }

    #[test]
    fn catalog_hostile_shapes_do_not_collapse_to_cube_fixture() {
        let cube = catalog_breadth(WorkloadCatalog::cube());
        let tetrahedron = catalog_breadth(WorkloadCatalog::tetrahedron());
        let single_face_loop = catalog_breadth(WorkloadCatalog::single_face_loop());
        let thin_feature_wall = catalog_breadth(WorkloadCatalog::thin_feature_wall());
        let high_valence_vertex = catalog_breadth(WorkloadCatalog::high_valence_vertex());
        let open_sheet = catalog_breadth(WorkloadCatalog::open_sheet());
        let retained_chain = catalog_breadth(WorkloadCatalog::retained_cancellation_chain());

        assert_ne!(cube.topology_entities, tetrahedron.topology_entities);
        assert_ne!(cube.topology_relations, tetrahedron.topology_relations);
        assert!(thin_feature_wall.topology_entities > single_face_loop.topology_entities);
        assert!(thin_feature_wall.binding_targets > single_face_loop.binding_targets);
        assert!(thin_feature_wall.projected_entities > single_face_loop.projected_entities);
        assert_ne!(
            cube.topology_relations,
            high_valence_vertex.topology_relations
        );
        assert_ne!(cube.topology_entities, open_sheet.topology_entities);
        assert!(retained_chain.retained_artifacts > 0);
        assert!(retained_chain.replay_checkpoints > 0);
    }

    #[test]
    fn workload_catalog_rejects_blank_declaration_before_query_build() {
        let error = WorkloadCatalog::cube()
            .declared("   ")
            .inspect_support()
            .expect_err("blank catalog declaration should be denied before Query admission");

        assert_eq!(error, WorkloadCatalogError::MissingDeclaration);
    }

    #[test]
    fn workload_catalog_blocks_static_fixture_substitution() {
        let built = WorkloadCatalog::cube()
            .declared("compile-time catalog boundary companion")
            .build()
            .expect("real catalog cube should build");

        assert_eq!(built.workload().evidence_ledger().counters().rows(), 8);
        assert!(built
            .workload()
            .evidence_ledger()
            .rows()
            .iter()
            .all(|row| row.is_receipt_backed()));

        let error = manually_substituted_topology_ledger(built.workload().evidence_ledger())
            .expect("static fixture ledger should still have valid row shape")
            .certify_complete()
            .expect_err("manual topology evidence must not certify as complete");

        assert_eq!(
            error,
            WorkloadEvidenceLedgerError::ManualAuthorityStage(WorkloadEvidenceStage::Topology)
        );
        assert_eq!(
            error.human_reason(),
            "workload evidence ledger has hand-filled topology evidence instead of a source receipt"
        );
    }

    fn admitted_catalog_recipes() -> Vec<WorkloadCatalogRecipe> {
        vec![
            WorkloadCatalog::cube(),
            WorkloadCatalog::tetrahedron(),
            WorkloadCatalog::single_face_loop(),
            WorkloadCatalog::coplanar_overlap_storm(),
            WorkloadCatalog::thin_feature_wall(),
            WorkloadCatalog::high_valence_vertex(),
            WorkloadCatalog::open_sheet(),
            WorkloadCatalog::transform_cycle(),
            WorkloadCatalog::retained_cancellation_chain(),
        ]
    }

    fn assert_authority_stage(
        ledger: &CompleteWorkloadEvidenceLedger,
        stage: WorkloadEvidenceStage,
    ) {
        let row = ledger
            .row_for_stage(stage)
            .expect("catalog ledger must include authority stage");

        assert!(row.is_receipt_backed());
        assert!(row.is_admitted());
        assert!(!row.evidence_identity().trim().is_empty());
    }

    fn assert_real_geometry_breadth(ledger: &CompleteWorkloadEvidenceLedger) {
        let topology = ledger
            .row_for_stage(WorkloadEvidenceStage::Topology)
            .expect("topology row");
        assert!(topology.counters().topology_entity_count() > 0);
        assert!(topology.counters().topology_relation_count() > 0);

        let binding = ledger
            .row_for_stage(WorkloadEvidenceStage::GeometryBinding)
            .expect("binding row");
        assert!(binding.counters().binding_target_count() > 0);

        let projection = ledger
            .row_for_stage(WorkloadEvidenceStage::Projection)
            .expect("projection row");
        assert!(projection.counters().projected_entity_count() > 0);
        assert!(projection.counters().local_basis_part_count() > 0);

        let transform = ledger
            .row_for_stage(WorkloadEvidenceStage::Transform)
            .expect("transform row");
        assert!(transform.counters().transform_step_count() > 0);
    }

    fn assert_catalog_query_receipt_is_digest(digest: &str) {
        assert!(!digest.trim().is_empty());
        assert!(!digest.contains("catalog"));
        assert!(!digest.contains("workload"));
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct CatalogBreadth {
        topology_entities: usize,
        topology_relations: usize,
        binding_targets: usize,
        projected_entities: usize,
        retained_artifacts: usize,
        replay_checkpoints: usize,
    }

    fn catalog_breadth(recipe: WorkloadCatalogRecipe) -> CatalogBreadth {
        let built = recipe.build().expect("catalog recipe should build");
        let ledger = built.workload().evidence_ledger();
        let topology = ledger
            .row_for_stage(WorkloadEvidenceStage::Topology)
            .expect("topology row")
            .counters();
        let binding = ledger
            .row_for_stage(WorkloadEvidenceStage::GeometryBinding)
            .expect("binding row")
            .counters();
        let projection = ledger
            .row_for_stage(WorkloadEvidenceStage::Projection)
            .expect("projection row")
            .counters();
        let replay = ledger
            .row_for_stage(WorkloadEvidenceStage::RetainedReplay)
            .expect("retained replay row")
            .counters();

        CatalogBreadth {
            topology_entities: topology.topology_entity_count(),
            topology_relations: topology.topology_relation_count(),
            binding_targets: binding.binding_target_count(),
            projected_entities: projection.projected_entity_count(),
            retained_artifacts: replay.retained_artifact_count(),
            replay_checkpoints: replay.replay_checkpoint_count(),
        }
    }

    fn manually_substituted_topology_ledger(
        ledger: &CompleteWorkloadEvidenceLedger,
    ) -> Result<WorkloadEvidenceLedger, WorkloadEvidenceLedgerError> {
        let rows = ledger
            .rows()
            .iter()
            .map(|row| {
                if row.stage() == WorkloadEvidenceStage::Topology {
                    WorkloadEvidenceRow::new(
                        WorkloadEvidenceStage::Topology,
                        row.evidence_identity(),
                    )
                } else {
                    row.clone()
                }
            })
            .collect();
        WorkloadEvidenceLedger::from_rows(rows)
    }
}
