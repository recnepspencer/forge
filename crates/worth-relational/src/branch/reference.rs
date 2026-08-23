use serde::{Deserialize, Serialize};
use std::sync::Arc;
use worth_foundational::{
    FoundationalBranchComparisonBasis, FoundationalBranchForkBasis, FoundationalBranchId,
    FoundationalBranchIdConstructionDenial, FoundationalBranchReferenceGeneration,
    FoundationalBranchReferenceObservation, FoundationalBranchTarget,
};

use super::identity::RelationalBranchIdentity;
use super::target::RelationalBranchTarget;
use super::RelationalBranchVersion;
use crate::history::data::BranchId;

/// Exact descriptive branch-reference observation, not a repeatable-read artifact.
pub type RelationalBranchReferenceObservation =
    FoundationalBranchReferenceObservation<RelationalBranchTarget>;
pub type RelationalBranchForkBasis = FoundationalBranchForkBasis<RelationalBranchTarget>;
pub type RelationalBranchComparisonBasis =
    FoundationalBranchComparisonBasis<RelationalBranchTarget>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalBranchObservationConstructionDenial {
    InvalidBranchId(FoundationalBranchIdConstructionDenial),
    EmptyBranchName,
    RuntimeInstanceMismatch {
        observation_runtime_instance_id: u64,
        target_runtime_instance_id: u64,
    },
    ForkProvenanceMismatch,
}

/// Lower an owner branch name into the shared exact observation grammar.
///
/// A basis target must belong to the same runtime as the observation. The
/// typed empty target has no target runtime and remains affine through the
/// owner-qualified branch identity.
pub fn relational_branch_observation(
    runtime_instance_id: u64,
    branch_name: impl AsRef<str>,
    target: FoundationalBranchTarget<RelationalBranchTarget>,
    generation: FoundationalBranchReferenceGeneration,
) -> Result<RelationalBranchReferenceObservation, RelationalBranchObservationConstructionDenial> {
    let branch_name = branch_name.as_ref();
    if branch_name.trim().is_empty() {
        return Err(RelationalBranchObservationConstructionDenial::EmptyBranchName);
    }
    if let FoundationalBranchTarget::Basis(target) = &target {
        if target.runtime_instance_id() != runtime_instance_id {
            return Err(
                RelationalBranchObservationConstructionDenial::RuntimeInstanceMismatch {
                    observation_runtime_instance_id: runtime_instance_id,
                    target_runtime_instance_id: target.runtime_instance_id(),
                },
            );
        }
    }
    let branch_id =
        FoundationalBranchId::new(format!("relational/{runtime_instance_id}/{branch_name}"))?;
    Ok(RelationalBranchReferenceObservation::new(
        branch_id, target, generation,
    ))
}

impl From<FoundationalBranchIdConstructionDenial>
    for RelationalBranchObservationConstructionDenial
{
    fn from(denial: FoundationalBranchIdConstructionDenial) -> Self {
        Self::InvalidBranchId(denial)
    }
}

/// Mutable owner cell for one branch reference. The exact Foundational observation
/// owns target/generation; owner-local truth movement stays in the version counter.
#[derive(Debug, Clone)]
pub(crate) struct RelationalBranchReferenceCell {
    identity: RelationalBranchIdentity,
    observation: RelationalBranchReferenceObservation,
    truth_version: RelationalBranchVersion,
    head_retention_obligations: u32,
    fork_provenance: Option<RelationalBranchReferenceObservation>,
    fork_source_branch_id: Option<BranchId>,
    root: Option<Arc<super::RelationalBranchRoot>>,
    basis_registry: super::RelationalBranchBasisRegistry,
    coordination: Arc<super::coordination::RelationalBranchCoordinationCell>,
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

/// Read-only owner observation of one mutable branch-reference cell.
///
/// This is an evidence surface, not an authority constructor: callers can
/// compare every currentness axis, but they cannot turn the value back into a
/// transaction binding or mutate the cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalBranchReferenceState {
    runtime_instance_id: u64,
    branch_id: BranchId,
    observation: RelationalBranchReferenceObservation,
    truth_version: RelationalBranchVersion,
    head_retention_obligations: u32,
    fork_provenance: Option<RelationalBranchReferenceObservation>,
    fork_source_branch_id: Option<BranchId>,
}

impl RelationalBranchReferenceState {
    pub const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn observation(&self) -> &RelationalBranchReferenceObservation {
        &self.observation
    }

