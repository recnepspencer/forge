//! MakeEmptyShell — creates an empty shell in a region.
//!
//! DOMAIN: Extents an existing Region by adding a new Shell with no geometry.

use forge_core::KernelError;

use crate::arena::{ShellData, ShellKind};
use crate::handles::{FaceId, RegionId, ShellId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

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

impl EulerOperator for MakeEmptyShell {
    type Output = MakeEmptyShellOutput;

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let shell_lineage = Lineage::root(0, sig.clone());

        let shell = draft.insert_shell(ShellData::with_lineage(
            FaceId::new(u32::MAX, 0),
            self.kind,
            self.region,
            Some(shell_lineage),
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

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_empty_shell")
    }
}
