use std::path::Path;
use std::time::Instant;

use crate::classification::{
    build_consolidated_suite_inventory, classify_from_authority, validate,
    validate_inventory_build_graph_policy, validate_proof_behavior_authority,
    validate_proof_behavior_authority_for_source_edit, validate_suite_semantic_authority,
    validate_suite_semantic_authority_for_source_edit, ClassifiedInventory,
    ConsolidatedSuiteInventory, ConsolidationEvidenceStatus, PostBaselineProofAuthority,
    ProofBehaviorAuthority,
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
use crate::selection::ObservedSourceEditIdentity;
use crate::structural_preflight::{RepositoryPredicateFailure, StructuralPredicate};
use crate::ClassifiedProofInventory;

use super::{authority_root, join_violations};

pub(super) fn validate_repository(
    workspace_root: &Path,
) -> Result<crate::ValidatedProofInventory, String> {
    let inventory =
        validate_repository_inputs(workspace_root).map_err(|failure| failure.to_string())?;
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
) -> Result<crate::ValidatedProofInventory, RepositoryPredicateFailure> {
    validate_repository_inputs_with_source_edit(workspace_root, None)
}

pub(super) fn validate_repository_inputs_with_source_edit(
    workspace_root: &Path,
    source_edit: Option<&ObservedSourceEditIdentity>,
) -> Result<crate::ValidatedProofInventory, RepositoryPredicateFailure> {
    let started = Instant::now();
    let fingerprint_waiver = admitted_fingerprint_waiver(source_edit)
        .map_err(|error| inventory_failure("source_edit_scope_invalid", error, workspace_root))?;
    let baseline_root = authority_root(workspace_root);
    let baseline_discovery_path = baseline_root.join("discovered-test-surface.json");
    let baseline_discovery: TestSurfaceInventory =
        read_json(&baseline_discovery_path).map_err(|error| {
            preservation_failure(
                "baseline_discovery_unavailable",
                error,
                &baseline_discovery_path,
            )
        })?;
    let baseline_classified_path = baseline_root.join("classified-proof-inventory.json");
    let baseline_classified: ClassifiedInventory =
        read_json(&baseline_classified_path).map_err(|error| {
            preservation_failure(
                "baseline_classification_unavailable",
                error,
                &baseline_classified_path,
            )
        })?;
    let ledger_path = baseline_root.join("proof-preservation-ledger.json");
    let ledger: ProofPreservationLedger = read_json(&ledger_path).map_err(|error| {
        preservation_failure("preservation_ledger_unavailable", error, &ledger_path)
    })?;
    let baseline_capture_path = baseline_root.join("baseline-capture-status.json");
    let baseline_capture: BaselineCaptureStatus =
        read_json(&baseline_capture_path).map_err(|error| {
            preservation_failure("baseline_status_unavailable", error, &baseline_capture_path)
        })?;
    let post_baseline_path = workspace_root.join("test-control/post-baseline-proof-authority.json");
    let post_baseline: PostBaselineProofAuthority =
        read_json(&post_baseline_path).map_err(|error| {
            preservation_failure(
                "post_baseline_authority_unavailable",
                error,
                &post_baseline_path,
            )
        })?;
    baseline_capture.validate().map_err(|error| {
        preservation_failure("baseline_status_invalid", error, &baseline_capture_path)
    })?;
    if baseline_discovery.cases.len() != baseline_classified.proofs.len() {
        return Err(preservation_failure(
            "baseline_cardinality_mismatch",
            "baseline discovery/classification cardinality mismatch",
            &baseline_classified_path,
        ));
    }
    let baseline_validated = validate(ClassifiedProofInventory::from_discovered(
        baseline_classified.clone(),
    ))
    .map_err(|violations| {
        preservation_failure(
            "baseline_inventory_invalid",
            join_violations(violations),
            &baseline_classified_path,
        )
    })?;
    validate_ledger(&baseline_validated, &ledger).map_err(|violations| {
        preservation_failure(
            "preservation_ledger_invalid",
            join_violations(violations),
            &ledger_path,
        )
    })?;
    let current_authority =
        semantic_authority_from_ledger(&baseline_classified, &ledger).map_err(|error| {
            preservation_failure("preservation_authority_invalid", error, &ledger_path)
        })?;
    let baseline_elapsed = started.elapsed();
    let current = current_inventory_without_build_graph_policy(
        workspace_root,
        &current_authority,
        &post_baseline,
    )
    .map_err(|error| inventory_failure("current_inventory_invalid", error, workspace_root))?;
    let behavior_authority_path =
        workspace_root.join("test-control/current-proof-behavior-authority.json");
    let behavior_authority: ProofBehaviorAuthority =
        read_json(&behavior_authority_path).map_err(|error| {
            inventory_failure(
                "behavior_authority_unavailable",
                error,
                &behavior_authority_path,
            )
        })?;
    let behavior_validation = fingerprint_waiver.map_or_else(
        || validate_proof_behavior_authority(&behavior_authority, current.inventory()),
        |source_path| {
            validate_proof_behavior_authority_for_source_edit(
                &behavior_authority,
                current.inventory(),
                source_path,
            )
        },
    );
    behavior_validation.map_err(|violations| {
        inventory_failure(
            "behavior_authority_invalid",
            join_violations(violations),
            &behavior_authority_path,
        )
    })?;
    let discovery_elapsed = started.elapsed();
    let executable_listing_path =
        workspace_root.join("test-control/current-executable-listing.json");
    let executable_listing: CurrentExecutableListing = read_json(&executable_listing_path)
        .map_err(|error| {
            inventory_failure(
                "executable_listing_unavailable",
                error,
                &executable_listing_path,
            )
        })?;
    validate_executable_listing(&current.inventory().discovered, &executable_listing).map_err(
        |violations| {
            inventory_failure(
                "executable_listing_invalid",
                join_violations(violations),
                &executable_listing_path,
            )
        },
    )?;
    validate_current_reachability(
        &ledger,
        &current,
        &historical_non_case_aggregate_ids(&baseline_classified),
    )
    .map_err(|violations| {
        preservation_failure(
            "current_reachability_invalid",
            join_violations(violations),
            &ledger_path,
        )
    })?;
    let owner_closures = generate_owner_build_closures(&current.inventory().discovered);
    validate_owner_build_closures(&owner_closures).map_err(|violations| {
        inventory_failure(
            "owner_build_closure_invalid",
            join_violations(violations),
            workspace_root,
        )
    })?;
    let suites = build_consolidated_suite_inventory(workspace_root, current.inventory()).map_err(
        |violations| {
            inventory_failure(
                "suite_inventory_invalid",
                join_violations(violations),
                workspace_root,
            )
        },
    )?;
    let suite_authority_path = workspace_root.join("test-control/scenario-semantic-authority.json");
    let suite_authority: ConsolidatedSuiteInventory =
        read_json(&suite_authority_path).map_err(|error| {
            inventory_failure(
                "scenario_authority_unavailable",
                error,
                &suite_authority_path,
            )
        })?;
    let suite_validation = fingerprint_waiver.map_or_else(
        || validate_suite_semantic_authority(&suite_authority, &suites),
        |source_path| {
            validate_suite_semantic_authority_for_source_edit(
                &suite_authority,
                &suites,
                source_path,
            )
        },
    );
    suite_validation.map_err(|violations| {
        inventory_failure(
            "scenario_authority_invalid",
            join_violations(violations),
            &suite_authority_path,
        )
    })?;
    let consolidation_status_path =
        workspace_root.join("test-control/consolidation-evidence-status.json");
    let consolidation_status: ConsolidationEvidenceStatus = read_json(&consolidation_status_path)
        .map_err(|error| {
        preservation_failure(
            "consolidation_status_unavailable",
            error,
            &consolidation_status_path,
        )
    })?;
    consolidation_status.validate().map_err(|error| {
        preservation_failure(
            "consolidation_status_invalid",
            error,
            &consolidation_status_path,
        )
    })?;
    let topology_elapsed = started.elapsed();
    write_json(
        &workspace_root.join("test-control/owner-build-closures.json"),
        &owner_closures,
    )
    .map_err(|error| inventory_failure("owner_projection_write_failed", error, workspace_root))?;
    write_json(
        &workspace_root.join("test-control/consolidated-suite-inventory.json"),
        &suites,
    )
    .map_err(|error| inventory_failure("suite_projection_write_failed", error, workspace_root))?;
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

fn admitted_fingerprint_waiver(
    edit: Option<&ObservedSourceEditIdentity>,
) -> Result<Option<&str>, String> {
    let Some(edit) = edit else {
        return Ok(None);
    };
    if !matches!(
        edit.purpose.as_str(),
        "certification-scenario-assertion" | "fresh-process-crash-reopen"
    ) {
        return Ok(None);
    }
    if edit
        .source_path
        .starts_with("crates/worth-store-certification/tests/scenarios/")
        && edit.source_path.ends_with(".rs")
    {
        Ok(Some(&edit.source_path))
    } else {
        Err(format!(
            "{} may waive fingerprints only for a certification scenario source",
            edit.purpose
        ))
    }
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

fn inventory_failure(
    code: &'static str,
    message: impl Into<String>,
    input: &Path,
) -> RepositoryPredicateFailure {
    repository_failure(StructuralPredicate::Inventory, code, message, input)
}

fn preservation_failure(
    code: &'static str,
    message: impl Into<String>,
    input: &Path,
) -> RepositoryPredicateFailure {
    repository_failure(StructuralPredicate::Preservation, code, message, input)
}

fn repository_failure(
    predicate: StructuralPredicate,
    code: &'static str,
    message: impl Into<String>,
    input: &Path,
) -> RepositoryPredicateFailure {
    RepositoryPredicateFailure::new(
        predicate,
        code,
        message,
        [input.to_string_lossy().replace('\\', "/")],
    )
}
