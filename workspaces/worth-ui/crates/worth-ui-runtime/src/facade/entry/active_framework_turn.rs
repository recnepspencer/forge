use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::runtime::WorthUiFrameworkTurnCompletion;

use super::active_host_output_handoff::consume_active_host_output;

mod frame_completion;
mod host_output_projection;

pub use frame_completion::{
    WorthUiActiveCanvasSpatialFrameCompletion, WorthUiActiveOrdinaryFrameCompletion,
    WorthUiActiveRealtimeFrameCompletion, WorthUiActiveVirtualizedDataFrameCompletion,
};
use host_output_projection::{
    canvas_spatial_output, ordinary_output, realtime_output, virtualized_data_output,
};

/// One framework-turn result bound to the active application generation.
pub struct WorthUiActiveFrameworkTurnCompletion<'session> {
    pub(super) generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    pub(super) host_session_identity: crate::facade::WorthUiHostSessionIdentity,
    pub(super) host_adapter: &'session dyn worth_ui_host_contract::WorthUiOperationalHostAdapter,
    pub(super) completion: WorthUiFrameworkTurnCompletion<'session>,
}

/// Executable framework-turn authority lent by one active application session.
pub struct WorthUiActiveFrameworkTurnExecution<'session> {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    host_session_identity: crate::facade::WorthUiHostSessionIdentity,
    host_adapter: &'session dyn worth_ui_host_contract::WorthUiOperationalHostAdapter,
    execution: crate::runtime::WorthUiFrameworkTurnExecution<'session>,
}

impl<'session> WorthUiActiveFrameworkTurnCompletion<'session> {
    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation_identity
    }

    pub fn into_completion(self) -> WorthUiFrameworkTurnCompletion<'session> {
        self.completion
    }

    pub fn into_execution(
        self,
    ) -> Result<WorthUiActiveFrameworkTurnExecution<'session>, Box<Self>> {
        let Self {
            generation_identity,
            host_session_identity,
            host_adapter,
            completion,
        } = self;
        match completion.into_execution() {
            Ok(execution) => Ok(WorthUiActiveFrameworkTurnExecution {
                generation_identity,
                host_session_identity,
                host_adapter,
                execution,
            }),
            Err(completion) => Err(Box::new(Self {
                generation_identity,
                host_session_identity,
                host_adapter,
                completion: *completion,
            })),
        }
    }
}

impl WorthUiActiveFrameworkTurnExecution<'_> {
    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation_identity
    }

    pub fn activation_boundary(&self) -> &crate::runtime::WorthUiFrameBoundary {
        self.execution.activation_boundary()
    }

    pub fn into_activation_boundary(self) -> crate::runtime::WorthUiFrameBoundary {
        self.execution.into_activation_boundary()
    }

    pub fn planning_counters(&self) -> crate::runtime::UiFrameworkTransitionPlanningCounters {
        self.execution.planning_counters()
    }

    pub fn execute_ordinary_frame(
        &self,
        target: crate::runtime::WorthUiOrdinaryFrameTarget,
    ) -> Result<
        WorthUiActiveOrdinaryFrameCompletion<'_>,
        crate::runtime::WorthUiOrdinaryLaneFrameDenial,
    > {
        let receipt = self.execution.execute_active_ordinary_frame(target)?;
        let output = ordinary_output(self.host_output_generation(), target, &receipt);
        let disposition = self.consume_output(&output);
        Ok(WorthUiActiveOrdinaryFrameCompletion::new(
            &self.generation_identity,
            receipt,
            output,
            disposition,
        ))
    }

    pub fn execute_canvas_spatial_frame(
        &self,
        target: crate::runtime::WorthUiCanvasSpatialFrameTarget,
    ) -> Result<
        WorthUiActiveCanvasSpatialFrameCompletion<'_>,
        crate::runtime::WorthUiCanvasSpatialFrameDenial,
    > {
        let receipt = self.execution.execute_active_canvas_spatial_frame(target)?;
        let output = canvas_spatial_output(self.host_output_generation(), target, &receipt);
        let disposition = self.consume_output(&output);
        Ok(WorthUiActiveCanvasSpatialFrameCompletion::new(
            &self.generation_identity,
            receipt,
            output,
            disposition,
        ))
    }

    pub fn execute_realtime_frame(
        &self,
        target: crate::runtime::WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiActiveRealtimeFrameCompletion<'_>, crate::runtime::WorthUiRealtimeFrameDenial>
    {
        let receipt = self.execution.execute_active_realtime_frame(target)?;
        let output = realtime_output(self.host_output_generation(), &receipt);
        let disposition = self.consume_output(&output);
        Ok(WorthUiActiveRealtimeFrameCompletion::new(
            &self.generation_identity,
            receipt,
            output,
            disposition,
        ))
    }

    pub fn execute_virtualized_data_frame(
        &self,
        target: crate::runtime::WorthUiVirtualizedDataFrameTarget,
    ) -> Result<
        WorthUiActiveVirtualizedDataFrameCompletion<'_>,
        crate::runtime::WorthUiVirtualizedDataFrameDenial,
    > {
        let receipt = self
            .execution
            .execute_active_virtualized_data_frame(target)?;
        let output = virtualized_data_output(self.host_output_generation(), &receipt);
        let disposition = self.consume_output(&output);
        Ok(WorthUiActiveVirtualizedDataFrameCompletion::new(
            &self.generation_identity,
            receipt,
            output,
            disposition,
        ))
    }

    fn consume_output(
        &self,
        output: &worth_ui_host_contract::WorthUiHostOutputEnvelope,
    ) -> worth_ui_host_contract::WorthUiHostOutputDisposition {
        consume_active_host_output(self.host_adapter, self.host_output_generation(), output)
    }

    fn host_output_generation(&self) -> worth_ui_host_contract::WorthUiHostOutputGeneration {
        worth_ui_host_contract::WorthUiHostOutputGeneration::new(
            self.host_session_identity.as_u64(),
            self.execution.active_artifact_digest(),
            self.execution.active_plan_digest(),
            self.execution.active_frame_epoch().as_u64(),
        )
    }
}
