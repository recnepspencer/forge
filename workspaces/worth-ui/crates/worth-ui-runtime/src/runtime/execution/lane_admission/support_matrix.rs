use std::collections::BTreeMap;

use crate::runtime::{WorthUiExecutionLane, WorthUiExecutionLaneDescriptor, WorthUiLaneSupportRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionLaneSupport {
    rows: BTreeMap<WorthUiExecutionLane, WorthUiLaneSupportRow>,
}

impl WorthUiExecutionLaneSupport {
    pub(crate) fn for_prepared_application(query_installed: bool) -> Self {
        let mut support = Self::platform_default();
        if !query_installed {
            support.rows.insert(
                WorthUiExecutionLane::QueryBound,
                WorthUiLaneSupportRow::unsupported(WorthUiExecutionLaneDescriptor::for_lane(
                    WorthUiExecutionLane::QueryBound,
                    true,
                )),
            );
        }
        support
    }

    pub fn platform_default() -> Self {
        Self::from_supported_lanes([
            WorthUiExecutionLane::OrdinaryWidgetShell,
            WorthUiExecutionLane::VirtualizedData,
            WorthUiExecutionLane::CanvasSpatial,
            WorthUiExecutionLane::RealtimeOverlayHud,
            WorthUiExecutionLane::QueryBound,
            WorthUiExecutionLane::CommandSurface,
            WorthUiExecutionLane::StyleToken,
            WorthUiExecutionLane::DiagnosticsProjection,
            WorthUiExecutionLane::LaneBoundary,
            WorthUiExecutionLane::RenderResource,
            WorthUiExecutionLane::SpecialCaseExtension,
        ])
    }

    pub(crate) fn from_supported_lanes<const N: usize>(lanes: [WorthUiExecutionLane; N]) -> Self {
        let rows = lanes
            .into_iter()
            .map(|lane| {
                let descriptor = WorthUiExecutionLaneDescriptor::for_lane(
                    lane,
                    lane == WorthUiExecutionLane::QueryBound,
                );
                (lane, WorthUiLaneSupportRow::supported(descriptor))
            })
            .collect();
        Self { rows }
    }

    #[cfg(test)]
    pub(crate) fn without_lane_for_test(lane: WorthUiExecutionLane) -> Self {
        let mut support = Self::platform_default();
        support.rows.insert(
            lane,
            WorthUiLaneSupportRow::unsupported(WorthUiExecutionLaneDescriptor::for_lane(
                lane,
                lane == WorthUiExecutionLane::QueryBound,
            )),
        );
        support
    }

    pub fn row_for_lane(&self, lane: WorthUiExecutionLane) -> Option<&WorthUiLaneSupportRow> {
        self.rows.get(&lane)
    }

    pub fn rows(&self) -> impl Iterator<Item = &WorthUiLaneSupportRow> {
        self.rows.values()
    }
}
