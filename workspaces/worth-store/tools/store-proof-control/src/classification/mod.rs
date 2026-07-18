mod build_graph_policy;
mod compiler_boundary_policy;
mod proof_behavior_authority;
mod proof_disposition;
mod proof_family;
mod registration_alias;
mod scenario_suite;
mod semantic_authority;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub use build_graph_policy::{
    validate_build_graph_policy, validate_feature_semantic_authority_policy,
    validate_inventory_build_graph_policy, validate_production_dependency_policy,
    BuildGraphPolicyViolation, DependencyBoundaryDenial, DependencyBoundaryViolation,
    FeatureAuthorityClass, FeatureSemanticAuthority, FeatureSemanticAuthorityDenial,
    FeatureSemanticAuthoritySubject, FeatureSemanticAuthorityViolation, FeatureSemanticDeclaration,
    ValidatedFeatureSemanticAuthority,
};
pub use proof_behavior_authority::{
    validate_proof_behavior_authority, validate_proof_behavior_authority_for_source_edit,
    ProofBehaviorAuthority, ProofBehaviorDeclaration,
};
pub use proof_disposition::ProofDisposition;
pub use proof_family::ProofFamily;
pub use scenario_suite::{
    build_consolidated_suite_inventory, validate_suite_process_cohesion,
    validate_suite_semantic_authority, validate_suite_semantic_authority_for_source_edit,
    CertificationScenarioDeclaration, CertificationSuiteDeclaration, ConsolidatedSuiteInventory,
    ConsolidationEvidenceStatus, ScenarioIdentity, ScenarioProcessTopology, ScenarioProofContract,
};
pub use semantic_authority::{
    classify, classify_from_authority, PostBaselineProofAuthority, ProofSemanticDeclaration,
};

use crate::discovery::{CaseKind, TestCaseSurface, TestSurfaceInventory};
use crate::{ClassifiedProofInventory, ValidatedProofInventory};
use registration_alias::registration_alias_violations;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProofOwner {
    pub package: String,
    pub responsibility: String,
}

