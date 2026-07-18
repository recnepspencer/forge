use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::discovery::TestTargetIdentity;
use crate::evidence::sha256_serialized;

use super::proof_mode::{ProofProductUnavailable, StoreProofMode, StoreProofRequest};
use super::repository_identity::RepositoryIdentity;
use super::{StoreBuildProfileIdentity, StoreFeatureLane};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPreflightReference {
    pub evidence_identity: String,
    pub bundle_path: String,
    pub profile: String,
    pub predicates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreProofSelection {
    pub included_products: Vec<String>,
    pub included_packages: Vec<String>,
    pub excluded_packages: BTreeMap<String, String>,
    pub included_targets: Vec<String>,
    pub excluded_targets: BTreeMap<String, String>,
    pub included_case_responsibilities: BTreeMap<String, Vec<String>>,
    pub included_fixtures: Vec<String>,
    pub excluded_fixtures: BTreeMap<String, String>,
    pub included_suites: Vec<String>,
    pub excluded_suites: BTreeMap<String, String>,
    pub feature_lanes: Vec<StoreFeatureLane>,
    pub build_profiles: Vec<StoreBuildProfileIdentity>,
    pub subprocess_probes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProofExecutionUnit {
    pub package: String,
    pub target_name: String,
    pub target_selector: String,
    pub case_filter: Option<String>,
    pub feature_lane: StoreFeatureLane,
    pub build_profile: StoreBuildProfileIdentity,
    pub process_model: String,
    pub expected_evidence: Vec<String>,
}

impl ProofExecutionUnit {
    pub(crate) fn from_target(
        target: &TestTargetIdentity,
        request: &StoreProofRequest,
        case_filter: Option<String>,
    ) -> Self {
        let target_selector = if target.kinds.iter().any(|kind| kind == "doc") {
            "doc".to_owned()
        } else if target.kinds.iter().any(|kind| kind == "lib") {
            "lib".to_owned()
        } else if target.kinds.iter().any(|kind| kind == "bin") {
            "bin".to_owned()
        } else {
            "test".to_owned()
        };
        let process_model = if target.kinds.iter().any(|kind| kind == "doc") {
            "rustdoc-test-process".to_owned()
        } else if target.name.contains("compile_fail") {
            "nested-cargo-process".to_owned()
        } else {
            "libtest-process".to_owned()
        };
        Self {
            package: target.package.clone(),
            target_name: target.name.clone(),
            target_selector,
            case_filter,
            feature_lane: StoreFeatureLane::from_required_features(&target.required_features),
            build_profile: StoreBuildProfileIdentity::for_mode(request.mode()),
            process_model,
            expected_evidence: vec![
                "behavioral_verdict".to_owned(),
                "target_breadth".to_owned(),
                "process_count".to_owned(),
            ],
        }
    }

    pub(crate) fn with_process_model(mut self, process_model: impl Into<String>) -> Self {
        self.process_model = process_model.into();
        if self.process_model == "compiler-boundary-suite"
            && !self
                .expected_evidence
                .iter()
                .any(|item| item == "ui_proof_run_evidence")
        {
            self.expected_evidence
                .push("ui_proof_run_evidence".to_owned());
        }
        self
    }

    pub(crate) fn with_feature_lane(mut self, feature_lane: StoreFeatureLane) -> Self {
        self.feature_lane = feature_lane;
        self
    }

    pub(crate) fn feature_compatibility(package: String, features: Vec<String>) -> Self {
        Self {
            package,
            target_name: "library-feature-compatibility".to_owned(),
            target_selector: "check-lib".to_owned(),
            case_filter: None,
            feature_lane: StoreFeatureLane::declared(features),
            build_profile: StoreBuildProfileIdentity::CiTest,
            process_model: "cargo-check-process".to_owned(),
            expected_evidence: vec![
                "compiler_verdict".to_owned(),
                "resolved_feature_graph".to_owned(),
            ],
        }
    }

    pub fn cargo_arguments(&self, mode: StoreProofMode) -> Vec<String> {
        let cargo_operation = if self.target_selector == "check-lib" {
            "check"
        } else {
            "test"
        };
        let mut arguments = vec![
            cargo_operation.to_owned(),
            "-p".to_owned(),
            self.package.clone(),
        ];
        match self.target_selector.as_str() {
            "doc" => arguments.push("--doc".to_owned()),
            "lib" => arguments.push("--lib".to_owned()),
            "check-lib" => arguments.push("--lib".to_owned()),
            "bin" => {
                arguments.push("--bin".to_owned());
                arguments.push(self.target_name.clone());
            }
            _ => {
                arguments.push("--test".to_owned());
                arguments.push(self.target_name.clone());
            }
        }
        if self.build_profile.cargo_profile() != "test" {
            arguments.push("--profile".to_owned());
            arguments.push(self.build_profile.cargo_profile().to_owned());
        }
        if !self.feature_lane.cargo_features().is_empty() {
            arguments.push("--features".to_owned());
            arguments.push(self.feature_lane.cargo_features().join(","));
        }
        if let Some(filter) = &self.case_filter {
            arguments.push(filter.clone());
        }
        if mode == StoreProofMode::Soak {
            arguments.push("--".to_owned());
            arguments.push("--ignored".to_owned());
            arguments.push("--nocapture".to_owned());
        }
        arguments
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedProofExecutionPlan {
    pub schema_version: u32,
    pub plan_digest: String,
    pub product: String,
    pub request: StoreProofRequest,
    pub repository: RepositoryIdentity,
    pub selection: StoreProofSelection,
    pub units: Vec<ProofExecutionUnit>,
    pub excluded_products: BTreeMap<String, String>,
    pub cache_posture: String,
    pub evidence_destination: String,
    pub closeout_posture: String,
    pub structural_preflight: StructuralPreflightReference,
}

#[derive(Serialize)]
struct PlanDigestBasis<'a> {
    request: &'a StoreProofRequest,
    repository: &'a RepositoryIdentity,
    selection: &'a StoreProofSelection,
    units: &'a [ProofExecutionUnit],
    structural_preflight: &'a StructuralPreflightReference,
}

impl SelectedProofExecutionPlan {
    pub(crate) fn lower(
        workspace_root: &Path,
        request: StoreProofRequest,
        selection: StoreProofSelection,
        units: Vec<ProofExecutionUnit>,
        excluded_products: BTreeMap<String, String>,
        repository: RepositoryIdentity,
        structural_preflight: StructuralPreflightReference,
    ) -> Result<Self, ProofProductUnavailable> {
        let digest = sha256_serialized(&PlanDigestBasis {
            request: &request,
            repository: &repository,
            selection: &selection,
            units: &units,
            structural_preflight: &structural_preflight,
        })
        .map_err(ProofProductUnavailable::RepositoryObservation)?;
        let destination = workspace_root
            .join(".store-proof/evidence/plans")
            .join(format!("{digest}.json"));
        let cache_posture = cache_posture(&units);
        Ok(Self {
            schema_version: 1,
            plan_digest: digest,
            product: request.display_name(),
            request,
            repository,
            selection,
            units,
            excluded_products,
            cache_posture,
            evidence_destination: destination.to_string_lossy().replace('\\', "/"),
            closeout_posture: "product-evidence-only; C.1 closeout is unavailable before phase 13"
                .to_owned(),
            structural_preflight,
        })
    }
}

impl StructuralPreflightReference {
    pub fn from_evidence(
        bundle_path: &Path,
        evidence: &worth_store_test_support::structural_preflight::StructuralPreflightEvidence,
    ) -> Self {
        Self {
            evidence_identity: evidence.evidence_identity.0.clone(),
            bundle_path: bundle_path.to_string_lossy().replace('\\', "/"),
            profile: format!("{:?}", evidence.plan.request.profile),
            predicates: evidence
                .plan
                .request
                .predicates
                .iter()
                .map(|predicate| format!("{predicate:?}"))
                .collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn synthetic_for_selection(workspace_root: &Path) -> Self {
        Self {
            evidence_identity: "selection-test-preflight".to_owned(),
            bundle_path: workspace_root
                .join(".store-proof/evidence/preflight/selection-test.json")
                .to_string_lossy()
                .replace('\\', "/"),
            profile: "SelectionTest".to_owned(),
            predicates: vec!["Inventory".to_owned()],
        }
    }
}

fn cache_posture(units: &[ProofExecutionUnit]) -> String {
    let profiles: std::collections::BTreeSet<_> =
        units.iter().map(|unit| unit.build_profile).collect();
    if profiles == std::collections::BTreeSet::from([StoreBuildProfileIdentity::LocalTest]) {
        "local-test; incremental=true; clean-or-warm target root admitted".to_owned()
    } else if profiles == std::collections::BTreeSet::from([StoreBuildProfileIdentity::CiTest]) {
        "ci-test; incremental=false; evidence validity is independent of local incremental state"
            .to_owned()
    } else {
        format!(
            "mixed declared profiles [{}]; cache identity is profile-bound",
            profiles
                .iter()
                .map(|profile| profile.cargo_profile())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
