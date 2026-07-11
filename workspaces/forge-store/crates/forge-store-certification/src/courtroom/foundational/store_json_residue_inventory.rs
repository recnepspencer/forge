use crate::{
    courtroom::foundational::store_json_residue_scan::scan_current_store_json_residue,
    StoreJsonAuthorityRisk, StoreJsonResidueClassification, StoreJsonResidueDenial,
    StoreJsonResidueOccurrence, StoreJsonResidueTokenKind, StoreJsonResidueZone,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreJsonResidueInventory {
    classified: Vec<StoreJsonResidueClassification>,
}

impl StoreJsonResidueInventory {
    pub(crate) fn from_current_sources() -> Result<Self, StoreJsonResidueDenial> {
        Self::from_occurrences(scan_current_store_json_residue()?)
    }

    pub(crate) fn from_occurrences(
        occurrences: Vec<StoreJsonResidueOccurrence>,
    ) -> Result<Self, StoreJsonResidueDenial> {
        let classified = classify_store_json_residue_occurrences(occurrences)?;
        Ok(Self { classified })
    }

    pub fn classified(&self) -> &[StoreJsonResidueClassification] {
        &self.classified
    }

    pub fn contains_zone(&self, zone: StoreJsonResidueZone) -> bool {
        self.classified
            .iter()
            .any(|classification| classification.zone() == zone)
    }

    pub fn dedicated_workspace_classified(
        &self,
    ) -> impl Iterator<Item = &StoreJsonResidueClassification> {
        self.classified.iter().filter(|classification| {
            classification
                .occurrence()
                .path()
                .starts_with("workspaces/forge-store/")
        })
    }
}

pub(crate) fn classify_store_json_residue_occurrences(
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
        .starts_with("crates/forge-store/tests/ui/")
    {
        return checked(
            occurrence,
            StoreJsonResidueZone::LegacyHostileDenialTest,
            "legacy store compile-fail certification",
            StoreJsonAuthorityRisk::HostileDenialOnly,
            "kept only to prove legacy authority types reject serde reconstruction",
        );
    }
    if occurrence.path().starts_with("crates/forge-store/") {
        return classify_legacy_root_crate_occurrence(occurrence);
    }
    if occurrence.path().starts_with(
        "workspaces/forge-store/crates/forge-store-certification/src/courtroom/foundational/store_json_residue",
    ) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "forge-store aspect-native certification",
            StoreJsonAuthorityRisk::CertificationScannerVocabulary,
            "allowed only as scanner vocabulary that denies JSON authority",
        );
    }
    if is_exact_json_authority_denial_certification_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "forge-store aspect-native certification",
            StoreJsonAuthorityRisk::CertificationScannerVocabulary,
            "allowed only as denial vocabulary proving JSON cannot enter authority",
        );
    }
    if is_exact_hostile_readmission_json_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceHostileReadmission,
            "forge-store aspect-native harness certification",
            StoreJsonAuthorityRisk::HostileReadmissionOnly,
            "allowed only in terminal or hostile/readmission harness proof",
        );
    }
    if is_exact_compile_fail_json_denial_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "forge-store compile-fail courtroom",
            StoreJsonAuthorityRisk::HostileDenialOnly,
            "allowed only as compile-fail input or runner material proving JSON cannot satisfy native authority",
        );
    }
    if is_exact_public_facade_json_enforcement_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "forge-store public facade certification",
            StoreJsonAuthorityRisk::CertificationScannerVocabulary,
            "allowed only as public facade dependency denial proof",
        );
    }
    if is_exact_terminal_projection_json_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceTerminalBoundary,
            "forge-store aspect-native terminal projection",
            StoreJsonAuthorityRisk::TerminalProjectionOnly,
            "allowed only as one-way terminal projection that requires explicit Store readmission",
        );
    }
    if is_exact_json_ingress_readmission_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceHostileReadmission,
            "forge-store aspect-native JSON ingress readmission",
            StoreJsonAuthorityRisk::HostileReadmissionOnly,
            "allowed only to lower terminal JSON into validated native Store aspect material",
        );
    }
    if is_exact_durable_serde_contract_home(occurrence.path()) {
        if !matches!(
            occurrence.token(),
            StoreJsonResidueTokenKind::Serialize | StoreJsonResidueTokenKind::Deserialize
        ) {
            return Err(StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(
                occurrence,
            ));
        }
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceDurableSerdeContract,
            "forge-store durable compatibility contract",
            StoreJsonAuthorityRisk::DurableSerdeContractOnly,
            "allowed only for exact durable contract encoding; JSON helpers and generic JSON admission remain forbidden",
        );
    }
    if is_exact_physical_courtroom_json_denial_home(occurrence.path()) {
        return checked(
            occurrence,
            StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement,
            "forge-store physical scenario authority courtroom",
            StoreJsonAuthorityRisk::HostileDenialOnly,
            "allowed only as a hostile raw-JSON authority rejection surface",
        );
    }
    if occurrence.path().starts_with("workspaces/forge-store/") {
        return Err(StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(occurrence));
    }
    Err(StoreJsonResidueDenial::MissingClassification(occurrence))
}

