//! Solid containment hierarchy construction.
//!
//! DOMAIN: Creates the Body → Lump → Region → Shell containment
//! hierarchy that every closed solid requires. This is the standard
//! structure setup consumed by all solid-building operations.
//!
//! CONSUMED BY: primitives, booleans, Euler operators.

use forge_core::KernelError;
use forge_topo::b_rep::{
    BodyData, LumpData, RegionData, ShellData, ShellKind, ShellOrientation,
};
use forge_topo::handles::{FaceId, ShellId};
use forge_topo::provenance::LineageRecorder;
use forge_topo::transactions::MutableDraft;

/// Result of creating a solid containment hierarchy.
pub struct SolidHierarchy {
    /// The shell to which faces should be added.
    pub shell: ShellId,
}

/// Create a Body → Lump → Region → Shell hierarchy for a closed outer solid.
///
/// Allocates all four containment entities and wires parent-child links.
/// Returns the `ShellId` for face insertion.
///
/// The shell's representative face must be set after faces are created.
pub fn make_solid_hierarchy(
    draft: &mut MutableDraft,
    recorder: &mut LineageRecorder,
) -> Result<SolidHierarchy, KernelError> {
    let body = draft.insert_body(BodyData::new());
    recorder.stamp(draft.lineage_store_mut(), body);
    let lump = draft.insert_lump(LumpData::new(body));
    recorder.stamp(draft.lineage_store_mut(), lump);
    let region = draft.insert_region(RegionData::new(lump));
    recorder.stamp(draft.lineage_store_mut(), region);
    draft.arena_mut().get_body_mut(body)?.add_lump(lump);
    draft.arena_mut().get_lump_mut(lump)?.add_region(region);

    let shell = draft.insert_shell(ShellData::new(
        FaceId::new(u32::MAX, 0),
        ShellKind::Solid(ShellOrientation::Outer),
        region,
    ));
    recorder.stamp(draft.lineage_store_mut(), shell);
    draft.arena_mut().get_region_mut(region)?.add_shell(shell);

    Ok(SolidHierarchy { shell })
}