pub type ProofProductSet = BTreeSet<String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedProof {
    pub case: TestCaseSurface,
    pub family: ProofFamily,
    pub owner: ProofOwner,
    pub products: BTreeSet<String>,
    pub disposition: ProofDisposition,
    pub expected_evidence: Vec<String>,
    pub physical_reality_audit_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedInventory {
    pub schema_version: u32,
    pub discovered: TestSurfaceInventory,
    pub proofs: Vec<ClassifiedProof>,
}

pub fn validate(
    classified: ClassifiedProofInventory,
) -> Result<ValidatedProofInventory, Vec<String>> {
    let inventory = classified.into_inventory();
    let mut violations = Vec::new();
    let mut identities = BTreeSet::new();
    let package_names: BTreeSet<_> = inventory
        .discovered
        .packages
        .iter()
        .map(|package| package.name.as_str())
        .collect();
    let target_identities: BTreeSet<_> = inventory
        .discovered
        .targets
        .iter()
        .map(|target| target.identity.as_str())
        .collect();
    for proof in &inventory.proofs {
        if inventory.discovered.schema_version >= 3 {
            violations.extend(compiler_boundary_policy::violations(proof));
        }
        if !identities.insert(&proof.case.identity.stable_id) {
            violations.push(format!(
                "duplicate proof identity: {}",
                proof.case.identity.stable_id
            ));
        }
        if proof.products.is_empty()
            && proof.disposition != ProofDisposition::InvalidClaimQuarantine
        {
            violations.push(format!(
                "proof has no execution product: {}",
                proof.case.identity.stable_id
            ));
        }
        if !package_names.contains(proof.owner.package.as_str()) {
            violations.push(format!(
                "proof owner is not a workspace package for {}: {}",
                proof.case.identity.stable_id, proof.owner.package
            ));
        }
        if matches!(
            proof.family,
            ProofFamily::OwnerBehavior | ProofFamily::OwnerInvariant
        ) && proof.owner.package != proof.case.identity.package
        {
            violations.push(format!(
                "owner-local proof {} leaked from {} to {}",
                proof.case.identity.stable_id, proof.case.identity.package, proof.owner.package
            ));
        }
        if proof.owner.responsibility.trim().is_empty() {
            violations.push(format!(
                "proof owner has no semantic responsibility: {}",
                proof.case.identity.stable_id
            ));
        }
        if proof.case.current_invocation == "unregistered" {
            violations.push(format!(
                "test source is unreachable from Cargo targets: {}",
                proof.case.source_path
            ));
        }
        if inventory.discovered.schema_version >= 2
            && proof.case.registration_authority == "unregistered"
        {
            violations.push(format!(
                "test source lacks a registration authority: {}",
                proof.case.source_path
            ));
        }
        violations.extend(registration_alias_violations(
            &proof.case,
            &target_identities,
        ));
        if inventory.discovered.schema_version >= 2 && proof.case.process_model.is_empty() {
            violations.push(format!(
                "test source has no declared process model: {}",
                proof.case.identity.stable_id
            ));
        }
        if inventory.discovered.schema_version >= 2
            && proof.case.launches_nested_cargo
            && !proof.case.external_tools.iter().any(|tool| tool == "cargo")
        {
            violations.push(format!(
                "nested Cargo test omits cargo from external-tool authority: {}",
                proof.case.identity.stable_id
            ));
        }
        if matches!(
            proof.case.kind,
            CaseKind::UiFixture | CaseKind::DoctestCompileFail
        ) && proof.family != ProofFamily::CompilerBoundary
            && proof.family != ProofFamily::DependencyBoundary
        {
            violations.push(format!(
                "UI fixture lacks compiler/dependency classification: {}",
                proof.case.identity.stable_id
            ));
        }
    }
    if inventory.proofs.len() != inventory.discovered.cases.len() {
        violations.push("classified inventory cardinality differs from discovery".to_owned());
    }
    if inventory.discovered.schema_version >= 2 {
        if !inventory
            .discovered
            .workflow_commands
            .iter()
            .any(|workflow| workflow.command.contains("cargo store-ci"))
        {
            violations.push("CI has no repository-owned cargo store-ci proof command".to_owned());
        }
        for workflow in &inventory.discovered.workflow_commands {
            if workflow.command.contains("cargo test")
                && workflow.command.contains("--workspace")
                && workflow.command.contains("--all-features")
            {
                violations.push(format!(
                    "CI bypasses proof-product selection at {}:{}: {}",
                    workflow.workflow_path, workflow.source_line, workflow.command
                ));
            }
        }
    }
    if violations.is_empty() {
        Ok(ValidatedProofInventory::from_classified(inventory))
    } else {
        Err(violations)
    }
}

fn classify_case(case: TestCaseSurface) -> ClassifiedProof {
    let family = if execution_package(&case).is_some_and(|package| package != case.identity.package)
    {
        ProofFamily::CrossOwnerIntegration
    } else {
        ProofFamily::from_case(&case)
    };
    let owner = owner_for(&case, family);
    let products = products_for(&case, family);
    let disposition = if case.kind == CaseKind::DoctestIgnored {
        ProofDisposition::InvalidClaimQuarantine
    } else if case
        .source_path
        .contains("/worth-store-certification/tests/scenarios/")
    {
        ProofDisposition::PreserveAndConsolidate
    } else {
        ProofDisposition::PreserveUnchanged
    };
    let physical_reality_audit_required = case.source_path.contains("/worth-store-certification/")
        && matches!(
            family,
            ProofFamily::CrossOwnerIntegration
                | ProofFamily::DeterministicSimulation
                | ProofFamily::FreshProcessIsolation
                | ProofFamily::PerformanceEnvelope
        );
    ClassifiedProof {
        case,
        family,
        owner,
        products,
        disposition,
        expected_evidence: evidence_for(family),
        physical_reality_audit_required,
    }
}

fn owner_for(case: &TestCaseSurface, family: ProofFamily) -> ProofOwner {
    let responsibility = if case
        .target_identity
        .as_deref()
        .is_some_and(|target| target.ends_with("::test::io_scheduling"))
        && case.source_path.ends_with("/producer_declarations.rs")
    {
        "scheduling/producer_declarations".to_owned()
    } else if matches!(
        family,
        ProofFamily::CompilerBoundary | ProofFamily::DependencyBoundary
    ) {
        "compiler-boundary".to_owned()
    } else {
        case.identity.responsibility.clone()
    };
    let package = if family == ProofFamily::CrossOwnerIntegration {
        execution_package(case).unwrap_or(&case.identity.package)
    } else {
        &case.identity.package
    };
    ProofOwner {
        package: package.to_owned(),
        responsibility,
    }
}

fn products_for(case: &TestCaseSurface, family: ProofFamily) -> BTreeSet<String> {
    let mut products = BTreeSet::new();
    if case.kind == CaseKind::DoctestIgnored {
        return products;
    }
    if matches!(
        family,
        ProofFamily::CompilerBoundary | ProofFamily::DependencyBoundary
    ) {
        products.insert("store-ui".to_owned());
        if is_ui_smoke_specimen(case) {
            products.insert("store-smoke".to_owned());
        }
        return products;
    }
    if case
        .required_features
        .iter()
        .any(|feature| feature.contains("certification"))
    {
        products.insert(format!(
            "store-ci:owner-certification:{}",
            case.identity.package
        ));
        return products;
    }
    let execution_package = execution_package(case).unwrap_or(&case.identity.package);
    if execution_package == "worth-store-certification" {
        let partition = certification_partition(case);
        products.insert(format!("store-ci:{partition}"));
        if is_smoke_specimen(case) {
            products.insert("store-smoke".to_owned());
        }
    } else if execution_package == "worth-store-physical-certification" {
        products.insert("store-ci:physical-certification".to_owned());
    } else if execution_package == "worth-store-formal-models" {
        products.insert("store-ci:formal-conformance".to_owned());
    } else {
        products.insert(format!("store-owner:{}", case.identity.package));
    }
    if case.ignored {
        products.insert("store-soak".to_owned());
    }
    products
}

fn execution_package(case: &TestCaseSurface) -> Option<&str> {
    case.target_identity
        .as_deref()
        .and_then(|identity| identity.split_once("::"))
        .map(|(package, _)| package)
}

fn certification_partition(case: &TestCaseSurface) -> &'static str {
    if let Some(target) = case.target_identity.as_deref() {
        for (suite, partition) in [
            ("::test::durability_recovery", "recovery"),
            ("::test::physical_isolation", "physical_isolation"),
            ("::test::io_scheduling", "scheduling"),
            ("::test::layout_access", "layout"),
            ("::test::blob_chunks", "blobs"),
            ("::test::operational_security", "security"),
        ] {
            if target.ends_with(suite) {
                return partition;
            }
        }
    }
    let responsibility = case.identity.responsibility.as_str();
    for (prefix, partition) in [
        ("recovery/", "recovery"),
        ("physical_isolation/", "physical_isolation"),
        ("scheduling/", "scheduling"),
        ("layout/", "layout"),
        ("blobs/", "blobs"),
        ("security/", "security"),
    ] {
        if responsibility.starts_with(prefix) {
            return partition;
        }
    }
    for (target, partition) in [
        ("target/durability_recovery", "recovery"),
        ("target/physical_isolation", "physical_isolation"),
        ("target/io_scheduling", "scheduling"),
        ("target/layout_access", "layout"),
        ("target/blob_chunks", "blobs"),
        ("target/operational_security", "security"),
    ] {
        if responsibility == target {
            return partition;
        }
    }
    "certification-owner"
}

fn is_smoke_specimen(case: &TestCaseSurface) -> bool {
    [
        "wal_durability_ack",
        "stable_read_plan_admission",
        "access_policy",
        "btree_lookup_authority",
        "security_scope_propagation",
    ]
    .iter()
    .any(|specimen| case.identity.responsibility.ends_with(specimen))
}

fn is_ui_smoke_specimen(case: &TestCaseSurface) -> bool {
    case.identity.case_name == "future_chunk_placeholder_boundary_misuse_does_not_compile"
}

fn evidence_for(family: ProofFamily) -> Vec<String> {
    let mut evidence = vec![
        "behavioral_verdict".to_owned(),
        "assertion_predicates".to_owned(),
    ];
    if family == ProofFamily::FreshProcessIsolation {
        evidence.push("process_identity".to_owned());
        evidence.push("termination_mode".to_owned());
    }
    if matches!(
        family,
        ProofFamily::PerformanceEnvelope | ProofFamily::Soak | ProofFamily::HardwareQualification
    ) {
        evidence.push("structural_cost_counters".to_owned());
    }
    evidence
}
