use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::discovery::TestTargetIdentity;
use crate::selection::{
    ProofExecutionUnit, ProofFailurePolicy, RepositoryIdentity, SelectedProofExecutionPlan,
    StoreProofMode, StoreProofRequest, StoreProofSelection, StructuralPreflightReference,
};

use super::{
    command_attempt, execute_validated, ProofAttemptOutcome, ProofUnitExecutionVerdict,
    ValidatedPreflight,
};

struct ScratchPackage {
    root: PathBuf,
}

impl ScratchPackage {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "worth-store-runner-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(
            root.join("Cargo.toml"),
            "[package]\nname='runner-probe'\nversion='0.0.0'\nedition='2021'\n",
        )
        .unwrap();
        std::fs::write(
            root.join("src/lib.rs"),
            "#[cfg(test)]\nmod tests {\n    #[test]\n    fn passes() {}\n    #[test]\n    fn always_fails() { panic!(\"named deterministic failure\"); }\n    #[test]\n    fn sleeps() { std::thread::sleep(std::time::Duration::from_secs(30)); }\n    #[test]\n    fn fails_once() {\n        let marker = std::env::var(\"RUNNER_FLAKE_MARKER\").unwrap();\n        if !std::path::Path::new(&marker).exists() {\n            std::fs::write(marker, b\"first failure retained\").unwrap();\n            panic!(\"admitted first failure\");\n        }\n    }\n}\n",
        )
        .unwrap();
        Self { root }
    }
}

