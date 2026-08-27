use crate::durability::data::{DurabilityError, RecoveryFailureClass};
use crate::history::data::CanonicalCommitEnvelope;
use crate::runtime::RelationalRuntime;

use super::root_inventory::RecoveredRootInventory;

pub(super) fn admit_legacy_branch_from_first_parent(
    restored: &mut RelationalRuntime,
    envelope: &CanonicalCommitEnvelope,
    recovered_roots: &RecoveredRootInventory,
) -> Result<(), DurabilityError> {
    let parent_id = envelope
        .commit
        .ordered_parents()
        .as_slice()
        .first()
        .copied()
        .ok_or_else(|| {
            DurabilityError::new(
                RecoveryFailureClass::CorruptCheckpoint,
                format!(
                    "legacy branch `{}` has no recoverable fork parent",
                    envelope.branch_context.0
                ),
            )
        })?;
    let parent_envelope = restored
        .history
        .canonical_envelope(parent_id)
        .ok_or_else(|| {
            DurabilityError::new(
                RecoveryFailureClass::MissingAuthoritativeParentClosure,
                format!("legacy fork parent {} is unavailable", parent_id.0),
            )
        })?;
    let parent_root = recovered_roots.resolve(parent_id).ok_or_else(|| {
        DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("legacy fork parent root {} is unavailable", parent_id.0),
        )
    })?;
    let root_descriptor = parent_root.descriptor().cloned().ok_or_else(|| {
        DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("legacy fork parent root {} has no descriptor", parent_id.0),
        )
    })?;
    let source_observation = crate::branch::relational_branch_observation(
        restored.runtime_instance_id(),
        &parent_envelope.branch_context.0,
        worth_foundational::FoundationalBranchTarget::basis(
            crate::branch::RelationalBranchTarget::from_commit_receipt(
                restored.runtime_instance_id(),
                &parent_envelope.commit,
                root_descriptor,
            ),
        ),
        worth_foundational::FoundationalBranchReferenceGeneration::initial(),
    )
    .map_err(|denial| {
        DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("legacy fork source admission denied: {denial:?}"),
        )
    })?;
    let cell = crate::branch::RelationalBranchReferenceCell::from_source_with_root(
        restored.runtime_instance_id(),
        envelope.branch_context.clone(),
        parent_envelope.branch_context.clone(),
        &source_observation,
        parent_root,
    )
    .map_err(|denial| {
        DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("legacy branch admission denied: {denial:?}"),
        )
    })?;
    restored
        .history
        .install_branch_head(
            cell.identity().clone(),
            &cell
                .root()
                .expect("legacy branch admission carries its parent root"),
            cell.head_retention(),
        )
        .map_err(|denial| {
            DurabilityError::new(
                RecoveryFailureClass::ReplayFailure,
                format!("legacy branch head retention denied: {denial:?}"),
            )
        })?;
    let recovered_head_version = match cell.observation().target() {
        worth_foundational::FoundationalBranchTarget::Empty => None,
        worth_foundational::FoundationalBranchTarget::Basis(target) => {
            Some(crate::identity::data::VersionId(target.version_id()))
        }
    };
    restored.history.insert_branch_cell(cell);
    restored
        .history
        .move_branch_head_version(None, recovered_head_version);
    Ok(())
}

pub(super) fn recovered_branch_basis(
    restored: &RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
) -> Result<crate::branch::AdmittedRelationalBranchBasis, DurabilityError> {
    let identity = restored.branch_identity(branch_id).map_err(|denial| {
        DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("recovery branch identity admission denied: {denial:?}"),
        )
    })?;
    restored
        .admitted_branch_basis_for_identity(&identity)
        .map_err(|denial| {
            DurabilityError::new(
                RecoveryFailureClass::CorruptCheckpoint,
                format!("recovery branch binding admission denied: {denial:?}"),
            )
        })
}
