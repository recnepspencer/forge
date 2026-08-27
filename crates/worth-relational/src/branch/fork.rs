use std::sync::Arc;

use worth_foundational::FoundationalBranchTarget;

use crate::history::data::{BranchCreateError, BranchId, CommitId};
use crate::history::HistoryAuthority;
use crate::runtime::RelationalRuntime;

use super::authority::admit_relational_fork_source;
use super::fork_source_basis::{AdmittedRelationalForkSourceBasis, RelationalForkSourceDescriptor};
use super::identity::RelationalBranchIdentity;
use super::reference::{RelationalBranchCellDenial, RelationalBranchReferenceObservation};
use super::RelationalBranchVersion;

/// Typed denials for the Phase-4 fork transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalForkDenial {
    SourceBranchMissing,
    EmptySource,
    DuplicateTarget,
    ForeignRuntime,
    StaleSource,
    MissingArtifact,
    InvalidTarget(BranchCreateError),
    Cell(RelationalBranchCellDenial),
}

/// Read-only evidence returned after a successful branch-reference fork.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalForkOutcome {
    target_identity: RelationalBranchIdentity,
    source_observation: RelationalBranchReferenceObservation,
    target_observation: RelationalBranchReferenceObservation,
    fork_provenance: RelationalBranchReferenceObservation,
    source_truth_version: RelationalBranchVersion,
    target_truth_version: RelationalBranchVersion,
    shared_commit_id: Option<CommitId>,
}

impl RelationalForkOutcome {
    pub fn target_identity(&self) -> &RelationalBranchIdentity {
        &self.target_identity
    }

    pub fn source_observation(&self) -> &RelationalBranchReferenceObservation {
        &self.source_observation
    }

    pub fn target_observation(&self) -> &RelationalBranchReferenceObservation {
        &self.target_observation
    }

    pub fn fork_provenance(&self) -> &RelationalBranchReferenceObservation {
        &self.fork_provenance
    }

    pub const fn source_truth_version(&self) -> RelationalBranchVersion {
        self.source_truth_version
    }

    pub const fn target_truth_version(&self) -> RelationalBranchVersion {
        self.target_truth_version
    }

    pub const fn shared_commit_id(&self) -> Option<CommitId> {
        self.shared_commit_id
    }
}

impl RelationalRuntime {
    /// Observe an exact live source and issue the owner-sealed fork-only
    /// token. Empty branches do not produce a fork source.
    pub fn observe_fork_source(
        &mut self,
        source_branch: &BranchId,
    ) -> Result<
        (
            RelationalForkSourceDescriptor,
            AdmittedRelationalForkSourceBasis,
        ),
        RelationalForkDenial,
    > {
        let source_cell = self
            .history
            .branch_cell(source_branch)
            .ok_or(RelationalForkDenial::SourceBranchMissing)?;
        let source_snapshot = source_cell.atomic_snapshot();
        let observation = source_snapshot.observation();
        let truth_version = source_snapshot.truth_version();
        self.history.phase4_costs.branch_cell_lookups = self
            .history
            .phase4_costs
            .branch_cell_lookups
            .saturating_add(1);
        self.history.phase4_costs.branch_cell_contacts = self
            .history
            .phase4_costs
            .branch_cell_contacts
            .saturating_add(1);
        if observation.target().is_empty() {
            return Err(RelationalForkDenial::EmptySource);
        }
        let descriptor = RelationalForkSourceDescriptor::new(
            self.runtime_instance_id(),
            observation,
            source_branch.clone(),
            truth_version,
        );
        let token = admit_relational_fork_source(descriptor.clone());
        Ok((descriptor, token))
    }

    /// Consume a fork-only source token and create a fresh branch reference.
    /// The immutable source artifact is shared by identity; no envelope copy
    /// or source-cell mutation is performed.
    pub fn fork_branch(
        &mut self,
        target_branch: BranchId,
        source: AdmittedRelationalForkSourceBasis,
    ) -> Result<RelationalForkOutcome, RelationalForkDenial> {
        let (descriptor, _authority) = source.into_parts();
        let validated = self.validate_fork_source(&target_branch, descriptor)?;
        let prepared = self.prepare_fork_target(target_branch, &validated)?;
        self.install_fork_target(validated, prepared)
    }

