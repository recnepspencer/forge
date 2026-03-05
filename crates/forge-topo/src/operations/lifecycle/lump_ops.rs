//! Lump lifecycle operations — Rehome, Extract, Insert, Split, Merge.
//!
//! DOMAIN: Reparenting and structural operations on Lump entities.
//!
//! DEPENDENCIES: `arena` (entity storage)

use forge_core::KernelError;

use crate::b_rep::{LumpData, RegionData};
use crate::handles::{BodyId, LumpId, RegionId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;

// ── RehomeLump ──────────────────────────────────────────────────────

/// Move a lump from its current body to a different body.
#[derive(Debug)]
pub struct RehomeLump {
    /// The lump to rehome.
    pub lump: LumpId,
    /// The target body.
    pub target_body: BodyId,
}

impl TopoOperator for RehomeLump {
    type Output = ();

    const NAME: &'static str = "rehome_lump";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Move lump {} to body {}", self.lump.index(), self.target_body.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let old_body = draft.arena().get_lump(self.lump)?.body();

        if old_body == self.target_body {
            return Err(KernelError::InvalidInput {
                message: "RehomeLump: lump already belongs to the target body".to_string(),
                context: None,
            });
        }

        draft.arena_mut().get_body_mut(old_body)?.remove_lump(self.lump);
        draft.arena_mut().get_lump_mut(self.lump)?.set_body(self.target_body);
        draft.arena_mut().get_body_mut(self.target_body)?.add_lump(self.lump);

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta::default(),
        })
    }
}

// ── ExtractLump ─────────────────────────────────────────────────────

/// Extract a lump from its parent body (orphan it into a new body).
#[derive(Debug)]
pub struct ExtractLump {
    /// The lump to extract.
    pub lump: LumpId,
}

/// Output of the ExtractLump operator.
pub struct ExtractLumpOutput {
    /// The new body that now owns the extracted lump.
    pub new_body: BodyId,
}

impl TopoOperator for ExtractLump {
    type Output = ExtractLumpOutput;

    const NAME: &'static str = "extract_lump";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Extract lump {} into its own body", self.lump.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let old_body = draft.arena().get_lump(self.lump)?.body();

        let remaining = draft.arena().get_body(old_body)?.lumps().len();
        if remaining <= 1 {
            return Err(KernelError::InvalidInput {
                message: "ExtractLump: cannot extract the last lump from a body".to_string(),
                context: None,
            });
        }

        let new_body = draft.insert_body(crate::b_rep::BodyData::new());
        draft.arena_mut().get_body_mut(old_body)?.remove_lump(self.lump);
        draft.arena_mut().get_lump_mut(self.lump)?.set_body(new_body);
        draft.arena_mut().get_body_mut(new_body)?.add_lump(self.lump);

        Ok(ExecutionResult {
            value: ExtractLumpOutput { new_body },
            declared_delta: EulerDelta {
                solids: 1,
                ..EulerDelta::default()
            },
        })
    }
}

// ── InsertLump ──────────────────────────────────────────────────────

/// Insert a lump into a body (from another body).
///
/// Same as RehomeLump — provided as an alias.

// ── SplitLump ───────────────────────────────────────────────────────

/// Split a lump by moving a subset of its regions into a new lump.
#[derive(Debug)]
pub struct SplitLump {
    /// The lump to split.
    pub lump: LumpId,
    /// The regions to move to the new lump.
    pub regions_to_move: Vec<RegionId>,
}

/// Output of the SplitLump operator.
pub struct SplitLumpOutput {
    /// The new lump containing the moved regions.
    pub new_lump: LumpId,
}

impl TopoOperator for SplitLump {
    type Output = SplitLumpOutput;

    const NAME: &'static str = "split_lump";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!(
            "Split lump {} by moving {} regions to new lump",
            self.lump.index(), self.regions_to_move.len()
        )
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        if self.regions_to_move.is_empty() {
            return Err(KernelError::InvalidInput {
                message: "SplitLump: must move at least one region".to_string(),
                context: None,
            });
        }

        let existing_regions: Vec<RegionId> = draft.arena().get_lump(self.lump)?.regions().to_vec();
        if self.regions_to_move.len() >= existing_regions.len() {
            return Err(KernelError::InvalidInput {
                message: "SplitLump: cannot move all regions — original lump would be empty".to_string(),
                context: None,
            });
        }

        let body = draft.arena().get_lump(self.lump)?.body();
        let new_lump = draft.insert_lump(LumpData::new(body));
        draft.arena_mut().get_body_mut(body)?.add_lump(new_lump);

        for &region in &self.regions_to_move {
            draft.arena_mut().get_lump_mut(self.lump)?.remove_region(region);
            draft.arena_mut().get_region_mut(region)?.set_lump(new_lump);
            draft.arena_mut().get_lump_mut(new_lump)?.add_region(region);
        }

        Ok(ExecutionResult {
            value: SplitLumpOutput { new_lump },
            declared_delta: EulerDelta {
                lumps: 1,
                ..EulerDelta::default()
            },
        })
    }
}

// ── MergeLumps ──────────────────────────────────────────────────────

/// Merge all regions from source lump into target lump, then destroy source.
#[derive(Debug)]
pub struct MergeLumps {
    /// The target lump (survives).
    pub target: LumpId,
    /// The source lump (destroyed after merge).
    pub source: LumpId,
}

impl TopoOperator for MergeLumps {
    type Output = ();

    const NAME: &'static str = "merge_lumps";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::CONTAINER_LIFECYCLE;

    fn semantic_summary(&self) -> String {
        format!("Merge lump {} into lump {}", self.source.index(), self.target.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        if self.target == self.source {
            return Err(KernelError::InvalidInput {
                message: "MergeLumps: cannot merge a lump with itself".to_string(),
                context: None,
            });
        }

        let source_body = draft.arena().get_lump(self.source)?.body();
        let source_regions: Vec<RegionId> = draft.arena().get_lump(self.source)?.regions().to_vec();

        for &region in &source_regions {
            draft.arena_mut().get_lump_mut(self.source)?.remove_region(region);
            draft.arena_mut().get_region_mut(region)?.set_lump(self.target);
            draft.arena_mut().get_lump_mut(self.target)?.add_region(region);
        }

        draft.arena_mut().get_body_mut(source_body)?.remove_lump(self.source);
        draft.remove_lump(self.source)?;

        Ok(ExecutionResult {
            value: (),
            declared_delta: EulerDelta {
                lumps: -1,
                ..EulerDelta::default()
            },
        })
    }
}
