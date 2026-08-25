use std::sync::{Arc, Mutex};
use worth_foundational::{FoundationalBranchReferenceGeneration, FoundationalBranchTarget};

use super::identity::RelationalBranchIdentity;
use super::reference_state::RelationalBranchReferenceState;
use super::target::RelationalBranchTarget;
use super::RelationalBranchVersion;
use crate::history::data::BranchId;

#[path = "reference_observation.rs"]
mod observation;
pub use observation::{
    relational_branch_observation, RelationalBranchComparisonBasis, RelationalBranchForkBasis,
    RelationalBranchObservationConstructionDenial, RelationalBranchReferenceObservation,
};

/// Mutable owner cell for one branch reference. The exact Foundational observation
/// owns target/generation; owner-local truth movement stays in the version counter.
#[derive(Debug)]
pub(crate) struct RelationalBranchReferenceCell {
    pub(super) identity: RelationalBranchIdentity,
    pub(super) state: Arc<Mutex<RelationalBranchReferenceMutableState>>,
    pub(super) basis_registry: super::RelationalBranchBasisRegistry,
    pub(super) coordination: Arc<super::coordination::RelationalBranchCoordinationCell>,
}

#[derive(Debug, Clone)]
pub(crate) struct RelationalBranchReferenceMutableState {
    observation: RelationalBranchReferenceObservation,
    truth_version: RelationalBranchVersion,
    head_retention_obligations: u32,
    fork_provenance: Option<RelationalBranchReferenceObservation>,
    fork_source_branch_id: Option<BranchId>,
    root: Option<Arc<super::RelationalBranchRoot>>,
}

impl RelationalBranchReferenceMutableState {
    pub(super) fn currently_selects_root(
        &self,
        expected: &Arc<super::RelationalBranchRoot>,
    ) -> bool {
        self.root
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, expected))
    }
}

