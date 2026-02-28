//! MakeSolid / DestroyBody — creates and destroys root solid hierarchies.
//!
//! DOMAIN: Creates/destroys a Body, Lump, and Region.
//!
//! INVARIANTS:
//! - MakeSolid: ΔSo=+1, ΔLp=+1, ΔR=+1
//! - DestroyBody: ΔSo=-1, ΔLp=-1, ΔR=-1
//! - Body owns Lump, Lump owns Region.

use forge_core::KernelError;

use crate::arena::{BodyData, LumpData, RegionData};
use crate::handles::{BodyId, LumpId, RegionId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;


/// Creates a new, empty solid hierarchy.
#[derive(Debug)]
pub struct MakeSolid;

/// Output of the MakeSolid operator.
pub struct MakeSolidOutput {
    /// The created body.
    pub body: BodyId,
    /// The created lump.
    pub lump: LumpId,
    /// The created region.
    pub region: RegionId,
}

impl EulerOperator for MakeSolid {
    type Output = MakeSolidOutput;

    const NAME: &'static str = "make_solid";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let body = draft.insert_body(BodyData::new());
        let lump = draft.insert_lump(LumpData::new(body));
        let region = draft.insert_region(RegionData::new(lump));

        draft.arena_mut().get_body_mut(body)?.add_lump(lump);
        draft.arena_mut().get_lump_mut(lump)?.add_region(region);

        Ok(ExecutionResult {
            value: MakeSolidOutput {
                body,
                lump,
                region,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: 0,
                shells: 0,
                solids: 1,
                lumps: 1,
                regions: 1,
            },
        })
    }
}

/// Destroys an empty Body, its single Lump, and its single Region.
///
/// The body must contain exactly one lump with exactly one region,
/// and the region must have no shells.
#[derive(Debug)]
pub struct DestroyBody {
    /// The body to destroy.
    pub body: BodyId,
}

impl EulerOperator for DestroyBody {
    type Output = ();

    const NAME: &'static str = "destroy_body";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let lumps: Vec<LumpId> = draft.arena().get_body(self.body)?.lumps().to_vec();
        if lumps.len() != 1 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "DestroyBody: body must have exactly 1 lump, has {}",
                    lumps.len()
                ),
                context: None,
            });
        }
        let lump = lumps[0];
        let regions: Vec<RegionId> = draft.arena().get_lump(lump)?.regions().to_vec();
        if regions.len() != 1 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "DestroyBody: lump must have exactly 1 region, has {}",
                    regions.len()
                ),
                context: None,
            });
        }
        let region = regions[0];
        if draft.arena().get_region(region)?.shell_count() > 0 {
            return Err(KernelError::InvalidInput {
                message: "DestroyBody: region must have no shells".to_string(),
                context: None,
            });
        }

        draft.remove_region(region)?;
        draft.remove_lump(lump)?;
        draft.remove_body(self.body)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: 0,
                shells: 0,
                solids: -1,
                lumps: -1,
                regions: -1,
            },
        })
    }
}

/// Alias for MakeSolid (operators-list.md §B1 name).
pub type CreateBody = MakeSolid;
