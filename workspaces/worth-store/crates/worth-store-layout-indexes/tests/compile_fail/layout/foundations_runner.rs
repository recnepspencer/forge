use super::compile_fail_support;

#[test]
fn layout_foundations_deny_forged_or_weaker_authority() {
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

fn compile_fail_fixtures() -> [CompileFailFixture; 43] {
    [
        fixture(
            "raw_struct_cannot_construct_admitted_layout_strategy.rs",
            &["AdmittedLayoutStrategy"],
            &[],
        ),
        fixture(
            "bootstrap_catalog_fields_are_not_constructible.rs",
            &["private field", "BootstrapLayoutCatalog"],
            &[],
        ),
        fixture(
            "deep_import_internal_skeleton_is_unavailable.rs",
            &[
                "module `access` is private",
                "S8ExecutionReadyAccessReceipt",
            ],
            &[],
        ),
        fixture(
            "access_planning_surface_is_not_public.rs",
            &["no `access_lowering`", "worth_store_layout_indexes"],
            &[],
        ),
        fixture(
            "certification_report_cannot_satisfy_owner_outcome.rs",
            &["BTreeLookupExecutionOutcome", "LayoutCourtroomReport"],
            &["worth_store_certification"],
        ),
        fixture(
            "physical_certification_harness_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["worth_store_physical_certification"],
        ),
        fixture(
            "test_support_fixture_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["worth_store_test_support"],
        ),
        fixture(
            "offline_report_cannot_satisfy_readmission_witness.rs",
            &[
                "LayoutReadmissionWitness",
                "OfflineVerifierLayoutProjection",
            ],
            &[],
        ),
        fixture(
            "foundational_materialized_report_cannot_satisfy_readmission_witness.rs",
            &[
                "LayoutReadmissionWitness",
                "FoundationalMaterializedPerformanceReport",
            ],
            &[],
        ),
        fixture(
            "copied_counter_rows_cannot_satisfy_planned_vs_observed.rs",
            &["BaselineBTreeLookupCounterReceipt", "private field"],
            &[],
        ),
        fixture(
            "terminal_projection_fixture_cannot_satisfy_readmission_witness.rs",
            &[
                "LayoutReadmissionWitness",
                "StoreTerminalProjectionJsonFixture",
            ],
            &["worth_store_test_support"],
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
            &["LayoutStrategyRegistrySnapshot"],
            &["worth_foundational"],
        ),
        fixture(
            "materialized_report_cannot_satisfy_layout_registry_snapshot.rs",
            &["LayoutStrategyRegistrySnapshot"],
            &["worth_foundational"],
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
                "module `access_planning` is private",
                "LayoutMaterializationState",
            ],
            &[],
        ),
        fixture(
            "raw_strategy_family_cannot_satisfy_future_customization_request.rs",
            &["FutureLayoutCustomizationRequest", "LayoutStrategyFamily"],
            &[],
        ),
        fixture(
            "callback_cannot_satisfy_future_customization_request.rs",
            &["FutureLayoutCustomizationRequest", "fn()"],
            &[],
        ),
        fixture(
            "layout_registry_surface_is_not_public.rs",
            &["layout_admission_registry", "LayoutAdmissionRequest"],
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
            &["worth_store_extensions"],
        ),
        fixture(
            "legacy_access_shape_wrappers_are_not_public.rs",
            &[
                "AccessShapeDeclaration",
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
            &["no `AccessPlanIdentity`"],
            &[],
        ),
        fixture(
            "layout_readmission_legacy_for_stale_surface_is_not_public.rs",
            &["layout_readmission", "could not find"],
            &[],
        ),
        fixture(
            "layout_readmission_witness_raw_constructor_is_not_public.rs",
            &["LayoutReadmissionWitness", "terminal_import"],
            &[],
        ),
        fixture(
            "execution_rebind_admission_raw_constructor_is_not_public.rs",
            &["private field", "DegradedScanRebindAdmission"],
            &[],
        ),
        fixture(
            "derived_index_rebuild_plan_raw_constructor_is_not_public.rs",
            &["private field", "DerivedIndexRebuildPlan"],
            &[],
        ),
        fixture(
            "derived_index_parity_witness_raw_constructor_is_not_public.rs",
            &["private field", "DerivedIndexParityWitness"],
            &[],
        ),
        fixture(
            "layout_corruption_classification_is_owner_issued.rs",
            &["private struct", "LayoutCorruptionClassification"],
            &[],
        ),
        fixture(
            "derived_projection_cannot_satisfy_exact_access_admission.rs",
            &["no method named `require_exact_point_access`"],
            &[],
        ),
        fixture(
            "derived_projection_cannot_bind_rollback_authority.rs",
            &["StoreCurrentAuthorityWitness", "DerivedIndexParityWitness"],
            &[],
        ),
        fixture(
            "cache_hit_cannot_satisfy_execution_readiness.rs",
            &["could not find `access_lowering`"],
            &[],
        ),
        fixture(
            "layout_mutation_plan_raw_constructor_is_not_public.rs",
            &["private field", "LayoutMutationPlan"],
            &[],
        ),
        fixture(
            "live_maintenance_request_cannot_mint_lower_mutation_proof.rs",
            &[
                "no method named `prove_wal_before_data`",
                "LiveMaintenanceRequest",
            ],
            &[],
        ),
        fixture(
            "live_exact_maintenance_witness_raw_constructor_is_not_public.rs",
            &["private field", "LiveExactMaintenanceWitness"],
            &[],
        ),
        fixture(
            "physical_root_manifest_rebuild_witness_raw_constructor_is_not_public.rs",
            &["private field", "PhysicalRootManifestRebuildWitness"],
            &["worth_store_physical_format"],
        ),
        fixture(
            "physical_root_manifest_rebuild_source_raw_constructor_is_not_public.rs",
            &["private field", "PhysicalRootManifestRebuildSource"],
            &["worth_store_physical_format"],
        ),
        fixture(
            "blob_wal_replay_rebuild_witness_raw_constructor_is_not_public.rs",
            &["private field", "BlobWalReplayRebuildWitness"],
            &["worth_store_wal"],
        ),
        fixture(
            "raw_blob_identity_cannot_construct_materialization_source.rs",
            &["private field", "ImportedBlobMaterializationSourceIdentity"],
            &[],
        ),
        fixture(
            "raw_physical_reference_cannot_admit_btree_materialization.rs",
            &[
                "expected `RootPublicationValidationWitness`",
                "found `PhysicalReference`",
            ],
            &["worth_store_physical_format"],
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