impl Clone for RelationalBranchReferenceCell {
    fn clone(&self) -> Self {
        Self {
            identity: self.identity.clone(),
            state: Arc::new(Mutex::new(self.state_snapshot())),
            basis_registry: self.basis_registry.clone(),
            coordination: Arc::clone(&self.coordination),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalBranchCellDenial {
    GenerationOverflow,
    TruthVersionOverflow,
    RetentionOverflow,
    RuntimeInstanceMismatch,
    BranchIdentityMismatch,
    CheckpointRuntimeMismatch,
    CheckpointObservationMismatch,
    CheckpointForkProvenanceMismatch,
    CheckpointRootReadmissionRequired,
}

/// Exact durable image of one owner branch cell. This is intentionally a
/// checkpoint DTO rather than the live cell: restoring it must validate the
/// runtime-affine identity and never synthesize currentness from a legacy
/// branch-head projection.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct RelationalBranchCellCheckpoint {
    pub(crate) runtime_instance_id: u64,
    pub(crate) branch_id: BranchId,
    pub(crate) observation: RelationalBranchReferenceObservation,
    pub(crate) truth_version: RelationalBranchVersion,
    pub(crate) head_retention_obligations: u32,
    pub(crate) fork_provenance: Option<RelationalBranchReferenceObservation>,
    pub(crate) fork_source_branch_id: Option<BranchId>,
}

impl RelationalBranchReferenceCell {
    /// Detach one exact image of every mutable branch-reference axis.
    ///
    /// Public observations and fork admission must derive reference, truth,
    /// and root evidence from this single lock acquisition.
    pub(crate) fn atomic_snapshot(&self) -> Self {
        Self {
            identity: self.identity.clone(),
            state: Arc::new(Mutex::new(self.state_snapshot())),
            basis_registry: self.basis_registry.clone(),
            coordination: Arc::clone(&self.coordination),
        }
    }

    pub(crate) fn empty(
        runtime_instance_id: u64,
        branch_id: BranchId,
    ) -> Result<Self, RelationalBranchObservationConstructionDenial> {
        let observation = relational_branch_observation(
            runtime_instance_id,
            &branch_id.0,
            FoundationalBranchTarget::empty(),
            FoundationalBranchReferenceGeneration::initial(),
        )?;
        Ok(Self {
            identity: RelationalBranchIdentity::new(runtime_instance_id, branch_id.clone()),
            state: Arc::new(Mutex::new(RelationalBranchReferenceMutableState {
                observation,
                truth_version: RelationalBranchVersion::initial(),
                head_retention_obligations: 0,
                fork_provenance: None,
                fork_source_branch_id: None,
                root: None,
            })),
            basis_registry: super::RelationalBranchBasisRegistry::default(),
            coordination: super::coordination::RelationalBranchCoordinationCell::fresh(
                runtime_instance_id,
                &branch_id,
            ),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_source(
        runtime_instance_id: u64,
        branch_id: BranchId,
        source_branch_id: BranchId,
        source: &RelationalBranchReferenceObservation,
    ) -> Result<Self, RelationalBranchObservationConstructionDenial> {
        Self::from_source_with_root(
            runtime_instance_id,
            branch_id,
            source_branch_id,
            source,
            super::RelationalBranchRoot::empty(),
        )
    }

    pub(crate) fn from_source_with_root(
        runtime_instance_id: u64,
        branch_id: BranchId,
        source_branch_id: BranchId,
        source: &RelationalBranchReferenceObservation,
        source_root: Arc<super::RelationalBranchRoot>,
    ) -> Result<Self, RelationalBranchObservationConstructionDenial> {
        let target = match source.target() {
            FoundationalBranchTarget::Empty => FoundationalBranchTarget::empty(),
            FoundationalBranchTarget::Basis(target) => FoundationalBranchTarget::basis(
                target.rebind_runtime_instance_id(runtime_instance_id),
            ),
        };
        let observation = relational_branch_observation(
            runtime_instance_id,
            &branch_id.0,
            target,
            FoundationalBranchReferenceGeneration::initial(),
        )?;
        Ok(Self {
            identity: RelationalBranchIdentity::new(runtime_instance_id, branch_id.clone()),
            state: Arc::new(Mutex::new(RelationalBranchReferenceMutableState {
                observation,
                truth_version: RelationalBranchVersion::initial(),
                head_retention_obligations: 1,
                fork_provenance: Some(source.clone()),
                fork_source_branch_id: Some(source_branch_id),
                root: Some(source_root),
            })),
            basis_registry: super::RelationalBranchBasisRegistry::default(),
            coordination: super::coordination::RelationalBranchCoordinationCell::fresh(
                runtime_instance_id,
                &branch_id,
            ),
        })
    }

    pub(crate) fn rebind_runtime(
        &self,
        runtime_instance_id: u64,
    ) -> Result<Self, RelationalBranchObservationConstructionDenial> {
        let state = self.state_snapshot();
        let target = match state.observation.target() {
            FoundationalBranchTarget::Empty => FoundationalBranchTarget::empty(),
            FoundationalBranchTarget::Basis(target) => FoundationalBranchTarget::basis(
                target.rebind_runtime_instance_id(runtime_instance_id),
            ),
        };
        let observation = relational_branch_observation(
            runtime_instance_id,
            self.identity.branch_id().0.as_str(),
            target,
            state.observation.generation(),
        )?;
        let fork_provenance = match (
            state.fork_provenance.as_ref(),
            state.fork_source_branch_id.as_ref(),
        ) {
            (None, None) => Ok(None),
            (Some(source), Some(source_branch_id)) => {
                let expected_branch_id = format!(
                    "relational/{}/{}",
                    self.identity.runtime_instance_id(),
                    source_branch_id.0
                );
                if source.branch_id().as_str() != expected_branch_id {
                    return Err(
                        RelationalBranchObservationConstructionDenial::ForkProvenanceMismatch,
                    );
                }
                let target = match source.target() {
                    FoundationalBranchTarget::Empty => FoundationalBranchTarget::empty(),
                    FoundationalBranchTarget::Basis(target) => FoundationalBranchTarget::basis(
                        target.rebind_runtime_instance_id(runtime_instance_id),
                    ),
                };
                relational_branch_observation(
                    runtime_instance_id,
                    &source_branch_id.0,
                    target,
                    source.generation(),
                )
                .map(Some)
            }
            _ => return Err(RelationalBranchObservationConstructionDenial::ForkProvenanceMismatch),
        }?;
        Ok(Self {
            identity: self.identity.rebind(runtime_instance_id),
            state: Arc::new(Mutex::new(RelationalBranchReferenceMutableState {
                observation,
                truth_version: state.truth_version,
                head_retention_obligations: state.head_retention_obligations,
                fork_provenance,
                fork_source_branch_id: state.fork_source_branch_id,
                root: state.root,
            })),
            basis_registry: super::RelationalBranchBasisRegistry::default(),
            coordination: super::coordination::RelationalBranchCoordinationCell::fresh(
                runtime_instance_id,
                self.identity.branch_id(),
            ),
        })
    }

    pub(crate) fn identity(&self) -> &RelationalBranchIdentity {
        &self.identity
    }

    pub(crate) fn checkpoint(&self) -> RelationalBranchCellCheckpoint {
        let state = self.state_snapshot();
        RelationalBranchCellCheckpoint {
            runtime_instance_id: self.identity.runtime_instance_id(),
            branch_id: self.identity.branch_id().clone(),
            observation: state.observation,
            truth_version: state.truth_version,
            head_retention_obligations: state.head_retention_obligations,
            fork_provenance: state.fork_provenance,
            fork_source_branch_id: state.fork_source_branch_id,
        }
    }

    pub(crate) fn evidence_state(&self) -> RelationalBranchReferenceState {
        let state = self.state_snapshot();
        RelationalBranchReferenceState::new(
            self.identity.runtime_instance_id(),
            self.identity.branch_id().clone(),
            state.observation,
            state.truth_version,
            state.head_retention_obligations,
            state.fork_provenance,
            state.fork_source_branch_id,
        )
    }

    pub(crate) fn observation(&self) -> RelationalBranchReferenceObservation {
        self.state_snapshot().observation
    }

    pub(crate) fn truth_version(&self) -> RelationalBranchVersion {
        self.state_snapshot().truth_version
    }

    pub(crate) fn fork_provenance(&self) -> Option<RelationalBranchReferenceObservation> {
        self.state_snapshot().fork_provenance
    }

    pub(crate) fn fork_source_branch_id(&self) -> Option<BranchId> {
        self.state_snapshot().fork_source_branch_id
    }

    pub(crate) fn advance_metadata(&mut self) -> Result<(), RelationalBranchCellDenial> {
        self.advance_metadata_to(self.observation().target().clone())
    }

    pub(crate) fn advance_metadata_to(
        &mut self,
        target: FoundationalBranchTarget<RelationalBranchTarget>,
    ) -> Result<(), RelationalBranchCellDenial> {
        if let FoundationalBranchTarget::Basis(target) = &target {
            if target.runtime_instance_id() != self.identity.runtime_instance_id() {
                return Err(RelationalBranchCellDenial::RuntimeInstanceMismatch);
            }
        }
        let mut state = self.state();
        state.observation = RelationalBranchReferenceObservation::new(
            state.observation.branch_id().clone(),
            target,
            state
                .observation
                .generation()
                .checked_advance()
                .map_err(|_| RelationalBranchCellDenial::GenerationOverflow)?,
        );
        Ok(())
    }

    pub(crate) fn advance_truth(
        &mut self,
        target: FoundationalBranchTarget<RelationalBranchTarget>,
    ) -> Result<(), RelationalBranchCellDenial> {
        let mut state = self.state();
        let generation = state
            .observation
            .generation()
            .checked_advance()
            .map_err(|_| RelationalBranchCellDenial::GenerationOverflow)?;
        let truth_version = state
            .truth_version
            .checked_advance()
            .ok_or(RelationalBranchCellDenial::TruthVersionOverflow)?;
        let candidate = RelationalBranchReferenceObservation::new(
            state.observation.branch_id().clone(),
            target,
            generation,
        );
        if let FoundationalBranchTarget::Basis(target) = candidate.target() {
            if target.runtime_instance_id() != self.identity.runtime_instance_id() {
                return Err(RelationalBranchCellDenial::RuntimeInstanceMismatch);
            }
        }
        state.observation = candidate;
        state.truth_version = truth_version;
        Ok(())
    }

    pub(crate) fn state_snapshot(&self) -> RelationalBranchReferenceMutableState {
        self.state().clone()
    }

    pub(crate) fn replace_state(&self, next: RelationalBranchReferenceMutableState) {
        *self.state() = next;
    }

    pub(super) fn state(&self) -> std::sync::MutexGuard<'_, RelationalBranchReferenceMutableState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

#[cfg(test)]
#[path = "reference_tests.rs"]
mod tests;

#[path = "reference_checkpoint.rs"]
mod checkpoint;
#[path = "reference_coordination_access.rs"]
mod coordination_access;
#[path = "reference_head_retention.rs"]
mod head_retention;
#[path = "reference_root_access.rs"]
mod root_access;
