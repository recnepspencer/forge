mod arguments;
mod artifact_commands;
mod presentation;
mod repository_validation;

use std::path::{Path, PathBuf};

use arguments::{CliCommand, ParsedArguments};
use repository_validation::{current_inventory, validate_repository, validate_repository_inputs};

use crate::classification::{
    build_consolidated_suite_inventory, classify, validate, validate_proof_behavior_authority,
    ClassifiedInventory, PostBaselineProofAuthority, ProofBehaviorAuthority,
    ProofSemanticDeclaration,
};
use crate::discovery::{
    discover_workspace, observe_executable_listing, validate_executable_listing,
    BaselineCaptureStatus,
};
use crate::evidence::{
    evidence_plan_path, read_json, write_immutable_json, write_json, write_new_json,
};
use crate::execution::execute;
use crate::preservation::{
    build_ledger, semantic_authority_from_ledger, validate_ledger, ProofPreservationLedger,
};
use crate::selection::{select, StructuralPreflightReference};
use crate::structural_preflight::{
    consume as consume_preflight, execute as execute_preflight, forge_root,
};
use crate::{ClassifiedProofInventory, DiscoveredTestSurface};

pub fn run(arguments: impl Iterator<Item = String>) -> Result<(), String> {
    let parsed = ParsedArguments::parse(arguments)?;
    let workspace_root = workspace_root()?;
    match parsed.command {
        CliCommand::Baseline { observe_artifacts } => {
            create_baseline(&workspace_root, observe_artifacts)
        }
        CliCommand::Validate => validate_repository(&workspace_root).map(|_| ()),
        CliCommand::AuditExecutableListing => audit_executable_listing(&workspace_root),
        CliCommand::SealProofAuthority => seal_proof_authority(&workspace_root),
        CliCommand::SealProofBehaviorAuthority => seal_proof_behavior_authority(&workspace_root),
        CliCommand::SealScenarioAuthority => seal_scenario_authority(&workspace_root),
        CliCommand::InternalObserve { request_path } => {
            crate::execution::observe_external_request(Path::new(&request_path))
        }
        CliCommand::CiAggregate { evidence_root } => {
            aggregate_ci_evidence(&workspace_root, Path::new(&evidence_root))
        }
        CliCommand::ArtifactInspect {
            target_root,
            protected_run,
        } => artifact_commands::inspect_artifacts(
            &workspace_root,
            Path::new(&target_root),
            protected_run.as_deref().map(Path::new),
        ),
        CliCommand::ArtifactPlan {
            inventory_path,
            policy,
        } => artifact_commands::plan_artifact_cleanup(Path::new(&inventory_path), &policy),
        CliCommand::ArtifactExecute { plan_path } => {
            artifact_commands::execute_artifact_cleanup(Path::new(&plan_path))
        }
        CliCommand::Proof {
            request,
            preflight_bundle,
        } => run_product(&workspace_root, request, preflight_bundle.as_deref()),
    }
}

fn aggregate_ci_evidence(workspace_root: &Path, evidence_root: &Path) -> Result<(), String> {
    let inventory = validate_repository(workspace_root)?;
    let evidence = crate::ci::read_partition_evidence(evidence_root)?;
    let aggregate =
        crate::ci::CiCertificationAggregate::certify(&inventory, &evidence).map_err(|missing| {
            missing
                .into_iter()
                .map(|lane| {
                    format!(
                        "{}/{}: {}",
                        lane.partition, lane.operating_system, lane.reason
                    )
                })
                .collect::<Vec<_>>()
                .join("\n  - ")
        })?;
    let path = aggregate.output_path(workspace_root);
    write_new_json(&path, &aggregate)?;
    println!("CI certification aggregate: {}", path.display());
    println!("CI source identity: {}", aggregate.source_identity);
    Ok(())
}

fn seal_proof_behavior_authority(workspace_root: &Path) -> Result<(), String> {
    let baseline: ClassifiedInventory =
        read_json(&authority_root(workspace_root).join("classified-proof-inventory.json"))?;
    let ledger: ProofPreservationLedger =
        read_json(&authority_root(workspace_root).join("proof-preservation-ledger.json"))?;
    let post_baseline: PostBaselineProofAuthority =
        read_json(&workspace_root.join("test-control/post-baseline-proof-authority.json"))?;
    let semantic_authority = semantic_authority_from_ledger(&baseline, &ledger)?;
    let current = current_inventory(workspace_root, &semantic_authority, &post_baseline)?;
    let authority = ProofBehaviorAuthority::from_inventory(current.inventory());
    let path = workspace_root.join("test-control/current-proof-behavior-authority.json");
    write_new_json(&path, &authority)?;
    println!(
        "sealed {} current proof behavior fingerprints at {}",
        authority.declarations.len(),
        path.display()
    );
    Ok(())
}

