use super::WorthUiActiveFrameworkTurnExecution;

pub(crate) struct WorthUiActiveMountedProjectionFrame<'session> {
    execution: crate::runtime::WorthUiFrameworkTurnExecution<'session>,
    assembler: crate::mounting::UiMountedFrameAssembler<'session>,
}

#[derive(Debug, PartialEq)]
pub enum WorthUiMountedLaneProjectionDenial {
    Ordinary(crate::runtime::WorthUiOrdinaryLaneFrameDenial),
    Virtualized(crate::runtime::WorthUiVirtualizedDataFrameDenial),
    Canvas(crate::runtime::WorthUiCanvasSpatialFrameDenial),
    Realtime(crate::runtime::WorthUiRealtimeFrameDenial),
    Projection(crate::mounting::UiMountedProjectionDenial),
}

impl<'session> WorthUiActiveFrameworkTurnExecution<'session> {
    pub fn classify_mounted_frame_reuse(
        &self,
        request: &crate::mounting::UiMountedFrameRequest,
    ) -> crate::mounting::UiMountedFrameReuse {
        let plan = self.execution.runtime.active.active_plan_ref();
        let lanes = mounted_lanes(plan, request.virtualized_range().is_some());
        let allocation_truth_revision = self
            .execution
            .runtime
            .allocation_receipt_ledger
            .truth_revision()
            .revision();
        self.mounted_identity.classify_reuse(self.reuse_contract(
            request,
            lanes,
            allocation_truth_revision,
        ))
    }

    pub fn prepare_mounted_frame(
        self,
        request: crate::mounting::UiMountedFrameRequest,
    ) -> Result<
        crate::mounting::UiPreparedMountedFrame,
        crate::mounting::UiMountedFramePreparationDenial,
    > {
        use worth_ui_host_contract::UiMountedLaneParticipation as Lane;

        let virtualized_range = request.virtualized_range();
        let plan = self.execution.runtime.active.active_plan_ref();
        let lanes = mounted_lanes(plan, request.virtualized_range().is_some());
        let generation = self.generation_identity.clone();
        let allocation_truth_revision = self
            .execution
            .runtime
            .allocation_receipt_ledger
            .truth_revision()
            .revision();
        let plan_digest = plan.digest().as_u64();
        let allocation_catalog = self
            .execution
            .runtime
            .allocation_receipt_ledger
            .mounted_projection_catalog();
        let reuse_contract = self.reuse_contract(&request, lanes, allocation_truth_revision);
        let assembler = crate::mounting::UiMountedFrameAssembler::begin(
            self.mounted_identity,
            crate::mounting::UiMountedFrameAssemblyInput {
                graph: self.graph,
                generation,
                plan_digest,
                plan: crate::mounting::UiMountedPlanProjectionSource::Executed(plan),
                allocation_catalog,
                allocation_truth_revision,
                request,
                lanes,
                preview: None,
                reuse_contract,
            },
        )?;
        let mut projection = WorthUiActiveMountedProjectionFrame {
            execution: self.execution,
            assembler,
        };
        if lanes.ordinary {
            projection
                .execute_ordinary(crate::runtime::WorthUiOrdinaryFrameTarget::root_shell())
                .map_err(crate::mounting::UiMountedFramePreparationDenial::Lane)?;
        }
        if let Some(range) = virtualized_range.filter(|_| lanes.virtualized) {
            let target = projection
                .execution
                .runtime
                .active
                .active_plan_ref()
                .virtualized_summary(
                    &projection.execution.runtime.query_binding,
                    crate::runtime::WorthUiVirtualizedPlanSummaryRequest::first_view(),
                )
                .map_err(|_| {
                    crate::mounting::UiMountedFramePreparationDenial::LaneWorkUnavailable(
                        Lane::Virtualized,
                    )
                })?
                .target(range);
            projection
                .execute_virtualized(target)
                .map_err(crate::mounting::UiMountedFramePreparationDenial::Lane)?;
        }
        if lanes.canvas {
            let handle = projection
                .execution
                .runtime
                .active
                .active_plan_ref()
                .first_canvas_spatial_handle()
                .ok_or(
                    crate::mounting::UiMountedFramePreparationDenial::LaneWorkUnavailable(
                        Lane::CanvasSpatial,
                    ),
                )?;
            projection
                .execute_canvas(crate::runtime::WorthUiCanvasSpatialFrameTarget::draw(
                    handle,
                ))
                .map_err(crate::mounting::UiMountedFramePreparationDenial::Lane)?;
        }
        if lanes.realtime {
            let handle = projection
                .execution
                .runtime
                .active
                .active_plan_ref()
                .first_realtime_handle()
                .ok_or(
                    crate::mounting::UiMountedFramePreparationDenial::LaneWorkUnavailable(
                        Lane::Realtime,
                    ),
                )?;
            projection
                .execute_realtime(
                    crate::runtime::WorthUiRealtimeFrameTarget::renderer_surface(handle),
                )
                .map_err(crate::mounting::UiMountedFramePreparationDenial::Lane)?;
        }
        projection.finish()
    }

