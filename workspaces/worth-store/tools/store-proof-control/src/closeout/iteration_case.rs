use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::discovery::ObservedArtifactFootprint;
use crate::execution::ExecutedProofRun;
use crate::selection::{ProofProcessModel, SelectedProofExecutionPlan, StoreProofMode};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum DeveloperEditCase {
    PrivateLeafOwner,
    SharedPhysicalContract,
    UiFixtureExpectation,
    CertificationScenarioAssertion,
    FreshProcessCrashReopen,
}

impl DeveloperEditCase {
    pub const ALL: [Self; 5] = [
        Self::PrivateLeafOwner,
        Self::SharedPhysicalContract,
        Self::UiFixtureExpectation,
        Self::CertificationScenarioAssertion,
        Self::FreshProcessCrashReopen,
    ];

    fn admits_product(self, product: &str) -> bool {
        match self {
            Self::PrivateLeafOwner => product.starts_with("store-owner:"),
            Self::SharedPhysicalContract => product == "store-smoke",
            Self::UiFixtureExpectation => product == "store-ui",
            Self::CertificationScenarioAssertion => product.starts_with("store-ci:"),
            Self::FreshProcessCrashReopen => {
                matches!(product, "store-ci:physical-isolation" | "store-ci:recovery")
            }
        }
    }

