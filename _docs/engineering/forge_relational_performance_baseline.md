# Forge Relational Perf Summary

- Cases: 77
- Metrics: 427
- Compared baseline: yes

## Top Regressions

| Suite | Case | Median Delta (us) | Current Median | Baseline Median | Likely Owner |
| --- | --- | ---: | ---: | ---: | --- |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | +111712 | 3359582 | 3247870 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | +68528 | 4133324 | 4064796 | general runtime surface |
| sustained_load_matrix | rocketship_hot_update_endurance | +27406 | 463841 | 436435 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | +15710 | 4054058 | 4038348 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | +13509 | 4062560 | 4049051 | general runtime surface |

| Suite | Case | Metric | Median Delta | Current Median | Baseline Median | Likely Owner |
| --- | --- | --- | ---: | ---: | ---: | --- |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | bootstrap_relation_commit_micros | +108628 | 2691609 | 2582981 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | bootstrap_relation_commit_micros | +60352 | 2628878 | 2568526 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_relation_commit_micros | +25772 | 2636761 | 2610989 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | bootstrap_relation_commit_micros | +18553 | 1943810 | 1925257 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_entity_commit_micros | +14544 | 1419819 | 1405275 | general runtime surface |

## Diagnostic Hotspots

### Phase regressions

| Suite | Case | Metric | Median Delta | Current Median | Baseline Median | Likely Owner |
| --- | --- | --- | ---: | ---: | ---: | --- |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | bootstrap_relation_commit_micros | +108628 | 2691609 | 2582981 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | bootstrap_relation_commit_micros | +60352 | 2628878 | 2568526 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_relation_commit_micros | +25772 | 2636761 | 2610989 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | bootstrap_relation_commit_micros | +18553 | 1943810 | 1925257 | general runtime surface |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_entity_commit_micros | +14544 | 1419819 | 1405275 | general runtime surface |

### Packet inflation

No packet inflation against the comparison baseline.

### Scope inflation

No scope inflation against the comparison baseline.

### Observability inflation

| Suite | Case | Metric | Median Delta | Current Median | Baseline Median | Likely Owner |
| --- | --- | --- | ---: | ---: | ---: | --- |
| commit_delta_matrix | persisted_single_entity_create | artifact_assembly_micros | +2 | 13 | 11 | durability/authority |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | artifact_assembly_micros | +2 | 9 | 7 | general runtime surface |

### Profile drift

No profile drift against the comparison baseline.

### Owner Radar

| Likely Owner | Aggregate Median Delta |
| --- | ---: |
| general runtime surface | +473954 |
| diagnostics/profile + publication | +177 |
| workflow integration + durability/replay | +100 |
| merge/facade + durability | +22 |
| durability/authority | +6 |

## Case Summaries

