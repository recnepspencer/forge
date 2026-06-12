#[cfg(test)]
mod tests {
    use self::catalog_contract_support::{
        admitted_catalog_recipes, assert_authority_stage, assert_catalog_query_receipt_is_digest,
        assert_real_geometry_breadth, catalog_breadth,
    };
    use worth_kernel::workload_composition::{
        WorkloadCatalog, WorkloadCatalogError, WorkloadCatalogSupportPosture,
        WorkloadTopologyBreadth,
    };
    use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

    #[path = "catalog_contract_support.rs"]
    mod catalog_contract_support;
    #[path = "nmt_construction.rs"]
    mod nmt_construction;
    #[path = "static_fixture_substitution.rs"]
    mod static_fixture_substitution;

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
            .with_topology_breadth(WorkloadTopologyBreadth::HighValenceVertex { valence: 128 })
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
            128
        );
        assert!(
            catalog_breadth(
                WorkloadCatalog::high_valence_vertex().with_topology_breadth(
                    WorkloadTopologyBreadth::HighValenceVertex { valence: 128 }
                )
            )
            .topology_entities
                > catalog_breadth(WorkloadCatalog::high_valence_vertex()).topology_entities
        );
    }

    #[test]
    fn high_valence_unsupported_breadth_denies_before_workload_construction() {
        for valence in [2, 129] {
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
                    "high valence vertex workload recipe supports valence 3 through 128 today; valence {valence} needs an explicit widening phase"
                )
            );

            match unsupported
                .build()
                .expect_err("unsupported high-valence breadth must deny before workload build")
            {
                WorkloadCatalogError::UnsupportedRecipe { recipe, reason } => {
                    assert_eq!(recipe.human_name(), "high valence vertex workload recipe");
                    assert!(reason.contains("valence 3 through 128"));
                    assert!(reason.contains(&format!("valence {valence}")));
                }
                other => panic!("expected unsupported high-valence breadth, got {other:?}"),
            }
        }
    }

    #[test]
    fn mixed_surface_kill_box_catalog_is_named_stable_topology_carrier() {
        let built = WorkloadCatalog::mixed_surface_kill_box()
            .declared("MB-M6-NMT-2 mixed surface kill box carrier")
            .build()
            .expect("mixed surface kill box carrier should build");
        let cube = catalog_breadth(WorkloadCatalog::cube());
        let carrier = catalog_breadth(WorkloadCatalog::mixed_surface_kill_box());

        assert_eq!(
            built.recipe().human_name(),
            "mixed surface kill box workload recipe"
        );
        assert_eq!(
            built.declaration().recipe().query_key(),
            "worth.catalog.mixed_surface_kill_box"
        );
        assert_eq!(carrier.topology_faces, cube.topology_faces);
        assert_eq!(carrier.topology_entities, cube.topology_entities);
        assert_eq!(carrier.topology_relations, cube.topology_relations);
        assert!(carrier.retained_artifacts > 0);
        assert!(carrier.replay_checkpoints > 0);
    }

    #[test]
    fn open_class_triad_catalog_builds_distinct_open_members() {
        let triad = WorkloadCatalog::open_class_triad(128)
            .declared("MB-M6-NMT-3 catalog triad")
            .build()
            .expect("open class triad catalog should build");

        assert_eq!(
            triad.wire().recipe().human_name(),
            "open wire workload recipe"
        );
        assert_eq!(
            triad.sheet().recipe().human_name(),
            "open sheet workload recipe"
        );
        assert_eq!(
            triad.fan().recipe().human_name(),
            "open shell NMT edge fan workload recipe"
        );
        assert_ne!(
            triad
                .wire()
                .topology_construction()
                .expect("wire topology")
                .pattern_identity()
                .identity_digest(),
            triad
                .sheet()
                .topology_construction()
                .expect("sheet topology")
                .pattern_identity()
                .identity_digest()
        );
        assert_eq!(
            triad
                .fan()
                .topology_construction()
                .expect("fan topology")
                .counters()
                .face_count(),
            128
        );
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
}
