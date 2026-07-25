use crate::{
    StoreJsonAuthorityRisk, StoreJsonResidueClassification, StoreJsonResidueDenial,
    StoreJsonResidueOccurrence, StoreJsonResidueTokenKind, StoreJsonResidueZone,
};

pub(super) fn classify_store_json_residue_occurrences(
    occurrences: Vec<StoreJsonResidueOccurrence>,
) -> Result<Vec<StoreJsonResidueClassification>, StoreJsonResidueDenial> {
    occurrences
        .into_iter()
        .map(classify_store_json_residue_occurrence)
        .collect()
}

fn classify_store_json_residue_occurrence(
    occurrence: StoreJsonResidueOccurrence,
) -> Result<StoreJsonResidueClassification, StoreJsonResidueDenial> {
    if occurrence
        .path()
        .starts_with("crates/worth-store/tests/ui/")
    {
        return checked(
            occurrence,
            StoreJsonResidueZone::LegacyHostileDenialTest,
            "legacy store compile-fail certification",
            StoreJsonAuthorityRisk::HostileDenialOnly,
            "kept only to prove legacy authority types reject serde reconstruction",
        );
    }
    if occurrence.path().starts_with("crates/worth-store/") {
        return classify_legacy_root_crate_occurrence(occurrence);
    }
    if occurrence.path().starts_with(
        "workspaces/worth-store/crates/worth-store-certification/src/courtroom/foundational/store_json_residue",
    ) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store aspect-native certification",
            StoreJsonAuthorityRisk::CertificationScannerVocabulary,
            "allowed only as scanner vocabulary that denies JSON authority",
        );
    }
    if is_exact_json_authority_denial_certification_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store aspect-native certification",
            StoreJsonAuthorityRisk::CertificationScannerVocabulary,
            "allowed only as denial vocabulary proving JSON cannot enter authority",
        );
    }
    if is_exact_hostile_readmission_json_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceHostileReadmission,
            "worth-store aspect-native harness certification",
            StoreJsonAuthorityRisk::HostileReadmissionOnly,
            "allowed only in terminal or hostile/readmission harness proof",
        );
    }
    if is_exact_compile_fail_json_denial_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store compile-fail courtroom",
            StoreJsonAuthorityRisk::HostileDenialOnly,
            "allowed only as compile-fail input or runner material proving JSON cannot satisfy native authority",
        );
    }
    if is_exact_public_facade_json_enforcement_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store public facade certification",
            StoreJsonAuthorityRisk::CertificationScannerVocabulary,
            "allowed only as public facade dependency denial proof",
        );
    }
    if is_exact_physical_store_test_projection_manifest(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceTerminalBoundary,
            "worth-store physical courtroom terminal projection",
            StoreJsonAuthorityRisk::TerminalProjectionOnly,
            "allowed only as a dev-dependency for terminal test evidence; production dependencies remain JSON-free",
        );
    }
    if is_exact_physical_store_terminal_evidence_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceTerminalBoundary,
            "worth-store physical courtroom terminal evidence",
            StoreJsonAuthorityRisk::TerminalProjectionOnly,
            "allowed only for one-way terminal courtroom evidence after native authority transitions complete",
        );
    }
    if is_exact_physical_store_json_denial_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store physical authority compile-fail courtroom",
            StoreJsonAuthorityRisk::HostileDenialOnly,
            "allowed only as a hostile JSON value that must fail to satisfy physical authority",
        );
    }
    if is_exact_terminal_projection_json_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceTerminalBoundary,
            "worth-store aspect-native terminal projection",
            StoreJsonAuthorityRisk::TerminalProjectionOnly,
            "allowed only as one-way terminal projection that requires explicit Store readmission",
        );
    }
    if is_exact_json_ingress_readmission_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceHostileReadmission,
            "worth-store aspect-native JSON ingress readmission",
            StoreJsonAuthorityRisk::HostileReadmissionOnly,
            "allowed only to lower terminal JSON into validated native Store aspect material",
        );
    }
    if is_exact_physical_store_courtroom_tool_protocol_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store physical courtroom evidence protocol",
            StoreJsonAuthorityRisk::CertificationToolProtocolOnly,
            "allowed only to decode and independently verify sealed mutation and Cargo evidence; it cannot satisfy Store runtime or semantic authority",
        );
    }
    if is_test_support_tool_protocol_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store certification tool protocols",
            StoreJsonAuthorityRisk::CertificationToolProtocolOnly,
            "allowed only for Cargo diagnostic and structural-preflight tool transport; it cannot satisfy Store authority",
        );
    }
    if is_process_probe_binary_contract_home(occurrence.path()) {
        if !matches!(
            occurrence.token(),
            StoreJsonResidueTokenKind::Serialize
                | StoreJsonResidueTokenKind::Deserialize
                | StoreJsonResidueTokenKind::DeserializeOwned
        ) {
            return Err(StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(occurrence));
        }
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceDurableSerdeContract,
            "worth-store process-probe binary wire contract",
            StoreJsonAuthorityRisk::DurableSerdeContractOnly,
            "allowed only for the bounded binary process-probe codec; JSON and generic authority derivation remain forbidden",
        );
    }
    if is_exact_durable_serde_contract_home(occurrence.path()) {
        if !matches!(
            occurrence.token(),
            StoreJsonResidueTokenKind::Serialize | StoreJsonResidueTokenKind::Deserialize
        ) {
            return Err(StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(occurrence));
        }
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceDurableSerdeContract,
            "worth-store durable compatibility contract",
            StoreJsonAuthorityRisk::DurableSerdeContractOnly,
            "allowed only for exact durable contract encoding; JSON helpers and generic JSON admission remain forbidden",
        );
    }
    if is_exact_physical_courtroom_json_denial_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "worth-store physical scenario authority courtroom",
            StoreJsonAuthorityRisk::HostileDenialOnly,
            "allowed only as a hostile raw-JSON authority rejection surface",
        );
    }
    if occurrence.path().starts_with("workspaces/worth-store/") {
        return Err(StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(occurrence));
    }
    Err(StoreJsonResidueDenial::MissingClassification(occurrence))
}

