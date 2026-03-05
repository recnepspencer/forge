//! MakeLumpRegion / DestroyLump — creates and destroys Lumps inside Bodies.
//!
//! DOMAIN: Extends/shrinks an existing Body by adding/removing a Lump+Region.

use forge_core::KernelError;

use crate::b_rep::{LumpData, RegionData};
use crate::handles::{BodyId, LumpId, RegionId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::contract_registry;
use crate::validators::invariant_id::InvariantContract;


/// Creates a new Lump and Region inside an existing Body.
#[derive(Debug)]
pub struct MakeLumpRegion {
    /// The body to attach the new lump to.
    pub body: BodyId,
}

/// Output of the MakeLumpRegion operator.
pub struct MakeLumpRegionOutput {
    /// The created lump.
    pub lump: LumpId,
    /// The created region.
    pub region: RegionId,
}

impl TopoOperator for MakeLumpRegion {
    type Output = MakeLumpRegionOutput;

    const NAME: &'static str = "make_lump_region";

    const INVARIANT_CONTRACT: InvariantContract = contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Create lump with region in body {}", self.body.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let lump = draft.insert_lump(LumpData::new(self.body));
        let region = draft.insert_region(RegionData::new(lump));

        draft.arena_mut().get_body_mut(self.body)?.add_lump(lump);
        draft.arena_mut().get_lump_mut(lump)?.add_region(region);

        Ok(ExecutionResult {
            value: MakeLumpRegionOutput { lump, region },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: 0,
                shells: 0,
                solids: 0,
                lumps: 1,
                regions: 1,
            },
        })
    }
}

/// Destroys an empty Lump and its single Region, removing it from its parent Body.
///
/// The lump must have exactly one region with no shells.
#[derive(Debug)]
pub struct DestroyLump {
    /// The lump to destroy.
    pub lump: LumpId,
}

impl TopoOperator for DestroyLump {
    type Output = ();

    const NAME: &'static str = "destroy_lump";

    const INVARIANT_CONTRACT: InvariantContract = contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Destroy lump {} and its region", self.lump.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let body = draft.arena().get_lump(self.lump)?.body();
        let regions: Vec<RegionId> = draft.arena().get_lump(self.lump)?.regions().to_vec();

        if regions.len() != 1 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "DestroyLump: lump must have exactly 1 region, has {}",
                    regions.len()
                ),
                context: None,
            });
        }
        let region = regions[0];
        if draft.arena().get_region(region)?.shell_count() > 0 {
            return Err(KernelError::InvalidInput {
                message: "DestroyLump: region must have no shells".to_string(),
                context: None,
            });
        }

        draft.arena_mut().get_body_mut(body)?.remove_lump(self.lump);
        draft.remove_region(region)?;
        draft.remove_lump(self.lump)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: 0,
                loops: 0,
                edges: 0,
                shells: 0,
                solids: 0,
                lumps: -1,
                regions: -1,
            },
        })
    }
}
