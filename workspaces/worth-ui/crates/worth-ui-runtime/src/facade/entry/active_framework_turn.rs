use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::runtime::WorthUiFrameworkTurnCompletion;

mod frame_completion;
mod mounted_projection;

pub use frame_completion::{
    WorthUiActiveCanvasSpatialFrameCompletion, WorthUiActiveOrdinaryFrameCompletion,
    WorthUiActiveRealtimeFrameCompletion, WorthUiActiveVirtualizedDataFrameCompletion,
};
pub use mounted_projection::WorthUiMountedLaneProjectionDenial;

/// One framework-turn result bound to the active application generation.
pub struct WorthUiActiveFrameworkTurnCompletion<'session> {
    pub(super) generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    pub(super) graph: crate::graph::UiGraphAuthority<'session>,
    pub(super) active_plan_digest: u64,
    pub(super) host_session_identity: crate::facade::WorthUiHostSessionIdentity,
    pub(super) completion: WorthUiFrameworkTurnCompletion<'session>,
    pub(super) mounted_identity: &'session mut crate::mounting::UiMountedIdentityState,
    pub(super) host_session: &'session crate::facade::WorthUiHostSessionAuthority,
    pub(super) mounted_presentation:
        &'session mut crate::mounting::UiMountedPresentationCoordinator,
    pub(super) mounted_publication_reservations: &'session mut std::collections::BTreeMap<
        worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        crate::mounting::UiMountedFramePublicationCandidate,
    >,
    pub(super) host_observations: &'session mut crate::host_exchange::observation_report_validation::UiHostObservationReportValidation,
}

/// Executable framework-turn authority lent by one active application session.
pub struct WorthUiActiveFrameworkTurnExecution<'session> {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    graph: crate::graph::UiGraphAuthority<'session>,
    host_session_identity: crate::facade::WorthUiHostSessionIdentity,
    execution: crate::runtime::WorthUiFrameworkTurnExecution<'session>,
    mounted_identity: &'session mut crate::mounting::UiMountedIdentityState,
    host_protocol: worth_ui_host_contract::UiHostProtocolAgreement,
    host_capability_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    host_capability_profile_digest: u64,
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
            graph,
            active_plan_digest,
            host_session_identity,
            completion,
            mounted_identity,
            host_session,
            mounted_presentation,
            mounted_publication_reservations,
            host_observations,
        } = self;
        let host_protocol = host_session.protocol();
        let capability_report = host_session.capability_report();
        match completion.into_execution() {
            Ok(execution) => Ok(WorthUiActiveFrameworkTurnExecution {
                generation_identity,
                graph,
                host_session_identity,
                execution,
                mounted_identity,
                host_protocol,
                host_capability_generation: capability_report.observation_generation(),
                host_capability_profile_digest: capability_report.profile_identity_digest(),
            }),
            Err(completion) => Err(Box::new(Self {
                generation_identity,
                graph,
                active_plan_digest,
                host_session_identity,
                completion: *completion,
                mounted_identity,
                host_session,
                mounted_presentation,
                mounted_publication_reservations,
                host_observations,
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
        Ok(WorthUiActiveOrdinaryFrameCompletion::new(
            &self.generation_identity,
            receipt,
            self.frame_execution_basis(),
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
        Ok(WorthUiActiveCanvasSpatialFrameCompletion::new(
            &self.generation_identity,
            receipt,
            self.frame_execution_basis(),
        ))
    }

    pub fn execute_realtime_frame(
        &self,
        target: crate::runtime::WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiActiveRealtimeFrameCompletion<'_>, crate::runtime::WorthUiRealtimeFrameDenial>
    {
        let receipt = self.execution.execute_active_realtime_frame(target)?;
        Ok(WorthUiActiveRealtimeFrameCompletion::new(
            &self.generation_identity,
            receipt,
            self.frame_execution_basis(),
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
        Ok(WorthUiActiveVirtualizedDataFrameCompletion::new(
            &self.generation_identity,
            receipt,
            self.frame_execution_basis(),
        ))
    }

    fn frame_execution_basis(&self) -> crate::runtime::WorthUiFrameExecutionBasis {
        crate::runtime::WorthUiFrameExecutionBasis::new(
            self.host_session_identity.as_u64(),
            self.execution.active_artifact_digest(),
            self.execution.active_plan_digest(),
            self.execution.active_frame_epoch().as_u64(),
        )
    }
}
