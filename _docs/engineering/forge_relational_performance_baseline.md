# Forge Relational Perf Summary

- Cases: 72
- Metrics: 385
- Compared baseline: no

## Case Summaries

| Suite | Case | Mean (us) | Median (us) | Delta Mean (us) | Delta Median (us) | Samples |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | 7218.00 | 7218 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | 2890.00 | 2890 |  |  | 1 |
| cad_topology_matrix | assembly_interface_bridge_wave | 206.00 | 206 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | 74.00 | 74 |  |  | 1 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | 2196.00 | 2196 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | 460.00 | 460 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | 617.00 | 617 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | 1284.00 | 1284 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_window | 1729.00 | 1729 |  |  | 1 |
| commit_delta_matrix | cross_partition_relation_burst | 700.00 | 700 |  |  | 1 |
| commit_delta_matrix | persisted_single_entity_create | 759.00 | 759 |  |  | 1 |
| commit_delta_matrix | single_partition_create_burst | 1403.00 | 1403 |  |  | 1 |
| durability_append_matrix | append_canonical_envelope_existing_segment | 765.00 | 765 |  |  | 1 |
| durability_append_matrix | append_canonical_envelope_fresh_store | 550.00 | 550 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | 8732441.00 | 8732441 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | 108.00 | 108 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | 98.00 | 98 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | 78.00 | 78 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | 3039.00 | 3039 |  |  | 1 |
| harness_measurement_matrix | post_measurement_metrics_do_not_pollute_elapsed | 0.00 | 0 |  |  | 1 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | 2781.00 | 2781 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | 2867.00 | 2867 |  |  | 1 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | 8053.00 | 8053 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | 2362.00 | 2362 |  |  | 1 |
| index_parity_matrix | entity_field_equals_build_failed_fallback | 13.00 | 13 |  |  | 1 |
| index_parity_matrix | entity_field_equals_warm_generation | 84.00 | 84 |  |  | 1 |
| index_parity_matrix | persisted_recovery_generation_parity | 5170.00 | 5170 |  |  | 1 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | 20.00 | 20 |  |  | 1 |
| inspection_budget_matrix | retention_commit_window | 12.00 | 12 |  |  | 1 |
| inspection_budget_matrix | structural_identity_historical_window | 15.00 | 15 |  |  | 1 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | 253.00 | 253 |  |  | 1 |
| merge_lineage_matrix | lineage_branch_divergence_breadth | 10.00 | 10 |  |  | 1 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | 422.00 | 422 |  |  | 1 |
| merge_lineage_matrix | merge_execution_feature_adoption | 449.00 | 449 |  |  | 1 |
| merge_lineage_matrix | merge_execution_feature_adoption_zero_diagnostics_budget | 397.00 | 397 |  |  | 1 |
| merge_lineage_matrix | merge_execution_vs_persisted_commit_floor | 437.00 | 437 |  |  | 1 |
| merge_lineage_matrix | merge_planning_divergent_update | 138.00 | 138 |  |  | 1 |
| merge_lineage_matrix | merge_prepare_vs_execute_feature_adoption | 490.00 | 490 |  |  | 1 |
| merge_lineage_matrix | merge_verify_vs_execute_feature_adoption | 417.00 | 417 |  |  | 1 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | 75.00 | 75 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | 203.33 | 196 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | 201.00 | 187 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | 214.67 | 208 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | 94.00 | 81 |  |  | 3 |
| mixed_load_matrix | concurrent_relation_index_parity_pressure | 301.00 | 301 |  |  | 1 |
| mixed_load_matrix | concurrent_snapshot_version_read_pressure | 380.00 | 380 |  |  | 1 |
| profile_matrix | certification_core_rich_commit_query_round_trip | 671.40 | 573 |  |  | 5 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | 227.20 | 228 |  |  | 5 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | 422.60 | 362 |  |  | 5 |
| query_packet_matrix | connectivity_traversal_cross_partition | 112.00 | 112 |  |  | 1 |
| query_packet_matrix | entity_kind_scan_partition_matrix | 202.00 | 202 |  |  | 1 |
| query_packet_matrix | explicit_targets_cross_partition | 161.00 | 161 |  |  | 1 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | 875.00 | 875 |  |  | 1 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | 1160.00 | 1160 |  |  | 1 |
| replay_recovery_matrix | checkpoint_recover_suffix_replay | 625.00 | 625 |  |  | 1 |
| replay_recovery_matrix | durable_replay_lineage_basis | 357.00 | 357 |  |  | 1 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | 6.00 | 6 |  |  | 1 |
| retention_reclaim_matrix | snapshot_release_to_reclaimable_entity | 7.00 | 7 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | 8574181.00 | 8574181 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | 10159146.00 | 10159146 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | 9471160.00 | 9471160 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | 9469406.00 | 9469406 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | 7768674.00 | 7768674 |  |  | 1 |
| snapshot_materialization_matrix | projection_entity_identity_surface | 27.00 | 27 |  |  | 1 |
| snapshot_materialization_matrix | snapshot_read_view_current | 40.00 | 40 |  |  | 1 |
| snapshot_materialization_matrix | version_read_view_historical | 36.00 | 36 |  |  | 1 |
| sustained_load_matrix | commit_query_churn_stability | 49441.00 | 49441 |  |  | 1 |
| sustained_load_matrix | mixed_topology_query_churn_stability | 9764.00 | 9764 |  |  | 1 |
| sustained_load_matrix | replay_window_drift_stability | 148282.00 | 148282 |  |  | 1 |
| sustained_load_matrix | retention_pass_drift_stability | 119.00 | 119 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | 212.00 | 212 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | 354.00 | 354 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | 1831.00 | 1831 |  |  | 1 |
| workflow_matrix | retention_release_reclaim_round_trip | 19.00 | 19 |  |  | 1 |
| workflow_matrix | trade_correction_analysis_round_trip | 1932.00 | 1932 |  |  | 1 |