| Suite | Case | Mean (us) | Median (us) | Delta Mean (us) | Delta Median (us) | Samples |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | 3410.67 | 3420 | +306.67 | +316.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | 2779.67 | 2483 | +79.67 | -217.00 | 3 |
| cad_topology_matrix | assembly_interface_bridge_wave | 99.00 | 90 | -49.00 | -58.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | 65.33 | 69 | -0.67 | +3.00 | 3 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | 2362.00 | 2283 | -31.00 | -110.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | 299.00 | 298 | -47.00 | -48.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | 562.33 | 560 | +1.33 | -1.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | 867.33 | 864 | -113.67 | -117.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_window | 1272.00 | 1178 | +7.00 | -87.00 | 3 |
| commit_delta_matrix | cross_partition_relation_burst | 682.67 | 680 | +73.67 | +71.00 | 3 |
| commit_delta_matrix | persisted_single_entity_create | 647.33 | 577 | -157.67 | -228.00 | 3 |
| commit_delta_matrix | single_partition_create_burst | 875.00 | 875 | -153.00 | -153.00 | 3 |
| durability_append_matrix | append_canonical_envelope_existing_segment | 581.33 | 578 | -170.67 | -174.00 | 3 |
| durability_append_matrix | append_canonical_envelope_fresh_store | 525.33 | 523 | -68.67 | -71.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | 3308888.67 | 3359582 | +61018.67 | +111712.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | 84.00 | 78 | +14.00 | +8.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | 63.33 | 63 | -4.67 | -5.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | 60.33 | 65 | +1.33 | +6.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | 2800.00 | 2223 | -1568.00 | -2145.00 | 3 |
| harness_measurement_matrix | post_measurement_metrics_do_not_pollute_elapsed | 0.00 | 0 | 0.00 | 0.00 | 3 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | 2966.33 | 2753 | +153.33 | -60.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | 2817.33 | 2775 | -49.67 | -92.00 | 3 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | 2399.33 | 2280 | -5445.67 | -5565.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | 2444.67 | 2373 | +270.67 | +199.00 | 3 |
| index_parity_matrix | entity_field_equals_build_failed_fallback | 8.33 | 8 | -3.67 | -4.00 | 3 |
| index_parity_matrix | entity_field_equals_warm_generation | 41.00 | 34 | -26.00 | -33.00 | 3 |
| index_parity_matrix | persisted_recovery_generation_parity | 5602.67 | 5459 | +656.67 | +513.00 | 3 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | 15.33 | 13 | -6.67 | -9.00 | 3 |
| inspection_budget_matrix | retention_commit_window | 3.67 | 2 | -7.33 | -9.00 | 3 |
| inspection_budget_matrix | structural_identity_historical_window | 8.67 | 6 | -7.33 | -10.00 | 3 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | 304.00 | 328 | +78.00 | +102.00 | 3 |
| merge_lineage_matrix | lineage_branch_divergence_breadth | 7.33 | 5 | -8.67 | -11.00 | 3 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | 428.67 | 425 | +15.67 | +12.00 | 3 |
| merge_lineage_matrix | merge_execution_feature_adoption | 435.33 | 434 | -81.67 | -83.00 | 3 |
| merge_lineage_matrix | merge_execution_feature_adoption_zero_diagnostics_budget | 435.00 | 421 | -6.00 | -20.00 | 3 |
| merge_lineage_matrix | merge_execution_vs_persisted_commit_floor | 442.67 | 440 | +18.67 | +16.00 | 3 |
| merge_lineage_matrix | merge_planning_divergent_update | 93.00 | 78 | -82.00 | -97.00 | 3 |
| merge_lineage_matrix | merge_prepare_vs_execute_feature_adoption | 565.67 | 562 | +65.67 | +62.00 | 3 |
| merge_lineage_matrix | merge_verify_vs_execute_feature_adoption | 500.00 | 469 | +23.00 | -8.00 | 3 |
| mixed_load_matrix | concurrent_relation_index_parity_pressure | 387.33 | 376 | +102.33 | +91.00 | 3 |
| mixed_load_matrix | concurrent_snapshot_version_read_pressure | 342.67 | 352 | +5.67 | +15.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | 390.67 | 389 | -54.33 | -56.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | 290.00 | 276 | +103.00 | +89.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | 210.67 | 196 | -20.33 | -35.00 | 3 |
| query_packet_matrix | connectivity_traversal_cross_partition | 92.67 | 87 | -56.33 | -62.00 | 3 |
| query_packet_matrix | entity_kind_scan_partition_matrix | 105.33 | 105 | -12.67 | -13.00 | 3 |
| query_packet_matrix | explicit_targets_cross_partition | 130.67 | 131 | -25.33 | -25.00 | 3 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | 868.33 | 868 | -98.67 | -99.00 | 3 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | 842.33 | 828 | -32.67 | -47.00 | 3 |
| replay_recovery_matrix | checkpoint_recover_suffix_replay | 225.33 | 221 | -6.67 | -11.00 | 3 |
| replay_recovery_matrix | durable_replay_lineage_basis | 254.00 | 250 | -26.00 | -30.00 | 3 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | 51.67 | 6 | +45.67 | 0.00 | 3 |
| retention_reclaim_matrix | snapshot_release_to_reclaimable_entity | 4.33 | 3 | -1.67 | -3.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | 3070553.33 | 3004474 | -42883.67 | -108963.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | 4033094.33 | 4054058 | -5253.67 | +15710.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | 4110406.33 | 4133324 | +45610.33 | +68528.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | 4028194.33 | 4062560 | -20856.67 | +13509.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | 3081274.00 | 3022778 | +2887.00 | -55609.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | 40.00 | 40 | -8.00 | -8.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | 143.00 | 140 | -11.00 | -14.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | 145.00 | 125 | +23.00 | +3.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | 177.33 | 184 | +10.33 | +17.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | 47.00 | 43 | -20.00 | -24.00 | 3 |
| snapshot_materialization_matrix | projection_entity_identity_surface | 29.00 | 26 | -15.00 | -18.00 | 3 |
| snapshot_materialization_matrix | snapshot_read_view_current | 19.33 | 19 | +0.33 | 0.00 | 3 |
| snapshot_materialization_matrix | version_read_view_historical | 28.33 | 27 | -3.67 | -5.00 | 3 |
| sustained_load_matrix | commit_query_churn_stability | 42488.67 | 42284 | -1844.33 | -2049.00 | 3 |
| sustained_load_matrix | mixed_topology_query_churn_stability | 4526.33 | 4526 | -409.67 | -410.00 | 3 |
| sustained_load_matrix | replay_window_drift_stability | 123793.33 | 123919 | +350.33 | +476.00 | 3 |
| sustained_load_matrix | retention_pass_drift_stability | 93.67 | 91 | -3.33 | -6.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | 462237.00 | 463841 | +25802.00 | +27406.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | 257104.33 | 257618 | +2483.33 | +2997.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | 131.00 | 125 | -24.00 | -30.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | 170.00 | 165 | -21.00 | -26.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | 1785.67 | 1807 | -1301.33 | -1280.00 | 3 |
| workflow_matrix | retention_release_reclaim_round_trip | 11.00 | 10 | -2.00 | -3.00 | 3 |
| workflow_matrix | trade_correction_analysis_round_trip | 1535.67 | 1437 | -531.33 | -630.00 | 3 |

