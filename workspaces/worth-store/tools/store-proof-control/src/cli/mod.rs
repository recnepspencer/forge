mod arguments;
mod presentation;

use std::path::{Path, PathBuf};
use std::time::Instant;

use arguments::{CliCommand, ParsedArguments};

use crate::classification::{
    build_consolidated_suite_inventory, classify, classify_from_authority, validate,
    validate_inventory_build_graph_policy, validate_proof_behavior_authority,
    validate_suite_semantic_authority, ClassifiedInventory, ConsolidatedSuiteInventory,
    ConsolidationEvidenceStatus, PostBaselineProofAuthority, ProofBehaviorAuthority,
    ProofSemanticDeclaration,
};
use crate::discovery::{
    discover_workspace, generate_owner_build_closures, observe_executable_listing,
    validate_executable_listing, validate_owner_build_closures, BaselineCaptureStatus,
    CurrentExecutableListing, TestSurfaceInventory,
};
use crate::evidence::{
    evidence_plan_path, read_json, write_immutable_json, write_json, write_new_json,
};
use crate::execution::execute;
use crate::preservation::{
    build_ledger, historical_non_case_aggregate_ids, semantic_authority_from_ledger,
    validate_current_reachability, validate_ledger, ProofPreservationLedger,
};
use crate::selection::select;
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
        CliCommand::Proof(request) => run_product(&workspace_root, request),
    }
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

fn validate_repository(workspace_root: &Path) -> Result<crate::ValidatedProofInventory, String> {
    let started = Instant::now();
    let baseline_root = authority_root(workspace_root);
    let baseline_discovery: TestSurfaceInventory =
        read_json(&baseline_root.join("discovered-test-surface.json"))?;
    let baseline_classified: ClassifiedInventory =
        read_json(&baseline_root.join("classified-proof-inventory.json"))?;
    let ledger: ProofPreservationLedger =
        read_json(&baseline_root.join("proof-preservation-ledger.json"))?;
    let baseline_capture: BaselineCaptureStatus =
        read_json(&baseline_root.join("baseline-capture-status.json"))?;
    let post_baseline: PostBaselineProofAuthority =
        read_json(&workspace_root.join("test-control/post-baseline-proof-authority.json"))?;
    baseline_capture.validate()?;
    if baseline_discovery.cases.len() != baseline_classified.proofs.len() {
        return Err("baseline discovery/classification cardinality mismatch".to_owned());
    }
    let baseline_validated = validate(ClassifiedProofInventory::from_discovered(
        baseline_classified.clone(),
    ))
    .map_err(join_violations)?;
    validate_ledger(&baseline_validated, &ledger).map_err(join_violations)?;
    let current_authority = semantic_authority_from_ledger(&baseline_classified, &ledger)?;
    let baseline_elapsed = started.elapsed();
    let current = current_inventory(workspace_root, &current_authority, &post_baseline)?;
    let behavior_authority: ProofBehaviorAuthority =
        read_json(&workspace_root.join("test-control/current-proof-behavior-authority.json"))?;
    validate_proof_behavior_authority(&behavior_authority, current.inventory())
        .map_err(join_violations)?;
    let discovery_elapsed = started.elapsed();
    let executable_listing: CurrentExecutableListing =
        read_json(&workspace_root.join("test-control/current-executable-listing.json"))?;
    validate_executable_listing(&current.inventory().discovered, &executable_listing)
        .map_err(join_violations)?;
    validate_current_reachability(
        &ledger,
        &current,
        &historical_non_case_aggregate_ids(&baseline_classified),
    )
    .map_err(join_violations)?;
    let owner_closures = generate_owner_build_closures(&current.inventory().discovered);
    validate_owner_build_closures(&owner_closures).map_err(join_violations)?;
    let suites = build_consolidated_suite_inventory(workspace_root, current.inventory())
        .map_err(join_violations)?;
    let suite_authority: ConsolidatedSuiteInventory =
        read_json(&workspace_root.join("test-control/scenario-semantic-authority.json"))?;
    validate_suite_semantic_authority(&suite_authority, &suites).map_err(join_violations)?;
    let consolidation_status: ConsolidationEvidenceStatus =
        read_json(&workspace_root.join("test-control/consolidation-evidence-status.json"))?;
    consolidation_status.validate()?;
    let topology_elapsed = started.elapsed();
    write_json(
        &workspace_root.join("test-control/owner-build-closures.json"),
        &owner_closures,
    )?;
    write_json(
        &workspace_root.join("test-control/consolidated-suite-inventory.json"),
        &suites,
    )?;
    println!(
        "proof preservation valid: {} baseline cases remain reachable through {} current targets",
        ledger.rows.len(),
        current.inventory().discovered.targets.len()
    );
    if !baseline_capture.closeout_eligible {
        println!("baseline limitation: {}", baseline_capture.limitation);
    }
    if !consolidation_status.closeout_eligible {
        println!(
            "consolidation limitation: {}",
            consolidation_status.limitation
        );
    }
    println!(
        "validation timing: baseline={}ms discovery={}ms topology={}ms total={}ms",
        baseline_elapsed.as_millis(),
        (discovery_elapsed - baseline_elapsed).as_millis(),
        (topology_elapsed - discovery_elapsed).as_millis(),
        started.elapsed().as_millis()
    );
    Ok(current)
}

fn run_product(
    workspace_root: &Path,
    request: crate::selection::StoreProofRequest,
) -> Result<(), String> {
    let inventory = validate_repository(workspace_root)?;
    let plan = select(workspace_root, &inventory, request).map_err(|error| error.to_string())?;
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
        .join(format!("{}.json", run.attempt_identity));
    write_new_json(&run_path, &run)?;
    if run.behavioral_verdict == "passed" {
        Ok(())
    } else {
        Err(format!("proof run failed at {:?}", run.failed_unit))
    }
}

fn current_inventory(
    workspace_root: &Path,
    semantic_authority: &ClassifiedInventory,
    post_baseline: &PostBaselineProofAuthority,
) -> Result<crate::ValidatedProofInventory, String> {
    let started = Instant::now();
    let discovered = discover_workspace(workspace_root, false)?;
    let discovery_elapsed = started.elapsed();
    validate_inventory_build_graph_policy(discovered.inventory()).map_err(|violations| {
        violations
            .into_iter()
            .map(|violation| violation.to_string())
            .collect::<Vec<_>>()
            .join("\n  - ")
    })?;
    let classified = classify_from_authority(discovered, semantic_authority, post_baseline)
        .map_err(join_violations)?;
    let validated = validate(classified).map_err(join_violations)?;
    println!(
        "discovery timing: surface={}ms classification={}ms",
        discovery_elapsed.as_millis(),
        (started.elapsed() - discovery_elapsed).as_millis()
    );
    Ok(validated)
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