fn is_process_probe_binary_contract_home(path: &str) -> bool {
    path.starts_with(
        "workspaces/worth-store/crates/worth-store-physical-certification/src/process_probe/",
    )
}

fn is_exact_terminal_projection_json_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-aspect-native/Cargo.toml"
            | "workspaces/worth-store/crates/worth-store-aspect-native/src/terminal_json_projection.rs"
    )
}

fn is_exact_physical_store_test_projection_manifest(path: &str) -> bool {
    path == "workspaces/worth-store/crates/worth-store/Cargo.toml"
}

fn is_exact_physical_store_terminal_evidence_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store/tests/c5/courtrooms.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/manifest_scale/evidence.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/courtroom_environment.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/phase_16_lifecycle_maelstrom/terminal_projection.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/publication_failure_topology/evidence.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/scenario_artifact_evidence.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/scenario_evidence.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/scenario_process_evidence.rs"
    )
}

fn is_exact_physical_store_courtroom_tool_protocol_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/phase_16_lifecycle_maelstrom/mutant_report.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/phase_16_lifecycle_maelstrom/mutant_report/campaign_source.rs"
            | "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/phase_16_lifecycle_maelstrom/mutant_report/decoding.rs"
    )
}

fn is_exact_physical_store_json_denial_home(path: &str) -> bool {
    path
        == "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/untyped_physical_work_basis_is_rejected.rs"
}

fn is_exact_json_ingress_readmission_home(path: &str) -> bool {
    path
        == "workspaces/worth-store/crates/worth-store-aspect-native/src/json_ingress_readmission.rs"
}

fn is_exact_durable_serde_contract_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-contracts/src/compatibility_family.rs"
            | "workspaces/worth-store/crates/worth-store-contracts/src/existing_artifact_family.rs"
    )
}

fn is_exact_physical_courtroom_json_denial_home(path: &str) -> bool {
    path == "workspaces/worth-store/crates/worth-store-physical-certification/src/scenario/proof_progression.rs"
}

fn is_exact_hostile_readmission_json_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-certification/Cargo.toml"
            | "workspaces/worth-store/crates/worth-store-certification/src/scenario/foundational/hostile_readmission_json_fixture_boundary_tests.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/aspect_native_terminal_projection.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/aspect_native_terminal_projection_hostile_readmission.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/terminal_projection_quarantine_ui.rs"
    )
}

fn is_exact_public_facade_json_enforcement_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-certification/src/courtroom/cross_cutting/public_facade_dependency_tests.rs"
    )
}

fn is_test_support_tool_protocol_home(path: &str) -> bool {
    path == "workspaces/worth-store/crates/worth-store-test-support/Cargo.toml"
        || path.starts_with(
            "workspaces/worth-store/crates/worth-store-test-support/src/compiler_boundary/",
        )
        || path.starts_with("workspaces/worth-store/tools/store-test-runner/")
}

