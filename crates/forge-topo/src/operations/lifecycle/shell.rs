//! MakeEmptyShell / DestroyShell — creates and destroys Shells inside Regions.
//!
//! DOMAIN: Extends/shrinks an existing Region by adding/removing a Shell.

use forge_core::KernelError;

use crate::b_rep::{ShellData, ShellKind};
use crate::handles::{FaceId, RegionId, ShellId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;


/// Creates an empty shell attached to a region.
#[derive(Debug)]
pub struct MakeEmptyShell {
    /// The region to attach this shell to.
    pub region: RegionId,
    /// The kind of shell.
    pub kind: ShellKind,
}

/// Output of the MakeEmptyShell operator.
pub struct MakeEmptyShellOutput {
    /// The created shell.
    pub shell: ShellId,
}

impl TopoOperator for MakeEmptyShell {
    type Output = MakeEmptyShellOutput;

    const NAME: &'static str = "make_empty_shell";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let shell = draft.insert_shell(ShellData::new(
            FaceId::new(u32::MAX, 0),
            self.kind,
            self.region,
        ));

        draft.arena_mut().get_region_mut(self.region)?.add_shell(shell);

        Ok(ExecutionResult {
            value: MakeEmptyShellOutput { shell },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: 0,
                shells: 1,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }
}

/// Destroys an empty Shell, removing it from its parent Region.
///
/// The shell must have no faces (representative_face must be sentinels).
#[derive(Debug)]
pub struct DestroyShell {
    /// The shell to destroy.
    pub shell: ShellId,
}

impl TopoOperator for DestroyShell {
    type Output = ();

    const NAME: &'static str = "destroy_shell";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let region = draft.arena().get_shell(self.shell)?.region();

        // Validate shell is empty (no faces reference it)
        let face_count = draft.arena().faces_of_shell(self.shell).len();

        if face_count > 0 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "DestroyShell: shell still has {} faces",
                    face_count
                ),
                context: None,
            });
        }

        draft.arena_mut().get_region_mut(region)?.remove_shell(self.shell);
        draft.remove_shell(self.shell)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: 0,
                shells: -1,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }
}
