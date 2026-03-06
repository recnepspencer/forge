//! Shell lifecycle operations — Rehome, Extract, Insert, Split, Merge, Promote, Demote.
//!
//! DOMAIN: Reparenting and structural operations on Shell entities.
//!
//! DEPENDENCIES: `arena` (entity storage)

use forge_core::KernelError;

use crate::handles::FaceId;

use crate::b_rep::ShellKind;
use crate::handles::{RegionId, ShellId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;

// ── RehomeShell ─────────────────────────────────────────────────────

/// Move a shell from its current region to a different region.
#[derive(Debug)]
pub struct RehomeShell {
    /// The shell to rehome.
    pub shell: ShellId,
    /// The target region.
    pub target_region: RegionId,
}

impl TopoOperator for RehomeShell {
    type Output = ();

    const NAME: &'static str = "rehome_shell";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Move shell {} to region {}", self.shell.index(), self.target_region.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let old_region = draft.arena().get_shell(self.shell)?.region();

        if old_region == self.target_region {
            return Err(KernelError::InvalidInput {
                message: "RehomeShell: shell already belongs to the target region".to_string(),
                context: None,
            });
        }

        draft.arena_mut().get_region_mut(old_region)?.remove_shell(self.shell);
        draft.arena_mut().get_shell_mut(self.shell)?.set_region(self.target_region);
        draft.arena_mut().get_region_mut(self.target_region)?.add_shell(self.shell);

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta::default(),
        })
    }
}

// ── ExtractShell ────────────────────────────────────────────────────

/// Extract a shell from its parent region into a new region+lump structure.
#[derive(Debug)]
pub struct ExtractShell {
    /// The shell to extract.
    pub shell: ShellId,
}

/// Output of the ExtractShell operator.
pub struct ExtractShellOutput {
    /// The new region now owning the extracted shell.
    pub new_region: RegionId,
}

impl TopoOperator for ExtractShell {
    type Output = ExtractShellOutput;

    const NAME: &'static str = "extract_shell";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Extract shell {} into its own region", self.shell.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let old_region = draft.arena().get_shell(self.shell)?.region();
        let lump = draft.arena().get_region(old_region)?.lump();

        // Guard: reject extracting the outer shell. The outer boundary defines the region.
        let is_outer = draft.arena().get_region(old_region)?.outer_shell() == Some(self.shell);
        if is_outer {
            return Err(KernelError::InvalidInput {
                message: "ExtractShell: cannot extract outer shell; outer boundary defines the region".to_string(),
                context: None,
            });
        }

        let new_region = draft.insert_region(crate::b_rep::RegionData::new(lump));
        draft.arena_mut().get_lump_mut(lump)?.add_region(new_region);

        draft.arena_mut().get_region_mut(old_region)?.remove_shell(self.shell);
        draft.arena_mut().get_shell_mut(self.shell)?.set_region(new_region);
        draft.arena_mut().get_region_mut(new_region)?.add_shell(self.shell);

        Ok(ExecutionResult {
            value: ExtractShellOutput { new_region },
            declared_delta: EulerDelta {
                regions: 1,
                ..EulerDelta::default()
            },
        })
    }
}

// ── InsertShell ─────────────────────────────────────────────────────

/// Insert a shell into a region (same as RehomeShell).

// ── SplitShell ──────────────────────────────────────────────────────

/// Split a shell by moving a subset of its faces into a new shell.
///
/// The faces to move must form a connected component — but this
/// operator does not validate connectivity (that's a geometric concern).
/// It only performs the topological reparenting.
#[derive(Debug)]
pub struct SplitShell {
    /// The shell to split.
    pub shell: ShellId,
    /// The faces to move to the new shell.
    pub faces_to_move: Vec<crate::handles::FaceId>,
}

/// Output of the SplitShell operator.
pub struct SplitShellOutput {
    /// The new shell containing the moved faces.
    pub new_shell: ShellId,
}

impl TopoOperator for SplitShell {
    type Output = SplitShellOutput;

