use forge_proof::TransitionOutcome;
use serde::{Deserialize, Serialize};

use crate::state::{SignalBranchHandle, SignalBranchId, SignalSnapshotId, SignalSnapshotV1};

use super::super::runtime_state::{ExplicitBranchForkPacket, SignalRuntime};
use super::branches::{BranchAncestryState, BranchState};
use super::fork_snapshot::materialize_snapshot_fork_state;
use super::{SignalBranchBasisArtifact, SignalBranchBasisDenial};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalBranchForkRequestBasis {
    CurrentBranchHead,
    ParentBranchHead {
        parent_branch_id: SignalBranchId,
    },
    ParentBranchSnapshot {
        parent_branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalBranchForkRequest {
    branch_name: String,
    basis: SignalBranchForkRequestBasis,
}

impl SignalBranchForkRequest {
    pub fn from_current_branch_head(name: impl Into<String>) -> Self {
        Self {
            branch_name: name.into(),
            basis: SignalBranchForkRequestBasis::CurrentBranchHead,
        }
    }

    pub fn from_parent_branch_head(
        name: impl Into<String>,
        parent_branch_id: SignalBranchId,
    ) -> Self {
        Self {
            branch_name: name.into(),
            basis: SignalBranchForkRequestBasis::ParentBranchHead { parent_branch_id },
        }
    }

    pub fn from_parent_branch_snapshot(
        name: impl Into<String>,
        parent_branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    ) -> Self {
        Self {
            branch_name: name.into(),
            basis: SignalBranchForkRequestBasis::ParentBranchSnapshot {
                parent_branch_id,
                snapshot_id,
            },
        }
    }

    pub fn branch_name(&self) -> &str {
        &self.branch_name
    }

    pub fn basis(&self) -> &SignalBranchForkRequestBasis {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalBranchForkDenial {
    UnknownParentBranch {
        parent_branch_id: SignalBranchId,
    },
    UnknownForkSnapshot {
        parent_branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    },
    SnapshotBasisMismatch {
        requested_snapshot_id: SignalSnapshotId,
        provided_snapshot_id: SignalSnapshotId,
    },
    SnapshotPayloadRequiredForFork {
        request: SignalBranchForkRequest,
    },
    IncompatibleForkSnapshotLineage {
        parent_branch_id: SignalBranchId,
        snapshot_branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
    },
}

#[derive(Debug, Clone)]
pub struct SignalBranchForkReceipt {
    request: SignalBranchForkRequest,
    parent_basis: SignalBranchBasisArtifact,
    requested_snapshot_basis: Option<SignalBranchBasisArtifact>,
    created_branch: SignalBranchHandle,
    created_branch_basis: SignalBranchBasisArtifact,
    active_branch_after_fork_basis: SignalBranchBasisArtifact,
}

impl SignalBranchForkReceipt {
    pub fn request(&self) -> &SignalBranchForkRequest {
        &self.request
    }

    pub fn parent_basis(&self) -> &SignalBranchBasisArtifact {
        &self.parent_basis
    }

    pub fn requested_snapshot_basis(&self) -> Option<&SignalBranchBasisArtifact> {
        self.requested_snapshot_basis.as_ref()
    }

    pub fn created_branch(&self) -> &SignalBranchHandle {
        &self.created_branch
    }

    pub fn created_branch_basis(&self) -> &SignalBranchBasisArtifact {
        &self.created_branch_basis
    }

    pub fn active_branch_after_fork_basis(&self) -> &SignalBranchBasisArtifact {
        &self.active_branch_after_fork_basis
    }
}

struct ResolvedForkRequest<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    parent_branch: SignalBranchHandle,
    parent_basis: SignalBranchBasisArtifact,
    requested_snapshot_basis: Option<SignalBranchBasisArtifact>,
    created_branch_head_snapshot_id: Option<SignalSnapshotId>,
    source_branch_state: BranchState<D, I, T>,
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn fork_branch(
        &mut self,
        request: SignalBranchForkRequest,
    ) -> TransitionOutcome<SignalBranchForkReceipt, SignalBranchForkDenial> {
        if matches!(
            request.basis(),
            SignalBranchForkRequestBasis::ParentBranchSnapshot { .. }
        ) {
            self.telemetry.transaction.explicit_fork_denial_count += 1;
            return TransitionOutcome::denied(
                SignalBranchForkDenial::SnapshotPayloadRequiredForFork {
                    request: request.clone(),
                },
            );
        }
        self.fork_branch_resolved(request, None)
    }

    pub fn fork_branch_with_snapshot(
        &mut self,
        request: SignalBranchForkRequest,
        snapshot: &SignalSnapshotV1,
    ) -> TransitionOutcome<SignalBranchForkReceipt, SignalBranchForkDenial> {
        self.fork_branch_resolved(request, Some(snapshot))
    }

    fn fork_branch_resolved(
        &mut self,
        request: SignalBranchForkRequest,
        snapshot: Option<&SignalSnapshotV1>,
    ) -> TransitionOutcome<SignalBranchForkReceipt, SignalBranchForkDenial> {
        let resolved = match self.resolve_branch_fork_request(&request, snapshot) {
            Ok(resolved) => resolved,
            Err(denial) => {
                self.telemetry.transaction.explicit_fork_denial_count += 1;
                return TransitionOutcome::denied(denial);
            }
        };

        let current_branch_name = resolved.parent_branch.name.clone();
        let parent_branch_id = resolved.parent_branch.id;
        let mut branch_state = resolved.source_branch_state;
        let handle = self.graph.diagnostics_state_mut().create_branch_from_basis(
            request.branch_name().to_owned(),
            parent_branch_id,
            resolved.created_branch_head_snapshot_id,
        );
        *branch_state.ancestry_mut() = BranchAncestryState::new(
            handle.id,
            Some(parent_branch_id),
            resolved.created_branch_head_snapshot_id,
        );
        branch_state.reset_mutation_ledger(resolved.created_branch_head_snapshot_id);
        branch_state.clear_branch_mutation_nodes();
        self.branches
            .branch_mutation_ledger_mut(parent_branch_id, resolved.parent_branch.head_snapshot_id)
            .clear_all(resolved.parent_branch.head_snapshot_id);
        branch_state
            .graph_mut()
            .diagnostics_state_mut()
            .set_active_branch(handle.id);
        self.telemetry.transaction.explicit_fork_count += 1;
        if resolved.requested_snapshot_basis.is_some() {
            self.telemetry.transaction.explicit_snapshot_fork_count += 1;
        }
        self.branches
            .store_fork_packet(ExplicitBranchForkPacket::new(
                parent_branch_id,
                handle.id,
                branch_state,
            ))
            .expect("validated fork admission must produce a well-formed fork packet");
        let branch_catalog = self.graph.diagnostics_state().branch_catalog().clone();
        self.synchronize_branch_catalogs(branch_catalog);
        crate::diagnostics::recorder::record_snapshot_event(
            &mut self.graph,
            crate::diagnostics::replay::ReplayEventKind::BranchCreated,
            None,
            format!("created branch `{}`", handle.name),
        );
        crate::diagnostics::recorder::record_branch_fork_lineage(
            &mut self.graph,
            handle.id,
            parent_branch_id,
            handle.name.clone(),
            current_branch_name,
        );

        let created_branch_basis = match self.branch_basis_artifact(handle.clone()) {
            TransitionOutcome::Success(basis) => basis,
            other => {
                panic!("created branch basis must validate immediately after admission: {other:?}")
            }
        };
        let active_branch_after_fork_basis = self.current_branch_basis_artifact();

        TransitionOutcome::success(SignalBranchForkReceipt {
            request,
            parent_basis: resolved.parent_basis,
            requested_snapshot_basis: resolved.requested_snapshot_basis,
            created_branch: handle,
            created_branch_basis,
            active_branch_after_fork_basis,
        })
    }

    fn resolve_branch_fork_request(
        &mut self,
        request: &SignalBranchForkRequest,
        snapshot: Option<&SignalSnapshotV1>,
    ) -> Result<ResolvedForkRequest<D, I, T>, SignalBranchForkDenial> {
        match request.basis() {
            SignalBranchForkRequestBasis::CurrentBranchHead => {
                let parent_branch = self.graph.current_branch();
                Ok(ResolvedForkRequest {
                    parent_basis: self.current_branch_basis_artifact(),
                    created_branch_head_snapshot_id: parent_branch.head_snapshot_id,
                    requested_snapshot_basis: None,
                    source_branch_state: self.capture_heavy_branch_state(),
                    parent_branch,
                })
            }
            SignalBranchForkRequestBasis::ParentBranchHead { parent_branch_id } => {
                let parent_branch = self.branch_handle(*parent_branch_id).ok_or(
                    SignalBranchForkDenial::UnknownParentBranch {
                        parent_branch_id: *parent_branch_id,
                    },
                )?;
                let parent_basis =
                    expect_branch_basis(self.branch_basis_artifact(parent_branch.clone()));
                let source_branch_state =
                    self.materialize_parent_head_fork_state(parent_branch.clone())?;
                Ok(ResolvedForkRequest {
                    created_branch_head_snapshot_id: parent_branch.head_snapshot_id,
                    parent_basis,
                    requested_snapshot_basis: None,
                    source_branch_state,
                    parent_branch,
                })
            }
            SignalBranchForkRequestBasis::ParentBranchSnapshot {
                parent_branch_id,
                snapshot_id,
            } => {
                let snapshot = snapshot.ok_or_else(|| {
                    SignalBranchForkDenial::SnapshotPayloadRequiredForFork {
                        request: request.clone(),
                    }
                })?;
                let parent_branch = self.branch_handle(*parent_branch_id).ok_or(
                    SignalBranchForkDenial::UnknownParentBranch {
                        parent_branch_id: *parent_branch_id,
                    },
                )?;
                if snapshot.meta.snapshot_id != *snapshot_id {
                    return Err(SignalBranchForkDenial::SnapshotBasisMismatch {
                        requested_snapshot_id: *snapshot_id,
                        provided_snapshot_id: snapshot.meta.snapshot_id,
                    });
                }
                if snapshot.meta.branch_id != parent_branch.id {
                    return Err(SignalBranchForkDenial::IncompatibleForkSnapshotLineage {
                        parent_branch_id: parent_branch.id,
                        snapshot_branch_id: snapshot.meta.branch_id,
                        snapshot_id: snapshot.meta.snapshot_id,
                    });
                }
                if self
                    .branches
                    .snapshot_state(snapshot.meta.branch_id, snapshot.meta.snapshot_id)
                    .is_none()
                {
                    return Err(SignalBranchForkDenial::UnknownForkSnapshot {
                        parent_branch_id: parent_branch.id,
                        snapshot_id: snapshot.meta.snapshot_id,
                    });
                }
                let parent_basis =
                    expect_branch_basis(self.branch_basis_artifact(parent_branch.clone()));
                let requested_snapshot_basis = expect_branch_basis(
                    self.snapshot_branch_basis_artifact(parent_branch.clone(), snapshot),
                );
                let source_branch_state =
                    materialize_snapshot_fork_state(self, parent_branch.clone(), snapshot)?;
                Ok(ResolvedForkRequest {
                    created_branch_head_snapshot_id: Some(snapshot.meta.snapshot_id),
                    parent_basis,
                    requested_snapshot_basis: Some(requested_snapshot_basis),
                    source_branch_state,
                    parent_branch,
                })
            }
        }
    }

    fn materialize_parent_head_fork_state(
        &mut self,
        parent_branch: SignalBranchHandle,
    ) -> Result<BranchState<D, I, T>, SignalBranchForkDenial> {
        if parent_branch.id == self.graph.current_branch().id {
            return Ok(self.capture_heavy_branch_state());
        }
        self.branches.branch_state(parent_branch.id).cloned().ok_or(
            SignalBranchForkDenial::UnknownParentBranch {
                parent_branch_id: parent_branch.id,
            },
        )
    }
}

fn expect_branch_basis(
    outcome: TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial>,
) -> SignalBranchBasisArtifact {
    match outcome {
        TransitionOutcome::Success(basis) => basis,
        other => panic!("validated branch fork basis must succeed, got {other:?}"),
    }
}
