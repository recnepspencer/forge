use std::rc::Rc;

use crate::runtime::{
    WorthUiPlanChildRange, WorthUiPlanExecutionLane, WorthUiPlanNode, WorthUiPlanNodeFamily,
    WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily, WorthUiPlanRegionStructure,
    WorthUiRenderResourceRef, WorthUiRuntimeHandle,
};

/// Immutable execution facts lowered once for one regional slot generation.
///
/// These facts deliberately omit candidate-wide iteration layout. A stable slot
/// can therefore share this record with a successor without rebuilding flat
/// topology, lane partitions, or authored-identity lookup vectors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPlanRegionExecutable {
    family: WorthUiPlanNodeInputFamily,
    lane: WorthUiPlanExecutionLane,
    region_structure: Option<WorthUiPlanRegionStructure>,
    has_render_resource: bool,
    child_range_count: Option<u32>,
    linked_child_range: Option<super::WorthUiPlanRegionHandle>,
    child_targets: Rc<[super::WorthUiPlanRegionHandle]>,
    owned_region_identities: Rc<[super::WorthUiPlanRegionIdentity]>,
    ordinary_meaning: Option<Rc<crate::runtime::execution_plan_input::WorthUiPlanOrdinaryMeaning>>,
    spatial_meaning: Option<Rc<crate::runtime::execution_plan_input::WorthUiSpatialPlanMeaning>>,
    realtime_meaning: Option<Rc<crate::runtime::execution_plan_input::WorthUiRealtimePlanMeaning>>,
    query_binding_identity: Option<Rc<crate::runtime::WorthUiQueryBindingIdentity>>,
    query_installed_reference:
        Option<Rc<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>>,
    root_shell: bool,
}

impl WorthUiPlanRegionExecutable {
    pub(super) fn lower(
        input: &WorthUiPlanNodeInput,
        mut resolve: impl FnMut(&str) -> Option<super::WorthUiPlanRegionHandle>,
    ) -> Result<Self, super::WorthUiPlanRegionStoreDenial> {
        let family = input.family();
        if input
            .ordinary_meaning()
            .is_some_and(|meaning| meaning.family() != family)
        {
            return Err(super::WorthUiPlanRegionStoreDenial::OrdinaryMeaningFamilyMismatch);
        }
        let region_structure =
            WorthUiPlanRegionStructure::from_topology_input(input.topology_input());
        let linked_child_range = input
            .ordinary_meaning()
            .and_then(|meaning| meaning.child_range_identity())
            .map(|identity| {
                resolve(identity).ok_or(super::WorthUiPlanRegionStoreDenial::MissingLinkedRegion)
            })
            .transpose()?;
        let child_targets = resolve_child_targets(input, &mut resolve)?;
        let child_range_count = (!child_targets.is_empty())
            .then(|| {
                crate::runtime::handle_allocation::WorthUiHandleCapacity::child_range(
                    child_targets.len(),
                )
            })
            .transpose()?;
        let owned_region_identities = input
            .owned_region_identity_bases()
            .iter()
            .map(|identity| super::WorthUiPlanRegionIdentity::from_exact_basis(identity.clone()))
            .collect::<Vec<_>>()
            .into();
        let ordinary_meaning = input.ordinary_meaning_reference();
        let spatial_meaning = input.spatial_meaning_reference();
        if (family == WorthUiPlanNodeInputFamily::CanvasSpatial) != spatial_meaning.is_some() {
            return Err(super::WorthUiPlanRegionStoreDenial::SpatialMeaningFamilyMismatch);
        }
        let realtime_meaning = input.realtime_meaning_reference();
        if (family == WorthUiPlanNodeInputFamily::RealtimeOverlay) != realtime_meaning.is_some() {
            return Err(super::WorthUiPlanRegionStoreDenial::RealtimeMeaningFamilyMismatch);
        }
        let query_binding_identity = input.query_binding_identity_reference();
        let query_installed_reference = input.query_installed_reference_shared();
        let is_query_row = family == WorthUiPlanNodeInputFamily::QueryViewBinding;
        if is_query_row != (query_binding_identity.is_some() && query_installed_reference.is_some())
        {
            return Err(super::WorthUiPlanRegionStoreDenial::QueryBindingFactsMismatch);
        }
        let root_shell = input.owner_identity_basis().is_none()
            && matches!(
                family,
                WorthUiPlanNodeInputFamily::ComponentInvocation
                    | WorthUiPlanNodeInputFamily::LayoutRegion
            );
        Ok(Self {
            family,
            lane: lane_for_family(family),
            region_structure,
            has_render_resource: family == WorthUiPlanNodeInputFamily::RenderResourceRef,
            child_range_count,
            linked_child_range,
            child_targets,
            owned_region_identities,
            ordinary_meaning,
            spatial_meaning,
            realtime_meaning,
            query_binding_identity,
            query_installed_reference,
            root_shell,
        })
    }