    pub const fn purpose(self) -> &'static str {
        match self {
            Self::PrivateLeafOwner => "private-leaf-owner",
            Self::SharedPhysicalContract => "shared-physical-contract",
            Self::UiFixtureExpectation => "ui-fixture-expectation",
            Self::CertificationScenarioAssertion => "certification-scenario-assertion",
            Self::FreshProcessCrashReopen => "fresh-process-crash-reopen",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceEditReceipt {
    pub case: DeveloperEditCase,
    pub source_path: String,
    pub original_sha256: String,
    pub edited_sha256: String,
    pub restored_sha256: String,
    pub edit_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationRunObservation {
    pub product: String,
    pub plan_digest: String,
    pub run_identity: String,
    pub run_evidence_identity: String,
    pub source_revision: String,
    pub repository_source_tree_digest: String,
    pub lockfile_sha256: String,
    pub rustc_identity: String,
    pub operating_system: String,
    pub architecture: String,
    pub target_root: String,
    pub elapsed_millis: u128,
    pub included_packages: Vec<String>,
    pub included_targets: Vec<String>,
    pub compiler_artifacts: usize,
    pub freshly_compiled_artifacts: usize,
    pub reused_artifacts: usize,
    pub linked_executables: usize,
    pub externally_observed_processes: usize,
    pub externally_observed_compilers: usize,
    pub externally_observed_linkers: usize,
    pub process_probe_receipts: usize,
    pub process_models: Vec<ProofProcessModel>,
    pub observer_authorities: Vec<String>,
    pub before: ObservedArtifactFootprint,
    pub after: ObservedArtifactFootprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperIterationCaseEvidence {
    pub edit: SourceEditReceipt,
    pub cold: IterationRunObservation,
    pub warm: IterationRunObservation,
}

impl SourceEditReceipt {
    pub fn from_restored_plans(
        workspace_root: &Path,
        case: DeveloperEditCase,
        cold: &SelectedProofExecutionPlan,
        warm: &SelectedProofExecutionPlan,
    ) -> Result<Self, String> {
        let cold_edit = cold
            .source_edit
            .as_ref()
            .ok_or_else(|| "cold iteration plan omitted source edit identity".to_owned())?;
        let warm_edit = warm
            .source_edit
            .as_ref()
            .ok_or_else(|| "warm iteration plan omitted source edit identity".to_owned())?;
        if cold_edit != warm_edit
            || cold_edit.purpose != case.purpose()
            || cold.repository.source_tree_digest != warm.repository.source_tree_digest
        {
            return Err("cold and warm plans are not bound to one declared source edit".to_owned());
        }
        let restored = crate::evidence::sha256_file(&workspace_root.join(&cold_edit.source_path))?;
        let receipt = Self {
            case,
            source_path: cold_edit.source_path.clone(),
            original_sha256: cold_edit.original_sha256.clone(),
            edited_sha256: cold_edit.edited_sha256.clone(),
            restored_sha256: restored,
            edit_description: cold_edit.description.clone(),
        };
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn validate(&self) -> Result<(), String> {
        let path = Path::new(&self.source_path);
        if path.is_absolute()
            || path
                .components()
                .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
            || self.source_path.trim().is_empty()
            || self.edit_description.trim().is_empty()
            || !is_sha256(&self.original_sha256)
            || !is_sha256(&self.edited_sha256)
            || !is_sha256(&self.restored_sha256)
            || self.original_sha256 == self.edited_sha256
            || self.original_sha256 != self.restored_sha256
        {
            return Err(format!(
                "source edit receipt is not a real reversible edit: {}",
                self.source_path
            ));
        }
        Ok(())
    }
}

impl IterationRunObservation {
    pub fn from_evidence(
        plan: &SelectedProofExecutionPlan,
        run: &ExecutedProofRun,
    ) -> Result<Self, String> {
        plan.validate_integrity()?;
        run.validate_integrity(plan)?;
        validate_run(plan, run)?;
        let target_cost = run
            .observed_cost
            .target_roots
            .first()
            .ok_or_else(|| "iteration run omitted target-root cost".to_owned())?;
        if run.observed_cost.target_roots.len() != 1 {
            return Err("iteration run must observe exactly one admitted target root".to_owned());
        }
        Ok(Self {
            product: plan.product.clone(),
            plan_digest: plan.plan_digest.clone(),
            run_identity: run.run_identity.clone(),
            run_evidence_identity: run.evidence_identity().to_owned(),
            source_revision: plan.repository.source_revision.clone(),
            repository_source_tree_digest: plan.repository.source_tree_digest.clone(),
            lockfile_sha256: plan.repository.lockfile_digest.clone(),
            rustc_identity: plan.repository.rustc_identity.clone(),
            operating_system: plan.repository.operating_system.clone(),
            architecture: plan.repository.architecture.clone(),
            target_root: target_cost.target_root.clone(),
            elapsed_millis: wall_elapsed(run),
            included_packages: plan.selection.included_packages.clone(),
            included_targets: plan.selection.included_targets.clone(),
            compiler_artifacts: run.observed_cost.cargo_compiler_artifact_messages,
            freshly_compiled_artifacts: run.observed_cost.freshly_compiled_cargo_artifacts,
            reused_artifacts: run.observed_cost.reused_cargo_artifacts,
            linked_executables: run.observed_cost.linked_executable_artifacts.len(),
            externally_observed_processes: run.observed_cost.externally_observed_processes,
            externally_observed_compilers: run.observed_cost.externally_observed_compilers,
            externally_observed_linkers: run.observed_cost.externally_observed_linkers,
            process_probe_receipts: run.observed_cost.declared_subprocess_evidence,
            process_models: plan.units.iter().map(|unit| unit.process_model).collect(),
            observer_authorities: run.observed_cost.observer_authorities.clone(),
            before: target_cost.before.clone(),
            after: target_cost.after.clone(),
        })
    }

    fn validate_common(&self, case: DeveloperEditCase) -> Result<(), String> {
        if !case.admits_product(&self.product)
            || !is_sha256(&self.plan_digest)
            || self.run_identity.trim().is_empty()
            || !is_sha256(&self.run_evidence_identity)
            || !is_revision(&self.source_revision)
            || !is_sha256(&self.repository_source_tree_digest)
            || !is_sha256(&self.lockfile_sha256)
            || !self.rustc_identity.contains("rustc")
            || self.operating_system.trim().is_empty()
            || self.architecture.trim().is_empty()
            || self.target_root.trim().is_empty()
            || self.elapsed_millis == 0
            || self.included_packages.is_empty()
            || self.included_targets.is_empty()
            || self.compiler_artifacts == 0
            || self.externally_observed_processes == 0
            || !self
                .observer_authorities
                .iter()
                .any(|authority| authority == "independent-observer-process")
            || self.before.target_root != self.target_root
            || self.after.target_root != self.target_root
            || self.after.file_count == 0
        {
            return Err(format!(
                "iteration observation for {case:?} lacks structural or external cost evidence"
            ));
        }
        if case == DeveloperEditCase::PrivateLeafOwner && self.included_packages.len() != 1 {
            return Err("owner iteration reached an unrelated proof owner".to_owned());
        }
        if case == DeveloperEditCase::FreshProcessCrashReopen
            && validate_fresh_process_contract(&self.process_models, self.process_probe_receipts)
                .is_err()
        {
            return Err("crash/reopen iteration lacks fresh-process probe receipts".to_owned());
        }
        Ok(())
    }
}

pub(crate) fn validate_fresh_process_contract(
    process_models: &[ProofProcessModel],
    process_probe_receipts: usize,
) -> Result<(), String> {
    if process_probe_receipts == 0
        || !process_models.iter().any(|model| {
            matches!(
                model,
                ProofProcessModel::LibtestWithFreshChildProcess
                    | ProofProcessModel::LibtestWithDeclaredSubprocesses
            )
        })
    {
        Err("fresh-process crash/reopen evidence was replaced by same-process execution".to_owned())
    } else {
        Ok(())
    }
}

impl DeveloperIterationCaseEvidence {
    pub fn validate(&self) -> Result<(), String> {
        self.edit.validate()?;
        self.cold.validate_common(self.edit.case)?;
        self.warm.validate_common(self.edit.case)?;
        if self.cold.product != self.warm.product
            || self.cold.source_revision != self.warm.source_revision
            || self.cold.repository_source_tree_digest != self.warm.repository_source_tree_digest
            || self.cold.lockfile_sha256 != self.warm.lockfile_sha256
            || self.cold.rustc_identity != self.warm.rustc_identity
            || self.cold.operating_system != self.warm.operating_system
            || self.cold.architecture != self.warm.architecture
            || self.cold.target_root != self.warm.target_root
            || !clean_artifact_root(&self.cold.before)
            || self.warm.before.file_count == 0
        {
            return Err(format!(
                "iteration case {:?} does not carry independent clean and warmed roots",
                self.edit.case
            ));
        }
        if self.edit.case == DeveloperEditCase::SharedPhysicalContract
            && self.warm.elapsed_millis >= 60_000
        {
            return Err(format!(
                "warm store-smoke exceeded one minute: {}ms",
                self.warm.elapsed_millis
            ));
        }
        if self.edit.case == DeveloperEditCase::PrivateLeafOwner
            && self.warm.elapsed_millis >= 15_000
        {
            return Err(format!(
                "warm owner feedback no longer targets seconds: {}ms",
                self.warm.elapsed_millis
            ));
        }
        Ok(())
    }
}

fn clean_artifact_root(footprint: &ObservedArtifactFootprint) -> bool {
    footprint.file_count <= 1
        && footprint.produced_executables == 0
        && footprint.pdb_files == 0
        && footprint.rlib_files == 0
        && footprint.rmeta_files == 0
        && footprint.incremental_directories == 0
}

fn validate_run(plan: &SelectedProofExecutionPlan, run: &ExecutedProofRun) -> Result<(), String> {
    if run.plan_digest != plan.plan_digest
        || run.behavioral_verdict != "passed"
        || run.executed_units != run.planned_units
        || run.failed_units != 0
        || !run.skipped_units.is_empty()
        || run.attempts.len() != run.executed_units
        || run.attempts.iter().any(|attempt| {
            attempt.ordinal != 0
                || !attempt.outcome.passed()
                || attempt.plan_digest != plan.plan_digest
        })
    {
        return Err(
            "iteration evidence is failed, flaky, skipped, retried, or plan-mismatched".to_owned(),
        );
    }
    if plan.request.mode() == StoreProofMode::Owner && plan.selection.included_packages.len() != 1 {
        return Err("owner iteration plan crossed an owner target boundary".to_owned());
    }
    Ok(())
}

fn wall_elapsed(run: &ExecutedProofRun) -> u128 {
    let first = run
        .attempts
        .iter()
        .map(|attempt| attempt.started_unix_millis)
        .min()
        .unwrap_or(run.run_started_unix_millis);
    let last = run
        .attempts
        .iter()
        .map(|attempt| {
            attempt
                .started_unix_millis
                .saturating_add(attempt.elapsed_millis)
        })
        .max()
        .unwrap_or(first);
    last.saturating_sub(first)
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_revision(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
