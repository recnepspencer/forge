mod compile_fail_support;

#[test]
fn phase28_layout_surfaces_reject_forgeable_terminal_rule_and_report_shortcuts() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
    extern_crates: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 23] {
    [
        fixture(
            "caller_defined_rule_cannot_open_export_bundle_layout.rs",
            &["AdmittedExportBundleLayoutRule", "private field"],
            &[],
        ),
        fixture(
            "caller_defined_rule_cannot_open_capsule_manifest_layout.rs",
            &["AdmittedCapsuleManifestLayoutRule", "private field"],
            &[],
        ),
        fixture(
            "caller_defined_rule_cannot_open_offline_verifier_layout.rs",
            &["AdmittedOfflineVerifierLayoutRule", "private field"],
            &[],
        ),
        fixture(
            "caller_defined_rule_cannot_open_restore_evidence_layout.rs",
            &["AdmittedRestoreEvidenceLayoutRule", "private field"],
            &[],
        ),
        fixture(
            "caller_defined_rule_cannot_open_import_readmission_layout.rs",
            &["AdmittedImportReadmissionLayoutRule", "private field"],
            &[],
        ),
        fixture(
            "admitted_export_bundle_layout_rule_constructor_is_not_public.rs",
            &["AdmittedExportBundleLayoutRule", "internal_phase28"],
            &[],
        ),
        fixture(
            "admitted_capsule_manifest_layout_rule_constructor_is_not_public.rs",
            &["AdmittedCapsuleManifestLayoutRule", "internal_phase28"],
            &[],
        ),
        fixture(
            "admitted_offline_verifier_layout_rule_constructor_is_not_public.rs",
            &["AdmittedOfflineVerifierLayoutRule", "internal_phase28"],
            &[],
        ),
        fixture(
            "admitted_restore_evidence_layout_rule_constructor_is_not_public.rs",
            &["AdmittedRestoreEvidenceLayoutRule", "internal_phase28"],
            &[],
        ),
        fixture(
            "admitted_import_readmission_layout_rule_constructor_is_not_public.rs",
            &["AdmittedImportReadmissionLayoutRule", "internal_phase28"],
            &[],
        ),
        fixture(
            "caller_defined_report_cannot_open_export_bundle_layout.rs",
            &["ExportLayoutEvidenceReport", "private field"],
            &["forge_store_contracts", "forge_store_operations"],
        ),
        fixture(
            "caller_defined_report_cannot_open_capsule_manifest_layout.rs",
            &["CapsuleOperationLayoutReport", "private field"],
            &["forge_store_contracts", "forge_store_operations"],
        ),
        fixture(
            "caller_defined_report_cannot_open_restore_evidence_layout.rs",
            &["RestoreLayoutEvidenceReport", "private field"],
            &["forge_store_contracts", "forge_store_operations"],
        ),
        fixture(
            "caller_defined_report_cannot_open_import_readmission_layout.rs",
            &["ImportLayoutEvidenceReport", "private field"],
            &["forge_store_contracts", "forge_store_operations"],
        ),
        fixture(
            "offline_layout_report_cannot_open_export_layout.rs",
            &["OfflineLayoutReport", "new"],
            &["forge_store_offline_verifier", "forge_store_operations"],
        ),
        fixture(
            "blob_chunk_phase28_reports_are_no_longer_public.rs",
            &["unresolved imports", "forge_store_blob_chunks"],
            &["forge_store_blob_chunks"],
        ),
        fixture(
            "offline_capsule_observation_cannot_open_import_layout.rs",
            &["ReadmittedBlobImport", "OfflineCustodyCapsuleObservation"],
            &["forge_store_offline_verifier", "forge_store_operations"],
        ),
        fixture(
            "backup_layout_report_cannot_open_capsule_layout.rs",
            &["BlobCapsuleReadinessWitness", "BackupLayoutEvidenceReport"],
            &["forge_store_operations"],
        ),
        fixture(
            "caller_defined_report_cannot_open_offline_verifier_layout.rs",
            &["OfflineVerifierLayoutReport", "private field"],
            &["forge_store_contracts"],
        ),
        fixture(
            "offline_verifier_phase28_helper_is_not_public_even_when_feature_enabled.rs",
            &["phase28_offline_verifier_layout_rule_construction"],
            &["forge_store_offline_verifier"],
        ),
        fixture(
            "backup_export_custody_admission_constructor_is_not_public.rs",
            &["BackupExportCustodyAdmission", "from_outbound_declaration"],
            &["forge_store_operations"],
        ),
        fixture(
            "backup_export_custody_readiness_constructor_is_not_public.rs",
            &["BackupExportCustodyReadiness", "from_admitted_readiness"],
            &["forge_store_operations"],
        ),
        fixture(
            "backup_export_terminal_preparation_constructor_is_not_public.rs",
            &["BackupExportTerminalProjectionPreparation", "prepare"],
            &["forge_store_operations"],
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

fn assert_compile_fails(fixture: CompileFailFixture) {
    compile_fail_support::assert_compile_fails_in_ui_dir(
        "phase28",
        fixture.name,
        fixture.expected_stderr,
        fixture.extern_crates,
    );
}
