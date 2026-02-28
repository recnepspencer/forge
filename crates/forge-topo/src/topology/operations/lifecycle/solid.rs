//! MakeSolid — creates a new root solid hierarchy.
//!
//! DOMAIN: Creates a new Body, Lump, and Region.
//!
//! INVARIANTS:
//! - ΔSo=+1, ΔLp=+1, ΔR=+1
//! - Body owns Lump, Lump owns Region.

use forge_core::KernelError;

use crate::arena::{BodyData, LumpData, RegionData};
use crate::handles::{BodyId, LumpId, RegionId};
use crate::lineage::{Lineage, OpSignature};
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

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let body_lineage = Lineage::root(0, sig.clone());
        let lump_lineage = Lineage::root(1, sig.clone());
        let region_lineage = Lineage::root(2, sig.clone());

        let body = draft.insert_body(BodyData::with_lineage(Some(body_lineage)));
        let lump = draft.insert_lump(LumpData::with_lineage(body, Some(lump_lineage)));
        let region = draft.insert_region(RegionData::with_lineage(lump, Some(region_lineage)));

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

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_solid")
    }
}