    fn validate_fork_source(
        &mut self,
        target_branch: &BranchId,
        descriptor: RelationalForkSourceDescriptor,
    ) -> Result<ValidatedForkSource, RelationalForkDenial> {
        if descriptor.runtime_instance_id() != self.runtime_instance_id() {
            return Err(RelationalForkDenial::ForeignRuntime);
        }
        if self.history.has_branch(target_branch) {
            return Err(RelationalForkDenial::DuplicateTarget);
        }
        let (source_observation, source_truth_version, source_root) = {
            let source_cell = self
                .history
                .branch_cell(descriptor.source_branch())
                .ok_or(RelationalForkDenial::SourceBranchMissing)?;
            let snapshot = source_cell.atomic_snapshot();
            (
                snapshot.observation(),
                snapshot.truth_version(),
                snapshot.root(),
            )
        };
        self.history.phase4_costs.branch_cell_lookups = self
            .history
            .phase4_costs
            .branch_cell_lookups
            .saturating_add(1);
        self.history.phase4_costs.branch_cell_contacts = self
            .history
            .phase4_costs
            .branch_cell_contacts
            .saturating_add(1);
        if descriptor.truth_version() != source_truth_version
            || descriptor
                .observation()
                .compare(&source_observation)
                .is_err()
        {
            return Err(RelationalForkDenial::StaleSource);
        }
        Ok(ValidatedForkSource {
            descriptor,
            source_observation,
            source_truth_version,
            source_root: source_root.ok_or(RelationalForkDenial::MissingArtifact)?,
        })
    }

    fn prepare_fork_target(
        &mut self,
        target_branch: BranchId,
        source: &ValidatedForkSource,
    ) -> Result<PreparedForkTarget, RelationalForkDenial> {
        let target_branch_id = target_branch.clone();
        let source_root = Arc::clone(&source.source_root);
        let target_cell = crate::branch::RelationalBranchReferenceCell::from_source_with_root(
            self.runtime_instance_id(),
            target_branch,
            source.descriptor.source_branch().clone(),
            &source.source_observation,
            source_root.clone(),
        )
        .map_err(|_| RelationalForkDenial::InvalidTarget(BranchCreateError::invalid_target()))?;
        let target_observation = target_cell.observation().clone();
        let fork_provenance = target_cell
            .fork_provenance()
            .expect("forked branch must retain exact source provenance");
        let target_truth_version = target_cell.truth_version();
        let shared_commit_id = match target_observation.target() {
            FoundationalBranchTarget::Empty => None,
            FoundationalBranchTarget::Basis(target) => Some(CommitId(target.selected_commit_id())),
        };
        let source_head_version = if let Some(commit_id) = shared_commit_id {
            self.history.phase4_costs.catalog_lookups =
                self.history.phase4_costs.catalog_lookups.saturating_add(1);
            let version_id = self
                .history
                .commit_catalog
                .get(commit_id)
                .map(|artifact| artifact.identity().version_id())
                .or_else(|| {
                    source_root
                        .canonical_envelope()
                        .filter(|envelope| envelope.commit.commit_id == commit_id)
                        .map(|envelope| envelope.commit.version_id)
                })
                .ok_or(RelationalForkDenial::MissingArtifact)?;
            Some(version_id)
        } else {
            None
        };
        Ok(PreparedForkTarget {
            target_cell,
            target_branch: target_branch_id,
            source_root,
            target_observation,
            fork_provenance,
            target_truth_version,
            shared_commit_id,
            source_head_version,
        })
    }