    const NAME: &'static str = "split_shell";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!(
            "Split shell {} by moving {} faces to new shell",
            self.shell.index(), self.faces_to_move.len()
        )
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        if self.faces_to_move.is_empty() {
            return Err(KernelError::InvalidInput {
                message: "SplitShell: must move at least one face".to_string(),
                context: None,
            });
        }

        let region = draft.arena().get_shell(self.shell)?.region();
        let kind = draft.arena().get_shell(self.shell)?.kind();

        let new_shell = draft.insert_shell(crate::b_rep::ShellData::new(
            self.faces_to_move[0],
            kind,
            region,
        ));
        draft.arena_mut().get_region_mut(region)?.add_shell(new_shell);

        for &face in &self.faces_to_move {
            let face_shell = draft.arena().get_face(face)?.shell();
            if face_shell != self.shell {
                return Err(KernelError::InvalidInput {
                    message: format!(
                        "SplitShell: face {} does not belong to shell {}",
                        face.index(), self.shell.index()
                    ),
                    context: None,
                });
            }
            draft.arena_mut().reassign_face_shell(face, new_shell)?;
        }

        // Fix: update source shell's representative_face if it was moved,
        // and garbage collect the shell if it is now completely empty.
        let mut delta_shells = 1;
        let current_rep = draft.arena().get_shell(self.shell)?.representative_face();
        if self.faces_to_move.contains(&current_rep) {
            let remaining = draft.arena().faces_of_shell(self.shell).to_vec();
            if remaining.is_empty() {
                draft.arena_mut().get_region_mut(region)?.remove_shell(self.shell);
                draft.remove_shell(self.shell)?;
                delta_shells = 0;
            } else {
                draft.arena_mut().get_shell_mut(self.shell)?.set_representative_face(remaining[0]);
            }
        }

        Ok(ExecutionResult {
            value: SplitShellOutput { new_shell },
            declared_delta: EulerDelta {
                shells: delta_shells,
                ..EulerDelta::default()
            },
        })
    }
}

// ── MergeShells ─────────────────────────────────────────────────────

/// Merge all faces from source shell into target shell, then destroy source.
#[derive(Debug)]
pub struct MergeShells {
    /// The target shell (survives).
    pub target: ShellId,
    /// The source shell (destroyed after merge).
    pub source: ShellId,
}

impl TopoOperator for MergeShells {
    type Output = ();

    const NAME: &'static str = "merge_shells";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Merge shell {} into shell {}", self.source.index(), self.target.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        if self.target == self.source {
            return Err(KernelError::InvalidInput {
                message: "MergeShells: cannot merge a shell with itself".to_string(),
                context: None,
            });
        }

        let source_region = draft.arena().get_shell(self.source)?.region();
        let target_region = draft.arena().get_shell(self.target)?.region();
        if source_region != target_region {
            return Err(KernelError::InvalidInput {
                message: "MergeShells: source and target must belong to the same region".to_string(),
                context: None,
            });
        }

        let faces_to_move: Vec<crate::handles::FaceId> =
            draft.arena().faces_of_shell(self.source).to_vec();

        for &face in &faces_to_move {
            draft.arena_mut().reassign_face_shell(face, self.target)?;
        }

        draft.arena_mut().get_region_mut(source_region)?.remove_shell(self.source);
        draft.remove_shell(self.source)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                shells: -1,
                ..EulerDelta::default()
            },
        })
    }
}

// ── PromoteShell ────────────────────────────────────────────────────

/// Promote an inner shell to outer shell of its region.
///
/// Swaps the shell into the region's outer_shell position.
/// The current outer shell (if any) becomes an inner shell.
#[derive(Debug)]
pub struct PromoteShell {
    /// The inner shell to promote.
    pub shell: ShellId,
}

impl TopoOperator for PromoteShell {
    type Output = ();

    const NAME: &'static str = "promote_shell";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Promote inner shell {} to outer shell", self.shell.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let region = draft.arena().get_shell(self.shell)?.region();

        let inner_shells: Vec<ShellId> = draft.arena().get_region(region)?.inner_shells().to_vec();
        if !inner_shells.contains(&self.shell) {
            return Err(KernelError::InvalidInput {
                message: "PromoteShell: shell is not an inner shell of its region".to_string(),
                context: None,
            });
        }

        let old_outer = draft.arena().get_region(region)?.outer_shell();

        // Remove promoted shell from inner list
        draft.arena_mut().get_region_mut(region)?.remove_shell(self.shell);

        // Demote old outer to inner (if it existed)
        if let Some(old) = old_outer {
            draft.arena_mut().get_region_mut(region)?.add_inner_shell(old);
        }

        // Set the promoted shell as outer
        draft.arena_mut().get_region_mut(region)?.set_outer_shell(self.shell);

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta::default(),
        })
    }
}

// ── DemoteShell ─────────────────────────────────────────────────────

/// Demote the outer shell of a region to an inner shell.
///
/// The region will have no outer shell after this operation.
#[derive(Debug)]
pub struct DemoteShell {
    /// The region whose outer shell to demote.
    pub region: RegionId,
}

impl TopoOperator for DemoteShell {
    type Output = ();

    const NAME: &'static str = "demote_shell";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Demote outer shell of region {} to inner shell", self.region.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let outer = draft.arena().get_region(self.region)?.outer_shell();

        match outer {
            None => Err(KernelError::InvalidInput {
                message: "DemoteShell: region has no outer shell to demote".to_string(),
                context: None,
            }),
            Some(shell) => {
                // remove_shell already clears outer to None
                draft.arena_mut().get_region_mut(self.region)?.remove_shell(shell);
                draft.arena_mut().get_region_mut(self.region)?.add_inner_shell(shell);

                Ok(ExecutionResult {
                    value: (),
                    declared_delta: EulerDelta::default(),
                })
            }
        }
    }
}
