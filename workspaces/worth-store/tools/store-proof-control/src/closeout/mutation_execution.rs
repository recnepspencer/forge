use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;
use worth_store_test_support::compiler_boundary::UiProofRunEvidence;
use worth_store_test_support::structural_preflight::{
    PreflightEvidenceFreshness, StructuralPreflightEvidence,
};

use crate::classification::{validate, validate_inventory_build_graph_policy, ClassifiedInventory};
use crate::discovery::{
    generate_owner_build_closures, validate_owner_build_closures, DependencyEdge,
};
use crate::evidence::{read_json, sha256_bytes, sha256_serialized};
use crate::selection::ProofProcessModel;
use crate::ValidatedProofInventory;

use super::iteration_case::validate_fresh_process_contract;
use super::{
    ControlledDefectKind, ControlledDefectObservation, InterpretableProductPosture,
    InterpretableProofProduct, MutationExecutionEvidence, ProofMutationSensitivityReport,
};

pub(crate) fn execute_mutation_matrix(
    workspace_root: &Path,
    inventory: &ValidatedProofInventory,
    control_identity: &str,
) -> Result<ProofMutationSensitivityReport, String> {
    let observations = vec![
        lost_ui_denial(workspace_root, control_identity)?,
        inverted_scenario_assertion(workspace_root, control_identity)?,
        broad_support_dependency(inventory, control_identity)?,
        hidden_nested_cargo(inventory, control_identity)?,
        omitted_ci_partition(inventory, control_identity)?,
        same_process_crash_substitute(control_identity)?,
        stale_preflight_evidence(workspace_root, control_identity)?,
        feature_leakage(inventory, control_identity)?,
    ];
    ProofMutationSensitivityReport::certify(control_identity.to_owned(), observations)
}

fn lost_ui_denial(
    workspace_root: &Path,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let mut evidence = find_valid_ui_evidence(workspace_root)?;
    evidence.fixtures[0].semantic_denial_matched = false;
    let denial = evidence
        .validate_integrity()
        .expect_err("lost semantic denial must invalidate UI evidence");
    validator_observation(
        ControlledDefectKind::LostUiDenial,
        "UiProofRunEvidence::validate_integrity",
        &evidence,
        &denial,
        control_identity,
    )
}

