#[cfg(test)]
mod tests {
    use worth_kernel::workload_composition::{
        WorkloadCatalog, WorkloadCatalogError, WorkloadCatalogRecipe,
        WorkloadCatalogSupportPosture, WorkloadTopologyBreadth,
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
    fn workload_catalog_reports_open_sheet_without_bounded_operator_admission() {
        let open_sheet = WorkloadCatalog::open_sheet();
        let support = open_sheet
            .inspect_support()
            .expect("open sheet support should be inspectable");

        assert_eq!(
            support.posture(),
            WorkloadCatalogSupportPosture::Unsupported
        );
        assert!(support.human_reason().contains("open sheet"));
        assert!(support.human_reason().contains("not yet supported"));
        assert_catalog_query_receipt_is_digest(support.query_support_digest());

        match open_sheet
            .build()
            .expect_err("open sheet must not enter the bounded workload build lane")
        {
            WorkloadCatalogError::UnsupportedRecipe { recipe, reason } => {
                assert_eq!(recipe.human_name(), "open sheet workload recipe");
                assert!(reason.contains("open sheet"));
                assert!(reason.contains("not yet supported"));
            }
            other => panic!("expected unsupported open sheet recipe, got {other:?}"),
        }
    }

    #[test]
    fn dirty_catalog_recipe_builds_topology_backed_clean_fail_lane() {
        let built = WorkloadCatalog::dirty_self_intersecting_loop()
            .declared("MB-M6-5 dirty self-intersecting clean-fail catalog")
            .build_clean_fail()
            .expect("dirty catalog recipe should build clean-fail evidence");

        assert_eq!(
            built.support().posture(),
            WorkloadCatalogSupportPosture::Admitted
        );
        assert!(built
            .support()
            .human_reason()
            .contains("clean-fail evidence"));
        assert_eq!(
            built.topology_clean_fail().kind().as_str(),
            "self-intersecting-loop"
        );
        assert!(!built.topology_clean_fail().can_enter_spatial_binding());
        assert!(built
            .topology_clean_fail()
            .reason()
            .contains("spatial binding"));
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
    fn catalog_recipe_can_require_retained_replay_artifacts_for_parity_workloads() {
        for recipe in [
            WorkloadCatalog::cube(),
            WorkloadCatalog::thin_feature_wall(),
            WorkloadCatalog::coplanar_overlap_storm(),
        ] {
            let built = recipe
                .with_retained_replay_artifacts()
                .build()
                .expect("parity catalog recipe should build retained replay artifacts");
            let replay = built
                .workload()
                .evidence_ledger()
                .row_for_stage(WorkloadEvidenceStage::RetainedReplay)
                .expect("retained replay evidence row");

            assert!(replay.counters().retained_artifact_count() > 0);
            assert!(replay.counters().replay_checkpoint_count() > 0);
        }
    }

    #[test]
    fn catalog_hostile_shapes_do_not_collapse_to_cube_fixture() {
        let cube = catalog_breadth(WorkloadCatalog::cube());
        let tetrahedron = catalog_breadth(WorkloadCatalog::tetrahedron());
        let single_face_loop = catalog_breadth(WorkloadCatalog::single_face_loop());
        let thin_feature_wall = catalog_breadth(WorkloadCatalog::thin_feature_wall());
        let high_valence_vertex = catalog_breadth(WorkloadCatalog::high_valence_vertex());
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
        assert!(retained_chain.retained_artifacts > 0);
        assert!(retained_chain.replay_checkpoints > 0);
    }

    #[test]
    fn coplanar_overlap_storm_topology_breadth_is_explicit_and_receipt_backed() {
        let default = catalog_breadth(WorkloadCatalog::coplanar_overlap_storm());
        let smaller_storm = catalog_breadth(
            WorkloadCatalog::coplanar_overlap_storm()
                .with_topology_breadth(WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 }),
        );

        assert_eq!(default.topology_faces, 64);
        assert_eq!(smaller_storm.topology_faces, 8);
        assert!(default.topology_entities > smaller_storm.topology_entities);
        assert!(default.topology_relations > smaller_storm.topology_relations);
        assert!(default.projected_entities > smaller_storm.projected_entities);
    }

    #[test]
    fn high_valence_topology_breadth_is_explicit_and_neighborhood_backed() {
        let default = WorkloadCatalog::high_valence_vertex()
            .build()
            .expect("default high-valence recipe should build");
        let upper_admitted_boundary = WorkloadCatalog::high_valence_vertex()
            .with_topology_breadth(WorkloadTopologyBreadth::HighValenceVertex { valence: 16 })
            .build()
            .expect("upper admitted high-valence boundary should build");

        assert_eq!(
            default
                .topology_neighborhood()
                .expect("default neighborhood")
                .valence(),
            5
        );
        assert_eq!(
            upper_admitted_boundary
                .topology_neighborhood()
                .expect("upper boundary neighborhood")
                .valence(),
            16
        );
        assert!(
            catalog_breadth(
                WorkloadCatalog::high_valence_vertex().with_topology_breadth(
                    WorkloadTopologyBreadth::HighValenceVertex { valence: 16 }
                )
            )
            .topology_entities
                > catalog_breadth(WorkloadCatalog::high_valence_vertex()).topology_entities
        );
    }

    #[test]
    fn high_valence_unsupported_breadth_denies_before_workload_construction() {
        for valence in [2, 17, 32] {
            let unsupported = WorkloadCatalog::high_valence_vertex()
                .with_topology_breadth(WorkloadTopologyBreadth::HighValenceVertex { valence });
            let support = unsupported
                .inspect_support()
                .expect("unsupported high-valence support should still be inspectable");

            assert_eq!(
                support.posture(),
                WorkloadCatalogSupportPosture::Unsupported
            );
            assert_eq!(
                support.human_reason(),
                format!(
                    "high valence vertex workload recipe supports valence 3 through 16 today; valence {valence} needs an explicit widening phase"
                )
            );

            match unsupported
                .build()
                .expect_err("unsupported high-valence breadth must deny before workload build")
            {
                WorkloadCatalogError::UnsupportedRecipe { recipe, reason } => {
                    assert_eq!(recipe.human_name(), "high valence vertex workload recipe");
                    assert!(reason.contains("valence 3 through 16"));
                    assert!(reason.contains(&format!("valence {valence}")));
                }
                other => panic!("expected unsupported high-valence breadth, got {other:?}"),
            }
        }
    }

    #[test]
    fn explicit_multi_face_breadth_is_not_a_general_fixture_knob() {
        let error = WorkloadCatalog::cube()
            .with_topology_breadth(WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 })
            .build()
            .expect_err("cube recipe must not accept coplanar storm breadth");

        match error {
            WorkloadCatalogError::UnsupportedRecipe { recipe, reason } => {
                assert_eq!(recipe.human_name(), "cube workload recipe");
                assert!(reason.contains("multi-face shell breadth"));
                assert!(reason.contains("coplanar overlap storm"));
            }
            other => panic!("expected unsupported breadth override, got {other:?}"),
        }
    }

