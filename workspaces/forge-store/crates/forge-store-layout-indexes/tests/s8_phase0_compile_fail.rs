mod compile_fail_support;

#[test]
fn phase_zero_public_boundary_denies_forged_or_weaker_authority() {
    for fixture in compile_fail_fixtures() {
        compile_fail_support::assert_compile_fails(
            fixture.name,
            fixture.expected_stderr,
            fixture.extern_crates,
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
    extern_crates: &'static [&'static str],
}

fn compile_fail_fixtures() -> [CompileFailFixture; 36] {
    [
        fixture(
            "raw_struct_cannot_construct_admitted_layout_strategy.rs",
            &["S8AdmittedLayoutStrategy"],
            &[],
        ),
        fixture(
            "raw_struct_cannot_construct_phase_obligation.rs",
            &["private field", "S8PhaseSkeletonObligationRow"],
            &[],
        ),
        fixture(
            "deep_import_internal_skeleton_is_unavailable.rs",
            &["private module", "skeleton"],
            &[],
        ),
        fixture(
            "certification_closeout_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["forge_store_certification"],
        ),
        fixture(
            "physical_certification_harness_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["forge_store_physical_certification"],
        ),
        fixture(
            "test_support_fixture_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["forge_store_test_support"],
        ),
        fixture(
            "offline_report_cannot_satisfy_readmission_witness.rs",
            &["S8ExecutionReadmissionWitness", "OfflineLayoutReport"],
            &["forge_store_offline_verifier"],
        ),
        fixture(
            "foundational_materialized_report_cannot_satisfy_readmission_witness.rs",
            &[
                "S8ExecutionReadmissionWitness",
                "FoundationalMaterializedPerformanceReport",
            ],
            &["forge_foundational"],
        ),
        fixture(
            "copied_counter_rows_cannot_satisfy_planned_vs_observed.rs",
            &[
                "S8PlannedVsObservedCounterReceipt",
                "FoundationalPerformanceCounterRow",
            ],
            &["forge_foundational"],
        ),
        fixture(
            "terminal_projection_fixture_cannot_satisfy_readmission_witness.rs",
            &[
                "S8ExecutionReadmissionWitness",
                "StoreTerminalProjectionJsonFixture",
            ],
            &["forge_store_test_support"],
        ),
        fixture(
            "certification_helper_surface_is_not_public.rs",
            &["certification_test_authority"],
            &[],
        ),
        fixture(
            "strategy_declaration_surface_is_not_public.rs",
            &["S8StrategyDeclaration", "S8StrategyCapability"],
            &[],
        ),
        fixture(
            "strategy_admission_surface_is_not_public.rs",
            &["strategy_admission"],
            &[],
        ),
        fixture(
            "policy_receipt_cannot_satisfy_layout_registry_snapshot.rs",
            &["S8LayoutStrategyRegistrySnapshot"],
            &["forge_foundational"],
        ),
        fixture(
            "materialized_report_cannot_satisfy_layout_registry_snapshot.rs",
            &["S8LayoutStrategyRegistrySnapshot"],
            &["forge_foundational"],
        ),
        fixture(
            "generic_execution_surface_is_not_public.rs",
            &[
                "access_execution",
                "S8ExecutedAccessEvidence",
                "S8ExecutionReadyAccessPlan",
                "S8LoweredAccessPlan",
            ],
            &[],
        ),
        fixture(
            "materialization_raw_constructor_surface_is_not_public.rs",
            &[
                "private associated function",
                "exact",
                "root_epoch",
                "physical_range",
                "partially_covered",
            ],
            &[],
        ),
        fixture(
            "raw_strategy_family_cannot_satisfy_future_customization_request.rs",
            &[
                "S8FutureLayoutCustomizationRequest",
                "S8LayoutStrategyFamily",
            ],
            &[],
        ),
        fixture(
            "callback_cannot_satisfy_future_customization_request.rs",
            &["S8FutureLayoutCustomizationRequest", "fn()"],
            &[],
        ),
        fixture(
            "layout_registry_surface_is_not_public.rs",
            &["layout_admission_registry", "S8LayoutAdmissionRequest"],
            &[],
        ),
        fixture(
            "declared_strategy_surface_is_not_public.rs",
            &["S8FutureDeclaredStrategyClass"],
            &[],
        ),
        fixture(
            "extensions_generic_target_builder_is_not_public.rs",
            &["declare_target"],
            &["forge_store_extensions"],
        ),
        fixture(
            "legacy_access_shape_wrappers_are_not_public.rs",
            &[
                "S8AccessShapeDeclaration",
                "S8PointAccessShape",
                "S8RangeAccessShape",
                "S8PrefixAccessShape",
            ],
            &[],
        ),
        fixture(
            "legacy_degraded_exact_scan_surface_is_not_public.rs",
            &["S8DegradedExactScan"],
            &[],
        ),
        fixture(
            "plan_fingerprint_raw_constructor_is_not_public.rs",
            &["private associated function", "S8PlanFingerprint"],
            &[],
        ),
        fixture(
            "layout_readmission_legacy_for_stale_surface_is_not_public.rs",
            &["unresolved import", "layout_readmission"],
            &[],
        ),
        fixture(
            "layout_readmission_witness_raw_constructor_is_not_public.rs",
            &["private associated function", "S8LayoutReadmissionWitness"],
            &[],
        ),
        fixture(
            "execution_readmission_witness_raw_constructor_is_not_public.rs",
            &[
                "private associated function",
                "S8ExecutionReadmissionWitness",
            ],
            &[],
        ),
        fixture(
            "execution_rebind_witness_raw_constructor_is_not_public.rs",
            &["private associated function", "S8ExecutionRebindWitness"],
            &[],
        ),
        fixture(
            "derived_index_rebuild_plan_raw_constructor_is_not_public.rs",
            &["private field", "S8DerivedIndexRebuildPlan"],
            &[],
        ),
        fixture(
            "derived_index_parity_witness_raw_constructor_is_not_public.rs",
            &["private field", "S8DerivedIndexParityWitness"],
            &[],
        ),
        fixture(
            "layout_mutation_plan_raw_constructor_is_not_public.rs",
            &["private field", "S8LayoutMutationPlan"],
            &[],
        ),
        fixture(
            "live_maintenance_request_cannot_mint_lower_mutation_proof.rs",
            &["no method named", "prove_wal_before_data"],
            &[],
        ),
        fixture(
            "live_exact_maintenance_witness_raw_constructor_is_not_public.rs",
            &["private field", "S8LiveExactMaintenanceWitness"],
            &[],
        ),
        fixture(
            "physical_root_manifest_rebuild_witness_raw_constructor_is_not_public.rs",
            &["private field", "PhysicalRootManifestRebuildWitness"],
            &["forge_store_physical_format"],
        ),
        fixture(
            "blob_wal_replay_rebuild_witness_raw_constructor_is_not_public.rs",
            &["private field", "BlobWalReplayRebuildWitness"],
            &["forge_store_wal"],
        ),
    ]
}

const fn fixture(
    name: &'static str,
    expected_stderr: &'static [&'static str],
    extern_crates: &'static [&'static str],
) -> CompileFailFixture {
    CompileFailFixture {
        name,
        expected_stderr,
        extern_crates,
    }
}