fn inverted_scenario_assertion(
    workspace_root: &Path,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let fixture_root = workspace_root.join(".store-proof/mutation-fixtures/inverted-assertion");
    ensure_file(
        &fixture_root.join("Cargo.toml"),
        b"[package]\nname = \"c1-inverted-assertion\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )?;
    ensure_file(
        &fixture_root.join("src/lib.rs"),
        b"#[cfg(test)]\nmod tests {\n    #[test]\n    fn inverted_scenario_assertion() {\n        assert_eq!(1, 2, \"controlled predicate scenario_outcome_matches\");\n    }\n}\n",
    )?;
    let target_root = workspace_root.join(".store-proof/mutation-target");
    crate::artifact_lifecycle::mark_disposable_artifact_root(&target_root)?;
    let arguments = vec![
        "test".to_owned(),
        "--manifest-path".to_owned(),
        normalized(&fixture_root.join("Cargo.toml")),
        "--target-dir".to_owned(),
        normalized(&target_root),
        "--".to_owned(),
        "--exact".to_owned(),
        "tests::inverted_scenario_assertion".to_owned(),
        "--nocapture".to_owned(),
    ];
    let output = Command::new("cargo")
        .args(&arguments)
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not launch controlled assertion fixture: {error}"))?;
    let transcript = [output.stdout.as_slice(), output.stderr.as_slice()].concat();
    let transcript_text = String::from_utf8_lossy(&transcript);
    if output.status.success()
        || !transcript_text.contains("tests::inverted_scenario_assertion")
        || !transcript_text.contains("controlled predicate scenario_outcome_matches")
    {
        return Err("inverted assertion fixture did not fail at its named predicate".to_owned());
    }
    ControlledDefectObservation::localized(
        ControlledDefectKind::InvertedScenarioAssertion,
        MutationExecutionEvidence::IsolatedCargoFixture {
            command: std::iter::once("cargo".to_owned())
                .chain(arguments)
                .collect(),
            exit_code: output.status.code().unwrap_or(-1),
            transcript_sha256: sha256_bytes(&transcript),
        },
        control(
            ControlledDefectKind::InvertedScenarioAssertion,
            control_identity,
        ),
    )
}

fn broad_support_dependency(
    inventory: &ValidatedProofInventory,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let mut closures = generate_owner_build_closures(&inventory.inventory().discovered);
    let closure = closures
        .iter_mut()
        .find(|closure| closure.boundary.owner_package != "worth-store-certification")
        .ok_or_else(|| "no owner closure available for dependency mutation".to_owned())?;
    closure
        .compiled_workspace_packages
        .insert("worth-store-certification".to_owned());
    let denial = validate_owner_build_closures(&closures)
        .expect_err("broad support dependency must violate owner closure")
        .join("\n");
    if !denial.contains("high-radius test package") {
        return Err("broad support mutation failed for an unrelated reason".to_owned());
    }
    validator_observation(
        ControlledDefectKind::BroadSupportDependency,
        "validate_owner_build_closures",
        &closures,
        &denial,
        control_identity,
    )
}

fn hidden_nested_cargo(
    inventory: &ValidatedProofInventory,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let mut mutated: ClassifiedInventory = inventory.inventory().clone();
    let proof = mutated
        .proofs
        .iter_mut()
        .find(|proof| !proof.case.launches_nested_cargo)
        .ok_or_else(|| "no non-nested proof available for process mutation".to_owned())?;
    proof.case.launches_nested_cargo = true;
    proof.case.external_tools.retain(|tool| tool != "cargo");
    let denial = validate(crate::ClassifiedProofInventory::from_discovered(
        mutated.clone(),
    ))
    .expect_err("hidden nested Cargo must violate classification")
    .join("\n");
    if !denial.contains("nested Cargo test omits cargo") {
        return Err("hidden nested Cargo mutation failed for an unrelated reason".to_owned());
    }
    validator_observation(
        ControlledDefectKind::HiddenNestedCargo,
        "classification::validate",
        &mutated,
        &denial,
        control_identity,
    )
}

fn omitted_ci_partition(
    inventory: &ValidatedProofInventory,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let required = crate::ci::required_lanes(inventory);
    let denial = crate::ci::CiCertificationAggregate::certify(inventory, &[])
        .expect_err("empty CI evidence must omit required lanes");
    if denial.len() != required.len()
        || denial
            .iter()
            .any(|item| item.reason != "required lane has no evidence")
    {
        return Err("omitted CI partition mutation did not localize to lane absence".to_owned());
    }
    validator_observation(
        ControlledDefectKind::OmittedCiPartition,
        "CiCertificationAggregate::certify",
        &required,
        &serde_json::to_string(&denial).map_err(|error| error.to_string())?,
        control_identity,
    )
}

fn same_process_crash_substitute(
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let models = [ProofProcessModel::LibtestProcess];
    let denial = validate_fresh_process_contract(&models, 0)
        .expect_err("same-process crash substitute must be denied");
    validator_observation(
        ControlledDefectKind::SameProcessCrashSubstitute,
        "DeveloperIterationCaseEvidence::validate_fresh_process_contract",
        &(models, 0_usize),
        &denial,
        control_identity,
    )
}

fn stale_preflight_evidence(
    workspace_root: &Path,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let evidence = find_valid_preflight_evidence(workspace_root)?;
    let mut mutated_plan = evidence.plan.clone();
    let scope = mutated_plan
        .predicates
        .iter_mut()
        .flat_map(|predicate| &mut predicate.input_scopes)
        .next()
        .ok_or_else(|| "preflight evidence has no input scope".to_owned())?;
    scope.input_identity = "0".repeat(64);
    mutated_plan.plan_identity.clear();
    mutated_plan.plan_identity = sha256_serialized(&mutated_plan)?;
    let freshness = crate::structural_preflight::compare_to_plan(&evidence, &mutated_plan)?;
    let PreflightEvidenceFreshness::Stale { failures } = freshness else {
        return Err("mutated preflight input remained fresh".to_owned());
    };
    if failures.is_empty()
        || failures
            .iter()
            .any(|failure| failure.failure_code != "stale_evidence")
    {
        return Err("stale preflight mutation failed for an unrelated reason".to_owned());
    }
    validator_observation(
        ControlledDefectKind::StalePreflightEvidence,
        "structural_preflight::compare_to_plan",
        &mutated_plan,
        &serde_json::to_string(&failures).map_err(|error| error.to_string())?,
        control_identity,
    )
}

fn feature_leakage(
    inventory: &ValidatedProofInventory,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    let mut mutated = inventory.inventory().discovered.clone();
    mutated.build_graph.dependency_edges.push(DependencyEdge {
        consumer: "worth-store-analysis".to_owned(),
        provider: "worth-store".to_owned(),
        manifest_name: "worth-store".to_owned(),
        dependency_kind: "normal".to_owned(),
        features: vec!["certification-test-authority".to_owned()],
        optional: false,
        uses_default_features: true,
        target: None,
    });
    let denial = validate_inventory_build_graph_policy(&mutated)
        .expect_err("production feature leakage must violate build graph policy")
        .into_iter()
        .map(|violation| violation.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    if !denial.contains("certification-test-authority") {
        return Err("feature leakage mutation failed for an unrelated reason".to_owned());
    }
    validator_observation(
        ControlledDefectKind::FeatureLeakage,
        "validate_inventory_build_graph_policy",
        &mutated,
        &denial,
        control_identity,
    )
}

fn validator_observation<T: Serialize>(
    defect: ControlledDefectKind,
    validator: &str,
    subject: &T,
    denial: &str,
    control_identity: &str,
) -> Result<ControlledDefectObservation, String> {
    ControlledDefectObservation::localized(
        defect,
        MutationExecutionEvidence::ProductionValidator {
            validator: validator.to_owned(),
            mutated_subject_sha256: sha256_serialized(subject)?,
            denial_sha256: sha256_bytes(denial.as_bytes()),
        },
        control(defect, control_identity),
    )
}

fn control(defect: ControlledDefectKind, control_identity: &str) -> Vec<InterpretableProofProduct> {
    let product = if defect.expected_product() == "store-smoke" {
        "store-ui"
    } else {
        "store-smoke"
    };
    vec![InterpretableProofProduct {
        product: product.to_owned(),
        evidence_identity: control_identity.to_owned(),
        posture: InterpretableProductPosture::ExplicitlyNotSelected,
    }]
}

fn find_valid_ui_evidence(workspace_root: &Path) -> Result<UiProofRunEvidence, String> {
    let root = workspace_root.join(".store-proof/evidence/ui/runs");
    for path in regular_json_files(&root)? {
        if let Ok(evidence) = read_json::<UiProofRunEvidence>(&path) {
            if evidence.validate_integrity().is_ok() {
                return Ok(evidence);
            }
        }
    }
    Err("controlled UI mutation requires one valid UiProofRunEvidence artifact".to_owned())
}

fn find_valid_preflight_evidence(
    workspace_root: &Path,
) -> Result<StructuralPreflightEvidence, String> {
    let root = workspace_root.join(".store-proof/evidence/preflight");
    for path in regular_json_files(&root)? {
        if let Ok(evidence) = read_json::<StructuralPreflightEvidence>(&path) {
            if evidence.validate_integrity().is_ok() {
                return Ok(evidence);
            }
        }
    }
    Err("controlled freshness mutation requires one valid preflight artifact".to_owned())
}

fn regular_json_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?
        {
            let entry = entry
                .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?;
            let kind = entry.file_type().map_err(|error| {
                format!("could not inspect {}: {error}", entry.path().display())
            })?;
            if kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                pending.push(entry.path());
            } else if kind.is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("json")
            {
                files.push(entry.path());
            }
        }
    }
    files.sort();
    Ok(files)
}

fn ensure_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if path.exists() {
        let existing = std::fs::read(path)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        return if existing == bytes {
            Ok(())
        } else {
            Err(format!(
                "controlled mutation fixture drifted: {}",
                path.display()
            ))
        };
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    std::fs::write(path, bytes)
        .map_err(|error| format!("could not write {}: {error}", path.display()))
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
