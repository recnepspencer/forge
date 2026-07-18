use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::discovery::TestTargetIdentity;
use crate::evidence::sha256_serialized;

use super::cache_posture::cache_posture;
use super::execution_contract::timeout_millis;
use super::proof_mode::{StoreProofMode, StoreProofRequest};
use super::repository_identity::RepositoryIdentity;
use super::ProofProductUnavailable;
use super::{
    ProofExecutionCommand, ProofExecutionIsolation, ProofExecutionResources, ProofFailurePolicy,
    ProofProcessModel, ProofRetryPolicy, StoreBuildProfileIdentity, StoreFeatureLane,
};

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
    pub process_model: ProofProcessModel,
    pub command: ProofExecutionCommand,
    pub isolation: ProofExecutionIsolation,
    pub resources: ProofExecutionResources,
    pub dependencies: Vec<String>,
    pub timeout_millis: u64,
    pub retry: ProofRetryPolicy,
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
            ProofProcessModel::RustdocTestProcess
        } else if target.name.contains("compile_fail") {
            ProofProcessModel::NestedCargoProcess
        } else {
            ProofProcessModel::LibtestProcess
        };
        let isolation = ProofExecutionIsolation::for_process_model(process_model);
        Self {
            package: target.package.clone(),
            target_name: target.name.clone(),
            target_selector,
            case_filter,
            feature_lane: StoreFeatureLane::from_required_features(&target.required_features),
            build_profile: StoreBuildProfileIdentity::for_mode(request.mode()),
            process_model,
            command: ProofExecutionCommand::Cargo,
            isolation,
            resources: ProofExecutionResources::unbound(process_model),
            dependencies: Vec::new(),
            timeout_millis: timeout_millis(request.mode()),
            retry: ProofRetryPolicy::never(),
            expected_evidence: vec![
                "behavioral_verdict".to_owned(),
                "target_breadth".to_owned(),
                "process_count".to_owned(),
            ],
        }
    }

    pub(crate) fn with_process_model(mut self, process_model: ProofProcessModel) -> Self {
        self.process_model = process_model;
        self.isolation = ProofExecutionIsolation::for_process_model(process_model);
        self.resources = ProofExecutionResources::unbound(process_model);
        if self.process_model.requires_ui_proof_evidence()
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
        let process_model = ProofProcessModel::CargoCheckProcess;
        Self {
            package,
            target_name: "library-feature-compatibility".to_owned(),
            target_selector: "check-lib".to_owned(),
            case_filter: None,
            feature_lane: StoreFeatureLane::declared(features),
            build_profile: StoreBuildProfileIdentity::CiTest,
            process_model,
            command: ProofExecutionCommand::Cargo,
            isolation: ProofExecutionIsolation::for_process_model(process_model),
            resources: ProofExecutionResources::unbound(process_model),
            dependencies: Vec::new(),
            timeout_millis: timeout_millis(StoreProofMode::Ci),
            retry: ProofRetryPolicy::never(),
            expected_evidence: vec![
                "compiler_verdict".to_owned(),
                "resolved_feature_graph".to_owned(),
            ],
        }
    }

    pub(crate) fn formal_tool(workspace_root: &Path) -> Self {
        let process_model = ProofProcessModel::ExternalToolProcess;
        let script = workspace_root
            .join("../../scripts/ci/verify_worth_store_formal_toolchain.sh")
            .to_string_lossy()
            .replace('\\', "/");
        let mut resources = ProofExecutionResources::unbound(process_model);
        resources.bind_target_root(workspace_root);
        resources.exclusive_external_tools.extend([
            "bash".to_owned(),
            "java".to_owned(),
            "tlc".to_owned(),
        ]);
        Self {
            package: "worth-store-formal-models".to_owned(),
            target_name: "production-protocol-toolchain".to_owned(),
            target_selector: "external-tool".to_owned(),
            case_filter: None,
            feature_lane: StoreFeatureLane::ProductionEquivalent,
            build_profile: StoreBuildProfileIdentity::CiTest,
            process_model,
            command: ProofExecutionCommand::ExternalTool {
                program: "bash".to_owned(),
                arguments: vec![script],
            },
            isolation: ProofExecutionIsolation::for_process_model(process_model),
            resources,
            dependencies: Vec::new(),
            timeout_millis: timeout_millis(StoreProofMode::Ci),
            retry: ProofRetryPolicy::never(),
            expected_evidence: vec![
                "formal_tool_receipt".to_owned(),
                "external_process_observation".to_owned(),
            ],
        }
    }

    pub(crate) fn bind_workspace(&mut self, workspace_root: &Path, request: &StoreProofRequest) {
        if let Some(target_root) = request.target_root() {
            self.resources
                .bind_explicit_target_root(Path::new(target_root));
        } else {
            self.resources.bind_target_root(workspace_root);
        }
        if let Some(seed) = request.seed() {
            self.resources
                .environment
                .insert("WORTH_STORE_PROOF_SEED".to_owned(), seed.to_string());
        }
        if let Some(backend) = request.backend() {
            self.resources
                .environment
                .insert("WORTH_STORE_BACKEND_PROFILE".to_owned(), backend.to_owned());
        }
    }

    pub fn identity(&self) -> String {
        format!(
            "{}::{}::{}",
            self.package,
            self.target_name,
            self.case_filter.as_deref().unwrap_or("all")
        )
    }

    pub fn command_line(&self, mode: StoreProofMode) -> (String, Vec<String>) {
        if let ProofExecutionCommand::ExternalTool { program, arguments } = &self.command {
            return (program.clone(), arguments.clone());
        }
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
        arguments.push("--message-format=json-render-diagnostics".to_owned());
        if let Some(filter) = &self.case_filter {
            arguments.push(filter.clone());
        }
        if mode == StoreProofMode::Soak {
            arguments.push("--".to_owned());
            arguments.push("--ignored".to_owned());
            arguments.push("--nocapture".to_owned());
        }
        ("cargo".to_owned(), arguments)
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
    pub ci_shard_plan: Option<crate::ci::CiShardPlan>,
    pub maximum_concurrency: usize,
    pub failure_policy: ProofFailurePolicy,
    pub excluded_products: BTreeMap<String, String>,
    pub cache_posture: String,
    pub evidence_destination: String,
    pub closeout_posture: String,
    pub structural_preflight: StructuralPreflightReference,
    pub source_edit: Option<super::ObservedSourceEditIdentity>,
}