    pub const fn truth_version(&self) -> RelationalBranchVersion {
        self.truth_version
    }

    pub const fn head_retention_obligations(&self) -> u32 {
        self.head_retention_obligations
    }

    pub fn fork_provenance(&self) -> Option<&RelationalBranchReferenceObservation> {
        self.fork_provenance.as_ref()
    }

    pub fn fork_source_branch_id(&self) -> Option<&BranchId> {
        self.fork_source_branch_id.as_ref()
    }
}

/// Exact durable image of one owner branch cell. This is intentionally a
/// checkpoint DTO rather than the live cell: restoring it must validate the
/// runtime-affine identity and never synthesize currentness from a legacy
/// branch-head projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
            observation,
            truth_version: RelationalBranchVersion::initial(),
            head_retention_obligations: 0,
            fork_provenance: None,
            fork_source_branch_id: None,
            root: None,
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
            observation,
            truth_version: RelationalBranchVersion::initial(),
            head_retention_obligations: 1,
            fork_provenance: Some(source.clone()),
            fork_source_branch_id: Some(source_branch_id),
            root: Some(source_root),
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
        let target = match self.observation.target() {
            FoundationalBranchTarget::Empty => FoundationalBranchTarget::empty(),
            FoundationalBranchTarget::Basis(target) => FoundationalBranchTarget::basis(
                target.rebind_runtime_instance_id(runtime_instance_id),
            ),
        };
        let observation = relational_branch_observation(
            runtime_instance_id,
            self.identity.branch_id().0.as_str(),
            target,
            self.observation.generation(),
        )?;
        let fork_provenance = match (
            self.fork_provenance.as_ref(),
            self.fork_source_branch_id.as_ref(),
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
            observation,
            truth_version: self.truth_version,
            head_retention_obligations: self.head_retention_obligations,
            fork_provenance,
            fork_source_branch_id: self.fork_source_branch_id.clone(),
            root: self.root.clone(),
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
        RelationalBranchCellCheckpoint {
            runtime_instance_id: self.identity.runtime_instance_id(),
            branch_id: self.identity.branch_id().clone(),
            observation: self.observation.clone(),
            truth_version: self.truth_version,
            head_retention_obligations: self.head_retention_obligations,
            fork_provenance: self.fork_provenance.clone(),
            fork_source_branch_id: self.fork_source_branch_id.clone(),
        }
    }

    pub(crate) fn evidence_state(&self) -> RelationalBranchReferenceState {
        RelationalBranchReferenceState {
            runtime_instance_id: self.identity.runtime_instance_id(),
            branch_id: self.identity.branch_id().clone(),
            observation: self.observation.clone(),
            truth_version: self.truth_version,
            head_retention_obligations: self.head_retention_obligations,
            fork_provenance: self.fork_provenance.clone(),
            fork_source_branch_id: self.fork_source_branch_id.clone(),
        }
    }

    pub(crate) fn observation(&self) -> &RelationalBranchReferenceObservation {
        &self.observation
    }

    pub(crate) fn truth_version(&self) -> RelationalBranchVersion {
        self.truth_version
    }

    pub(crate) fn fork_provenance(&self) -> Option<&RelationalBranchReferenceObservation> {
        self.fork_provenance.as_ref()
    }

    pub(crate) fn fork_source_branch_id(&self) -> Option<&BranchId> {
        self.fork_source_branch_id.as_ref()
    }

    pub(crate) fn advance_metadata(&mut self) -> Result<(), RelationalBranchCellDenial> {
        self.observation = RelationalBranchReferenceObservation::new(
            self.observation.branch_id().clone(),
            self.observation.target().clone(),
            self.observation
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
        let generation = self
            .observation
            .generation()
            .checked_advance()
            .map_err(|_| RelationalBranchCellDenial::GenerationOverflow)?;
        let truth_version = self
            .truth_version
            .checked_advance()
            .ok_or(RelationalBranchCellDenial::TruthVersionOverflow)?;
        let candidate = RelationalBranchReferenceObservation::new(
            self.observation.branch_id().clone(),
            target,
            generation,
        );
        if let FoundationalBranchTarget::Basis(target) = candidate.target() {
            if target.runtime_instance_id() != self.identity.runtime_instance_id() {
                return Err(RelationalBranchCellDenial::RuntimeInstanceMismatch);
            }
        }
        self.observation = candidate;
        self.truth_version = truth_version;
        Ok(())
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
