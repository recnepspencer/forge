#![cfg(test)]

use forge_core::errors::PersistentResolutionIncompatibility;
use forge_core::KernelError;

use crate::core::ResolutionCandidate;
use crate::core::ResolutionResult;

use super::super::schema::{MergePlan, MergeRegionSelection, PersistentFaceRef};

use super::plan::build_merge_plan;
use super::resolve::{
    map_resolution_incompatibility, resolve_face_ref_direct, resolve_face_ref_result,
    FaceResolutionFallbackPipeline,
};
use super::validate::validate_connectivity;

pub(super) struct NmtEvalTestApi;

impl NmtEvalTestApi {
    pub fn resolve_face_ref_direct(
        arena: &forge_topo::b_rep::TopologyArena,
        pref: &PersistentFaceRef,
    ) -> ResolutionResult<ResolutionCandidate> {
        resolve_face_ref_direct(arena, pref)
    }

    pub fn resolve_face_ref_with_lineage_fallback(
        topo: &forge_topo::transactions::TopologyState,
        pref: &PersistentFaceRef,
    ) -> ResolutionResult<ResolutionCandidate> {
        resolve_face_ref_result(
            topo,
            pref,
            FaceResolutionFallbackPipeline::DirectThenLineageThenHybrid,
        )
    }

    pub fn map_resolution_incompatibility(
        inc: &crate::core::ResolutionIncompatibility,
    ) -> PersistentResolutionIncompatibility {
        map_resolution_incompatibility(inc)
    }

    pub fn build_merge_plan(
        arena: &forge_topo::b_rep::TopologyArena,
        selection: &MergeRegionSelection,
    ) -> Result<MergePlan, KernelError> {
        build_merge_plan(arena, selection)
    }

    pub fn validate_connectivity(
        arena: &forge_topo::b_rep::TopologyArena,
        selection: &MergeRegionSelection,
    ) -> Result<(), KernelError> {
        validate_connectivity(arena, selection)
    }
}
