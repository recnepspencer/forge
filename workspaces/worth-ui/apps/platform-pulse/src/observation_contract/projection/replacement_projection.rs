use worth_ui::facade::rebind::UiRebindReceipt;
use worth_ui::facade::source::WorthUiSourcePackageRevision;

use super::{
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseObservationState,
};
use crate::observation_contract::envelope::PlatformPulseLifecycleObservationEnvelope;
use crate::observation_contract::lifecycle::{
    PlatformPulseApplicationGenerationObservation, PlatformPulseLifecycleObservation,
    PlatformPulseMountedFrameObservation, PlatformPulseReplacementPublished,
    PlatformPulseSourceSnapshotObservation,
};

impl PlatformPulseLifecycleObservationStream {
    pub fn project_replacement(
        &mut self,
        source: &WorthUiSourcePackageRevision,
        receipt: &UiRebindReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let (prior, _, _) = self.published_predecessor()?;
        if &prior != receipt.prior_generation() {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::PriorGenerationMismatch);
        }
        let mounted = receipt.mounted_publication().ok_or(
            PlatformPulseLifecycleObservationProjectionDenial::MissingMountedPublication,
        )?;
        let expected_mounted_generation = receipt
            .application_publication()
            .map_or(receipt.prior_generation(), |_| receipt.active_generation());
        if mounted.generation() != expected_mounted_generation {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::MountedGenerationMismatch,
            );
        }
        let generation = receipt.active_generation().clone();
        let generation_observation =
            PlatformPulseApplicationGenerationObservation::from_generation(&generation);
        let frame = PlatformPulseMountedFrameObservation {
            diagnostic_value: mounted.frame().diagnostic_value(),
        };
        let next_visual_state = self
            .visual_state
            .after_replacement(frame.diagnostic_value)?;
        let schema_transition = match receipt.projection_schema_transitions() {
            [] => None,
            [transition] => Some(
                crate::observation_contract::schema_transition::
                    PlatformPulseProjectionSchemaTransitionObservation::from_transition(
                        transition,
                    )?,
            ),
            _ => {
                return Err(
                    PlatformPulseLifecycleObservationProjectionDenial::MultipleSchemaTransitions,
                )
            }
        };
        let outcome =
            PlatformPulseLifecycleObservation::RebindPublished(PlatformPulseReplacementPublished {
                source: PlatformPulseSourceSnapshotObservation::from_revision(source),
                predecessor_generation:
                    PlatformPulseApplicationGenerationObservation::from_generation(
                        receipt.prior_generation(),
                    ),
                active_generation: generation_observation,
                successor_frame: frame,
                actual_native_effect_count: mounted.cost_report().adapter().translated_rows(),
                schema_transition,
            });
        let envelope = self.next_envelope(outcome)?;
        self.state = PlatformPulseObservationState::Published {
            generation,
            generation_observation,
            frame,
        };
        self.visual_state = next_visual_state;
        Ok(envelope)
    }
}