impl Drop for ScratchPackage {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn failed_and_successful_units_retain_independent_attempt_receipts() {
    let scratch = ScratchPackage::new();
    let plan = plan(
        &scratch.root,
        [
            unit(&scratch.root, "always_fails"),
            unit(&scratch.root, "passes"),
        ],
    );
    let preflight = preflight();
    let failed = command_attempt::execute_unit(&scratch.root, &plan, &preflight, "run", 0).unwrap();
    let passed = command_attempt::execute_unit(&scratch.root, &plan, &preflight, "run", 1).unwrap();
    assert!(matches!(
        failed[0].outcome,
        ProofAttemptOutcome::Failed {
            exit_code: Some(101)
        }
    ));
    assert!(passed[0].outcome.passed());
    assert!(attempt_receipt(&scratch.root, &plan, "run", 0, 0).is_file());
    assert!(attempt_receipt(&scratch.root, &plan, "run", 1, 0).is_file());
    assert_ne!(failed[0].stderr.sha256, passed[0].stderr.sha256);
}

#[test]
fn admitted_retry_cannot_turn_fail_then_pass_green() {
    let scratch = ScratchPackage::new();
    let marker = scratch.root.join("flake-marker");
    let mut flake = unit(&scratch.root, "fails_once");
    flake.retry.maximum_retries = 1;
    flake.retry.admitted_exit_codes = BTreeSet::from([101]);
    flake.resources.environment.insert(
        "RUNNER_FLAKE_MARKER".to_owned(),
        marker.to_string_lossy().to_string(),
    );
    let plan = plan(&scratch.root, [flake]);
    let attempts =
        command_attempt::execute_unit(&scratch.root, &plan, &preflight(), "flake", 0).unwrap();
    assert_eq!(attempts.len(), 2);
    assert!(matches!(
        attempts[0].outcome,
        ProofAttemptOutcome::Failed { .. }
    ));
    assert!(attempts[1].outcome.passed());
    let verdict = ProofUnitExecutionVerdict::from_attempts(
        plan.units[0].identity(),
        plan.units[0].case_filter.clone(),
        plan.units[0].process_model,
        &attempts,
    );
    assert_eq!(verdict.behavioral_verdict, "flaky-indeterminate");
    assert!(attempt_receipt(&scratch.root, &plan, "flake", 0, 0).is_file());
    assert!(attempt_receipt(&scratch.root, &plan, "flake", 0, 1).is_file());
}

#[test]
fn timeout_terminates_the_process_tree_and_retains_the_attempt() {
    let scratch = ScratchPackage::new();
    let mut sleeper = unit(&scratch.root, "sleeps");
    sleeper.timeout_millis = 100;
    let plan = plan(&scratch.root, [sleeper]);
    let attempts =
        command_attempt::execute_unit(&scratch.root, &plan, &preflight(), "timeout", 0).unwrap();
    assert_eq!(attempts.len(), 1);
    assert!(matches!(attempts[0].outcome, ProofAttemptOutcome::TimedOut));
    assert!(attempt_receipt(&scratch.root, &plan, "timeout", 0, 0).is_file());
}

#[test]
fn failed_unit_does_not_erase_independent_success_or_run_evidence() {
    let scratch = ScratchPackage::new();
    let mut proof_plan = plan(
        &scratch.root,
        [
            unit(&scratch.root, "always_fails"),
            unit(&scratch.root, "passes"),
        ],
    );
    proof_plan.failure_policy = ProofFailurePolicy::ContinueIndependent;
    let run = execute_validated(&scratch.root, &proof_plan, preflight()).unwrap();
    assert_eq!(run.executed_units, 2);
    assert_eq!(run.passed_units, 1);
    assert_eq!(run.failed_units, 1);
    assert!(run.skipped_units.is_empty());
    assert_eq!(run.attempts.len(), 2);
    assert_eq!(run.behavioral_verdict, "failed");
}

#[test]
fn failed_dependency_skips_only_its_dependents() {
    let scratch = ScratchPackage::new();
    let failed = unit(&scratch.root, "always_fails");
    let mut dependent = unit(&scratch.root, "passes");
    dependent.dependencies.push(failed.identity());
    let mut independent = unit(&scratch.root, "fails_once");
    let marker = scratch.root.join("independent-marker");
    std::fs::write(&marker, b"already admitted").unwrap();
    independent.resources.environment.insert(
        "RUNNER_FLAKE_MARKER".to_owned(),
        marker.to_string_lossy().to_string(),
    );
    let mut proof_plan = plan(&scratch.root, [failed, dependent, independent]);
    proof_plan.failure_policy = ProofFailurePolicy::ContinueIndependent;
    let run = execute_validated(&scratch.root, &proof_plan, preflight()).unwrap();
    assert_eq!(run.executed_units, 2);
    assert_eq!(run.passed_units, 1);
    assert_eq!(run.failed_units, 1);
    assert_eq!(run.skipped_units.len(), 1);
    assert_eq!(run.skipped_units[0].reason, "dependency-failed");
    assert!(run.skipped_units[0]
        .blocking_units
        .contains(&proof_plan.units[0].identity()));
}

fn unit(root: &Path, filter: &str) -> ProofExecutionUnit {
    let target = TestTargetIdentity {
        identity: "runner-probe::lib::runner-probe".to_owned(),
        package: "runner-probe".to_owned(),
        name: "runner-probe".to_owned(),
        kinds: vec!["lib".to_owned()],
        source_path: "src/lib.rs".to_owned(),
        required_features: Vec::new(),
    };
    let request = request();
    let mut unit = ProofExecutionUnit::from_target(&target, &request, Some(filter.to_owned()));
    unit.bind_workspace(root, &request);
    unit
}

fn plan<const N: usize>(root: &Path, units: [ProofExecutionUnit; N]) -> SelectedProofExecutionPlan {
    SelectedProofExecutionPlan::lower(
        root,
        request(),
        StoreProofSelection {
            included_products: vec!["store-ci:test-control".to_owned()],
            included_packages: vec!["runner-probe".to_owned()],
            excluded_packages: BTreeMap::new(),
            included_targets: vec!["runner-probe::lib::runner-probe".to_owned()],
            excluded_targets: BTreeMap::new(),
            included_case_responsibilities: BTreeMap::new(),
            included_fixtures: Vec::new(),
            excluded_fixtures: BTreeMap::new(),
            included_suites: Vec::new(),
            excluded_suites: BTreeMap::new(),
            feature_lanes: Vec::new(),
            build_profiles: Vec::new(),
            subprocess_probes: Vec::new(),
        },
        units.into(),
        None,
        BTreeMap::new(),
        RepositoryIdentity {
            source_revision: "revision".to_owned(),
            source_tree_digest: "tree".to_owned(),
            lockfile_digest: "lock".to_owned(),
            rustc_identity: "rustc".to_owned(),
            operating_system: std::env::consts::OS.to_owned(),
            architecture: std::env::consts::ARCH.to_owned(),
        },
        StructuralPreflightReference::synthetic_for_selection(root),
        None,
    )
    .unwrap()
}

fn request() -> StoreProofRequest {
    StoreProofRequest::new(StoreProofMode::Smoke, None, None, None, None, false)
}

fn preflight() -> ValidatedPreflight {
    ValidatedPreflight {
        evidence_identity: "preflight".to_owned(),
        bundle_path: "preflight.json".to_owned(),
    }
}

fn attempt_receipt(
    root: &Path,
    plan: &SelectedProofExecutionPlan,
    run: &str,
    unit: usize,
    ordinal: usize,
) -> PathBuf {
    root.join(".store-proof/evidence/runs")
        .join(&plan.plan_digest)
        .join(run)
        .join("attempts")
        .join(format!("{unit}-{ordinal:02}.json"))
}