## Metric Summaries

| Suite | Case | Metric | Mean | Median | Delta Mean | Delta Median | Samples |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | checkpoint_micros | 1701.67 | 1715 | +124.67 | +138.00 | 3 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | cold_compile_micros | 6.00 | 6 | +1.00 | +1.00 | 3 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | cold_compiled_record_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | hot_commit_micros | 803.67 | 680 | +58.67 | -65.00 | 3 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | hot_compile_micros | 7.00 | 7 | -9.00 | -9.00 | 3 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | hot_compiled_record_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | recover_micros | 401.00 | 392 | +27.00 | +18.00 | 3 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | replay_commit_micros | 491.33 | 481 | +104.33 | +94.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | checkpoint_micros | 1660.33 | 1398 | +210.33 | -52.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_commit_micros | 756.00 | 800 | +33.00 | +77.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_detailed_trace_artifact_count | 0.00 | 0 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_detailed_trace_entry_count | 0.00 | 0 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_summary_entry_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_total_artifacts | 18.00 | 18 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_total_entries | 18.00 | 18 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | recover_micros | 191.00 | 189 | -68.00 | -70.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | recovered_summary_entry_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | replay_commit_micros | 172.33 | 155 | -95.67 | -113.00 | 3 |
| cad_topology_matrix | assembly_interface_bridge_wave | bridge_commit_micros | 57.67 | 57 | -1.33 | -2.00 | 3 |
| cad_topology_matrix | assembly_interface_bridge_wave | connectivity_summary_micros | 19.33 | 16 | -18.67 | -22.00 | 3 |
| cad_topology_matrix | assembly_interface_bridge_wave | explicit_query_entities | 6.00 | 6 | 0.00 | 0.00 | 3 |
| cad_topology_matrix | assembly_interface_bridge_wave | explicit_query_micros | 22.00 | 17 | -29.00 | -34.00 | 3 |
| cad_topology_matrix | assembly_interface_bridge_wave | largest_component_size | 12.00 | 12 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | adjacency_micros | 1.33 | 1 | +0.33 | 0.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | commit_micros | 56.33 | 53 | +8.33 | +5.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | committed_changed_records | 1.00 | 1 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | compile_micros | 3.33 | 3 | +0.33 | 0.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | outgoing_relation_count | 8.00 | 8 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | rollback_discarded_creations | 8.00 | 8 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | rollback_effect_count | 8.00 | 8 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | branch_rollback_compile_step_window | rollback_micros | 4.33 | 2 | -9.67 | -12.00 | 3 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | adjacency_micros | 3.67 | 4 | +0.67 | +1.00 | 3 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | checkpoint_micros | 1863.67 | 1852 | -35.33 | -47.00 | 3 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | compile_micros | 5.67 | 5 | -0.33 | -1.00 | 3 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | outgoing_relation_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | recover_micros | 489.00 | 472 | +4.00 | -13.00 | 3 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | recovered_segment_count | 0.00 | 0 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | adjacency_micros | 5.00 | 5 | +1.00 | +1.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | changed_records | 24.00 | 24 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | commit_micros | 276.67 | 271 | -41.33 | -47.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | compile_micros | 17.33 | 16 | -6.67 | -8.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | dense_patch_record_count | 24.00 | 24 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | diagnostic_artifact_count | 53.00 | 53 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | outgoing_relation_count | 24.00 | 24 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | profile_diagnostics_boundary_code | 1.00 | 1 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | profile_execution_lane_code | 1.00 | 1 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | adjacency_micros | 5.00 | 5 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | changed_records | 24.00 | 24 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | commit_micros | 541.67 | 535 | +13.67 | +7.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | compile_micros | 15.67 | 14 | -12.33 | -14.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | dense_patch_record_count | 24.00 | 24 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | detailed_trace_entries | 332.00 | 332 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | diagnostic_artifact_count | 307.00 | 307 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | outgoing_relation_count | 24.00 | 24 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | profile_diagnostics_boundary_code | 1.00 | 1 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | profile_execution_lane_code | 1.00 | 1 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | profile_matches_defaults | 0.00 | 0 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | average_adjacency_micros | 2.00 | 2 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | average_compile_micros | 2.00 | 2 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | average_update_micros | 49.67 | 50 | -7.33 | -7.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | diagnostic_artifact_count | 114.00 | 114 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | max_compile_micros | 2.67 | 3 | -0.33 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | max_outgoing_relation_count | 16.00 | 16 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_window | average_adjacency_micros | 2.00 | 2 | 0.00 | 0.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_window | average_compile_micros | 1.33 | 1 | -0.67 | -1.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_window | average_update_micros | 48.67 | 45 | +0.67 | -3.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_window | max_compile_micros | 4.33 | 3 | -3.67 | -5.00 | 3 |
| chip_simulator_matrix | event_wave_compile_churn_window | max_outgoing_relation_count | 16.00 | 16 | 0.00 | 0.00 | 3 |
| commit_delta_matrix | persisted_single_entity_create | artifact_assembly_micros | 14.00 | 13 | +3.00 | +2.00 | 3 |
| commit_delta_matrix | persisted_single_entity_create | authoritative_mutation_micros | 19.67 | 21 | +2.67 | +4.00 | 3 |
| commit_delta_matrix | persisted_single_entity_create | durable_append_micros | 575.67 | 509 | -163.33 | -230.00 | 3 |
| commit_delta_matrix | persisted_single_entity_create | publication_micros | 11.67 | 10 | -1.33 | -3.00 | 3 |
| commit_delta_matrix | persisted_single_entity_create | working_state_preparation_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_count_total | 18.00 | 18 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_entry_count_total | 402.00 | 402 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_kind_detailed_trace_count | 0.00 | 0 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_kind_minimal_summary_count | 18.00 | 18 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_scope_count_distinct | 2.00 | 2 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | explicit_query_micros | 120.67 | 138 | -41.33 | -24.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | hot_update_micros | 11834.67 | 11650 | +306.67 | +122.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | resident_relation_count | 107307.00 | 107307 | 0.00 | 0.00 | 3 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | subsystem_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | bridge_commit_micros | 67.33 | 61 | +14.33 | +8.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | component_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | connectivity_summary_micros | 16.67 | 17 | -0.33 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | enumerated_entity_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | largest_component_size | 12.00 | 12 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | bridge_commit_micros | 49.67 | 50 | -1.33 | -1.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | component_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | connectivity_summary_micros | 13.67 | 13 | -3.33 | -4.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | diagnostic_artifact_count | 46.00 | 46 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | enumerated_entity_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | largest_component_size | 12.00 | 12 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | bridge_commit_micros | 46.33 | 49 | +3.33 | +6.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | component_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | connectivity_summary_micros | 14.00 | 14 | -2.00 | -2.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | diagnostic_artifact_count | 23.00 | 23 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | enumerated_entity_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | largest_component_size | 12.00 | 12 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | checkpoint_micros | 1715.00 | 1164 | -1206.00 | -1757.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | lineage_resolution_micros | 3.00 | 2 | -15.00 | -16.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | recover_micros | 137.67 | 130 | +26.67 | +19.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | recovered_lineage_resolution_micros | 1.67 | 2 | +0.67 | +1.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | resolved_lineage_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | traversed_event_count | 0.00 | 0 | 0.00 | 0.00 | 3 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | update_commit_micros | 942.67 | 872 | -374.33 | -445.00 | 3 |
| harness_measurement_matrix | post_measurement_metrics_do_not_pollute_elapsed | measurement_build_micros | 5748.33 | 5372 | -1284.67 | -1661.00 | 3 |
| harness_measurement_matrix | post_measurement_metrics_do_not_pollute_elapsed | measurement_item_count | 20000.00 | 20000 | 0.00 | 0.00 | 3 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | checkpoint_micros | 1680.00 | 1496 | +214.00 | +30.00 | 3 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | cold_compile_micros | 5.00 | 5 | -2.00 | -2.00 | 3 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | hot_commit_micros | 689.33 | 668 | +26.33 | +5.00 | 3 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | hot_compile_micros | 6.67 | 7 | +0.67 | +1.00 | 3 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | recover_micros | 295.33 | 266 | -29.67 | -59.00 | 3 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | replay_commit_micros | 290.00 | 289 | -56.00 | -57.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | checkpoint_micros | 1474.00 | 1500 | -27.00 | -1.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | cold_compile_micros | 5.67 | 5 | +1.67 | +1.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_commit_micros | 621.33 | 620 | -0.67 | -2.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_compile_micros | 6.00 | 6 | -1.00 | -1.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_detailed_trace_entries | 195.00 | 195 | 0.00 | 0.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_diagnostic_artifact_count | 197.00 | 197 | 0.00 | 0.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | recover_micros | 345.00 | 309 | -74.00 | -110.00 | 3 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | replay_commit_micros | 365.33 | 333 | +51.33 | +19.00 | 3 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | checkpoint_micros | 1400.00 | 1299 | +108.00 | +7.00 | 3 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | cold_query_micros | 15.67 | 14 | +1.67 | 0.00 | 3 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | hot_commit_micros | 667.33 | 700 | -5530.67 | -5498.00 | 3 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | hot_query_micros | 14.00 | 14 | -12.00 | -12.00 | 3 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | recover_micros | 168.67 | 156 | +36.67 | +24.00 | 3 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | replay_commit_micros | 133.67 | 127 | -49.33 | -56.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | artifact_assembly_micros | 8.67 | 9 | +1.67 | +2.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | checkpoint_micros | 1320.33 | 1262 | +111.33 | +53.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | cold_query_micros | 17.00 | 17 | +3.00 | +3.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | durable_append_micros | 611.67 | 604 | +41.67 | +34.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_commit_micros | 689.67 | 684 | +53.67 | +48.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_diagnostic_artifact_count | 18.00 | 18 | 0.00 | 0.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_query_micros | 15.00 | 15 | 0.00 | 0.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | publication_micros | 18.67 | 18 | +0.67 | 0.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | recover_micros | 220.33 | 225 | +51.33 | +56.00 | 3 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | replay_commit_micros | 182.33 | 194 | +51.33 | +63.00 | 3 |
| index_parity_matrix | entity_field_equals_build_failed_fallback | entity_result_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| index_parity_matrix | entity_field_equals_build_failed_fallback | query_micros | 8.33 | 8 | -3.67 | -4.00 | 3 |
| index_parity_matrix | entity_field_equals_warm_generation | build_micros | 14.67 | 11 | -6.33 | -10.00 | 3 |
| index_parity_matrix | entity_field_equals_warm_generation | entity_result_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| index_parity_matrix | entity_field_equals_warm_generation | query_micros | 26.33 | 23 | -19.67 | -23.00 | 3 |
| index_parity_matrix | persisted_recovery_generation_parity | entity_result_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| index_parity_matrix | persisted_recovery_generation_parity | query_micros | 20.67 | 20 | +2.67 | +2.00 | 3 |
| index_parity_matrix | persisted_recovery_generation_parity | recover_micros | 5582.00 | 5439 | +654.00 | +511.00 | 3 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | connectivity_component_count | 3.00 | 3 | 0.00 | 0.00 | 3 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | connectivity_micros | 5.33 | 5 | -0.67 | -1.00 | 3 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | graph_entity_count | 4.00 | 4 | 0.00 | 0.00 | 3 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | graph_micros | 3.67 | 2 | -4.33 | -6.00 | 3 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | kind_micros | 6.33 | 6 | -1.67 | -2.00 | 3 |
| inspection_budget_matrix | retention_commit_window | commit_micros | 0.67 | 0 | -2.33 | -3.00 | 3 |
| inspection_budget_matrix | retention_commit_window | recent_commit_count | 3.00 | 3 | 0.00 | 0.00 | 3 |
| inspection_budget_matrix | retention_commit_window | recent_micros | 2.33 | 2 | -0.67 | -1.00 | 3 |
| inspection_budget_matrix | retention_commit_window | retention_micros | 0.67 | 0 | -4.33 | -5.00 | 3 |
| inspection_budget_matrix | structural_identity_historical_window | direct_micros | 0.67 | 0 | +0.67 | 0.00 | 3 |
| inspection_budget_matrix | structural_identity_historical_window | historical_micros | 5.00 | 3 | -8.00 | -10.00 | 3 |
| inspection_budget_matrix | structural_identity_historical_window | query_match_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| inspection_budget_matrix | structural_identity_historical_window | query_micros | 3.00 | 3 | 0.00 | 0.00 | 3 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | authoritative_mutation_micros | 19.67 | 21 | +4.67 | +6.00 | 3 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | invariant_post_check_micros | 4.33 | 5 | +0.33 | +1.00 | 3 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | invariant_pre_check_micros | 36.67 | 41 | +9.67 | +14.00 | 3 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | artifact_assembly_micros | 16.33 | 16 | +0.33 | 0.00 | 3 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | authoritative_mutation_micros | 18.00 | 18 | +2.00 | +2.00 | 3 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | durable_append_micros | 302.00 | 298 | +12.00 | +8.00 | 3 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | history_resolution_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | publication_micros | 20.00 | 20 | -1.00 | -1.00 | 3 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | working_state_preparation_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| mixed_load_matrix | concurrent_relation_index_parity_pressure | matched_relation_count | 0.00 | 0 | 0.00 | 0.00 | 3 |
| mixed_load_matrix | concurrent_relation_index_parity_pressure | reader_count | 8.00 | 8 | 0.00 | 0.00 | 3 |
| mixed_load_matrix | concurrent_snapshot_version_read_pressure | reader_count | 8.00 | 8 | 0.00 | 0.00 | 3 |
| mixed_load_matrix | concurrent_snapshot_version_read_pressure | visibility_cache_hits | 24.00 | 24 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | commit_micros | 362.00 | 360 | -52.00 | -54.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | detailed_trace_entries | 57.00 | 57 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | diagnostic_artifact_count | 56.00 | 56 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | packet_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | query_micros | 28.67 | 29 | -2.33 | -2.00 | 3 |
| profile_matrix | certification_core_rich_commit_query_round_trip | scope_unit_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | commit_micros | 258.67 | 244 | +91.67 | +77.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | diagnostic_artifact_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | packet_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | profile_matches_defaults | 0.00 | 0 | 0.00 | 0.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | query_micros | 31.33 | 31 | +11.33 | +11.00 | 3 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | scope_unit_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | commit_micros | 183.33 | 175 | -22.67 | -31.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | diagnostic_artifact_count | 2.00 | 2 | 0.00 | 0.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | packet_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | query_micros | 27.33 | 23 | +2.33 | -2.00 | 3 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | scope_unit_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| query_packet_matrix | connectivity_traversal_cross_partition | execution_micros | 92.67 | 87 | -56.33 | -62.00 | 3 |
| query_packet_matrix | connectivity_traversal_cross_partition | packet_count | 3.00 | 3 | 0.00 | 0.00 | 3 |
| query_packet_matrix | connectivity_traversal_cross_partition | planning_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| query_packet_matrix | connectivity_traversal_cross_partition | scope_unit_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| query_packet_matrix | entity_kind_scan_partition_matrix | execution_micros | 105.33 | 105 | -12.67 | -13.00 | 3 |
| query_packet_matrix | entity_kind_scan_partition_matrix | packet_count | 4.00 | 4 | 0.00 | 0.00 | 3 |
| query_packet_matrix | entity_kind_scan_partition_matrix | scope_unit_count | 4.00 | 4 | 0.00 | 0.00 | 3 |
| query_packet_matrix | explicit_targets_cross_partition | execution_micros | 124.33 | 119 | -7.67 | -13.00 | 3 |
| query_packet_matrix | explicit_targets_cross_partition | packet_count | 4.00 | 4 | 0.00 | 0.00 | 3 |
| query_packet_matrix | explicit_targets_cross_partition | planning_micros | 6.33 | 4 | -17.67 | -20.00 | 3 |
| query_packet_matrix | explicit_targets_cross_partition | scope_unit_count | 4.00 | 4 | 0.00 | 0.00 | 3 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | cold_compile_micros | 4.33 | 4 | +0.33 | 0.00 | 3 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | hot_commit_micros | 688.00 | 687 | -4.00 | -5.00 | 3 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | hot_compile_micros | 6.00 | 6 | -10.00 | -10.00 | 3 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | must_be_hot_changed_records | 1.00 | 1 | 0.00 | 0.00 | 3 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | reconstructable_compiled_record_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | replay_commit_micros | 170.00 | 171 | -85.00 | -84.00 | 3 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | deferred_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | hot_commit_micros | 712.00 | 699 | -7.00 | -20.00 | 3 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | must_be_hot_changed_records | 1.00 | 1 | 0.00 | 0.00 | 3 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | reconstructable_summary_entries | 1.00 | 1 | 0.00 | 0.00 | 3 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | replay_commit_micros | 130.33 | 130 | -25.67 | -26.00 | 3 |
| replay_recovery_matrix | checkpoint_recover_suffix_replay | recovery_micros | 122.33 | 120 | +1.33 | -1.00 | 3 |
| replay_recovery_matrix | checkpoint_recover_suffix_replay | replay_commit_micros | 103.00 | 103 | -8.00 | -8.00 | 3 |
| replay_recovery_matrix | durable_replay_lineage_basis | replay_commit_micros | 254.00 | 250 | -26.00 | -30.00 | 3 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | inspect_pinned_micros | 2.33 | 2 | -0.67 | -1.00 | 3 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | inspect_released_micros | 47.33 | 2 | +46.33 | +1.00 | 3 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | release_replay_pin_micros | 2.00 | 1 | 0.00 | -1.00 | 3 |
| retention_reclaim_matrix | snapshot_release_to_reclaimable_entity | inspect_plan_micros | 4.00 | 3 | 0.00 | -1.00 | 3 |
| retention_reclaim_matrix | snapshot_release_to_reclaimable_entity | run_pass_micros | 0.33 | 0 | -1.67 | -2.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | bootstrap_entity_commit_micros | 1112812.00 | 1099667 | -64563.00 | -77708.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | bootstrap_relation_commit_micros | 1946847.00 | 1943810 | +21590.00 | +18553.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | diagnostic_artifact_count | 18.00 | 18 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | hot_query_execution_micros | 331.00 | 332 | +4.00 | +5.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | hot_query_planning_micros | 3.00 | 3 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | hot_update_micros | 10560.33 | 10567 | +85.33 | +92.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | query_result_entities | 256.00 | 256 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | query_target_count | 256.00 | 256 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | resident_relation_count | 101561.00 | 101561 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_entity_commit_micros | 1383941.00 | 1419819 | -21334.00 | +14544.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_relation_commit_micros | 2627363.00 | 2636761 | +16374.00 | +25772.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | diagnostic_artifact_count | 18.00 | 18 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | draft_preparation_micros | 2068.33 | 2108 | +47.33 | +87.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | explicit_query_micros | 55.33 | 55 | +3.33 | +3.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | explicit_result_entities | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | explicit_target_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | hot_update_micros | 21520.00 | 21584 | -301.00 | -237.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_execution_micros | 215.00 | 214 | +4.00 | +3.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_planning_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_result_entities | 28.00 | 28 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_result_relations | 27.00 | 27 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_seed_count | 4.00 | 4 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | resident_relation_count | 107307.00 | 107307 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | subsystem_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | bootstrap_entity_commit_micros | 1343012.67 | 1306282 | -117840.33 | -154571.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | bootstrap_relation_commit_micros | 2745549.00 | 2691609 | +162568.00 | +108628.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | draft_preparation_micros | 2158.00 | 2091 | +187.00 | +120.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | explicit_query_micros | 55.67 | 54 | +4.67 | +3.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | explicit_result_entities | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | explicit_target_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | hot_update_micros | 21570.67 | 22041 | +865.67 | +1336.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_execution_micros | 216.00 | 216 | +10.00 | +10.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_planning_micros | 2.33 | 0 | +2.33 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_result_entities | 28.00 | 28 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_result_relations | 27.00 | 27 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_seed_count | 4.00 | 4 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | resident_relation_count | 107307.00 | 107307 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | subsystem_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | bootstrap_entity_commit_micros | 1344371.33 | 1313400 | -115049.67 | -146021.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | bootstrap_relation_commit_micros | 2662770.00 | 2628878 | +94244.00 | +60352.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | explicit_query_execution_micros | 138.00 | 138 | -18.00 | -18.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | explicit_query_planning_micros | 4.33 | 4 | -0.67 | -1.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | explicit_query_result_entities | 36.00 | 36 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | hot_update_micros | 20550.33 | 20428 | -43.67 | -166.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | mixed_query_target_count | 36.00 | 36 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | profile_matches_defaults | 0.00 | 0 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | resident_relation_count | 107307.00 | 107307 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | subsystem_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_execution_micros | 360.33 | 358 | +11.33 | +9.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_planning_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_result_entities | 48.00 | 48 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_result_relations | 41.00 | 41 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_seed_count | 12.00 | 12 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | bootstrap_entity_commit_micros | 1120664.67 | 1115871 | -40917.33 | -45711.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | bootstrap_relation_commit_micros | 1948834.33 | 1900303 | +43151.33 | -5380.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | detailed_trace_entries | 0.00 | 0 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | diagnostic_artifact_count | 9.00 | 9 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | draft_preparation_micros | 2583.33 | 2477 | -123.67 | -230.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | hot_query_execution_micros | 395.33 | 391 | +44.33 | +40.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | hot_query_planning_micros | 3.67 | 3 | +0.67 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | hot_update_micros | 11376.00 | 10797 | +608.00 | +29.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | profile_matches_defaults | 0.00 | 0 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | query_result_entities | 256.00 | 256 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | query_target_count | 256.00 | 256 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | resident_relation_count | 101561.00 | 101561 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | affected_bridge_sources | 3.00 | 3 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_history_entries | 3.00 | 3 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_nodes_evaluated | 14.00 | 14 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_nodes_recomputed | 10.00 | 10 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_tasks_scheduled | 7.00 | 7 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | relational_commit_micros | 30.33 | 30 | -3.67 | -4.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | relational_query_micros | 9.67 | 10 | -4.33 | -4.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | affected_bridge_sources | 16.00 | 16 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | bridge_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | bridge_nodes_recomputed | 49.00 | 49 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | bridge_tasks_scheduled | 33.00 | 33 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | relational_commit_micros | 85.67 | 84 | +2.67 | +1.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | relational_query_micros | 57.33 | 56 | -13.67 | -15.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | resident_entities | 24.00 | 24 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | affected_bridge_sources | 16.00 | 16 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | bridge_micros | 0.33 | 0 | +0.33 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | bridge_nodes_recomputed | 49.00 | 49 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | bridge_tasks_scheduled | 33.00 | 33 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | relational_commit_micros | 59.67 | 59 | -2.33 | -3.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | relational_query_micros | 85.00 | 62 | +25.00 | +2.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | resident_entities | 24.00 | 24 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | affected_bridge_sources | 15.00 | 15 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | bridge_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | bridge_tasks_scheduled | 31.00 | 31 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | explicit_result_entities | 4.00 | 4 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | relational_commit_micros | 73.00 | 76 | +5.00 | +8.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | relational_query_micros | 104.33 | 108 | +5.33 | +9.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | traversal_result_entities | 11.00 | 11 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | affected_bridge_sources | 3.00 | 3 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_history_entries | 1.00 | 1 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_nodes_evaluated | 10.00 | 10 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_nodes_recomputed | 10.00 | 10 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_tasks_scheduled | 7.00 | 7 | 0.00 | 0.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | relational_commit_micros | 30.33 | 29 | -2.67 | -4.00 | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | relational_query_micros | 16.67 | 14 | -17.33 | -20.00 | 3 |
| sustained_load_matrix | commit_query_churn_stability | average_commit_micros | 328.33 | 327 | -12.67 | -14.00 | 3 |
| sustained_load_matrix | commit_query_churn_stability | average_query_micros | 3.00 | 3 | -1.00 | -1.00 | 3 |
| sustained_load_matrix | commit_query_churn_stability | final_entity_count | 128.00 | 128 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | commit_query_churn_stability | max_query_packets_per_iteration | 1.00 | 1 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | commit_query_churn_stability | max_query_scope_units_per_iteration | 1.00 | 1 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | mixed_topology_query_churn_stability | average_explicit_query_micros | 6.00 | 6 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | mixed_topology_query_churn_stability | average_traversal_micros | 14.00 | 14 | -1.00 | -1.00 | 3 |
| sustained_load_matrix | mixed_topology_query_churn_stability | average_update_micros | 73.00 | 73 | -7.00 | -7.00 | 3 |
| sustained_load_matrix | mixed_topology_query_churn_stability | max_packets_per_iteration | 3.00 | 3 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | mixed_topology_query_churn_stability | max_scope_units_per_iteration | 3.00 | 3 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | replay_window_drift_stability | average_replay_micros | 3868.00 | 3872 | +11.00 | +15.00 | 3 |
| sustained_load_matrix | replay_window_drift_stability | max_replay_micros | 7912.00 | 7815 | +265.00 | +168.00 | 3 |
| sustained_load_matrix | replay_window_drift_stability | replayed_commit_count | 32.00 | 32 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | replay_window_drift_stability | total_compared_surface_count | 192.00 | 192 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | replay_window_drift_stability | total_reconstructed_commit_closure | 1040.00 | 1040 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | retention_pass_drift_stability | average_inspect_micros | 1.33 | 1 | -0.67 | -1.00 | 3 |
| sustained_load_matrix | retention_pass_drift_stability | average_run_pass_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | retention_pass_drift_stability | max_reclaimable_entities | 48.00 | 48 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | retention_pass_drift_stability | total_entity_reclaimable | 1176.00 | 1176 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | retention_pass_drift_stability | total_entity_reclaimed | 0.00 | 0 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | average_update_micros | 1805.00 | 1811 | +101.00 | +107.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | first_window_average_update_micros | 2140.00 | 2108 | +73.00 | +41.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | iterations | 256.00 | 256 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | last_window_average_update_micros | 1841.33 | 1778 | +105.33 | +42.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | max_explicit_query_micros | 188.33 | 85 | +119.33 | +16.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | max_update_micros | 19107.00 | 19172 | +579.00 | +644.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_hot_update_endurance | resident_relation_count | 107307.00 | 107307 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | average_explicit_query_micros | 54.67 | 55 | +0.67 | +1.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | average_propagation_micros | 206.00 | 207 | +2.00 | +3.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | average_update_micros | 2415.67 | 2420 | +22.67 | +27.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | first_window_average_cycle_micros | 3429.00 | 3415 | +133.00 | +119.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | iterations | 96.00 | 96 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | last_window_average_cycle_micros | 2630.67 | 2629 | +27.67 | +26.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | max_packets_per_iteration | 12.00 | 12 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | max_scope_units_per_iteration | 15.00 | 15 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | resident_node_count | 100000.00 | 100000 | 0.00 | 0.00 | 3 |
| sustained_load_matrix | rocketship_propagation_endurance | resident_relation_count | 107307.00 | 107307 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | diagnostic_artifact_delta | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | packet_count | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | profile_diagnostics_boundary_code | 3.00 | 3 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | profile_execution_lane_code | 3.00 | 3 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | query_probe_micros | 35.33 | 33 | -12.67 | -15.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | scope_unit_count | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | stress_commit_micros | 95.67 | 92 | -11.33 | -15.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | audit_commit_micros | 65.00 | 65 | -8.00 | -8.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | correction_commit_micros | 74.67 | 70 | -6.33 | -11.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | diagnostic_artifact_delta | 4.00 | 4 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | packet_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | profile_diagnostics_boundary_code | 3.00 | 3 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | profile_execution_lane_code | 3.00 | 3 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | query_probe_micros | 30.33 | 30 | -6.67 | -7.00 | 3 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | scope_unit_count | 1.00 | 1 | 0.00 | 0.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | checkpoint_micros | 1197.33 | 1216 | -1391.67 | -1373.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | post_checkpoint_commit_micros | 306.33 | 312 | +32.33 | +38.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | post_recovery_query_micros | 14.33 | 14 | +2.33 | +2.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | recover_micros | 146.33 | 151 | +35.33 | +40.00 | 3 |
| workflow_matrix | persisted_recovery_replay_round_trip | replay_commit_micros | 121.33 | 121 | +20.33 | +20.00 | 3 |
| workflow_matrix | retention_release_reclaim_round_trip | inspect_plan_micros | 3.00 | 3 | -1.00 | -1.00 | 3 |
| workflow_matrix | retention_release_reclaim_round_trip | post_reclaim_query_micros | 8.00 | 8 | -1.00 | -1.00 | 3 |
| workflow_matrix | retention_release_reclaim_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | retention_release_reclaim_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | retention_release_reclaim_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| workflow_matrix | retention_release_reclaim_round_trip | run_pass_micros | 0.00 | 0 | 0.00 | 0.00 | 3 |
| workflow_matrix | trade_correction_analysis_round_trip | analysis_commit_micros | 936.33 | 836 | -510.67 | -611.00 | 3 |
| workflow_matrix | trade_correction_analysis_round_trip | merge_execute_micros | 582.67 | 585 | -20.33 | -18.00 | 3 |
| workflow_matrix | trade_correction_analysis_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | trade_correction_analysis_round_trip | profile_execution_lane_code | 2.00 | 2 | 0.00 | 0.00 | 3 |
| workflow_matrix | trade_correction_analysis_round_trip | profile_matches_defaults | 1.00 | 1 | 0.00 | 0.00 | 3 |
| workflow_matrix | trade_correction_analysis_round_trip | query_round_trip_micros | 16.67 | 16 | -0.33 | -1.00 | 3 |
