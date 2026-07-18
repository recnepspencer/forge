use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

use crate::classification::{validate, ClassifiedInventory};
use crate::closeout::{
    execute_mutation_matrix, CloseoutArtifactReference, DeveloperIterationCaseEvidence,
    DeveloperIterationEnvelope, IterationRunObservation, PreservationAuthorityDigests,
    PreservationCheckedProofRun, ReferenceDevelopmentProfile, SourceEditReceipt,
};
use crate::evidence::{read_json, sha256_file, sha256_serialized, write_immutable_json};
use crate::execution::ExecutedProofRun;
use crate::preservation::{historical_non_case_aggregate_ids, ProofPreservationLedger};
use crate::selection::SelectedProofExecutionPlan;
use crate::{ClassifiedProofInventory, ValidatedProofInventory};

use super::{authority_root, validate_repository};

pub(super) fn preservation(workspace_root: &Path) -> Result<(), String> {
    let (_, report, inventory_path, report_path) = build_preservation(workspace_root)?;
    println!("current proof inventory: {}", inventory_path.display());
    println!("preservation checked run: {}", report_path.display());
    println!(
        "known baseline/current cases: {}/{}",
        report.known_baseline_cases(),
        report.current_cases()
    );
    println!("historical quarantines: {}", report.quarantines().len());
    Ok(())
}

pub(super) fn mutations(workspace_root: &Path) -> Result<(), String> {
    let (inventory, preservation, _, _) = build_preservation(workspace_root)?;
    let report =
        execute_mutation_matrix(workspace_root, &inventory, preservation.evidence_identity())?;
    let path = closeout_root(workspace_root)
        .join("mutation-sensitivity")
        .join(format!("{}.json", report.evidence_identity()));
    write_immutable_json(&path, &report)?;
    println!("mutation sensitivity report: {}", path.display());
    println!(
        "localized controlled defects: {}",
        report.observations().len()
    );
    Ok(())
}

pub(super) fn iteration(workspace_root: &Path, manifest_path: &Path) -> Result<(), String> {
    let envelope = build_iteration(workspace_root, manifest_path)?;
    let path = closeout_root(workspace_root)
        .join("developer-iteration")
        .join(format!("{}.json", envelope.evidence_identity()));
    write_immutable_json(&path, &envelope)?;
    println!("developer iteration envelope: {}", path.display());
    Ok(())
}

pub(super) fn build_iteration(
    workspace_root: &Path,
    manifest_path: &Path,
) -> Result<DeveloperIterationEnvelope, String> {
    let manifest_path = input_path(workspace_root, manifest_path)?;
    let manifest: IterationEvidenceManifest = read_json(&manifest_path)?;
    manifest.validate()?;
    let mut cases = Vec::with_capacity(manifest.cases.len());
    for case in manifest.cases {
        let cold_plan: SelectedProofExecutionPlan =
            read_json(&input_path(workspace_root, Path::new(&case.cold_plan))?)?;
        let cold_run: ExecutedProofRun =
            read_json(&input_path(workspace_root, Path::new(&case.cold_run))?)?;
        let warm_plan: SelectedProofExecutionPlan =
            read_json(&input_path(workspace_root, Path::new(&case.warm_plan))?)?;
        let warm_run: ExecutedProofRun =
            read_json(&input_path(workspace_root, Path::new(&case.warm_run))?)?;
        cases.push(DeveloperIterationCaseEvidence {
            edit: SourceEditReceipt::from_restored_plans(
                workspace_root,
                case.case,
                &cold_plan,
                &warm_plan,
            )?,
            cold: IterationRunObservation::from_evidence(&cold_plan, &cold_run)?,
            warm: IterationRunObservation::from_evidence(&warm_plan, &warm_run)?,
        });
    }
    DeveloperIterationEnvelope::certify(manifest.reference_profile, cases)
}