    #[test]
    fn explicit_high_valence_breadth_is_not_a_general_fixture_knob() {
        let error = WorkloadCatalog::cube()
            .with_topology_breadth(WorkloadTopologyBreadth::HighValenceVertex { valence: 8 })
            .build()
            .expect_err("cube recipe must not accept high-valence breadth");

        match error {
            WorkloadCatalogError::UnsupportedRecipe { recipe, reason } => {
                assert_eq!(recipe.human_name(), "cube workload recipe");
                assert!(reason.contains("high-valence vertex breadth"));
                assert!(reason.contains("high-valence vertex recipe"));
            }
            other => panic!("expected unsupported breadth override, got {other:?}"),
        }
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
        let topology = stage_row(ledger, WorkloadEvidenceStage::Topology);
        assert!(topology.counters().topology_entity_count() > 0);
        assert!(topology.counters().topology_relation_count() > 0);

        let binding = stage_row(ledger, WorkloadEvidenceStage::GeometryBinding);
        assert!(binding.counters().binding_target_count() > 0);

        let projection = stage_row(ledger, WorkloadEvidenceStage::Projection);
        assert!(projection.counters().projected_entity_count() > 0);
        assert!(projection.counters().local_basis_part_count() > 0);

        let transform = stage_row(ledger, WorkloadEvidenceStage::Transform);
        assert!(transform.counters().transform_step_count() > 0);
    }

    fn assert_catalog_query_receipt_is_digest(digest: &str) {
        assert!(!digest.trim().is_empty());
        assert!(!digest.contains("catalog"));
        assert!(!digest.contains("workload"));
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct CatalogBreadth {
        topology_faces: usize,
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
        let topology = stage_row(ledger, WorkloadEvidenceStage::Topology).counters();
        let binding = stage_row(ledger, WorkloadEvidenceStage::GeometryBinding).counters();
        let projection = stage_row(ledger, WorkloadEvidenceStage::Projection).counters();
        let replay = stage_row(ledger, WorkloadEvidenceStage::RetainedReplay).counters();

        CatalogBreadth {
            topology_faces: topology.topology_face_count(),
            topology_entities: topology.topology_entity_count(),
            topology_relations: topology.topology_relation_count(),
            binding_targets: binding.binding_target_count(),
            projected_entities: projection.projected_entity_count(),
            retained_artifacts: replay.retained_artifact_count(),
            replay_checkpoints: replay.replay_checkpoint_count(),
        }
    }

    fn stage_row(
        ledger: &CompleteWorkloadEvidenceLedger,
        stage: WorkloadEvidenceStage,
    ) -> &WorkloadEvidenceRow {
        ledger.row_for_stage(stage).expect("catalog stage row")
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
