use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use super::controlled_defect::controlled_defect_case;
use super::{
    CertificationScenarioDeclaration, ScenarioIdentity, ScenarioProcessTopology,
    ScenarioProofContract,
};
use crate::discovery::TestCaseIdentity;

#[derive(Default)]
pub(super) struct ScenarioProofContractBuilder {
    cases: Vec<TestCaseIdentity>,
    sources: BTreeSet<String>,
    subjects: BTreeSet<String>,
    assertions: BTreeSet<String>,
    evidence: BTreeSet<String>,
    products: BTreeSet<String>,
    controlled_defects: BTreeSet<String>,
    standardized_ui_harness: bool,
    child_process: bool,
    nested_cargo: bool,
}

impl ScenarioProofContractBuilder {
    pub(super) fn include(
        &mut self,
        workspace_root: &Path,
        proof: &super::super::ClassifiedProof,
        subjects: &BTreeSet<String>,
    ) {
        self.cases.push(proof.case.identity.clone());
        self.sources
            .insert(super::source_identity::repository_relative(
                workspace_root,
                Path::new(&proof.case.source_path),
            ));
        self.assertions
            .extend(proof.case.assertion_predicates.iter().cloned());
        self.evidence
            .extend(proof.expected_evidence.iter().cloned());
        self.products.extend(proof.products.iter().cloned());
        if controlled_defect_case(&proof.case.identity.case_name) {
            self.controlled_defects
                .insert(proof.case.identity.case_name.clone());
        }
        self.child_process |= proof.case.launches_child_process;
        self.nested_cargo |= proof.case.launches_nested_cargo;
        self.standardized_ui_harness |= proof.case.compiler_boundary_harness.is_some();
        self.subjects.extend(subjects.iter().cloned());
    }

    pub(super) fn finish(
        mut self,
        identity: ScenarioIdentity,
        libtest_filter_prefix: String,
    ) -> CertificationScenarioDeclaration {
        self.cases.sort();
        let process_topology = if self.standardized_ui_harness {
            ScenarioProcessTopology::StandardizedUiHarness
        } else if self.nested_cargo {
            ScenarioProcessTopology::NestedCargoProcess
        } else if self.child_process {
            ScenarioProcessTopology::FreshChildProcess
        } else {
            ScenarioProcessTopology::InProcessLibtest
        };
        let mut oracle_owner_packages: BTreeSet<_> = self
            .subjects
            .iter()
            .filter(|package| {
                package.contains("physical-certification")
                    || package.contains("offline-verifier")
                    || package.contains("formal-models")
            })
            .cloned()
            .collect();
        self.subjects.retain(|package| {
            !oracle_owner_packages.contains(package) && package != "worth-store-test-support"
        });
        apply_explicit_subject_overrides(&identity, &mut self.subjects);
        if oracle_owner_packages.is_empty() {
            oracle_owner_packages.insert("worth-store-certification".to_owned());
        }
        let setup_authority_sources = self.sources.clone();
        CertificationScenarioDeclaration {
            identity,
            libtest_filter_prefix,
            case_identities: self.cases,
            source_paths: self.sources,
            process_topology,
            proof_contract: ScenarioProofContract {
                production_subject_packages: self.subjects,
                oracle_owner_packages,
                setup_authority_sources,
                assertion_predicates: self.assertions,
                expected_evidence: self.evidence,
                proof_products: self.products,
                controlled_defects: self.controlled_defects,
            },
        }
    }
}

fn apply_explicit_subject_overrides(identity: &ScenarioIdentity, subjects: &mut BTreeSet<String>) {
    if let Some(primary) = primary_subject(&identity.responsibility) {
        subjects.insert(primary.to_owned());
    }
    let declared: &[&str] = match identity.responsibility.as_str() {
        "recovery/foundational_proof_evidence" => &["worth-store-recovery-physics"],
        "recovery/recovery_harness_ui" => &[
            "worth-store-recovery-physics",
            "worth-store-physical-backend",
        ],
        "blobs/structural_closeout" => &[
            "worth-store-blob-chunks",
            "worth-store-physical-format",
            "worth-store-recovery-physics",
            "worth-store-test-support",
        ],
        _ => &[],
    };
    subjects.extend(declared.iter().map(|package| (*package).to_owned()));
}

fn primary_subject(responsibility: &str) -> Option<&'static str> {
    [
        ("recovery/", "worth-store-recovery-physics"),
        ("physical_isolation/", "worth-store-physical-isolation"),
        ("scheduling/", "worth-store-io-scheduler"),
        ("layout/", "worth-store-layout-indexes"),
        ("blobs/", "worth-store-blob-chunks"),
        ("security/", "worth-store-security"),
    ]
    .into_iter()
    .find_map(|(prefix, package)| responsibility.starts_with(prefix).then_some(package))
}

pub(super) fn subject_packages(workspace_root: &Path, source_path: &str) -> BTreeSet<String> {
    let path = Path::new(source_path);
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace_root.join(path)
    };
    let Ok(source) = fs::read_to_string(path) else {
        return BTreeSet::new();
    };
    source
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| token.starts_with("worth_store_") && *token != "worth_store_certification")
        .map(|token| token.replace('_', "-"))
        .collect()
}
