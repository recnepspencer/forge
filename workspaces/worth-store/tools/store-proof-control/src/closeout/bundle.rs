use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::artifact_lifecycle::{BuildArtifactCleanupPlan, BuildArtifactInventory};
use crate::ci::CiCertificationAggregate;
use crate::evidence::{sha256_bytes, sha256_serialized};
use crate::selection::RepositoryIdentity;

use super::command_contract::validate_commands;
use super::{
    C2QuarantinedClaim, CloseoutArtifactReference, DeveloperIterationEnvelope,
    PreservationCheckedProofRun, ProofMutationSensitivityReport, StableProofCommand,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestArchitectureCloseoutInputs {
    pub repository: RepositoryIdentity,
    pub proof_inventory: CloseoutArtifactReference,
    pub owner_build_closures: CloseoutArtifactReference,
    pub scenario_suite_inventory: CloseoutArtifactReference,
    pub preservation: PreservationCheckedProofRun,
    pub mutation_sensitivity: ProofMutationSensitivityReport,
    pub developer_iteration: DeveloperIterationEnvelope,
    pub ci: CiCertificationAggregate,
    pub artifact_inventory: BuildArtifactInventory,
    pub artifact_cleanup_plan: BuildArtifactCleanupPlan,
    pub stable_commands: Vec<StableProofCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestArchitectureCloseoutBundle {
    schema_version: u32,
    evidence_identity: String,
    repository: RepositoryIdentity,
    proof_inventory: CloseoutArtifactReference,
    owner_build_closures: CloseoutArtifactReference,
    scenario_suite_inventory: CloseoutArtifactReference,
    preservation: PreservationCheckedProofRun,
    mutation_sensitivity: ProofMutationSensitivityReport,
    developer_iteration: DeveloperIterationEnvelope,
    ci: CiCertificationAggregate,
    artifact_inventory: BuildArtifactInventory,
    artifact_cleanup_plan: BuildArtifactCleanupPlan,
    stable_commands: Vec<StableProofCommand>,
    closeout_predicates: Vec<CloseoutPredicateEvidence>,
    residual_quarantines: Vec<C2QuarantinedClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CloseoutPredicateEvidence {
    pub predicate: CloseoutPredicate,
    pub evidence_identity: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum CloseoutPredicate {
    ProofPreservation,
    MutationSensitivity,
    DeveloperIteration,
    OwnerTopology,
    ScenarioTopology,
    CiCoverage,
    ArtifactLifecycle,
}

impl TestArchitectureCloseoutBundle {
    pub(crate) fn certify(inputs: TestArchitectureCloseoutInputs) -> Result<Self, String> {
        inputs.proof_inventory.validate()?;
        inputs.owner_build_closures.validate()?;
        inputs.scenario_suite_inventory.validate()?;
        inputs.preservation.validate()?;
        inputs.mutation_sensitivity.validate()?;
        if inputs.mutation_sensitivity.control_evidence_identity()
            != inputs.preservation.evidence_identity()
        {
            return Err(
                "mutation sensitivity was not executed against this preservation authority"
                    .to_owned(),
            );
        }
        inputs.developer_iteration.validate()?;
        inputs.ci.validate()?;
        inputs.artifact_inventory.validate_integrity()?;
        inputs.artifact_cleanup_plan.validate_integrity()?;
        validate_repository_conjunction(&inputs)?;
        if inputs.artifact_cleanup_plan.inventory_identity()
            != inputs.artifact_inventory.inventory_identity()
        {
            return Err("artifact cleanup dry run belongs to another inventory".to_owned());
        }
        validate_commands(&inputs.stable_commands)?;
        let closeout_predicates = vec![
            predicate(
                CloseoutPredicate::ProofPreservation,
                inputs.preservation.evidence_identity(),
            ),
            predicate(
                CloseoutPredicate::MutationSensitivity,
                inputs.mutation_sensitivity.evidence_identity(),
            ),
            predicate(
                CloseoutPredicate::DeveloperIteration,
                inputs.developer_iteration.evidence_identity(),
            ),
            predicate(
                CloseoutPredicate::OwnerTopology,
                &inputs.owner_build_closures.sha256,
            ),
            predicate(
                CloseoutPredicate::ScenarioTopology,
                &inputs.scenario_suite_inventory.sha256,
            ),
            predicate(CloseoutPredicate::CiCoverage, &inputs.ci.aggregate_identity),
            predicate(
                CloseoutPredicate::ArtifactLifecycle,
                inputs.artifact_cleanup_plan.plan_identity(),
            ),
        ];
        let mut bundle = Self {
            schema_version: 1,
            evidence_identity: String::new(),
            repository: inputs.repository,
            proof_inventory: inputs.proof_inventory,
            owner_build_closures: inputs.owner_build_closures,
            scenario_suite_inventory: inputs.scenario_suite_inventory,
            residual_quarantines: inputs.preservation.quarantines().to_vec(),
            preservation: inputs.preservation,
            mutation_sensitivity: inputs.mutation_sensitivity,
            developer_iteration: inputs.developer_iteration,
            ci: inputs.ci,
            artifact_inventory: inputs.artifact_inventory,
            artifact_cleanup_plan: inputs.artifact_cleanup_plan,
            stable_commands: inputs.stable_commands,
            closeout_predicates,
        };
        bundle.validate_surface()?;
        bundle.evidence_identity = bundle.expected_identity()?;
        Ok(bundle)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_surface()?;
        if self.expected_identity()? != self.evidence_identity {
            return Err(
                "test architecture closeout identity does not match its contents".to_owned(),
            );
        }
        Ok(())
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn preservation(&self) -> &PreservationCheckedProofRun {
        &self.preservation
    }

    pub fn mutation_sensitivity(&self) -> &ProofMutationSensitivityReport {
        &self.mutation_sensitivity
    }

    pub fn developer_iteration(&self) -> &DeveloperIterationEnvelope {
        &self.developer_iteration
    }

    pub fn stable_commands(&self) -> &[StableProofCommand] {
        &self.stable_commands
    }

    pub fn residual_quarantines(&self) -> &[C2QuarantinedClaim] {
        &self.residual_quarantines
    }

    pub(super) fn proof_inventory(&self) -> &CloseoutArtifactReference {
        &self.proof_inventory
    }

    fn validate_surface(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported test architecture closeout schema: {}",
                self.schema_version
            ));
        }
        self.proof_inventory.validate()?;
        self.owner_build_closures.validate()?;
        self.scenario_suite_inventory.validate()?;
        self.preservation.validate()?;
        self.mutation_sensitivity.validate()?;
        if self.mutation_sensitivity.control_evidence_identity()
            != self.preservation.evidence_identity()
        {
            return Err(
                "closeout mutation sensitivity belongs to another preservation authority"
                    .to_owned(),
            );
        }
        self.developer_iteration.validate()?;
        self.ci.validate()?;
        self.artifact_inventory.validate_integrity()?;
        self.artifact_cleanup_plan.validate_integrity()?;
        validate_repository_evidence(&self.repository, &self.developer_iteration, &self.ci)?;
        validate_commands(&self.stable_commands)?;
        if self.artifact_cleanup_plan.inventory_identity()
            != self.artifact_inventory.inventory_identity()
            || self.residual_quarantines != self.preservation.quarantines()
        {
            return Err(
                "closeout bundle contains mismatched lifecycle or quarantine evidence".to_owned(),
            );
        }
        if self.artifact_inventory.reuse_basis().is_none()
            || !self.developer_iteration.cases().iter().any(|case| {
                case.cold.target_root == self.artifact_inventory.target_root()
                    || case.warm.target_root == self.artifact_inventory.target_root()
            })
        {
            return Err(
                "artifact lifecycle evidence is not bound to an observed iteration run".to_owned(),
            );
        }
        let predicates: BTreeSet<_> = self
            .closeout_predicates
            .iter()
            .map(|evidence| evidence.predicate)
            .collect();
        let expected: BTreeSet<_> = [
            CloseoutPredicate::ProofPreservation,
            CloseoutPredicate::MutationSensitivity,
            CloseoutPredicate::DeveloperIteration,
            CloseoutPredicate::OwnerTopology,
            CloseoutPredicate::ScenarioTopology,
            CloseoutPredicate::CiCoverage,
            CloseoutPredicate::ArtifactLifecycle,
        ]
        .into_iter()
        .collect();
        if predicates != expected || predicates.len() != self.closeout_predicates.len() {
            return Err("closeout conjunction is incomplete or duplicated".to_owned());
        }
        let expected_evidence = [
            (
                CloseoutPredicate::ProofPreservation,
                self.preservation.evidence_identity(),
            ),
            (
                CloseoutPredicate::MutationSensitivity,
                self.mutation_sensitivity.evidence_identity(),
            ),
            (
                CloseoutPredicate::DeveloperIteration,
                self.developer_iteration.evidence_identity(),
            ),
            (
                CloseoutPredicate::OwnerTopology,
                self.owner_build_closures.sha256.as_str(),
            ),
            (
                CloseoutPredicate::ScenarioTopology,
                self.scenario_suite_inventory.sha256.as_str(),
            ),
            (
                CloseoutPredicate::CiCoverage,
                self.ci.aggregate_identity.as_str(),
            ),
            (
                CloseoutPredicate::ArtifactLifecycle,
                self.artifact_cleanup_plan.plan_identity(),
            ),
        ];
        for (predicate, identity) in expected_evidence {
            if !self.closeout_predicates.iter().any(|evidence| {
                evidence.predicate == predicate && evidence.evidence_identity == identity
            }) {
                return Err(format!(
                    "closeout predicate {predicate:?} points at unrelated evidence"
                ));
            }
        }
        Ok(())
    }

    fn expected_identity(&self) -> Result<String, String> {
        let mut basis = self.clone();
        basis.evidence_identity.clear();
        sha256_serialized(&basis)
    }

    pub(super) fn ci_identity(&self) -> &str {
        &self.ci.aggregate_identity
    }

    pub(super) fn artifact_lifecycle_identity(&self) -> &str {
        self.artifact_cleanup_plan.plan_identity()
    }
}

fn validate_repository_conjunction(inputs: &TestArchitectureCloseoutInputs) -> Result<(), String> {
    validate_repository_evidence(&inputs.repository, &inputs.developer_iteration, &inputs.ci)
}

fn validate_repository_evidence(
    repository: &RepositoryIdentity,
    iteration: &DeveloperIterationEnvelope,
    ci: &CiCertificationAggregate,
) -> Result<(), String> {
    let profile = iteration.reference_profile();
    if repository.source_tree_digest != sha256_bytes(&[])
        || repository.source_revision != profile.source_revision
        || repository.lockfile_digest != profile.lockfile_sha256
        || repository.rustc_identity != profile.rust_toolchain
        || repository.operating_system != profile.operating_system
        || repository.architecture != profile.architecture
    {
        return Err(
            "closeout requires a clean current repository matching the iteration authority"
                .to_owned(),
        );
    }
    if crate::ci::repository_source_identity(repository)? != ci.source_identity {
        return Err("CI certification belongs to a different repository source".to_owned());
    }
    Ok(())
}

fn predicate(predicate: CloseoutPredicate, identity: &str) -> CloseoutPredicateEvidence {
    CloseoutPredicateEvidence {
        predicate,
        evidence_identity: identity.to_owned(),
    }
}
