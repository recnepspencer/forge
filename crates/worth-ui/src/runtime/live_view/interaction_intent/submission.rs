use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiLiveViewEmittedPayload, WorthUiLiveViewInteractionActivationEligibleReceipt,
    WorthUiLiveViewInteractionIntentReceipt, WorthUiLiveViewPayloadField,
    WorthUiLiveViewPayloadShape, WorthUiLiveViewValuePresencePosture, WorthUiRuntimeHost,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewInteractionSubmissionReceipt {
    interaction: WorthUiLiveViewInteractionIntentReceipt,
    emitted_payload: WorthUiLiveViewEmittedPayload,
    submission_digest: u64,
}

impl WorthUiRuntimeHost {
    pub fn submit_live_view_interaction(
        &self,
        eligible: WorthUiLiveViewInteractionActivationEligibleReceipt,
    ) -> WorthUiLiveViewInteractionSubmissionReceipt {
        let interaction = eligible.interaction();
        let fields = interaction
            .readiness()
            .required_bindings()
            .iter()
            .filter(|presence| presence.posture() == WorthUiLiveViewValuePresencePosture::Present)
            .filter_map(|presence| {
                self.live_view_state_value(presence.binding()).map(|value| {
                    WorthUiLiveViewPayloadField::new(presence.binding().binding_id(), value.clone())
                })
            })
            .collect::<Vec<_>>();
        let emitted_payload = match interaction.payload_projection().shape() {
            WorthUiLiveViewPayloadShape::PayloadValues => {
                WorthUiLiveViewEmittedPayload::payload(fields)
            }
            WorthUiLiveViewPayloadShape::DataPayloadValues => {
                WorthUiLiveViewEmittedPayload::data_payload(fields)
            }
            WorthUiLiveViewPayloadShape::Unsupported(_) => {
                unreachable!("payload projection shape was admitted before interaction submission")
            }
        };
        WorthUiLiveViewInteractionSubmissionReceipt::new(interaction.clone(), emitted_payload)
    }
}

impl WorthUiLiveViewInteractionSubmissionReceipt {
    fn new(
        interaction: WorthUiLiveViewInteractionIntentReceipt,
        emitted_payload: WorthUiLiveViewEmittedPayload,
    ) -> Self {
        let submission_digest = digest_parts([
            interaction.interaction_id(),
            interaction.interaction_intent_digest().to_string().as_str(),
            emitted_payload.display_shape().as_str(),
        ]);
        Self {
            interaction,
            emitted_payload,
            submission_digest,
        }
    }

    pub fn interaction(&self) -> &WorthUiLiveViewInteractionIntentReceipt {
        &self.interaction
    }

    pub fn emitted_payload(&self) -> &WorthUiLiveViewEmittedPayload {
        &self.emitted_payload
    }

    pub fn submission_digest(&self) -> u64 {
        self.submission_digest
    }
}
