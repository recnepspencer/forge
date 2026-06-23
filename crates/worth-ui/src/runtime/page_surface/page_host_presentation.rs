use std::collections::HashMap;

use crate::runtime::{
    WorthUiPageHostPlanDenial, WorthUiRuntimeAuthoringSnapshot, WorthUiRuntimeHost,
};
use crate::source::{
    WorthUiLayoutAxis, WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue,
    WorthUiLayoutTopologyChild, WorthUiLayoutTopologyNode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPageHostBoundaryPosture {
    Hidden,
    Merged,
    Emphasized,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPageHostResolvedSizing {
    Fit,
    Fill,
    Fixed(f32),
    Share(f32),
    Ratio {
        numerator: u32,
        denominator: u32,
    },
    Clamp {
        min: f32,
        preferred: Box<WorthUiPageHostResolvedSizing>,
        max: f32,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPageHostPresentation {
    page_name: String,
    root: WorthUiPageHostPresentationRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPageHostPresentationRegion {
    axis: WorthUiLayoutAxis,
    sizing: Option<WorthUiPageHostResolvedSizing>,
    gap: Option<f32>,
    padding: Option<f32>,
    scroll_owner: bool,
    resizable: bool,
    restorable: bool,
    children: Vec<WorthUiPageHostPresentationChild>,
    sibling_boundaries: Vec<WorthUiPageHostBoundaryPosture>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPageHostPresentationChild {
    Region(WorthUiPageHostPresentationRegion),
    Slot(WorthUiPageHostPresentationSlot),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostPresentationSlot {
    slot_name: String,
    surface_id: String,
    component_id: String,
}

impl WorthUiRuntimeHost {
    pub fn inspect_page_host_presentation(
        &self,
        page_name: &str,
    ) -> Result<WorthUiPageHostPresentation, WorthUiPageHostPlanDenial> {
        let authoring = self
            .active_authoring_snapshot()
            .ok_or(WorthUiPageHostPlanDenial::MissingAuthoringSnapshot)?;
        WorthUiPageHostPresentation::from_authoring(authoring, self, page_name)
    }
}

impl WorthUiPageHostPresentation {
    pub fn from_authoring(
        authoring: &WorthUiRuntimeAuthoringSnapshot,
        runtime: &WorthUiRuntimeHost,
        page_name: &str,
    ) -> Result<Self, WorthUiPageHostPlanDenial> {
        let layout = authoring
            .layout_topology()
            .page(page_name)
            .ok_or_else(|| WorthUiPageHostPlanDenial::MissingPage(page_name.to_owned()))?;
        let slots = authoring
            .content_slots()
            .page(page_name)
            .ok_or_else(|| WorthUiPageHostPlanDenial::MissingPage(page_name.to_owned()))?;
        let slot_map = slots
            .assignments()
            .iter()
            .map(|assignment| {
                (
                    assignment.slot_name().to_owned(),
                    assignment.surface_id().to_owned(),
                )
            })
            .collect::<HashMap<_, _>>();
        Ok(Self {
            page_name: page_name.to_owned(),
            root: build_region(layout.root(), &slot_map, runtime, authoring),
        })
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn root(&self) -> &WorthUiPageHostPresentationRegion {
        &self.root
    }
}

impl WorthUiPageHostPresentationRegion {
    pub fn axis(&self) -> &WorthUiLayoutAxis {
        &self.axis
    }

    pub fn sizing(&self) -> Option<&WorthUiPageHostResolvedSizing> {
        self.sizing.as_ref()
    }

    pub fn gap(&self) -> Option<f32> {
        self.gap
    }

    pub fn padding(&self) -> Option<f32> {
        self.padding
    }

    pub fn scroll_owner(&self) -> bool {
        self.scroll_owner
    }

    pub fn resizable(&self) -> bool {
        self.resizable
    }

    pub fn restorable(&self) -> bool {
        self.restorable
    }

    pub fn children(&self) -> &[WorthUiPageHostPresentationChild] {
        &self.children
    }

    pub fn sibling_boundaries(&self) -> &[WorthUiPageHostBoundaryPosture] {
        &self.sibling_boundaries
    }
}

impl WorthUiPageHostPresentationSlot {
    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }
}

impl WorthUiPageHostPresentationChild {
    pub fn as_region(&self) -> Option<&WorthUiPageHostPresentationRegion> {
        match self {
            Self::Region(region) => Some(region),
            Self::Slot(_) => None,
        }
    }

    pub fn as_slot(&self) -> Option<&WorthUiPageHostPresentationSlot> {
        match self {
            Self::Region(_) => None,
            Self::Slot(slot) => Some(slot),
        }
    }
}

fn build_region(
    node: &WorthUiLayoutTopologyNode,
    slot_map: &HashMap<String, String>,
    runtime: &WorthUiRuntimeHost,
    authoring: &WorthUiRuntimeAuthoringSnapshot,
) -> WorthUiPageHostPresentationRegion {
    let children = node
        .children()
        .iter()
        .map(|child| match child {
            WorthUiLayoutTopologyChild::Region(region) => WorthUiPageHostPresentationChild::Region(
                build_region(region, slot_map, runtime, authoring),
            ),
            WorthUiLayoutTopologyChild::Slot(slot) => WorthUiPageHostPresentationChild::Slot(
                build_slot(slot.slot_name(), slot_map, authoring),
            ),
        })
        .collect::<Vec<_>>();
    let sibling_boundaries = children
        .windows(2)
        .map(|window| classify_boundary(&window[0], &window[1]))
        .collect();
    WorthUiPageHostPresentationRegion {
        axis: node.axis().clone(),
        sizing: node
            .sizing()
            .map(|sizing| resolve_sizing(sizing, runtime))
            .or(None),
        gap: node.gap().map(|value| resolve_value(value, runtime)),
        padding: node.padding().map(|value| resolve_value(value, runtime)),
        scroll_owner: node.scroll_owner(),
        resizable: node.resizable(),
        restorable: node.restorable(),
        children,
        sibling_boundaries,
    }
}

fn build_slot(
    slot_name: &str,
    slot_map: &HashMap<String, String>,
    authoring: &WorthUiRuntimeAuthoringSnapshot,
) -> WorthUiPageHostPresentationSlot {
    let surface_id = slot_map
        .get(slot_name)
        .cloned()
        .unwrap_or_else(|| format!("unmapped.{slot_name}"));
    let component_id = authoring
        .authored_surfaces()
        .component_id_for_surface(&surface_id)
        .unwrap_or("worth.component.unknown")
        .to_owned();
    WorthUiPageHostPresentationSlot {
        slot_name: slot_name.to_owned(),
        surface_id,
        component_id,
    }
}

fn classify_boundary(
    left: &WorthUiPageHostPresentationChild,
    right: &WorthUiPageHostPresentationChild,
) -> WorthUiPageHostBoundaryPosture {
    match (left, right) {
        (WorthUiPageHostPresentationChild::Slot(_), _)
        | (_, WorthUiPageHostPresentationChild::Slot(_)) => WorthUiPageHostBoundaryPosture::Hidden,
        (
            WorthUiPageHostPresentationChild::Region(left),
            WorthUiPageHostPresentationChild::Region(right),
        ) => {
            if left.resizable() || right.resizable() || left.scroll_owner() || right.scroll_owner()
            {
                WorthUiPageHostBoundaryPosture::Emphasized
            } else {
                WorthUiPageHostBoundaryPosture::Merged
            }
        }
    }
}

fn resolve_sizing(
    sizing: &WorthUiLayoutSizingSpec,
    runtime: &WorthUiRuntimeHost,
) -> WorthUiPageHostResolvedSizing {
    match sizing {
        WorthUiLayoutSizingSpec::Fit => WorthUiPageHostResolvedSizing::Fit,
        WorthUiLayoutSizingSpec::Fill => WorthUiPageHostResolvedSizing::Fill,
        WorthUiLayoutSizingSpec::Fixed(value) => {
            WorthUiPageHostResolvedSizing::Fixed(resolve_value(value, runtime))
        }
        WorthUiLayoutSizingSpec::Share(value) => {
            WorthUiPageHostResolvedSizing::Share(*value as f32)
        }
        WorthUiLayoutSizingSpec::Ratio {
            numerator,
            denominator,
        } => WorthUiPageHostResolvedSizing::Ratio {
            numerator: *numerator,
            denominator: *denominator,
        },
        WorthUiLayoutSizingSpec::Clamp {
            min,
            preferred,
            max,
        } => WorthUiPageHostResolvedSizing::Clamp {
            min: resolve_value(min, runtime),
            preferred: Box::new(resolve_sizing(preferred, runtime)),
            max: resolve_value(max, runtime),
        },
    }
}

fn resolve_value(value: &WorthUiLayoutSizingValue, runtime: &WorthUiRuntimeHost) -> f32 {
    match value {
        WorthUiLayoutSizingValue::Number(value) => *value as f32,
        WorthUiLayoutSizingValue::NamedToken(token) => runtime
            .inspect_active_named_measurement_pixels(token)
            .unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_boundary, WorthUiPageHostBoundaryPosture};
    use crate::facade::WorthUiLayoutAxis;

    #[test]
    fn slot_boundaries_are_hidden() {
        assert_eq!(
            classify_boundary(&slot_child("left"), &slot_child("right")),
            WorthUiPageHostBoundaryPosture::Hidden
        );
    }

    #[test]
    fn passive_region_boundaries_are_merged() {
        assert_eq!(
            classify_boundary(&region_child(false, false), &region_child(false, false)),
            WorthUiPageHostBoundaryPosture::Merged
        );
    }

    #[test]
    fn scroll_or_resize_boundaries_are_emphasized() {
        assert_eq!(
            classify_boundary(&region_child(true, false), &region_child(false, false)),
            WorthUiPageHostBoundaryPosture::Emphasized
        );
        assert_eq!(
            classify_boundary(&region_child(false, false), &region_child(false, true)),
            WorthUiPageHostBoundaryPosture::Emphasized
        );
    }

    fn slot_child(slot_name: &str) -> super::WorthUiPageHostPresentationChild {
        super::WorthUiPageHostPresentationChild::Slot(super::WorthUiPageHostPresentationSlot {
            slot_name: slot_name.to_owned(),
            surface_id: format!("worth.surface.{slot_name}"),
            component_id: "worth.component.card".to_owned(),
        })
    }

    fn region_child(
        scroll_owner: bool,
        resizable: bool,
    ) -> super::WorthUiPageHostPresentationChild {
        super::WorthUiPageHostPresentationChild::Region(super::WorthUiPageHostPresentationRegion {
            axis: WorthUiLayoutAxis::Column,
            sizing: None,
            gap: None,
            padding: None,
            scroll_owner,
            resizable,
            restorable: false,
            children: Vec::new(),
            sibling_boundaries: Vec::new(),
        })
    }
}