#[derive(Serialize)]
pub(super) struct PlanDigestBasis<'a> {
    pub(super) request: &'a StoreProofRequest,
    pub(super) repository: &'a RepositoryIdentity,
    pub(super) selection: &'a StoreProofSelection,
    pub(super) units: &'a [ProofExecutionUnit],
    pub(super) ci_shard_plan: &'a Option<crate::ci::CiShardPlan>,
    pub(super) maximum_concurrency: usize,
    pub(super) failure_policy: ProofFailurePolicy,
    pub(super) structural_preflight: &'a StructuralPreflightReference,
    pub(super) source_edit: &'a Option<super::ObservedSourceEditIdentity>,
}

impl SelectedProofExecutionPlan {
    pub(crate) fn lower(
        workspace_root: &Path,
        request: StoreProofRequest,
        selection: StoreProofSelection,
        units: Vec<ProofExecutionUnit>,
        ci_shard_plan: Option<crate::ci::CiShardPlan>,
        excluded_products: BTreeMap<String, String>,
        repository: RepositoryIdentity,
        structural_preflight: StructuralPreflightReference,
        source_edit: Option<super::ObservedSourceEditIdentity>,
    ) -> Result<Self, ProofProductUnavailable> {
        let maximum_concurrency = declared_concurrency();
        let failure_policy = ProofFailurePolicy::for_mode(request.mode());
        let digest = sha256_serialized(&PlanDigestBasis {
            request: &request,
            repository: &repository,
            selection: &selection,
            units: &units,
            ci_shard_plan: &ci_shard_plan,
            maximum_concurrency,
            failure_policy,
            structural_preflight: &structural_preflight,
            source_edit: &source_edit,
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
            ci_shard_plan,
            maximum_concurrency,
            failure_policy,
            excluded_products,
            cache_posture,
            evidence_destination: destination.to_string_lossy().replace('\\', "/"),
            closeout_posture: "product-evidence-only; C.1 closeout is unavailable before phase 13"
                .to_owned(),
            structural_preflight,
            source_edit,
        })
    }
}

fn declared_concurrency() -> usize {
    std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .clamp(1, 8)
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