## Metric Summaries

| Suite | Case | Metric | Mean | Median | Delta Mean | Delta Median | Samples |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | checkpoint_micros | 1609.00 | 1609 |  |  | 1 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | cold_compile_micros | 6.00 | 6 |  |  | 1 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | cold_compiled_record_count | 1.00 | 1 |  |  | 1 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | hot_commit_micros | 4675.00 | 4675 |  |  | 1 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | hot_compile_micros | 21.00 | 21 |  |  | 1 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | hot_compiled_record_count | 1.00 | 1 |  |  | 1 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | recover_micros | 397.00 | 397 |  |  | 1 |
| artifact_recoverability_matrix | chip_compiled_artifact_recoverability | replay_commit_micros | 510.00 | 510 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | checkpoint_micros | 1547.00 | 1547 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_commit_micros | 926.00 | 926 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_detailed_trace_artifact_count | 0.00 | 0 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_detailed_trace_entry_count | 0.00 | 0 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_summary_entry_count | 1.00 | 1 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_total_artifacts | 18.00 | 18 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | hot_total_entries | 18.00 | 18 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | recover_micros | 197.00 | 197 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | recovered_summary_entry_count | 1.00 | 1 |  |  | 1 |
| artifact_recoverability_matrix | geometry_diagnostics_summary_vs_trace_recoverability | replay_commit_micros | 220.00 | 220 |  |  | 1 |
| cad_topology_matrix | assembly_interface_bridge_wave | bridge_commit_micros | 110.00 | 110 |  |  | 1 |
| cad_topology_matrix | assembly_interface_bridge_wave | connectivity_summary_micros | 45.00 | 45 |  |  | 1 |
| cad_topology_matrix | assembly_interface_bridge_wave | explicit_query_entities | 6.00 | 6 |  |  | 1 |
| cad_topology_matrix | assembly_interface_bridge_wave | explicit_query_micros | 51.00 | 51 |  |  | 1 |
| cad_topology_matrix | assembly_interface_bridge_wave | largest_component_size | 12.00 | 12 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | adjacency_micros | 1.00 | 1 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | commit_micros | 67.00 | 67 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | committed_changed_records | 1.00 | 1 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | compile_micros | 4.00 | 4 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | outgoing_relation_count | 8.00 | 8 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | rollback_discarded_creations | 8.00 | 8 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | rollback_effect_count | 8.00 | 8 |  |  | 1 |
| chip_simulator_matrix | branch_rollback_compile_step_window | rollback_micros | 2.00 | 2 |  |  | 1 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | adjacency_micros | 3.00 | 3 |  |  | 1 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | checkpoint_micros | 1663.00 | 1663 |  |  | 1 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | compile_micros | 6.00 | 6 |  |  | 1 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | outgoing_relation_count | 12.00 | 12 |  |  | 1 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | recover_micros | 524.00 | 524 |  |  | 1 |
| chip_simulator_matrix | checkpoint_window_recover_compile_round_trip | recovered_segment_count | 0.00 | 0 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | adjacency_micros | 4.00 | 4 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | changed_records | 24.00 | 24 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | commit_micros | 440.00 | 440 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | compile_micros | 16.00 | 16 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | dense_patch_record_count | 24.00 | 24 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | diagnostic_artifact_count | 53.00 | 53 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | outgoing_relation_count | 24.00 | 24 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | profile_diagnostics_boundary_code | 1.00 | 1 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | profile_execution_lane_code | 1.00 | 1 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave | profile_matches_defaults | 1.00 | 1 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | adjacency_micros | 5.00 | 5 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | changed_records | 24.00 | 24 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | commit_micros | 596.00 | 596 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | compile_micros | 16.00 | 16 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | dense_patch_record_count | 24.00 | 24 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | detailed_trace_entries | 332.00 | 332 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | diagnostic_artifact_count | 307.00 | 307 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | outgoing_relation_count | 24.00 | 24 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | profile_diagnostics_boundary_code | 1.00 | 1 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | profile_execution_lane_code | 1.00 | 1 |  |  | 1 |
| chip_simulator_matrix | dense_fanout_compile_wave_rich_diagnostics | profile_matches_defaults | 0.00 | 0 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | average_adjacency_micros | 2.00 | 2 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | average_compile_micros | 2.00 | 2 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | average_update_micros | 76.00 | 76 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | diagnostic_artifact_count | 114.00 | 114 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | max_compile_micros | 3.00 | 3 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_rich_diagnostics | max_outgoing_relation_count | 16.00 | 16 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_window | average_adjacency_micros | 2.00 | 2 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_window | average_compile_micros | 2.00 | 2 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_window | average_update_micros | 67.00 | 67 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_window | max_compile_micros | 7.00 | 7 |  |  | 1 |
| chip_simulator_matrix | event_wave_compile_churn_window | max_outgoing_relation_count | 16.00 | 16 |  |  | 1 |
| commit_delta_matrix | persisted_single_entity_create | artifact_assembly_micros | 14.00 | 14 |  |  | 1 |
| commit_delta_matrix | persisted_single_entity_create | authoritative_mutation_micros | 21.00 | 21 |  |  | 1 |
| commit_delta_matrix | persisted_single_entity_create | durable_append_micros | 671.00 | 671 |  |  | 1 |
| commit_delta_matrix | persisted_single_entity_create | publication_micros | 13.00 | 13 |  |  | 1 |
| commit_delta_matrix | persisted_single_entity_create | working_state_preparation_micros | 0.00 | 0 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_count_total | 32.00 | 32 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_entry_count_total | 11537.00 | 11537 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_kind_detailed_trace_count | 0.00 | 0 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_kind_minimal_summary_count | 32.00 | 32 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | artifact_scope_count_distinct | 2.00 | 2 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | explicit_query_micros | 171880.00 | 171880 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | hot_update_micros | 133878.00 | 133878 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | resident_node_count | 100000.00 | 100000 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | resident_relation_count | 107307.00 | 107307 |  |  | 1 |
| geometry_artifact_decomposition_matrix | hundred_k_nodes_pseudorealistic_rich_artifact_classes | subsystem_count | 12.00 | 12 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | bridge_commit_micros | 90.00 | 90 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | component_count | 1.00 | 1 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | connectivity_summary_micros | 18.00 | 18 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | enumerated_entity_count | 12.00 | 12 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave | largest_component_size | 12.00 | 12 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | bridge_commit_micros | 82.00 | 82 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | component_count | 1.00 | 1 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | connectivity_summary_micros | 16.00 | 16 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | diagnostic_artifact_count | 46.00 | 46 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | enumerated_entity_count | 12.00 | 12 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_rich_geometry_profile | largest_component_size | 12.00 | 12 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | bridge_commit_micros | 59.00 | 59 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | component_count | 1.00 | 1 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | connectivity_summary_micros | 19.00 | 19 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | diagnostic_artifact_count | 23.00 | 23 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | enumerated_entity_count | 12.00 | 12 |  |  | 1 |
| geometry_kernel_matrix | topology_bridge_connectivity_wave_zero_diagnostics | largest_component_size | 12.00 | 12 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | checkpoint_micros | 1706.00 | 1706 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | lineage_resolution_micros | 13.00 | 13 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | recover_micros | 112.00 | 112 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | recovered_lineage_resolution_micros | 1.00 | 1 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | resolved_lineage_count | 1.00 | 1 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | traversed_event_count | 0.00 | 0 |  |  | 1 |
| geometry_kernel_matrix | topology_identity_survival_recovery_round_trip | update_commit_micros | 1207.00 | 1207 |  |  | 1 |
| harness_measurement_matrix | post_measurement_metrics_do_not_pollute_elapsed | payload_build_micros | 6524.00 | 6524 |  |  | 1 |
| harness_measurement_matrix | post_measurement_metrics_do_not_pollute_elapsed | payload_item_count | 20000.00 | 20000 |  |  | 1 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | checkpoint_micros | 1476.00 | 1476 |  |  | 1 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | cold_compile_micros | 8.00 | 8 |  |  | 1 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | hot_commit_micros | 638.00 | 638 |  |  | 1 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | hot_compile_micros | 7.00 | 7 |  |  | 1 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | recover_micros | 341.00 | 341 |  |  | 1 |
| hot_cold_path_matrix | chip_hot_compile_vs_recovery_compile | replay_commit_micros | 311.00 | 311 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | checkpoint_micros | 1498.00 | 1498 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | cold_compile_micros | 5.00 | 5 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_commit_micros | 689.00 | 689 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_compile_micros | 6.00 | 6 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_detailed_trace_entries | 198.00 | 198 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | hot_diagnostic_artifact_count | 199.00 | 199 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | recover_micros | 343.00 | 343 |  |  | 1 |
| hot_cold_path_matrix | chip_rich_compile_hot_vs_recovery_compile | replay_commit_micros | 326.00 | 326 |  |  | 1 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | checkpoint_micros | 1634.00 | 1634 |  |  | 1 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | cold_query_micros | 18.00 | 18 |  |  | 1 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | hot_commit_micros | 6053.00 | 6053 |  |  | 1 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | hot_query_micros | 24.00 | 24 |  |  | 1 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | recover_micros | 140.00 | 140 |  |  | 1 |
| hot_cold_path_matrix | geometry_hot_commit_vs_replay_reconstruction | replay_commit_micros | 184.00 | 184 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | artifact_assembly_micros | 11.00 | 11 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | checkpoint_micros | 1311.00 | 1311 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | cold_query_micros | 17.00 | 17 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | durable_append_micros | 573.00 | 573 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_commit_micros | 667.00 | 667 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_diagnostic_artifact_count | 18.00 | 18 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | hot_query_micros | 16.00 | 16 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | publication_micros | 18.00 | 18 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | recover_micros | 198.00 | 198 |  |  | 1 |
| hot_cold_path_matrix | geometry_rich_publication_hot_vs_replay_truth | replay_commit_micros | 153.00 | 153 |  |  | 1 |
| index_parity_matrix | entity_field_equals_build_failed_fallback | entity_result_count | 1.00 | 1 |  |  | 1 |
| index_parity_matrix | entity_field_equals_build_failed_fallback | query_micros | 13.00 | 13 |  |  | 1 |
| index_parity_matrix | entity_field_equals_warm_generation | build_micros | 38.00 | 38 |  |  | 1 |
| index_parity_matrix | entity_field_equals_warm_generation | entity_result_count | 1.00 | 1 |  |  | 1 |
| index_parity_matrix | entity_field_equals_warm_generation | query_micros | 46.00 | 46 |  |  | 1 |
| index_parity_matrix | persisted_recovery_generation_parity | entity_result_count | 1.00 | 1 |  |  | 1 |
| index_parity_matrix | persisted_recovery_generation_parity | query_micros | 21.00 | 21 |  |  | 1 |
| index_parity_matrix | persisted_recovery_generation_parity | recover_micros | 5149.00 | 5149 |  |  | 1 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | connectivity_component_count | 3.00 | 3 |  |  | 1 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | connectivity_micros | 6.00 | 6 |  |  | 1 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | graph_entity_count | 4.00 | 4 |  |  | 1 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | graph_micros | 9.00 | 9 |  |  | 1 |
| inspection_budget_matrix | graph_kind_connectivity_bundle | kind_micros | 5.00 | 5 |  |  | 1 |
| inspection_budget_matrix | retention_commit_window | commit_micros | 3.00 | 3 |  |  | 1 |
| inspection_budget_matrix | retention_commit_window | recent_commit_count | 3.00 | 3 |  |  | 1 |
| inspection_budget_matrix | retention_commit_window | recent_micros | 3.00 | 3 |  |  | 1 |
| inspection_budget_matrix | retention_commit_window | retention_micros | 6.00 | 6 |  |  | 1 |
| inspection_budget_matrix | structural_identity_historical_window | direct_micros | 0.00 | 0 |  |  | 1 |
| inspection_budget_matrix | structural_identity_historical_window | historical_micros | 12.00 | 12 |  |  | 1 |
| inspection_budget_matrix | structural_identity_historical_window | query_match_count | 1.00 | 1 |  |  | 1 |
| inspection_budget_matrix | structural_identity_historical_window | query_micros | 3.00 | 3 |  |  | 1 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | authoritative_mutation_micros | 13.00 | 13 |  |  | 1 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | invariant_post_check_micros | 3.00 | 3 |  |  | 1 |
| invariant_materialization_matrix | custom_structural_surface_commit_wave | invariant_pre_check_micros | 26.00 | 26 |  |  | 1 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | artifact_assembly_micros | 16.00 | 16 |  |  | 1 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | authoritative_mutation_micros | 17.00 | 17 |  |  | 1 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | durable_append_micros | 300.00 | 300 |  |  | 1 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | history_resolution_micros | 0.00 | 0 |  |  | 1 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | publication_micros | 17.00 | 17 |  |  | 1 |
| merge_lineage_matrix | merge_execute_phase_timing_feature_adoption | working_state_preparation_micros | 0.00 | 0 |  |  | 1 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | affected_bridge_sources | 3.00 | 3 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_history_entries | 3.00 | 3 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_micros | 0.00 | 0 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_nodes_evaluated | 14.00 | 14 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_nodes_recomputed | 10.00 | 10 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | bridge_tasks_scheduled | 7.00 | 7 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | relational_commit_micros | 58.33 | 57 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_development | relational_query_micros | 16.67 | 18 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | affected_bridge_sources | 16.00 | 16 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | bridge_micros | 0.00 | 0 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | bridge_nodes_recomputed | 49.00 | 49 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | bridge_tasks_scheduled | 33.00 | 33 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | relational_commit_micros | 129.67 | 130 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | relational_query_micros | 73.67 | 66 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_development | resident_entities | 24.00 | 24 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | affected_bridge_sources | 16.00 | 16 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | bridge_micros | 1.00 | 1 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | bridge_nodes_recomputed | 49.00 | 49 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | bridge_tasks_scheduled | 33.00 | 33 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | relational_commit_micros | 127.33 | 110 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | relational_query_micros | 72.67 | 73 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_medium_region_operational | resident_entities | 24.00 | 24 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | affected_bridge_sources | 15.00 | 15 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | bridge_micros | 0.33 | 0 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | bridge_tasks_scheduled | 31.00 | 31 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | explicit_result_entities | 4.00 | 4 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | relational_commit_micros | 110.67 | 110 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | relational_query_micros | 103.67 | 97 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_mixed_locality_operational | traversal_result_entities | 11.00 | 11 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | affected_bridge_sources | 3.00 | 3 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_history_entries | 1.00 | 1 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_micros | 0.00 | 0 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_nodes_evaluated | 10.00 | 10 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_nodes_recomputed | 10.00 | 10 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | bridge_tasks_scheduled | 7.00 | 7 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | relational_commit_micros | 56.67 | 55 |  |  | 3 |
| runtime_bridge_mock_matrix | geometry_commit_bridge_wave_operational | relational_query_micros | 37.33 | 26 |  |  | 3 |
| mixed_load_matrix | concurrent_relation_index_parity_pressure | matched_relation_count | 0.00 | 0 |  |  | 1 |
| mixed_load_matrix | concurrent_relation_index_parity_pressure | reader_count | 8.00 | 8 |  |  | 1 |
| mixed_load_matrix | concurrent_snapshot_version_read_pressure | reader_count | 8.00 | 8 |  |  | 1 |
| mixed_load_matrix | concurrent_snapshot_version_read_pressure | visibility_cache_hits | 16.00 | 16 |  |  | 1 |
| profile_matrix | certification_core_rich_commit_query_round_trip | commit_micros | 620.80 | 534 |  |  | 5 |
| profile_matrix | certification_core_rich_commit_query_round_trip | detailed_trace_entries | 57.00 | 57 |  |  | 1 |
| profile_matrix | certification_core_rich_commit_query_round_trip | diagnostic_artifact_count | 56.00 | 56 |  |  | 1 |
| profile_matrix | certification_core_rich_commit_query_round_trip | packet_count | 1.00 | 1 |  |  | 1 |
| profile_matrix | certification_core_rich_commit_query_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 5 |
| profile_matrix | certification_core_rich_commit_query_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 5 |
| profile_matrix | certification_core_rich_commit_query_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 5 |
| profile_matrix | certification_core_rich_commit_query_round_trip | query_micros | 50.60 | 42 |  |  | 5 |
| profile_matrix | certification_core_rich_commit_query_round_trip | scope_unit_count | 1.00 | 1 |  |  | 1 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | commit_micros | 200.60 | 203 |  |  | 5 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | diagnostic_artifact_count | 1.00 | 1 |  |  | 1 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | packet_count | 1.00 | 1 |  |  | 1 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 5 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 5 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | profile_matches_defaults | 0.00 | 0 |  |  | 5 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | query_micros | 26.60 | 25 |  |  | 5 |
| profile_matrix | certification_core_zero_diagnostics_commit_query_round_trip | scope_unit_count | 1.00 | 1 |  |  | 1 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | commit_micros | 386.20 | 323 |  |  | 5 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | diagnostic_artifact_count | 2.00 | 2 |  |  | 1 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | packet_count | 1.00 | 1 |  |  | 1 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 5 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 5 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 5 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | query_micros | 36.40 | 39 |  |  | 5 |
| profile_matrix | geometry_kernel_rich_commit_query_round_trip | scope_unit_count | 1.00 | 1 |  |  | 1 |
| query_packet_matrix | connectivity_traversal_cross_partition | execution_micros | 112.00 | 112 |  |  | 1 |
| query_packet_matrix | connectivity_traversal_cross_partition | packet_count | 3.00 | 3 |  |  | 1 |
| query_packet_matrix | connectivity_traversal_cross_partition | planning_micros | 0.00 | 0 |  |  | 1 |
| query_packet_matrix | connectivity_traversal_cross_partition | scope_unit_count | 12.00 | 12 |  |  | 1 |
| query_packet_matrix | entity_kind_scan_partition_matrix | execution_micros | 202.00 | 202 |  |  | 1 |
| query_packet_matrix | entity_kind_scan_partition_matrix | packet_count | 4.00 | 4 |  |  | 1 |
| query_packet_matrix | entity_kind_scan_partition_matrix | scope_unit_count | 4.00 | 4 |  |  | 1 |
| query_packet_matrix | explicit_targets_cross_partition | execution_micros | 145.00 | 145 |  |  | 1 |
| query_packet_matrix | explicit_targets_cross_partition | packet_count | 4.00 | 4 |  |  | 1 |
| query_packet_matrix | explicit_targets_cross_partition | planning_micros | 16.00 | 16 |  |  | 1 |
| query_packet_matrix | explicit_targets_cross_partition | scope_unit_count | 4.00 | 4 |  |  | 1 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | cold_compile_micros | 4.00 | 4 |  |  | 1 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | hot_commit_micros | 685.00 | 685 |  |  | 1 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | hot_compile_micros | 6.00 | 6 |  |  | 1 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | must_be_hot_changed_records | 1.00 | 1 |  |  | 1 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | reconstructable_compiled_record_count | 1.00 | 1 |  |  | 1 |
| recoverability_policy_matrix | chip_compile_reconstructable_policy | replay_commit_micros | 180.00 | 180 |  |  | 1 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | deferred_trace_entries | 0.00 | 0 |  |  | 1 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | hot_commit_micros | 1007.00 | 1007 |  |  | 1 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | must_be_hot_changed_records | 1.00 | 1 |  |  | 1 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | reconstructable_summary_entries | 1.00 | 1 |  |  | 1 |
| recoverability_policy_matrix | geometry_hot_truth_vs_deferred_trace_policy | replay_commit_micros | 153.00 | 153 |  |  | 1 |
| replay_recovery_matrix | checkpoint_recover_suffix_replay | recovery_micros | 501.00 | 501 |  |  | 1 |
| replay_recovery_matrix | checkpoint_recover_suffix_replay | replay_commit_micros | 124.00 | 124 |  |  | 1 |
| replay_recovery_matrix | durable_replay_lineage_basis | replay_commit_micros | 357.00 | 357 |  |  | 1 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | inspect_pinned_micros | 2.00 | 2 |  |  | 1 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | inspect_released_micros | 2.00 | 2 |  |  | 1 |
| retention_reclaim_matrix | replay_pin_release_deleted_relation | release_replay_pin_micros | 2.00 | 2 |  |  | 1 |
| retention_reclaim_matrix | snapshot_release_to_reclaimable_entity | inspect_plan_micros | 7.00 | 7 |  |  | 1 |
| retention_reclaim_matrix | snapshot_release_to_reclaimable_entity | run_pass_micros | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | bootstrap_entity_commit_micros | 4419682.00 | 4419682 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | bootstrap_relation_commit_micros | 3897592.00 | 3897592 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | diagnostic_artifact_count | 30.00 | 30 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | hot_query_execution_micros | 129591.00 | 129591 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | hot_query_planning_micros | 3.00 | 3 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | hot_update_micros | 127313.00 | 127313 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | query_result_entities | 256.00 | 256 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | query_target_count | 256.00 | 256 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | resident_node_count | 100000.00 | 100000 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_narrow_round_trip | resident_relation_count | 101561.00 | 101561 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_entity_commit_micros | 4278417.00 | 4278417 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | bootstrap_relation_commit_micros | 5372820.00 | 5372820 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | diagnostic_artifact_count | 32.00 | 32 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | explicit_query_micros | 182222.00 | 182222 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | explicit_result_entities | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | explicit_target_count | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | hot_update_micros | 141416.00 | 141416 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_execution_micros | 184255.00 | 184255 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_planning_micros | 16.00 | 16 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_result_entities | 28.00 | 28 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_result_relations | 27.00 | 27 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | propagation_seed_count | 4.00 | 4 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | resident_node_count | 100000.00 | 100000 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | resident_relation_count | 107307.00 | 107307 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_geometry_profile_propagation_wave | subsystem_count | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | bootstrap_entity_commit_micros | 4203676.00 | 4203676 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | bootstrap_relation_commit_micros | 4800771.00 | 4800771 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | explicit_query_micros | 169661.00 | 169661 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | explicit_result_entities | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | explicit_target_count | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | hot_update_micros | 127423.00 | 127423 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_execution_micros | 169629.00 | 169629 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_planning_micros | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_result_entities | 28.00 | 28 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_result_relations | 27.00 | 27 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | propagation_seed_count | 4.00 | 4 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | resident_node_count | 100000.00 | 100000 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | resident_relation_count | 107307.00 | 107307 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_propagation_wave | subsystem_count | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | bootstrap_entity_commit_micros | 4182324.00 | 4182324 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | bootstrap_relation_commit_micros | 4819927.00 | 4819927 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | explicit_query_execution_micros | 172591.00 | 172591 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | explicit_query_planning_micros | 6.00 | 6 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | explicit_query_result_entities | 36.00 | 36 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | hot_update_micros | 124244.00 | 124244 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | mixed_query_target_count | 36.00 | 36 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | profile_matches_defaults | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | resident_node_count | 100000.00 | 100000 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | resident_relation_count | 107307.00 | 107307 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | subsystem_count | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_execution_micros | 170314.00 | 170314 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_planning_micros | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_result_entities | 48.00 | 48 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_result_relations | 41.00 | 41 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_pseudorealistic_subsystem_round_trip | traversal_seed_count | 12.00 | 12 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | bootstrap_entity_commit_micros | 3853713.00 | 3853713 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | bootstrap_relation_commit_micros | 3663922.00 | 3663922 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | detailed_trace_entries | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | diagnostic_artifact_count | 15.00 | 15 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | hot_query_execution_micros | 135336.00 | 135336 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | hot_query_planning_micros | 3.00 | 3 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | hot_update_micros | 115700.00 | 115700 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | profile_matches_defaults | 0.00 | 0 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | query_result_entities | 256.00 | 256 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | query_target_count | 256.00 | 256 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | resident_node_count | 100000.00 | 100000 |  |  | 1 |
| rocketship_scale_matrix | hundred_k_nodes_zero_diagnostics_narrow_round_trip | resident_relation_count | 101561.00 | 101561 |  |  | 1 |
| sustained_load_matrix | commit_query_churn_stability | average_commit_micros | 370.00 | 370 |  |  | 1 |
| sustained_load_matrix | commit_query_churn_stability | average_query_micros | 15.00 | 15 |  |  | 1 |
| sustained_load_matrix | commit_query_churn_stability | final_entity_count | 128.00 | 128 |  |  | 1 |
| sustained_load_matrix | commit_query_churn_stability | max_query_packets_per_iteration | 1.00 | 1 |  |  | 1 |
| sustained_load_matrix | commit_query_churn_stability | max_query_scope_units_per_iteration | 1.00 | 1 |  |  | 1 |
| sustained_load_matrix | mixed_topology_query_churn_stability | average_explicit_query_micros | 20.00 | 20 |  |  | 1 |
| sustained_load_matrix | mixed_topology_query_churn_stability | average_traversal_micros | 32.00 | 32 |  |  | 1 |
| sustained_load_matrix | mixed_topology_query_churn_stability | average_update_micros | 149.00 | 149 |  |  | 1 |
| sustained_load_matrix | mixed_topology_query_churn_stability | max_packets_per_iteration | 3.00 | 3 |  |  | 1 |
| sustained_load_matrix | mixed_topology_query_churn_stability | max_scope_units_per_iteration | 3.00 | 3 |  |  | 1 |
| sustained_load_matrix | replay_window_drift_stability | average_replay_micros | 4633.00 | 4633 |  |  | 1 |
| sustained_load_matrix | replay_window_drift_stability | max_replay_micros | 9309.00 | 9309 |  |  | 1 |
| sustained_load_matrix | replay_window_drift_stability | replayed_commit_count | 32.00 | 32 |  |  | 1 |
| sustained_load_matrix | replay_window_drift_stability | total_compared_surface_count | 192.00 | 192 |  |  | 1 |
| sustained_load_matrix | replay_window_drift_stability | total_reconstructed_commit_closure | 1040.00 | 1040 |  |  | 1 |
| sustained_load_matrix | retention_pass_drift_stability | average_inspect_micros | 2.00 | 2 |  |  | 1 |
| sustained_load_matrix | retention_pass_drift_stability | average_run_pass_micros | 0.00 | 0 |  |  | 1 |
| sustained_load_matrix | retention_pass_drift_stability | max_reclaimable_entities | 48.00 | 48 |  |  | 1 |
| sustained_load_matrix | retention_pass_drift_stability | total_entity_reclaimable | 1176.00 | 1176 |  |  | 1 |
| sustained_load_matrix | retention_pass_drift_stability | total_entity_reclaimed | 0.00 | 0 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | diagnostic_artifact_delta | 2.00 | 2 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | packet_count | 2.00 | 2 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | profile_diagnostics_boundary_code | 3.00 | 3 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | profile_execution_lane_code | 3.00 | 3 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | query_probe_micros | 58.00 | 58 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | scope_unit_count | 2.00 | 2 |  |  | 1 |
| workflow_matrix | fintech_intraday_risk_branch_round_trip | stress_commit_micros | 154.00 | 154 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | audit_commit_micros | 160.00 | 160 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | correction_commit_micros | 131.00 | 131 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | diagnostic_artifact_delta | 4.00 | 4 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | packet_count | 1.00 | 1 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | profile_diagnostics_boundary_code | 3.00 | 3 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | profile_execution_lane_code | 3.00 | 3 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | query_probe_micros | 63.00 | 63 |  |  | 1 |
| workflow_matrix | fintech_trade_correction_audit_round_trip | scope_unit_count | 1.00 | 1 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | checkpoint_micros | 1256.00 | 1256 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | post_checkpoint_commit_micros | 320.00 | 320 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | post_recovery_query_micros | 18.00 | 18 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | recover_micros | 104.00 | 104 |  |  | 1 |
| workflow_matrix | persisted_recovery_replay_round_trip | replay_commit_micros | 133.00 | 133 |  |  | 1 |
| workflow_matrix | retention_release_reclaim_round_trip | inspect_plan_micros | 4.00 | 4 |  |  | 1 |
| workflow_matrix | retention_release_reclaim_round_trip | post_reclaim_query_micros | 15.00 | 15 |  |  | 1 |
| workflow_matrix | retention_release_reclaim_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 1 |
| workflow_matrix | retention_release_reclaim_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 1 |
| workflow_matrix | retention_release_reclaim_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 1 |
| workflow_matrix | retention_release_reclaim_round_trip | run_pass_micros | 0.00 | 0 |  |  | 1 |
| workflow_matrix | trade_correction_analysis_round_trip | analysis_commit_micros | 1167.00 | 1167 |  |  | 1 |
| workflow_matrix | trade_correction_analysis_round_trip | merge_execute_micros | 745.00 | 745 |  |  | 1 |
| workflow_matrix | trade_correction_analysis_round_trip | profile_diagnostics_boundary_code | 2.00 | 2 |  |  | 1 |
| workflow_matrix | trade_correction_analysis_round_trip | profile_execution_lane_code | 2.00 | 2 |  |  | 1 |
| workflow_matrix | trade_correction_analysis_round_trip | profile_matches_defaults | 1.00 | 1 |  |  | 1 |
| workflow_matrix | trade_correction_analysis_round_trip | query_round_trip_micros | 20.00 | 20 |  |  | 1 |
