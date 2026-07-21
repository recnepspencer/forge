use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

/// Compact ordinary execution result bound to the exact active generation.
pub struct WorthUiActiveOrdinaryFrameCompletion<'execution> {
    generation_identity: &'execution WorthUiPreparedApplicationGenerationIdentity,
    receipt: crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    output: worth_ui_host_contract::WorthUiHostOutputEnvelope,
    disposition: worth_ui_host_contract::WorthUiHostOutputDisposition,
}

/// Compact virtualized execution result bound to the exact active generation.
pub struct WorthUiActiveVirtualizedDataFrameCompletion<'execution> {
    generation_identity: &'execution WorthUiPreparedApplicationGenerationIdentity,
    receipt: crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    output: worth_ui_host_contract::WorthUiHostOutputEnvelope,
    disposition: worth_ui_host_contract::WorthUiHostOutputDisposition,
}

/// Compact canvas execution result routed through the exact active host.
#[derive(Debug)]
pub struct WorthUiActiveCanvasSpatialFrameCompletion<'execution> {
    generation_identity: &'execution WorthUiPreparedApplicationGenerationIdentity,
    receipt: crate::runtime::WorthUiCanvasSpatialFrameReceipt,
    output: worth_ui_host_contract::WorthUiHostOutputEnvelope,
    disposition: worth_ui_host_contract::WorthUiHostOutputDisposition,
}

/// Compact realtime execution result routed through the exact active host.
#[derive(Debug)]
pub struct WorthUiActiveRealtimeFrameCompletion<'execution> {
    generation_identity: &'execution WorthUiPreparedApplicationGenerationIdentity,
    receipt: crate::runtime::WorthUiRealtimeFrameReceipt,
    output: worth_ui_host_contract::WorthUiHostOutputEnvelope,
    disposition: worth_ui_host_contract::WorthUiHostOutputDisposition,
}

macro_rules! frame_completion {
    ($completion:ident, $receipt:ty, $record:ident) => {
        impl<'execution> $completion<'execution> {
            pub(super) fn new(
                generation_identity: &'execution WorthUiPreparedApplicationGenerationIdentity,
                receipt: $receipt,
                output: worth_ui_host_contract::WorthUiHostOutputEnvelope,
                disposition: worth_ui_host_contract::WorthUiHostOutputDisposition,
            ) -> Self {
                Self {
                    generation_identity,
                    receipt,
                    output,
                    disposition,
                }
            }

            pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
                self.generation_identity
            }

            pub fn receipt(&self) -> &$receipt {
                &self.receipt
            }

            pub fn output(&self) -> &worth_ui_host_contract::WorthUiHostOutputEnvelope {
                &self.output
            }

            pub fn disposition(&self) -> worth_ui_host_contract::WorthUiHostOutputDisposition {
                self.disposition
            }

            pub fn cost_receipt(
                &self,
            ) -> Result<
                crate::runtime::WorthUiFrameExecutionReceipt,
                crate::runtime::WorthUiSteadyFrameCounterDenial,
            > {
                crate::runtime::WorthUiSteadyFrameCounterBoundary::for_active_generation(
                    self.output.generation(),
                )
                .$record(self.receipt.clone())
                .seal()
            }
        }
    };
}

frame_completion!(
    WorthUiActiveOrdinaryFrameCompletion,
    crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    record_ordinary_lane_frame
);
frame_completion!(
    WorthUiActiveVirtualizedDataFrameCompletion,
    crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    record_virtualized_data_frame
);
frame_completion!(
    WorthUiActiveCanvasSpatialFrameCompletion,
    crate::runtime::WorthUiCanvasSpatialFrameReceipt,
    record_canvas_spatial_frame
);
frame_completion!(
    WorthUiActiveRealtimeFrameCompletion,
    crate::runtime::WorthUiRealtimeFrameReceipt,
    record_realtime_overlay_frame
);

impl std::ops::Deref for WorthUiActiveCanvasSpatialFrameCompletion<'_> {
    type Target = crate::runtime::WorthUiCanvasSpatialFrameReceipt;

    fn deref(&self) -> &Self::Target {
        &self.receipt
    }
}

impl std::ops::Deref for WorthUiActiveRealtimeFrameCompletion<'_> {
    type Target = crate::runtime::WorthUiRealtimeFrameReceipt;

    fn deref(&self) -> &Self::Target {
        &self.receipt
    }
}