fn is_exact_terminal_projection_json_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/forge-store/crates/forge-store-aspect-native/Cargo.toml"
            | "workspaces/forge-store/crates/forge-store-aspect-native/src/terminal_json_projection.rs"
    )
}

fn is_exact_json_ingress_readmission_home(path: &str) -> bool {
    path
        == "workspaces/forge-store/crates/forge-store-aspect-native/src/json_ingress_readmission.rs"
}

fn is_exact_durable_serde_contract_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/forge-store/crates/forge-store-contracts/src/compatibility_family.rs"
            | "workspaces/forge-store/crates/forge-store-contracts/src/existing_artifact_family.rs"
    )
}

fn is_exact_physical_courtroom_json_denial_home(path: &str) -> bool {
    path == "workspaces/forge-store/crates/forge-store-physical-certification/src/scenario/proof_progression.rs"
}

fn is_exact_hostile_readmission_json_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/forge-store/crates/forge-store-certification/Cargo.toml"
            | "workspaces/forge-store/crates/forge-store-certification/src/scenario/foundational/hostile_readmission_json_fixture_boundary_tests.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/aspect_native_terminal_projection.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/aspect_native_terminal_projection_hostile_readmission.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/terminal_projection_quarantine_ui.rs"
    )
}

fn is_exact_public_facade_json_enforcement_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/forge-store/crates/forge-store-certification/src/courtroom/cross_cutting/public_facade_dependency_tests.rs"
    )
}

fn is_exact_compile_fail_json_denial_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/forge-store/crates/forge-store-certification/tests/s4_5_coverage_compile_fail.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/s4_5_forbidden_shortcut_rejection_compile_fail.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/s4_5_scenario_authority_compile_fail.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/s4_5_simulation_plan_boundary_compile_fail.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/s4_5_transcript_evidence_compile_fail.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/s5_1_security_scope_admission_compile_fail.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/s5_1_security_scope_vocabulary_compile_fail.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s4_5/terminal_json_cannot_satisfy_coverage.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s4_5_forbidden_shortcuts/raw_json_cannot_satisfy_certified_scenario.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s4_5_scenario_authority/json_value_cannot_define_scenario.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s4_5_simulation_plan_boundary/json_value_cannot_satisfy_lowered_plan.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s4_5_transcript_evidence/terminal_json_cannot_satisfy_evidence_bundle.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s4_5_transcript_evidence/terminal_json_cannot_satisfy_replay_bundle.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s5_1_security_scope_admission/serde_json_value_cannot_satisfy_admitted_scope.rs"
            | "workspaces/forge-store/crates/forge-store-certification/tests/ui/s5_1_security_scope_vocabulary/serde_json_value_cannot_satisfy_tenant_scope.rs"
    )
}

fn is_exact_json_authority_denial_certification_home(path: &str) -> bool {
    matches!(
        path,
        "workspaces/forge-store/crates/forge-store-certification/src/courtroom/foundational/canonical_basis_entry_denial_tests.rs"
            | "workspaces/forge-store/crates/forge-store-certification/src/courtroom/foundational/digest_authority_denial_tests.rs"
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
