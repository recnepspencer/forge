#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPortalAnchorIdentity {
    target: worth_ui_host_contract::UiPortalAnchorTargetIdentity,
    coordinate_space: crate::evidence::UiMeasurementCoordinateSpace,
}

impl UiPortalAnchorIdentity {
    pub(crate) fn from_measurement_result(
        result: &crate::evidence::UiMeasurementResult,
    ) -> Option<Self> {
        Some(Self {
            target: result.portal_anchor_target_identity()?,
            coordinate_space: result.coordinate_space(),
        })
    }

    #[cfg(test)]
    pub(super) const fn from_parts_for_test(
        target: worth_ui_host_contract::UiPortalAnchorTargetIdentity,
        coordinate_space: crate::evidence::UiMeasurementCoordinateSpace,
    ) -> Self {
        Self {
            target,
            coordinate_space,
        }
    }

    pub const fn target(self) -> worth_ui_host_contract::UiPortalAnchorTargetIdentity {
        self.target
    }

    pub const fn coordinate_space(self) -> crate::evidence::UiMeasurementCoordinateSpace {
        self.coordinate_space
    }

    pub fn identity_digest(self) -> u64 {
        crate::declaration::stable_text_digest("worth-ui.portal-anchor-identity")
            ^ self.target.raw().rotate_left(17)
            ^ coordinate_space_digest(self.coordinate_space).rotate_left(37)
    }
}

fn coordinate_space_digest(space: crate::evidence::UiMeasurementCoordinateSpace) -> u64 {
    crate::declaration::stable_text_digest(match space {
        crate::evidence::UiMeasurementCoordinateSpace::Viewport => "viewport",
        crate::evidence::UiMeasurementCoordinateSpace::Window => "window",
        crate::evidence::UiMeasurementCoordinateSpace::GraphNodeLocal => "graph-node-local",
        crate::evidence::UiMeasurementCoordinateSpace::HostSurface => "host-surface",
        crate::evidence::UiMeasurementCoordinateSpace::PortalLayer => "portal-layer",
    })
}
