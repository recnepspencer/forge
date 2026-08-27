use worth_ui_host_contract::{
    UiMountedInstanceIdentity, UiMountedPortalOverlayMechanic, UiMountedPortalOverlayReference,
};

use super::{UiMountedProjectionDenial, UiMountedProjectionFrame, UiMountedProjectionSurface};

type UiMountedPortalOverlayReferenceIndex =
    std::collections::BTreeMap<UiMountedInstanceIdentity, Vec<UiMountedPortalOverlayReference>>;

pub(super) struct UiMountedPortalOverlayViewRows {
    pub(super) rows: Vec<UiMountedPortalOverlayMechanic>,
    pub(super) references: UiMountedPortalOverlayReferenceIndex,
}

impl UiMountedProjectionFrame {
    pub(super) fn portal_overlay_view_rows(
        &self,
        surface: UiMountedProjectionSurface,
    ) -> Result<UiMountedPortalOverlayViewRows, UiMountedProjectionDenial> {
        let mut rows = Vec::new();
        for input in self.portal_overlays.iter().copied() {
            let owner = self
                .semantic
                .node(input.owner())
                .ok_or(UiMountedProjectionDenial::PortalOverlayOwnerMissing)?;
            if owner.receipt.semantic_surface() != surface.surface {
                continue;
            }
            let receipt = self
                .receipt_basis
                .receipt_for(input.owner())
                .ok_or(UiMountedProjectionDenial::PortalOverlayOwnerMissing)?;
            rows.push(
                input
                    .mechanic_for(self.frame, surface.surface, surface.binding, receipt)
                    .map_err(UiMountedProjectionDenial::PortalOverlayCompletion)?,
            );
        }
        let mut references = UiMountedPortalOverlayReferenceIndex::new();
        for (index, row) in rows.iter().enumerate() {
            let reference = u16::try_from(index)
                .map(UiMountedPortalOverlayReference::from_runtime_mounting)
                .map_err(|_| UiMountedProjectionDenial::PortalOverlayCapacityExceeded)?;
            references.entry(row.owner()).or_default().push(reference);
        }
        Ok(UiMountedPortalOverlayViewRows { rows, references })
    }

    pub(super) fn portal_overlay_visual_rows(&self) -> Vec<UiMountedPortalOverlayMechanic> {
        self.semantic
            .surfaces
            .iter()
            .map(|(_, surface)| *surface)
            .flat_map(|surface| {
                self.portal_overlay_view_rows(surface)
                    .expect("prepared Portal overlays retain exact mounted owners")
                    .rows
            })
            .collect()
    }
}
