use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::ClassifiedInventory;
use crate::discovery::TestCaseIdentity;
use crate::discovery::TestSurfaceInventory;
use crate::evidence::read_json;

mod consolidation_status;
mod controlled_defect;
mod entrypoint_filters;
mod process_topology;
mod proof_contract;
mod semantic_authority;
mod source_identity;
mod suite_source_authority;
pub use consolidation_status::ConsolidationEvidenceStatus;
use entrypoint_filters::scenario_filters_for_suite;
use process_topology::admitted_process_topologies;
pub use process_topology::{validate_suite_process_cohesion, ScenarioProcessTopology};
use proof_contract::{subject_packages, ScenarioProofContractBuilder};
pub use semantic_authority::validate_suite_semantic_authority;
use suite_source_authority::suite_source_fingerprints;

const SUITES: [(&str, &str); 6] = [
    (
        "durability_recovery",
        "durability, crash, and recovery topology",
    ),
    (
        "physical_isolation",
        "physical isolation and interleaving topology",
    ),
    ("io_scheduling", "I/O policy and scheduling topology"),
    (
        "layout_access",
        "layout, access path, and blob-adjacent topology",
    ),
    (
        "blob_chunks",
        "content-addressed blob and tier movement topology",
    ),
    (
        "operational_security",
        "operational custody and security topology",
    ),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScenarioIdentity {
    pub owner_package: String,
    pub responsibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScenarioProofContract {
    pub production_subject_packages: BTreeSet<String>,
    pub oracle_owner_packages: BTreeSet<String>,
    #[serde(default)]
    pub setup_authority_sources: BTreeSet<String>,
    pub assertion_predicates: BTreeSet<String>,
    pub expected_evidence: BTreeSet<String>,
    pub proof_products: BTreeSet<String>,
    pub controlled_defects: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CertificationScenarioDeclaration {
    pub identity: ScenarioIdentity,
    pub libtest_filter_prefix: String,
    pub case_identities: Vec<TestCaseIdentity>,
    pub source_paths: BTreeSet<String>,
    pub process_topology: ScenarioProcessTopology,
    pub proof_contract: ScenarioProofContract,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CertificationSuiteDeclaration {
    pub suite_identity: String,
    pub target_identity: String,
    pub responsibility_boundary: String,
    pub admitted_process_topologies: BTreeSet<ScenarioProcessTopology>,
    pub shared_support_sources: BTreeSet<String>,
    #[serde(default)]
    pub suite_source_fingerprints: BTreeMap<String, String>,
    pub scenarios: Vec<CertificationScenarioDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsolidatedSuiteInventory {
    pub schema_version: u32,
    pub pre_cleanup_scenario_executables: usize,
    pub consolidated_suite_executables: usize,
    pub suites: Vec<CertificationSuiteDeclaration>,
}

pub fn build_consolidated_suite_inventory(
    workspace_root: &Path,
    inventory: &ClassifiedInventory,
) -> Result<ConsolidatedSuiteInventory, Vec<String>> {
    let mut violations = Vec::new();
    let mut suites = Vec::new();
    for (suite_name, boundary) in SUITES {
        let target_identity = format!("worth-store-certification::test::{suite_name}");
        if !inventory
            .discovered
            .targets
            .iter()
            .any(|target| target.identity == target_identity)
        {
            violations.push(format!(
                "missing certification suite target: {target_identity}"
            ));
            continue;
        }
        let filters = scenario_filters_for_suite(target_source(inventory, &target_identity))
            .map_err(|violation| vec![violation])?;
        let scenarios = scenarios_for_target(
            workspace_root,
            inventory,
            &target_identity,
            &filters,
            &mut violations,
        );
        if scenarios.is_empty() {
            violations.push(format!(
                "certification suite has no scenarios: {suite_name}"
            ));
        }
        let shared_support_sources =
            shared_support_sources(workspace_root, inventory, &target_identity, &mut violations);
        let suite_source_fingerprints =
            suite_source_fingerprints(workspace_root, inventory, &target_identity, &mut violations);
        suites.push(CertificationSuiteDeclaration {
            suite_identity: suite_name.to_owned(),
            target_identity,
            responsibility_boundary: boundary.to_owned(),
            admitted_process_topologies: admitted_process_topologies(suite_name),
            shared_support_sources,
            suite_source_fingerprints,
            scenarios,
        });
    }
    if let Err(cohesion_violations) = validate_suite_process_cohesion(&suites) {
        violations.extend(cohesion_violations);
    }
    validate_consolidation_coverage(inventory, &suites, &mut violations);
    if violations.is_empty() {
        Ok(ConsolidatedSuiteInventory {
            schema_version: 1,
            pre_cleanup_scenario_executables: pre_cleanup_scenario_executable_count(workspace_root)
                .map_err(|violation| vec![violation])?,
            consolidated_suite_executables: suites.len(),
            suites,
        })
    } else {
        Err(violations)
    }
}

fn target_source<'a>(inventory: &'a ClassifiedInventory, identity: &str) -> &'a str {
    inventory
        .discovered
        .targets
        .iter()
        .find(|target| target.identity == identity)
        .map(|target| target.source_path.as_str())
        .unwrap_or("")
}

fn pre_cleanup_scenario_executable_count(workspace_root: &Path) -> Result<usize, String> {
    let baseline: TestSurfaceInventory =
        read_json(&workspace_root.join("test-control/pre-cleanup/discovered-test-surface.json"))?;
    Ok(baseline
        .targets
        .iter()
        .filter(|target| target.source_path.contains("/tests/scenarios/"))
        .count())
}

fn shared_support_sources(
    workspace_root: &Path,
    inventory: &ClassifiedInventory,
    target_identity: &str,
    violations: &mut Vec<String>,
) -> BTreeSet<String> {
    let Some(target) = inventory
        .discovered
        .targets
        .iter()
        .find(|target| target.identity == target_identity)
    else {
        return BTreeSet::new();
    };
    let mut pending = vec![std::path::PathBuf::from(&target.source_path)];
    let mut visited = BTreeSet::new();
    let mut counts = BTreeMap::<String, usize>::new();
    while let Some(source_path) = pending.pop() {
        let canonical = canonical_path(&source_path);
        if !visited.insert(canonical) {
            continue;
        }
        let Ok(source) = fs::read_to_string(&source_path) else {
            continue;
        };
        for included in explicit_path_modules(&source_path, &source) {
            let normalized = canonical_path(&included);
            if normalized.contains("/tests/support/") {
                *counts
                    .entry(source_identity::repository_relative(
                        workspace_root,
                        Path::new(&normalized),
                    ))
                    .or_default() += 1;
            }
            pending.push(included);
        }
    }
    for (source, count) in &counts {
        if *count > 1 {
            violations.push(format!(
                "suite {target_identity} textually compiles shared support {source} {count} times"
            ));
        }
    }
    counts.into_keys().collect()
}

fn explicit_path_modules(source_path: &Path, source: &str) -> Vec<std::path::PathBuf> {
    let parent = source_path.parent().unwrap_or_else(|| Path::new("."));
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let value = line.strip_prefix("#[path = \"")?.strip_suffix("\"]")?;
            Some(parent.join(value))
        })
        .collect()
}

fn canonical_path(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace("\\\\?\\", "")
        .replace('\\', "/")
}

fn scenarios_for_target(
    workspace_root: &Path,
    inventory: &ClassifiedInventory,
    target_identity: &str,
    filters: &BTreeMap<String, String>,
    violations: &mut Vec<String>,
) -> Vec<CertificationScenarioDeclaration> {
    let mut grouped = BTreeMap::<ScenarioIdentity, ScenarioProofContractBuilder>::new();
    let mut subject_cache = BTreeMap::<String, BTreeSet<String>>::new();
    for proof in inventory.proofs.iter().filter(|proof| {
        proof.case.target_identity.as_deref() == Some(target_identity)
            && proof.case.kind == crate::discovery::CaseKind::RustTest
    }) {
        let identity = ScenarioIdentity {
            owner_package: proof.owner.package.clone(),
            responsibility: semantic_scenario_responsibility(proof),
        };
        let subjects = subject_cache
            .entry(proof.case.source_path.clone())
            .or_insert_with(|| subject_packages(workspace_root, &proof.case.source_path));
        grouped
            .entry(identity)
            .or_default()
            .include(workspace_root, proof, subjects);
    }
    grouped
        .into_iter()
        .filter_map(|(identity, builder)| {
            let Some(filter) = filters.get(&identity.responsibility) else {
                violations.push(format!(
                    "scenario has no suite entrypoint filter: {}::{}",
                    identity.owner_package, identity.responsibility
                ));
                return None;
            };
            Some(builder.finish(identity, filter.clone()))
        })
        .collect()
}

fn semantic_scenario_responsibility(proof: &super::ClassifiedProof) -> String {
    if proof
        .case
        .identity
        .stable_id
        .ends_with("::shortcut_report::shortcut_report_still_names_required_shortcut_boundaries")
    {
        "physical_isolation/simulation_harness_readiness".to_owned()
    } else if proof
        .case
        .target_identity
        .as_deref()
        .is_some_and(|target| target.ends_with("::test::io_scheduling"))
        && proof
            .case
            .source_path
            .ends_with("/producer_declarations.rs")
    {
        "scheduling/producer_declarations".to_owned()
    } else if proof
        .case
        .target_identity
        .as_deref()
        .is_some_and(|target| target.ends_with("::test::physical_isolation"))
        && proof
            .case
            .source_path
            .ends_with("/security/stable_read_execution/security_scope.rs")
    {
        "physical_isolation/stable_read_execution".to_owned()
    } else {
        proof.case.identity.responsibility.clone()
    }
}

fn validate_consolidation_coverage(
    inventory: &ClassifiedInventory,
    suites: &[CertificationSuiteDeclaration],
    violations: &mut Vec<String>,
) {
    let declared: BTreeSet<_> = suites
        .iter()
        .flat_map(|suite| suite.scenarios.iter())
        .flat_map(|scenario| scenario.case_identities.iter())
        .map(|identity| identity.stable_id.as_str())
        .collect();
    for proof in &inventory.proofs {
        let consolidated = proof.case.source_path.contains("/tests/scenarios/")
            && proof
                .case
                .target_identity
                .as_deref()
                .is_some_and(|target| target.starts_with("worth-store-certification::test::"));
        if consolidated && !declared.contains(proof.case.identity.stable_id.as_str()) {
            violations.push(format!(
                "consolidated scenario lacks a suite declaration: {}",
                proof.case.identity.stable_id
            ));
        }
    }
}