pub(super) fn build_preservation(
    workspace_root: &Path,
) -> Result<
    (
        ValidatedProofInventory,
        PreservationCheckedProofRun,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let current = validate_repository(workspace_root)?;
    let baseline_path = authority_root(workspace_root).join("classified-proof-inventory.json");
    let baseline: ClassifiedInventory = read_json(&baseline_path)?;
    let baseline_validated = validate(ClassifiedProofInventory::from_discovered(baseline.clone()))
        .map_err(|denials| denials.join("\n  - "))?;
    let ledger_path = authority_root(workspace_root).join("proof-preservation-ledger.json");
    let ledger: ProofPreservationLedger = read_json(&ledger_path)?;
    let baseline_status_path = authority_root(workspace_root).join("baseline-capture-status.json");
    let consolidation_status_path =
        workspace_root.join("test-control/consolidation-evidence-status.json");
    let historical = crate::closeout::HistoricalEvidencePolicy::read_and_assess(
        &workspace_root.join("test-control/c1-historical-evidence-policy.json"),
        &baseline_status_path,
        &consolidation_status_path,
    )?;
    let report = PreservationCheckedProofRun::assess(
        &baseline,
        &baseline_validated,
        &ledger,
        &current,
        &historical_non_case_aggregate_ids(&baseline),
        historical,
        PreservationAuthorityDigests {
            current_executable_listing_sha256: sha256_file(
                &workspace_root.join("test-control/current-executable-listing.json"),
            )?,
            current_behavior_authority_sha256: sha256_file(
                &workspace_root.join("test-control/current-proof-behavior-authority.json"),
            )?,
            post_baseline_authority_sha256: sha256_file(
                &workspace_root.join("test-control/post-baseline-proof-authority.json"),
            )?,
        },
    )?;
    let inventory_identity = sha256_serialized(current.inventory())?;
    let inventory_path = closeout_root(workspace_root)
        .join("current-inventories")
        .join(format!("{inventory_identity}.json"));
    let report_path = closeout_root(workspace_root)
        .join("preservation")
        .join(format!("{}.json", report.evidence_identity()));
    write_immutable_json(&inventory_path, current.inventory())?;
    write_immutable_json(&report_path, &report)?;
    Ok((current, report, inventory_path, report_path))
}

pub(super) fn artifact_reference(
    workspace_root: &Path,
    raw_path: &str,
) -> Result<CloseoutArtifactReference, String> {
    let path = input_path(workspace_root, Path::new(raw_path))?;
    let workspace = workspace_root
        .canonicalize()
        .map_err(|error| format!("could not resolve {}: {error}", workspace_root.display()))?;
    let relative = path.strip_prefix(&workspace).map_err(|_| {
        format!(
            "closeout authority escaped the Worth Store workspace: {}",
            path.display()
        )
    })?;
    Ok(CloseoutArtifactReference {
        repository_relative_path: normalized(relative),
        sha256: sha256_file(&path)?,
    })
}

pub(super) fn read_manifest_input<T: serde::de::DeserializeOwned>(
    workspace_root: &Path,
    path: &str,
) -> Result<T, String> {
    read_json(&input_path(workspace_root, Path::new(path))?)
}

pub(super) fn input_path(workspace_root: &Path, path: &Path) -> Result<PathBuf, String> {
    if path
        .components()
        .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
    {
        return Err(format!(
            "closeout input contains path traversal: {}",
            path.display()
        ));
    }
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace_root.join(path)
    };
    let metadata = std::fs::symlink_metadata(&path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!(
            "closeout input is not a regular file: {}",
            path.display()
        ));
    }
    path.canonicalize()
        .map_err(|error| format!("could not resolve {}: {error}", path.display()))
}

pub(super) fn closeout_root(workspace_root: &Path) -> PathBuf {
    workspace_root.join(".store-proof/evidence/closeout")
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[derive(Debug, Deserialize)]
struct IterationEvidenceManifest {
    schema_version: u32,
    reference_profile: ReferenceDevelopmentProfile,
    cases: Vec<IterationCaseManifest>,
}

#[derive(Debug, Deserialize)]
struct IterationCaseManifest {
    case: crate::closeout::DeveloperEditCase,
    cold_plan: String,
    cold_run: String,
    warm_plan: String,
    warm_run: String,
}

impl IterationEvidenceManifest {
    fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 || self.cases.is_empty() {
            return Err("iteration manifest has an unsupported schema or no cases".to_owned());
        }
        Ok(())
    }
}