fn audit_executable_listing(workspace_root: &Path) -> Result<(), String> {
    let baseline: ClassifiedInventory =
        read_json(&authority_root(workspace_root).join("classified-proof-inventory.json"))?;
    let ledger: ProofPreservationLedger =
        read_json(&authority_root(workspace_root).join("proof-preservation-ledger.json"))?;
    let post_baseline: PostBaselineProofAuthority =
        read_json(&workspace_root.join("test-control/post-baseline-proof-authority.json"))?;
    let authority = semantic_authority_from_ledger(&baseline, &ledger)?;
    let current = current_inventory(workspace_root, &authority, &post_baseline)?;
    let listing = observe_executable_listing(workspace_root, &current.inventory().discovered)?;
    validate_executable_listing(&current.inventory().discovered, &listing)
        .map_err(join_violations)?;
    let path = workspace_root.join("test-control/current-executable-listing.json");
    write_json(&path, &listing)?;
    println!(
        "audited {} libtest targets and {} rustdoc targets at {}",
        listing.libtest_targets.len(),
        listing.rustdoc_targets.len(),
        path.display()
    );
    Ok(())
}

fn seal_proof_authority(workspace_root: &Path) -> Result<(), String> {
    let baseline: ClassifiedInventory =
        read_json(&authority_root(workspace_root).join("classified-proof-inventory.json"))?;
    let baseline_ids: std::collections::BTreeSet<_> = baseline
        .proofs
        .iter()
        .map(|proof| proof.case.identity.stable_id.as_str())
        .collect();
    let classified = classify(discover_workspace(workspace_root, false)?);
    let declarations = classified
        .inventory()
        .proofs
        .iter()
        .filter(|proof| !baseline_ids.contains(proof.case.identity.stable_id.as_str()))
        .filter(|proof| proof.case.identity.package != "store-proof-control")
        .filter(|proof| {
            !matches!(
                proof.case.kind,
                crate::discovery::CaseKind::DoctestCompileFail
                    | crate::discovery::CaseKind::DoctestIgnored
                    | crate::discovery::CaseKind::DoctestRunnable
            )
        })
        .map(|proof| ProofSemanticDeclaration {
            stable_case_id: proof.case.identity.stable_id.clone(),
            family: proof.family,
            owner: proof.owner.clone(),
            products: proof.products.clone(),
            disposition: proof.disposition,
            expected_evidence: proof.expected_evidence.clone(),
            physical_reality_audit_required: proof.physical_reality_audit_required,
        })
        .collect();
    let authority = PostBaselineProofAuthority {
        schema_version: 1,
        declarations,
    };
    let path = workspace_root.join("test-control/post-baseline-proof-authority.json");
    write_new_json(&path, &authority)?;
    println!(
        "sealed {} post-baseline proof semantics at {}",
        authority.declarations.len(),
        path.display()
    );
    Ok(())
}

fn seal_scenario_authority(workspace_root: &Path) -> Result<(), String> {
    let baseline: ClassifiedInventory =
        read_json(&authority_root(workspace_root).join("classified-proof-inventory.json"))?;
    let post_baseline: PostBaselineProofAuthority =
        read_json(&workspace_root.join("test-control/post-baseline-proof-authority.json"))?;
    let ledger: ProofPreservationLedger =
        read_json(&authority_root(workspace_root).join("proof-preservation-ledger.json"))?;
    let current_authority = semantic_authority_from_ledger(&baseline, &ledger)?;
    let current = current_inventory(workspace_root, &current_authority, &post_baseline)?;
    let behavior_authority: ProofBehaviorAuthority =
        read_json(&workspace_root.join("test-control/current-proof-behavior-authority.json"))?;
    validate_proof_behavior_authority(&behavior_authority, current.inventory())
        .map_err(join_violations)?;
    let suites = build_consolidated_suite_inventory(workspace_root, current.inventory())
        .map_err(join_violations)?;
    let path = workspace_root.join("test-control/scenario-semantic-authority.json");
    write_new_json(&path, &suites)?;
    println!(
        "sealed {} scenario suites at {}",
        suites.suites.len(),
        path.display()
    );
    Ok(())
}