    fn reuse_contract(
        &self,
        request: &crate::mounting::UiMountedFrameRequest,
        lanes: crate::mounting::UiMountedLaneAssembly,
        allocation_truth_revision: u64,
    ) -> crate::mounting::UiMountedFrameReuseContract {
        self.mounted_identity.seal_reuse_contract(
            crate::mounting::UiMountedFrameReuseExternalBasis {
                generation: self.generation_identity.clone(),
                host_session: self.host_session_identity.as_u64(),
                execution: crate::mounting::UiMountedFrameExecutionPosture::ActiveFrame {
                    frame_epoch: self.execution.active_frame_epoch().as_u64(),
                },
                plan_digest: self.execution.active_plan_digest(),
                allocation_truth_revision,
                request: request.reuse_identity(),
                lanes,
                protocol: self.host_protocol,
                capability_generation: self.host_capability_generation,
                capability_profile_digest: self.host_capability_profile_digest,
            },
        )
    }
}

fn mounted_lanes(
    plan: &crate::runtime::WorthUiActiveExecutionPlan,
    virtualized_range_present: bool,
) -> crate::mounting::UiMountedLaneAssembly {
    crate::mounting::UiMountedLaneAssembly {
        ordinary: matches!(
            plan.ordinary_availability(),
            crate::runtime::WorthUiOrdinaryPlanAvailability::Executable
        ),
        virtualized: virtualized_range_present
            && matches!(
                plan.virtualized_availability(),
                crate::runtime::WorthUiVirtualizedPlanAvailability::Executable
            ),
        canvas: matches!(
            plan.canvas_spatial_availability(),
            crate::runtime::WorthUiCanvasSpatialPlanAvailability::Executable
        ),
        realtime: matches!(
            plan.realtime_availability(),
            crate::runtime::WorthUiRealtimePlanAvailability::Executable
        ),
        preview: false,
    }
}

impl WorthUiActiveMountedProjectionFrame<'_> {
    pub(crate) fn execute_ordinary(
        &mut self,
        target: crate::runtime::WorthUiOrdinaryFrameTarget,
    ) -> Result<crate::runtime::WorthUiOrdinaryLaneFrameReceipt, WorthUiMountedLaneProjectionDenial>
    {
        let receipt = self
            .execution
            .execute_active_ordinary_frame(target)
            .map_err(WorthUiMountedLaneProjectionDenial::Ordinary)?;
        self.assembler
            .record_ordinary(&receipt)
            .map_err(WorthUiMountedLaneProjectionDenial::Projection)?;
        Ok(receipt)
    }

    pub(crate) fn execute_virtualized(
        &mut self,
        target: crate::runtime::WorthUiVirtualizedDataFrameTarget,
    ) -> Result<
        crate::runtime::WorthUiVirtualizedDataFrameReceipt,
        WorthUiMountedLaneProjectionDenial,
    > {
        let receipt = self
            .execution
            .execute_active_virtualized_data_frame(target)
            .map_err(WorthUiMountedLaneProjectionDenial::Virtualized)?;
        self.assembler
            .record_virtualized(&receipt)
            .map_err(WorthUiMountedLaneProjectionDenial::Projection)?;
        Ok(receipt)
    }

    pub(crate) fn execute_canvas(
        &mut self,
        target: crate::runtime::WorthUiCanvasSpatialFrameTarget,
    ) -> Result<crate::runtime::WorthUiCanvasSpatialFrameReceipt, WorthUiMountedLaneProjectionDenial>
    {
        let receipt = self
            .execution
            .execute_active_canvas_spatial_frame(target)
            .map_err(WorthUiMountedLaneProjectionDenial::Canvas)?;
        let runtime_handle = receipt
            .touched_runtime_handles()
            .first()
            .copied()
            .expect("sealed canvas receipts name one runtime handle");
        let lane_handle = crate::runtime::WorthUiLaneHandle::from_locator(runtime_handle.locator());
        let resource_content_identity = self
            .execution
            .runtime
            .active
            .active_plan_ref()
            .canvas_spatial_summary(lane_handle)
            .expect("sealed canvas receipt resolves in its active plan")
            .plan_basis_digest();
        self.assembler
            .record_canvas(&receipt, resource_content_identity)
            .map_err(WorthUiMountedLaneProjectionDenial::Projection)?;
        Ok(receipt)
    }

    pub(crate) fn execute_realtime(
        &mut self,
        target: crate::runtime::WorthUiRealtimeFrameTarget,
    ) -> Result<crate::runtime::WorthUiRealtimeFrameReceipt, WorthUiMountedLaneProjectionDenial>
    {
        let receipt = self
            .execution
            .execute_active_realtime_frame(target)
            .map_err(WorthUiMountedLaneProjectionDenial::Realtime)?;
        self.assembler
            .record_realtime(&receipt)
            .map_err(WorthUiMountedLaneProjectionDenial::Projection)?;
        Ok(receipt)
    }

    pub(crate) fn finish(
        self,
    ) -> Result<
        crate::mounting::UiPreparedMountedFrame,
        crate::mounting::UiMountedFramePreparationDenial,
    > {
        self.assembler.finish()
    }
}
