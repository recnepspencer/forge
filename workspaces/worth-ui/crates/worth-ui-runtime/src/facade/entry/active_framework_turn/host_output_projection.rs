pub(super) fn ordinary_output(
    generation: worth_ui_host_contract::WorthUiHostOutputGeneration,
    target: crate::runtime::WorthUiOrdinaryFrameTarget,
    receipt: &crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
) -> worth_ui_host_contract::WorthUiHostOutputEnvelope {
    worth_ui_host_contract::WorthUiHostOutputEnvelope::ordinary(
        generation,
        receipt.touch().touch_digest(),
        worth_ui_host_contract::WorthUiOrdinaryHostOutput::new(
            target.host_output_target(),
            receipt.touch().row_count(),
        ),
    )
}

pub(super) fn virtualized_data_output(
    generation: worth_ui_host_contract::WorthUiHostOutputGeneration,
    receipt: &crate::runtime::WorthUiVirtualizedDataFrameReceipt,
) -> worth_ui_host_contract::WorthUiHostOutputEnvelope {
    let range = receipt.visible_range();
    worth_ui_host_contract::WorthUiHostOutputEnvelope::virtualized_data(
        generation,
        receipt.evidence().evidence_identity_digest(),
        worth_ui_host_contract::WorthUiVirtualizedDataHostOutput::new(
            range.start_row(),
            range.row_count(),
            range.start_column(),
            range.column_count(),
            receipt.evidence().evidence_identity_digest(),
        ),
    )
}

pub(super) fn canvas_spatial_output(
    generation: worth_ui_host_contract::WorthUiHostOutputGeneration,
    target: crate::runtime::WorthUiCanvasSpatialFrameTarget,
    receipt: &crate::runtime::WorthUiCanvasSpatialFrameReceipt,
) -> worth_ui_host_contract::WorthUiHostOutputEnvelope {
    worth_ui_host_contract::WorthUiHostOutputEnvelope::canvas_spatial(
        generation,
        receipt.touch_digest()
            ^ receipt
                .certification()
                .handle_receipt()
                .basis_digest()
                .rotate_left(17),
        worth_ui_host_contract::WorthUiCanvasSpatialHostOutput::new(
            canvas_host_target(target),
            receipt.visible_primitive_count(),
            receipt.queried_hit_test_region_count(),
            receipt.touched_overlay_row_count(),
            receipt.touched_tool_state_row_count(),
        ),
    )
}

pub(super) fn realtime_output(
    generation: worth_ui_host_contract::WorthUiHostOutputGeneration,
    receipt: &crate::runtime::WorthUiRealtimeFrameReceipt,
) -> worth_ui_host_contract::WorthUiHostOutputEnvelope {
    worth_ui_host_contract::WorthUiHostOutputEnvelope::realtime_overlay(
        generation,
        receipt.touch_digest()
            ^ receipt
                .certification()
                .handle_receipt()
                .basis_digest()
                .rotate_left(17),
        worth_ui_host_contract::WorthUiRealtimeHostOutput::new(
            receipt.touched_overlay_row_count(),
            receipt.certification().policy_digest(),
        ),
    )
}

fn canvas_host_target(
    target: crate::runtime::WorthUiCanvasSpatialFrameTarget,
) -> worth_ui_host_contract::WorthUiCanvasSpatialHostOutputTarget {
    use crate::runtime::execution::canvas_spatial_lane::WorthUiCanvasSpatialFrameTargetKind;
    match target.kind() {
        WorthUiCanvasSpatialFrameTargetKind::Viewport(request) => {
            worth_ui_host_contract::WorthUiCanvasSpatialHostOutputTarget::Viewport {
                pan_delta_x: request.pan_delta_x(),
                pan_delta_y: request.pan_delta_y(),
                zoom_milli_factor: request.zoom_milli_factor(),
            }
        }
        WorthUiCanvasSpatialFrameTargetKind::Draw(_) => {
            worth_ui_host_contract::WorthUiCanvasSpatialHostOutputTarget::Draw
        }
        WorthUiCanvasSpatialFrameTargetKind::HitTest(request) => {
            worth_ui_host_contract::WorthUiCanvasSpatialHostOutputTarget::HitTest {
                viewport_x: request.viewport_point().x(),
                viewport_y: request.viewport_point().y(),
            }
        }
        WorthUiCanvasSpatialFrameTargetKind::Overlay(_) => {
            worth_ui_host_contract::WorthUiCanvasSpatialHostOutputTarget::Overlay
        }
        WorthUiCanvasSpatialFrameTargetKind::ToolState(_) => {
            worth_ui_host_contract::WorthUiCanvasSpatialHostOutputTarget::ToolState
        }
    }
}
