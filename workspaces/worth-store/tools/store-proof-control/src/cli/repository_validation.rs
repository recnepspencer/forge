use std::path::Path;
use std::time::Instant;

use crate::classification::{
    build_consolidated_suite_inventory, classify_from_authority, validate,
    validate_inventory_build_graph_policy, validate_proof_behavior_authority,
    validate_suite_semantic_authority, ClassifiedInventory, ConsolidatedSuiteInventory,
    ConsolidationEvidenceStatus, PostBaselineProofAuthority, ProofBehaviorAuthority,
};
use crate::discovery::{
    generate_owner_build_closures, validate_executable_listing, validate_owner_build_closures,
    BaselineCaptureStatus, CurrentExecutableListing, TestSurfaceInventory,
};
use crate::evidence::{read_json, write_json};
use crate::preservation::{
    historical_non_case_aggregate_ids, semantic_authority_from_ledger,
    validate_current_reachability, validate_ledger, ProofPreservationLedger,
};
use crate::ClassifiedProofInventory;

use super::{authority_root, join_violations};

pub(super) fn validate_repository(
    workspace_root: &Path,
) -> Result<crate::ValidatedProofInventory, String> {
    let inventory = validate_repository_inputs(workspace_root)?;
    validate_inventory_build_graph_policy(&inventory.inventory().discovered).map_err(
        |violations| {
            join_violations(
                violations
                    .into_iter()
                    .map(|item| item.to_string())
                    .collect(),
            )
        },
    )?;
    Ok(inventory)
}

pub(super) fn validate_repository_inputs(
    workspace_root: &Path,
) -> Result<crate::ValidatedProofInventory, String> {
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
    let current = current_inventory_without_build_graph_policy(
        workspace_root,
        &current_authority,
        &post_baseline,
    )?;
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

pub(super) fn current_inventory(
    workspace_root: &Path,
    semantic_authority: &ClassifiedInventory,
    post_baseline: &PostBaselineProofAuthority,
) -> Result<crate::ValidatedProofInventory, String> {
    let inventory = current_inventory_without_build_graph_policy(
        workspace_root,
        semantic_authority,
        post_baseline,
    )?;
    validate_inventory_build_graph_policy(&inventory.inventory().discovered).map_err(
        |violations| {
            violations
                .into_iter()
                .map(|violation| violation.to_string())
                .collect::<Vec<_>>()
                .join("\n  - ")
        },
    )?;
    Ok(inventory)
}

fn current_inventory_without_build_graph_policy(
    workspace_root: &Path,
    semantic_authority: &ClassifiedInventory,
    post_baseline: &PostBaselineProofAuthority,
) -> Result<crate::ValidatedProofInventory, String> {
    let started = Instant::now();
    let discovered = crate::discovery::discover_workspace(workspace_root, false)?;
    let discovery_elapsed = started.elapsed();
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