fn create_baseline(workspace_root: &Path, observe_artifacts: bool) -> Result<(), String> {
    let discovered = discover_workspace(workspace_root, observe_artifacts)?;
    let classified = classify(discovered);
    let validated = validate(classified).map_err(join_violations)?;
    let ledger = build_ledger(&validated);
    let capture_status = BaselineCaptureStatus::topology_only(
        validated
            .inventory()
            .discovered
            .historical_artifacts
            .observation_status
            .clone(),
    );
    validate_ledger(&validated, &ledger).map_err(join_violations)?;
    let authority_root = authority_root(workspace_root);
    write_new_json(
        &authority_root.join("discovered-test-surface.json"),
        &validated.inventory().discovered,
    )?;
    write_new_json(
        &authority_root.join("classified-proof-inventory.json"),
        validated.inventory(),
    )?;
    write_new_json(
        &authority_root.join("proof-preservation-ledger.json"),
        &ledger,
    )?;
    write_new_json(
        &authority_root.join("baseline-capture-status.json"),
        &capture_status,
    )?;
    println!(
        "baseline frozen: {} targets, {} proof cases, {} ledger rows",
        validated.inventory().discovered.targets.len(),
        validated.inventory().proofs.len(),
        ledger.rows.len()
    );
    Ok(())
}

fn run_product(
    workspace_root: &Path,
    request: crate::selection::StoreProofRequest,
    preflight_bundle: Option<&str>,
) -> Result<(), String> {
    let validation = validate_repository_inputs(workspace_root);
    let forge_root = forge_root(workspace_root)?;
    let preflight = match preflight_bundle {
        Some(path) => consume_preflight(&forge_root, request.mode(), Path::new(path))?,
        None => execute_preflight(
            &forge_root,
            workspace_root,
            request.mode(),
            validation.as_ref().ok(),
            validation.as_ref().err(),
        )?,
    };
    let failures = preflight.evidence.failures();
    if !failures.is_empty() {
        return Err(format!(
            "structural preflight {} failed:\n  - {}",
            preflight.bundle_path.display(),
            failures
                .iter()
                .map(|failure| format!(
                    "{:?}/{}: {}",
                    failure.predicate, failure.failure_code, failure.message
                ))
                .collect::<Vec<_>>()
                .join("\n  - ")
        ));
    }
    let inventory = validation.map_err(|failure| failure.to_string())?;
    let preflight_reference =
        StructuralPreflightReference::from_evidence(&preflight.bundle_path, &preflight.evidence);
    let plan = select(workspace_root, &inventory, request, preflight_reference)
        .map_err(|error| error.to_string())?;
    presentation::print_plan(&plan);
    write_immutable_json(
        &evidence_plan_path(workspace_root, &plan.plan_digest),
        &plan,
    )?;
    if plan.request.plan_only() {
        println!("plan-only request: no test process was started");
        return Ok(());
    }
    let run = execute(workspace_root, &plan)?;
    presentation::print_run(&plan, &run);
    let run_path = workspace_root
        .join(".store-proof/evidence/runs")
        .join(&plan.plan_digest)
        .join(format!("{}.json", run.run_identity));
    write_new_json(&run_path, &run)?;
    if let Some(partition) = plan.request.semantic_ci_partition() {
        let partition = partition.identity();
        let evidence = crate::ci::CiPartitionEvidence::from_run(
            workspace_root,
            partition,
            &plan,
            &run,
            plan.ci_shard_plan.clone(),
        )?;
        let evidence_path = evidence.output_path(workspace_root);
        write_new_json(&evidence_path, &evidence)?;
        println!("CI partition evidence: {}", evidence_path.display());
        println!("CI closeout eligibility: {}", evidence.closeout_eligible);
    }
    if run.behavioral_verdict == "passed" {
        Ok(())
    } else {
        Err(format!("proof run failed at {:?}", run.failed_unit))
    }
}

fn authority_root(workspace_root: &Path) -> PathBuf {
    workspace_root.join("test-control/pre-cleanup")
}

fn workspace_root() -> Result<PathBuf, String> {
    let current = std::env::current_dir()
        .map_err(|error| format!("could not observe current directory: {error}"))?;
    if current.join("Cargo.toml").exists() && current.join("crates").exists() {
        return Ok(current);
    }
    let output = std::process::Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .map_err(|error| format!("could not locate workspace: {error}"))?;
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("could not decode workspace location: {error}"))?;
    value["workspace_root"]
        .as_str()
        .map(PathBuf::from)
        .ok_or_else(|| "cargo metadata omitted workspace_root".to_owned())
}

fn join_violations(violations: Vec<String>) -> String {
    violations.join("\n  - ")
}

#[allow(dead_code)]
fn _phase_types_are_not_publicly_constructible(
    _discovered: DiscoveredTestSurface,
    _classified: ClassifiedProofInventory,
) {
}