fn is_exact_compile_fail_json_denial_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/coverage_runner.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/forbidden_shortcut_rejection_runner.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/scenario_authority_runner.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/simulation_plan_boundary_runner.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/transcript_evidence_runner.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/security/security_scope_admission_runner.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/security/security_scope_vocabulary_runner.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/coverage/terminal_json_cannot_satisfy_coverage.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/forbidden_shortcuts/raw_json_cannot_satisfy_certified_scenario.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/scenario_authority/json_value_cannot_define_scenario.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/simulation_plan_boundary/json_value_cannot_satisfy_lowered_plan.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/transcript_evidence/terminal_json_cannot_satisfy_evidence_bundle.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/recovery/transcript_evidence/terminal_json_cannot_satisfy_replay_bundle.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/security/security_scope_admission/serde_json_value_cannot_satisfy_admitted_scope.rs"
            | "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/security/security_scope_vocabulary/serde_json_value_cannot_satisfy_tenant_scope.rs"
    )
}

fn is_exact_json_authority_denial_certification_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-certification/src/courtroom/foundational/canonical_basis_entry_denial_tests.rs"
            | "workspaces/worth-store/crates/worth-store-certification/src/courtroom/foundational/digest_authority_denial_tests.rs"
    )
}

fn classify_legacy_root_crate_occurrence(
    occurrence: StoreJsonResidueOccurrence,
) -> Result<StoreJsonResidueClassification, StoreJsonResidueDenial> {
    let owner = legacy_owner(occurrence.path());
    let risk = legacy_risk(&occurrence);
    checked(
        occurrence,
        StoreJsonResidueZone::LegacyCompatibilityResidue,
        owner,
        risk,
        "legacy root crate compatibility residue; remove or readmit through native Store before Roadmap 2 use",
    )
}

fn legacy_owner(path: &str) -> &'static str {
    if path.contains("/compatibility/") {
        "legacy store compatibility program"
    } else if path.contains("/backend/sqlite/") {
        "legacy sqlite persistence adapter"
    } else if path.contains("/snapshot/") {
        "legacy snapshot persistence"
    } else if path.contains("/wal/") {
        "legacy wal model persistence"
    } else if path.contains("/storage_foundation/s0/") {
        "legacy S0 compatibility residue"
    } else {
        "legacy store semantic durability crate"
    }
}

fn legacy_risk(occurrence: &StoreJsonResidueOccurrence) -> StoreJsonAuthorityRisk {
    if is_legacy_json_digest_basis_residue(occurrence) {
        StoreJsonAuthorityRisk::LegacyDigestBasisResidue
    } else if occurrence.token() == StoreJsonResidueTokenKind::SerdeJson
        || occurrence.token() == StoreJsonResidueTokenKind::RawJsonHelper
    {
        StoreJsonAuthorityRisk::LegacyJsonPersistenceResidue
    } else {
        StoreJsonAuthorityRisk::LegacySerdeAuthorityResidue
    }
}

fn is_legacy_json_digest_basis_residue(occurrence: &StoreJsonResidueOccurrence) -> bool {
    is_legacy_snapshot_digest_text_reference(occurrence)
        || is_legacy_raw_json_digest_or_projection_helper(occurrence)
}

fn is_legacy_snapshot_digest_text_reference(occurrence: &StoreJsonResidueOccurrence) -> bool {
    occurrence.path().contains("/snapshot/") && occurrence.excerpt().contains("digest")
}

fn is_legacy_raw_json_digest_or_projection_helper(occurrence: &StoreJsonResidueOccurrence) -> bool {
    occurrence.token() == StoreJsonResidueTokenKind::RawJsonHelper
        && [
            "canonical_json",
            "semantic_json",
            "stable_json_digest",
            "to_canonical_json_bytes",
            "validate_canonical_json_bytes",
        ]
        .iter()
        .any(|needle| occurrence.excerpt().contains(needle))
}

fn checked(
    occurrence: StoreJsonResidueOccurrence,
    zone: StoreJsonResidueZone,
    owner: &'static str,
    authority_risk: StoreJsonAuthorityRisk,
    quarantine_or_removal_condition: &'static str,
) -> Result<StoreJsonResidueClassification, StoreJsonResidueDenial> {
    StoreJsonResidueClassification::checked(
        occurrence.clone(),
        zone,
        owner,
        authority_risk,
        quarantine_or_removal_condition,
    )
    .ok_or(StoreJsonResidueDenial::InvalidClassification(occurrence))
}
