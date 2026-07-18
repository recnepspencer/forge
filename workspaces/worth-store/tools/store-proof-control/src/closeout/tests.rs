use std::collections::BTreeMap;

use crate::discovery::ObservedArtifactFootprint;
use crate::selection::ProofProcessModel;

use super::*;

#[test]
fn retrospective_policy_preserves_missing_history_as_c2_quarantine() {
    let root = workspace_root();
    let disposition = HistoricalEvidencePolicy::read_and_assess(
        &root.join("test-control/c1-historical-evidence-policy.json"),
        &root.join("test-control/pre-cleanup/baseline-capture-status.json"),
        &root.join("test-control/consolidation-evidence-status.json"),
    )
    .unwrap();
    assert_eq!(disposition.quarantines.len(), 2);
    assert!(disposition
        .prohibited_claims
        .contains("same-seed-pre-consolidation-behavioral-parity"));
    disposition.validate().unwrap();
}

#[test]
fn mutation_report_requires_every_named_defect_and_unrelated_control() {
    let observations: Vec<_> = ControlledDefectKind::ALL
        .into_iter()
        .map(observation)
        .collect();
    let report = ProofMutationSensitivityReport::certify(observations.clone()).unwrap();
    report.validate().unwrap();
    assert_eq!(report.observations().len(), 8);

    let missing = observations.into_iter().skip(1).collect();
    assert!(ProofMutationSensitivityReport::certify(missing)
        .unwrap_err()
        .contains("incomplete"));
}

#[test]
fn developer_envelope_rejects_fake_speed_and_same_process_crash() {
    let cases: Vec<_> = DeveloperEditCase::ALL
        .into_iter()
        .map(iteration_case)
        .collect();
    let envelope = DeveloperIterationEnvelope::certify(reference_profile(), cases.clone()).unwrap();
    envelope.validate().unwrap();

    let missing_expensive_case: Vec<_> = cases
        .clone()
        .into_iter()
        .filter(|case| case.edit.case != DeveloperEditCase::FreshProcessCrashReopen)
        .collect();
    assert!(
        DeveloperIterationEnvelope::certify(reference_profile(), missing_expensive_case)
            .unwrap_err()
            .contains("incomplete")
    );

    let mut same_process = cases;
    let crash = same_process
        .iter_mut()
        .find(|case| case.edit.case == DeveloperEditCase::FreshProcessCrashReopen)
        .unwrap();
    crash.warm.process_models = vec![ProofProcessModel::LibtestProcess];
    assert!(
        DeveloperIterationEnvelope::certify(reference_profile(), same_process)
            .unwrap_err()
            .contains("fresh-process")
    );
}

#[test]
fn source_edit_must_change_and_restore_real_content_identity() {
    let mut receipt = edit_receipt(DeveloperEditCase::PrivateLeafOwner);
    receipt.validate().unwrap();
    receipt.restored_sha256 = "c".repeat(64);
    assert!(receipt.validate().is_err());
}

fn observation(defect: ControlledDefectKind) -> ControlledDefectObservation {
    ControlledDefectObservation::localized(
        defect,
        MutationExecutionEvidence::ProductionValidator {
            validator: "real-validator".to_owned(),
            mutated_subject_sha256: "a".repeat(64),
            denial_sha256: "b".repeat(64),
        },
        vec![InterpretableProofProduct {
            product: if defect.expected_product() == "store-smoke" {
                "store-ui".to_owned()
            } else {
                "store-smoke".to_owned()
            },
            evidence_identity: "c".repeat(64),
            posture: InterpretableProductPosture::ExplicitlyNotSelected,
        }],
    )
    .unwrap()
}

fn iteration_case(case: DeveloperEditCase) -> DeveloperIterationCaseEvidence {
    let product = match case {
        DeveloperEditCase::PrivateLeafOwner => "store-owner:worth-store-physical-format",
        DeveloperEditCase::SharedPhysicalContract => "store-smoke",
        DeveloperEditCase::UiFixtureExpectation => "store-ui",
        DeveloperEditCase::CertificationScenarioAssertion => "store-ci:recovery",
        DeveloperEditCase::FreshProcessCrashReopen => "store-ci:physical-isolation",
    };
    let target_root = format!("C:/workspace/.store-proof/iteration-{case:?}");
    DeveloperIterationCaseEvidence {
        edit: edit_receipt(case),
        cold: run_observation(case, product, target_root.clone(), true),
        warm: run_observation(case, product, target_root, false),
    }
}

fn edit_receipt(case: DeveloperEditCase) -> SourceEditReceipt {
    SourceEditReceipt {
        case,
        source_path: format!("crates/owner/src/{case:?}.rs"),
        original_sha256: "a".repeat(64),
        edited_sha256: "b".repeat(64),
        restored_sha256: "a".repeat(64),
        edit_description: "controlled semantic edit".to_owned(),
    }
}

fn run_observation(
    case: DeveloperEditCase,
    product: &str,
    target_root: String,
    clean: bool,
) -> IterationRunObservation {
    let process_models = if case == DeveloperEditCase::FreshProcessCrashReopen {
        vec![ProofProcessModel::LibtestWithFreshChildProcess]
    } else {
        vec![ProofProcessModel::LibtestProcess]
    };
    IterationRunObservation {
        product: product.to_owned(),
        plan_digest: "a".repeat(64),
        run_identity: "run".to_owned(),
        repository_source_tree_digest: "b".repeat(64),
        target_root: target_root.clone(),
        elapsed_millis: 1_000,
        included_packages: vec!["worth-store-physical-format".to_owned()],
        included_targets: vec!["worth-store-physical-format::lib".to_owned()],
        compiler_artifacts: 1,
        freshly_compiled_artifacts: usize::from(clean),
        reused_artifacts: usize::from(!clean),
        linked_executables: 1,
        externally_observed_processes: 1,
        externally_observed_compilers: usize::from(clean),
        externally_observed_linkers: usize::from(clean),
        process_probe_receipts: usize::from(case == DeveloperEditCase::FreshProcessCrashReopen),
        process_models,
        observer_authorities: vec!["independent-observer-process".to_owned()],
        before: footprint(&target_root, if clean { 1 } else { 4 }),
        after: footprint(&target_root, 5),
    }
}

fn footprint(target_root: &str, file_count: u64) -> ObservedArtifactFootprint {
    ObservedArtifactFootprint {
        target_root: target_root.to_owned(),
        observation_status: "historical_target_observed".to_owned(),
        file_count,
        logical_bytes: file_count * 10,
        produced_executables: u64::from(file_count > 1),
        pdb_files: 0,
        rlib_files: u64::from(file_count > 1),
        rmeta_files: u64::from(file_count > 1),
        incremental_directories: u64::from(file_count > 1),
        extension_counts: BTreeMap::new(),
    }
}

fn reference_profile() -> ReferenceDevelopmentProfile {
    ReferenceDevelopmentProfile {
        operating_system: "windows".to_owned(),
        filesystem: "ntfs".to_owned(),
        cpu: "reference-cpu".to_owned(),
        storage_class: "nvme".to_owned(),
        antivirus_posture: "declared-unknown".to_owned(),
        rust_toolchain: "rustc 1.90.0".to_owned(),
        source_revision: "d".repeat(40),
        lockfile_sha256: "e".repeat(64),
    }
}

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|path| path.join("test-control").is_dir())
        .unwrap()
        .to_path_buf()
}
