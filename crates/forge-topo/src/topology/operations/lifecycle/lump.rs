//! MakeLumpRegion — creates a new Lump and Region inside an existing Body.
//!
//! DOMAIN: Extends an existing Body by adding a Lump containing a Region.

use forge_core::KernelError;

use crate::arena::{LumpData, RegionData};
use crate::handles::{BodyId, LumpId, RegionId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

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

impl EulerOperator for MakeLumpRegion {
    type Output = MakeLumpRegionOutput;

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let lump_lineage = Lineage::root(0, sig.clone());
        let region_lineage = Lineage::root(1, sig.clone());

        let lump = draft.insert_lump(LumpData::with_lineage(self.body, Some(lump_lineage)));
        let region = draft.insert_region(RegionData::with_lineage(lump, Some(region_lineage)));

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

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_lump_region")
    }
}
