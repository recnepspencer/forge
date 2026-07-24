use worth_ui_host_contract::{
    UiMountedFrameManifest, UiMountedLaneParticipation as Lane, UiRequiredLaneContribution,
    UiRequiredLaneContributionStatus,
};

use super::{
    UiMountedFramePreparationDenial, UiMountedFrameRequest, UiMountedIdentityState,
    UiMountedPreviewProjectionInput, UiPreparedMountedFrame, UiPreparedMountedProjection,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiMountedLaneAssembly {
    pub ordinary: bool,
    pub virtualized: bool,
    pub canvas: bool,
    pub realtime: bool,
    pub preview: bool,
}

pub(crate) struct UiMountedFrameAssemblyInput<'input, 'graph> {
    pub graph: crate::graph::UiGraphAuthority<'graph>,
    pub generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    pub plan_digest: u64,
    pub allocation_truth_revision: u64,
    pub plan_rows: &'input [(u64, u32)],
    pub allocation_receipts: &'input [crate::runtime::UiAllocationReceipt],
    pub request: UiMountedFrameRequest,
    pub lanes: UiMountedLaneAssembly,
    pub preview: Option<UiMountedPreviewProjectionInput>,
}

pub(crate) struct UiMountedFrameAssembler<'state> {
    state: &'state UiMountedIdentityState,
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    manifest: UiMountedFrameManifest,
    projection: UiPreparedMountedProjection,
    graph_world: u64,
    allocation_truth_revision: u64,
    required: UiMountedLaneAssembly,
    recorded: UiMountedLaneAssembly,
}

impl<'state> UiMountedFrameAssembler<'state> {
    pub(crate) fn begin(
        state: &'state UiMountedIdentityState,
        input: UiMountedFrameAssemblyInput<'_, '_>,
    ) -> Result<Self, UiMountedFramePreparationDenial> {
        let bindings = input
            .request
            .resolve_requirements(state.view().surface_bindings())?;
        let surfaces = bindings
            .iter()
            .map(|binding| binding.semantic_surface_identity())
            .collect::<Vec<_>>();
        let requirements = bindings
            .iter()
            .copied()
            .map(super::binding_requirement)
            .collect();
        let manifest =
            UiMountedFrameManifest::new(requirements, lane_cells(&surfaces, input.lanes));
        super::validate_manifest(&manifest)?;
        let projection = super::prepare_projection(
            state,
            super::UiMountedProjectionInput {
                graph: input.graph,
                plan_digest: input.plan_digest,
                plan_rows: input.plan_rows,
                allocation_receipts: input.allocation_receipts,
                requested_surfaces: &surfaces,
                preview: input.preview,
            },
        )
        .map_err(UiMountedFramePreparationDenial::Projection)?;
        Ok(Self {
            graph_world: state.world_identity().diagnostic_value(),
            state,
            generation: input.generation,
            manifest,
            projection,
            allocation_truth_revision: input.allocation_truth_revision,
            required: input.lanes,
            recorded: UiMountedLaneAssembly {
                preview: input.preview.is_some(),
                ..Default::default()
            },
        })
    }

    pub(crate) fn record_ordinary(
        &mut self,
        receipt: &crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    ) -> Result<(), super::UiMountedProjectionDenial> {
        self.projection.record_ordinary(receipt)?;
        self.recorded.ordinary = true;
        Ok(())
    }

    pub(crate) fn record_virtualized(
        &mut self,
        receipt: &crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    ) -> Result<(), super::UiMountedProjectionDenial> {
        self.projection.record_virtualized(receipt)?;
        self.recorded.virtualized = true;
        Ok(())
    }

    pub(crate) fn record_canvas(
        &mut self,
        receipt: &crate::runtime::WorthUiCanvasSpatialFrameReceipt,
        resource_content_identity: u64,
    ) -> Result<(), super::UiMountedProjectionDenial> {
        self.projection
            .record_canvas(receipt, resource_content_identity)?;
        self.recorded.canvas = true;
        Ok(())
    }

    pub(crate) fn record_realtime(
        &mut self,
        receipt: &crate::runtime::WorthUiRealtimeFrameReceipt,
    ) -> Result<(), super::UiMountedProjectionDenial> {
        self.projection.record_realtime(receipt)?;
        self.recorded.realtime = true;
        Ok(())
    }

    pub(crate) fn finish(self) -> Result<UiPreparedMountedFrame, UiMountedFramePreparationDenial> {
        if self.recorded != self.required {
            return Err(UiMountedFramePreparationDenial::IncompleteManifest);
        }
        let candidate = self
            .projection
            .finish(self.state)
            .map_err(UiMountedFramePreparationDenial::Projection)?;
        UiPreparedMountedFrame::admit(
            candidate,
            self.generation,
            self.manifest,
            self.graph_world,
            self.allocation_truth_revision,
        )
    }
}

fn lane_cells(
    surfaces: &[worth_ui_host_contract::UiSemanticSurfaceIdentity],
    lanes: UiMountedLaneAssembly,
) -> Vec<UiRequiredLaneContribution> {
    surfaces
        .iter()
        .flat_map(|surface| {
            [
                lane_cell(*surface, Lane::Ordinary, lanes.ordinary),
                lane_cell(*surface, Lane::Virtualized, lanes.virtualized),
                lane_cell(*surface, Lane::CanvasSpatial, lanes.canvas),
                lane_cell(*surface, Lane::Realtime, lanes.realtime),
                lane_cell(*surface, Lane::Preview, lanes.preview),
            ]
        })
        .collect()
}

fn lane_cell(
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    lane: Lane,
    admitted: bool,
) -> UiRequiredLaneContribution {
    let status = if admitted {
        UiRequiredLaneContributionStatus::Admitted
    } else {
        UiRequiredLaneContributionStatus::ExplicitEmpty
    };
    UiRequiredLaneContribution::new(surface, lane, status)
}