    pub(crate) fn family(&self) -> WorthUiPlanNodeInputFamily {
        self.family
    }

    pub(crate) fn lane(&self) -> WorthUiPlanExecutionLane {
        self.lane
    }

    pub(crate) fn region_structure(&self) -> Option<WorthUiPlanRegionStructure> {
        self.region_structure
    }

    pub(crate) fn has_render_resource(&self) -> bool {
        self.has_render_resource
    }

    pub(crate) fn child_range_for_plan_index(
        &self,
        plan_index: u32,
    ) -> Option<WorthUiPlanChildRange> {
        self.child_range_count
            .filter(|count| *count > 0)
            .map(|count| WorthUiPlanChildRange::from_compact_row(plan_index, count))
    }

    pub(crate) fn linked_child_range(&self) -> Option<&super::WorthUiPlanRegionHandle> {
        self.linked_child_range.as_ref()
    }

    pub(crate) fn child_targets_rc(&self) -> Rc<[super::WorthUiPlanRegionHandle]> {
        Rc::clone(&self.child_targets)
    }

    pub(crate) fn owned_region_identities(&self) -> &[super::WorthUiPlanRegionIdentity] {
        &self.owned_region_identities
    }

    pub(crate) fn is_root_shell(&self) -> bool {
        self.root_shell
    }

    pub(crate) fn ordinary_semantic_digest(&self) -> u64 {
        self.ordinary_meaning.as_deref().map_or(
            0,
            crate::runtime::execution_plan_input::WorthUiPlanOrdinaryMeaning::semantic_digest,
        )
    }

    pub(crate) fn ordinary_meaning_reference(
        &self,
    ) -> Option<Rc<crate::runtime::execution_plan_input::WorthUiPlanOrdinaryMeaning>> {
        self.ordinary_meaning.as_ref().map(Rc::clone)
    }

    pub(crate) fn spatial_meaning_reference(
        &self,
    ) -> Option<Rc<crate::runtime::execution_plan_input::WorthUiSpatialPlanMeaning>> {
        self.spatial_meaning.as_ref().map(Rc::clone)
    }

    pub(crate) fn realtime_meaning_reference(
        &self,
    ) -> Option<Rc<crate::runtime::execution_plan_input::WorthUiRealtimePlanMeaning>> {
        self.realtime_meaning.as_ref().map(Rc::clone)
    }

    pub(crate) fn query_binding_identity_reference(
        &self,
    ) -> Option<Rc<crate::runtime::WorthUiQueryBindingIdentity>> {
        self.query_binding_identity.as_ref().map(Rc::clone)
    }

    pub(crate) fn query_installed_reference(
        &self,
    ) -> Option<Rc<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>> {
        self.query_installed_reference.as_ref().map(Rc::clone)
    }

