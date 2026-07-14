#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiLayoutOperatorPrimaryAxis {
    None,
    Vertical,
    Horizontal,
    TwoDimensional,
    Layered,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiLayoutOperatorCrossAxis {
    None,
    Horizontal,
    Vertical,
    TwoDimensional,
    Layered,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiLayoutOperatorChildParticipationRule {
    None,
    RootViewportFrame,
    VerticalPeerFlow,
    HorizontalPeerFlow,
    GridCellPeers,
    SplitPanels,
    MosaicTiles,
    OverlayLayers,
    ScrollContent,
    PortalAnchoredContent,
    ContainerChildren,
}

pub(crate) fn primary_axis_name(axis: UiLayoutOperatorPrimaryAxis) -> &'static str {
    match axis {
        UiLayoutOperatorPrimaryAxis::None => "none",
        UiLayoutOperatorPrimaryAxis::Vertical => "vertical",
        UiLayoutOperatorPrimaryAxis::Horizontal => "horizontal",
        UiLayoutOperatorPrimaryAxis::TwoDimensional => "two-dimensional",
        UiLayoutOperatorPrimaryAxis::Layered => "layered",
    }
}

pub(crate) fn cross_axis_name(axis: UiLayoutOperatorCrossAxis) -> &'static str {
    match axis {
        UiLayoutOperatorCrossAxis::None => "none",
        UiLayoutOperatorCrossAxis::Horizontal => "horizontal",
        UiLayoutOperatorCrossAxis::Vertical => "vertical",
        UiLayoutOperatorCrossAxis::TwoDimensional => "two-dimensional",
        UiLayoutOperatorCrossAxis::Layered => "layered",
    }
}

pub(crate) fn child_participation_rule_name(
    rule: UiLayoutOperatorChildParticipationRule,
) -> &'static str {
    match rule {
        UiLayoutOperatorChildParticipationRule::None => "none",
        UiLayoutOperatorChildParticipationRule::RootViewportFrame => "root-viewport-frame",
        UiLayoutOperatorChildParticipationRule::VerticalPeerFlow => "vertical-peer-flow",
        UiLayoutOperatorChildParticipationRule::HorizontalPeerFlow => "horizontal-peer-flow",
        UiLayoutOperatorChildParticipationRule::GridCellPeers => "grid-cell-peers",
        UiLayoutOperatorChildParticipationRule::SplitPanels => "split-panels",
        UiLayoutOperatorChildParticipationRule::MosaicTiles => "mosaic-tiles",
        UiLayoutOperatorChildParticipationRule::OverlayLayers => "overlay-layers",
        UiLayoutOperatorChildParticipationRule::ScrollContent => "scroll-content",
        UiLayoutOperatorChildParticipationRule::PortalAnchoredContent => "portal-anchored-content",
        UiLayoutOperatorChildParticipationRule::ContainerChildren => "container-children",
    }
}
