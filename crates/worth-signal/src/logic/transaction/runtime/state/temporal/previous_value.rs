use crate::data::error::SignalError;
use crate::data::temporal::{
    TemporalPreviousValueAccess, TemporalPreviousValueReference, TemporalWakeId,
};
use crate::state::SignalBranchId;

use super::super::runtime_state::SignalRuntime;
use super::TemporalRuntimeState;

impl TemporalRuntimeState {
    pub fn grant_previous_value_access(
        &self,
        branch_id: SignalBranchId,
        wake_id: TemporalWakeId,
    ) -> Result<TemporalPreviousValueAccess, SignalError> {
        let ready = self.ready_wakes.get(&wake_id).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "cannot grant previous-value access from non-ready temporal wake {}",
                wake_id.get()
            ))
        })?;
        Ok(TemporalPreviousValueAccess::from_ready_wake(
            branch_id,
            self.previous_value_capability_epoch,
            ready,
        ))
    }

    pub fn capture_previous_value_reference(
        &mut self,
        access: &TemporalPreviousValueAccess,
        node: crate::data::handle::NodeId,
        aspect_version: crate::data::aspect::AspectVersion,
        output_identity: Option<crate::data::output::OutputIdentity>,
        telemetry: Option<&mut crate::data::telemetry::TemporalTelemetry>,
    ) -> Result<TemporalPreviousValueReference, SignalError> {
        let Some(ready) = self.ready_wakes.get(&access.wake_id()) else {
            return Err(SignalError::invalid_input(format!(
                "cannot capture previous value from inactive temporal access wake {}",
                access.wake_id().get()
            )));
        };
        if access.capability_epoch() != self.previous_value_capability_epoch {
            return Err(SignalError::invalid_input(format!(
                "temporal previous-value access for wake {} belongs to stale restore epoch {} but active epoch is {}",
                access.wake_id().get(),
                access.capability_epoch(),
                self.previous_value_capability_epoch
            )));
        }
        if ready.ready_ordinal() != access.ready_ordinal()
            || ready.ready_tick() != access.ready_tick()
        {
            return Err(SignalError::invalid_input(format!(
                "temporal previous-value access for wake {} no longer matches active ready proof",
                access.wake_id().get()
            )));
        }

        let revision = self.issue_previous_value_revision();
        if let Some(telemetry) = telemetry {
            telemetry.previous_value_reference_count += 1;
        }
        Ok(TemporalPreviousValueReference::new(
            revision,
            access,
            node,
            aspect_version,
            output_identity,
        ))
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn grant_temporal_previous_value_access(
        &mut self,
        wake_id: TemporalWakeId,
    ) -> Result<TemporalPreviousValueAccess, SignalError> {
        self.ensure_active_temporal_wake_owner_live(wake_id, "grant previous-value access")?;
        self.temporal
            .grant_previous_value_access(self.graph.current_branch().id, wake_id)
    }

    pub fn previous_temporal_value(
        &mut self,
        access: &TemporalPreviousValueAccess,
        node: crate::data::handle::NodeId,
    ) -> Result<TemporalPreviousValueReference, SignalError> {
        let current_branch = self.graph.current_branch();
        if access.branch_id() != current_branch.id {
            return Err(SignalError::invalid_input(format!(
                "temporal previous-value access belongs to branch {} but current branch is {}",
                access.branch_id().0,
                current_branch.id.0
            )));
        }
        let aspect_version = self.graph.node_aspect_version(node)?;
        let output_identity = self
            .graph
            .observe()
            .runtime_artifact_warm(node)?
            .and_then(|warm| warm.output_identity.clone());
        let captures_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        let telemetry = captures_telemetry.then_some(&mut self.telemetry.temporal);
        self.temporal.capture_previous_value_reference(
            access,
            node,
            aspect_version,
            output_identity,
            telemetry,
        )
    }
}