    fn install_fork_target(
        &mut self,
        source: ValidatedForkSource,
        target: PreparedForkTarget,
    ) -> Result<RelationalForkOutcome, RelationalForkDenial> {
        let source_branch = source.descriptor.source_branch().clone();
        self.history
            .branch_cell_mut(&source_branch)
            .expect("source branch cell remains registered")
            .retain_head()
            .map_err(RelationalForkDenial::Cell)?;
        let fork_materialized_authoritative_bytes = target
            .target_cell
            .root()
            .filter(|target_root| !Arc::ptr_eq(target_root, &target.source_root))
            .map(|target_root| target_root.logical_partition_payload_bytes())
            .unwrap_or(0);
        let copied_commit_envelopes = target
            .target_cell
            .root()
            .filter(|target_root| !target_root.shares_canonical_envelope_with(&target.source_root))
            .map_or(0, |_| 1);
        let materialization_cost = crate::runtime::RelationalForkMaterializationCost {
            entity_count: 0,
            relation_count: 0,
            authoritative_bytes: fork_materialized_authoritative_bytes,
            copied_commit_envelopes,
        };
        self.history.phase4_costs.reference_allocations = self
            .history
            .phase4_costs
            .reference_allocations
            .saturating_add(1);
        self.history.phase4_costs.branch_cell_contacts = self
            .history
            .phase4_costs
            .branch_cell_contacts
            .saturating_add(1);
        self.history.insert_branch_cell(target.target_cell);
        self.history
            .record_fork_root_acquisition(&target.target_branch);
        self.history
            .record_fork_materialization(&target.target_branch, materialization_cost);
        if let Some(source_head_version) = target.source_head_version {
            self.visibility_pins()
                .move_branch_head_visibility_residency(
                    &target.target_branch,
                    None,
                    Some(source_head_version),
                );
            self.visibility_pins()
                .pin_branch_version(source_head_version);
        }
        Ok(RelationalForkOutcome {
            target_identity: self
                .history
                .branch_cell(&target.target_branch)
                .expect("forked branch cell must be registered")
                .identity()
                .clone(),
            source_observation: source.source_observation,
            target_observation: target.target_observation,
            fork_provenance: target.fork_provenance,
            source_truth_version: source.source_truth_version,
            target_truth_version: target.target_truth_version,
            shared_commit_id: target.shared_commit_id,
        })
    }
}

struct ValidatedForkSource {
    descriptor: RelationalForkSourceDescriptor,
    source_observation: RelationalBranchReferenceObservation,
    source_truth_version: RelationalBranchVersion,
    source_root: Arc<crate::branch::RelationalBranchRoot>,
}

struct PreparedForkTarget {
    target_cell: crate::branch::RelationalBranchReferenceCell,
    target_branch: BranchId,
    source_root: Arc<crate::branch::RelationalBranchRoot>,
    target_observation: RelationalBranchReferenceObservation,
    fork_provenance: RelationalBranchReferenceObservation,
    target_truth_version: RelationalBranchVersion,
    shared_commit_id: Option<CommitId>,
    source_head_version: Option<crate::identity::data::VersionId>,
}

impl HistoryAuthority<'_> {
    /// In-crate compatibility adapter for replay and preservation callers.
    /// It observes an owner fork token and delegates to `fork_branch`; it is
    /// not a public currentness door and is not a second fork authority.
    pub(crate) fn fork_branch_from(
        &mut self,
        new_branch: BranchId,
        from_branch: &BranchId,
    ) -> Result<(), BranchCreateError> {
        let runtime = self.runtime();
        let (_, basis) =
            runtime
                .observe_fork_source(from_branch)
                .map_err(|denial| match denial {
                    RelationalForkDenial::DuplicateTarget => {
                        BranchCreateError::branch_already_exists()
                    }
                    _ => BranchCreateError::source_branch_missing(),
                })?;
        runtime
            .fork_branch(new_branch, basis)
            .map(|_| ())
            .map_err(|denial| match denial {
                RelationalForkDenial::DuplicateTarget => BranchCreateError::branch_already_exists(),
                _ => BranchCreateError::source_branch_missing(),
            })
    }
}
