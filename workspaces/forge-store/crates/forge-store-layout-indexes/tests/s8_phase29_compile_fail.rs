mod compile_fail_support;

#[test]
fn phase29_dedicated_workspace_facade_rejects_legacy_access_path_reexports() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 29] {
    [
        fixture(
            "legacy_store_is_not_reexported_by_workspace_facade.rs",
            &["no `ForgeStore` in the root"],
        ),
        fixture(
            "legacy_builder_is_not_reexported_by_workspace_facade.rs",
            &["no `ForgeStoreBuilder` in the root"],
        ),
        fixture(
            "legacy_layout_request_is_not_reexported_by_workspace_facade.rs",
            &["no `AspectLayoutReadRequest` in the root"],
        ),
        fixture(
            "legacy_layout_plan_is_not_reexported_by_workspace_facade.rs",
            &["no `AdmittedAspectLayoutReadPlan` in the root"],
        ),
        fixture(
            "legacy_layout_execution_decision_is_not_reexported_by_workspace_facade.rs",
            &["no `AspectLayoutReadExecutionDecision` in the root"],
        ),
        fixture(
            "legacy_layout_plan_decision_is_not_reexported_by_workspace_facade.rs",
            &["no `AspectLayoutReadPlanDecision` in the root"],
        ),
        fixture(
            "legacy_layout_execution_result_is_not_reexported_by_workspace_facade.rs",
            &["no `AspectLayoutReadExecutionResult` in the root"],
        ),
        fixture(
            "legacy_layout_support_materialization_is_not_reexported_by_workspace_facade.rs",
            &["no `Milestone6LayoutMaterialization` in the root"],
        ),
        fixture(
            "legacy_chunk_model_export_is_not_reexported_by_workspace_facade.rs",
            &["no `Milestone6ChunkModelExport` in the root"],
        ),
        fixture(
            "legacy_derived_rebuild_report_is_not_reexported_by_workspace_facade.rs",
            &["no `Milestone6DerivedArtifactRebuildReport` in the root"],
        ),
        fixture(
            "legacy_prepared_layout_support_is_not_reexported_by_workspace_facade.rs",
            &["no `Milestone6PreparedLayoutSupport` in the root"],
        ),
        fixture(
            "legacy_fallback_class_is_not_reexported_by_workspace_facade.rs",
            &["no `AspectLayoutFallbackClass` in the root"],
        ),
        fixture(
            "legacy_explicit_fallback_plan_is_not_reexported_by_workspace_facade.rs",
            &["no `ExplicitBroadFallbackPlan` in the root"],
        ),
        fixture(
            "legacy_branch_delta_fallback_class_is_not_reexported_by_workspace_facade.rs",
            &["no `BranchDeltaFallbackClass` in the root"],
        ),
        fixture(
            "legacy_compatibility_registry_is_not_reexported_by_workspace_facade.rs",
            &["no `CompatibilityRegistry` in the root"],
        ),
        fixture(
            "legacy_compatibility_registry_snapshot_is_not_reexported_by_workspace_facade.rs",
            &["no `CompatibilityRegistrySnapshot` in the root"],
        ),
        fixture(
            "legacy_maintenance_declaration_is_not_reexported_by_workspace_facade.rs",
            &["no `MaintenanceDeclaration` in the root"],
        ),
        fixture(
            "legacy_rebuild_maintenance_declaration_is_not_reexported_by_workspace_facade.rs",
            &["no `RebuildMaintenanceDeclaration` in the root"],
        ),
        fixture(
            "legacy_maintenance_declaration_class_is_not_reexported_by_workspace_facade.rs",
            &["no `MaintenanceDeclarationClass` in the root"],
        ),
        fixture(
            "legacy_maintenance_declaration_id_is_not_reexported_by_workspace_facade.rs",
            &["no `MaintenanceDeclarationId` in the root"],
        ),
        fixture(
            "legacy_layout_module_is_not_publicly_reachable.rs",
            &["could not find `layout` in `forge_store`"],
        ),
        fixture(
            "legacy_facade_module_is_not_publicly_reachable.rs",
            &["could not find `facade` in `forge_store`"],
        ),
        fixture(
            "legacy_support_access_structure_is_not_reexported_by_workspace_facade.rs",
            &["no `SubscriptionSupportAccessStructure` in the root"],
        ),
        fixture(
            "legacy_support_access_structure_report_is_not_reexported_by_workspace_facade.rs",
            &["no `SubscriptionSupportAccessStructureReport` in the root"],
        ),
        fixture(
            "legacy_support_trust_access_path_is_not_reexported_by_workspace_facade.rs",
            &["no `SupportTrustAccessPath` in the root"],
        ),
        fixture(
            "legacy_support_trust_access_index_kind_is_not_reexported_by_workspace_facade.rs",
            &["no `SupportTrustAccessIndexKind` in the root"],
        ),
        fixture(
            "legacy_milestone7_independent_reference_is_not_reexported_by_workspace_facade.rs",
            &["no `Milestone7IndependentReference` in the root"],
        ),
        fixture(
            "legacy_milestone6_access_structure_claim_is_not_reexported_by_workspace_facade.rs",
            &["no `Milestone6AccessStructureClaim` in the root"],
        ),
        fixture(
            "legacy_milestone6_layout_read_report_is_not_reexported_by_workspace_facade.rs",
            &["no `Milestone6LayoutReadReport` in the root"],
        ),
    ]
}

const fn fixture(
    name: &'static str,
    expected_stderr: &'static [&'static str],
) -> CompileFailFixture {
    CompileFailFixture {
        name,
        expected_stderr,
    }
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    compile_fail_support::assert_compile_fails_in_ui_dir(
        "phase29",
        fixture.name,
        fixture.expected_stderr,
        &["forge_store"],
    );
}