    pub(super) fn semantic_digest(&self, seed: u64) -> u64 {
        let mut digest = fold(seed, self.family as u64);
        digest = fold(digest, self.lane as u64);
        digest = fold(digest, u64::from(self.has_render_resource));
        if let Some(structure) = self.region_structure {
            digest = fold(digest, structure.root_region_count() as u64);
            digest = fold(digest, structure.region_count() as u64);
            digest = fold(digest, structure.mount_count() as u64);
            digest = fold(digest, structure.max_region_depth() as u64);
        }
        digest = fold(digest, self.ordinary_semantic_digest());
        digest = fold(
            digest,
            self.spatial_meaning.as_deref().map_or(
                0,
                crate::runtime::execution_plan_input::WorthUiSpatialPlanMeaning::semantic_digest,
            ),
        );
        digest = fold(
            digest,
            self.realtime_meaning.as_deref().map_or(
                0,
                crate::runtime::execution_plan_input::WorthUiRealtimePlanMeaning::semantic_digest,
            ),
        );
        if let Some(reference) = &self.query_installed_reference {
            digest = fold(digest, reference.definition().digest().as_u64());
        }
        for target in self.child_targets.iter() {
            digest = fold(digest, target.stable_slot());
            digest = fold(digest, target.slot_generation());
        }
        digest
    }

    pub(crate) fn materialize_node(
        &self,
        runtime_handle: WorthUiRuntimeHandle,
        child_range: Option<WorthUiPlanChildRange>,
    ) -> WorthUiPlanNode {
        let render_resource_ref = self
            .has_render_resource
            .then(|| WorthUiRenderResourceRef::new(runtime_handle.locator()));
        WorthUiPlanNode::new(
            runtime_handle,
            WorthUiPlanNodeFamily::from_input_family(self.family),
            child_range,
            self.region_structure,
            render_resource_ref,
        )
    }
}

fn resolve_child_targets(
    input: &WorthUiPlanNodeInput,
    resolve: &mut impl FnMut(&str) -> Option<super::WorthUiPlanRegionHandle>,
) -> Result<Rc<[super::WorthUiPlanRegionHandle]>, super::WorthUiPlanRegionStoreDenial> {
    let Some(range) = input
        .ordinary_meaning()
        .and_then(|meaning| meaning.child_range())
    else {
        return Ok(Rc::from([]));
    };
    let mut seen = std::collections::BTreeSet::new();
    let mut targets = Vec::with_capacity(range.child_identities().len());
    for identity in range.child_identities() {
        if !seen.insert(identity) {
            return Err(super::WorthUiPlanRegionStoreDenial::DuplicateChildTarget);
        }
        targets.push(
            resolve(identity).ok_or(super::WorthUiPlanRegionStoreDenial::MissingLinkedRegion)?,
        );
    }
    Ok(targets.into())
}

fn lane_for_family(family: WorthUiPlanNodeInputFamily) -> WorthUiPlanExecutionLane {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation
        | WorthUiPlanNodeInputFamily::LayoutRegion
        | WorthUiPlanNodeInputFamily::ChildRange
        | WorthUiPlanNodeInputFamily::StateSlot => WorthUiPlanExecutionLane::UiStructure,
        WorthUiPlanNodeInputFamily::QueryViewBinding => WorthUiPlanExecutionLane::QueryView,
        WorthUiPlanNodeInputFamily::Command => WorthUiPlanExecutionLane::Command,
        WorthUiPlanNodeInputFamily::TokenStyle => WorthUiPlanExecutionLane::Style,
        WorthUiPlanNodeInputFamily::Accessibility | WorthUiPlanNodeInputFamily::DiagnosticsRef => {
            WorthUiPlanExecutionLane::Diagnostics
        }
        WorthUiPlanNodeInputFamily::LanePartitionRef => WorthUiPlanExecutionLane::LaneBoundary,
        WorthUiPlanNodeInputFamily::RenderResourceRef => WorthUiPlanExecutionLane::RenderResource,
        WorthUiPlanNodeInputFamily::CanvasSpatial => WorthUiPlanExecutionLane::CanvasSpatial,
        WorthUiPlanNodeInputFamily::RealtimeOverlay => WorthUiPlanExecutionLane::RealtimeOverlay,
    }
}

fn fold(digest: u64, value: u64) -> u64 {
    (digest ^ value).wrapping_mul(0x100000001b3)
}
