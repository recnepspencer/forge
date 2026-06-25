use crate::runtime::live_view::target_binding_stale_denial;
use crate::runtime::{
    WorthUiLiveViewInteractionIntentReceipt, WorthUiLiveViewReadinessPosture,
    WorthUiMountedInteractionNodeReceipt, WorthUiRuntimeHost,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewInteractionActivationEligibleReceipt {
    interaction: WorthUiLiveViewInteractionIntentReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewInteractionActivationDenial {
    ReadinessDenied {
        interaction_id: String,
        readiness_digest: u64,
        posture: WorthUiLiveViewReadinessPosture,
    },
    StaleTargetBinding {
        interaction_id: String,
        readiness_digest: u64,
        slot_name: String,
        surface_id: String,
        expected_component_id: String,
        actual_component_id: Option<String>,
    },
    ContextSuppressed {
        interaction_id: String,
        interaction_digest: u64,
        context_digest: u64,
        disabled: bool,
        inert: bool,
    },
}

impl WorthUiRuntimeHost {
    pub(crate) fn activate_live_view_interaction(
        &self,
        interaction: &WorthUiLiveViewInteractionIntentReceipt,
    ) -> Result<
        WorthUiLiveViewInteractionActivationEligibleReceipt,
        WorthUiLiveViewInteractionActivationDenial,
    > {
        if let Some(denial) =
            target_binding_stale_denial(self, interaction.readiness().target_binding())
        {
            return Err(stale_activation_denial(interaction, denial));
        }
        if !interaction.readiness().posture().is_enabled() {
            return Err(
                WorthUiLiveViewInteractionActivationDenial::ReadinessDenied {
                    interaction_id: interaction.interaction_id().to_owned(),
                    readiness_digest: interaction.readiness().readiness_digest(),
                    posture: interaction.readiness().posture(),
                },
            );
        }
        Ok(WorthUiLiveViewInteractionActivationEligibleReceipt {
            interaction: interaction.clone(),
        })
    }

    pub fn activate_mounted_live_view_interaction(
        &self,
        interaction: &WorthUiMountedInteractionNodeReceipt,
    ) -> Result<
        WorthUiLiveViewInteractionActivationEligibleReceipt,
        WorthUiLiveViewInteractionActivationDenial,
    > {
        if let Some(denial) = target_binding_stale_denial(
            self,
            interaction.interaction().readiness().target_binding(),
        ) {
            return Err(stale_activation_denial(interaction.interaction(), denial));
        }
        if let Some(context) = interaction.node_context() {
            if context.suppresses_interaction() {
                return Err(
                    WorthUiLiveViewInteractionActivationDenial::ContextSuppressed {
                        interaction_id: interaction.interaction().interaction_id().to_owned(),
                        interaction_digest: interaction.interaction().interaction_intent_digest(),
                        context_digest: context.receipt_digest(),
                        disabled: context.disabled(),
                        inert: context.inert(),
                    },
                );
            }
        }
        self.activate_live_view_interaction(interaction.interaction())
    }
}

fn stale_activation_denial(
    interaction: &WorthUiLiveViewInteractionIntentReceipt,
    denial: crate::runtime::WorthUiLiveViewDenial,
) -> WorthUiLiveViewInteractionActivationDenial {
    match denial {
        crate::runtime::WorthUiLiveViewDenial::StaleTargetBinding {
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id,
        } => WorthUiLiveViewInteractionActivationDenial::StaleTargetBinding {
            interaction_id: interaction.interaction_id().to_owned(),
            readiness_digest: interaction.readiness().readiness_digest(),
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id,
        },
        _ => unreachable!("target_binding_stale_denial only returns stale target denials"),
    }
}

impl WorthUiLiveViewInteractionActivationEligibleReceipt {
    pub fn interaction(&self) -> &WorthUiLiveViewInteractionIntentReceipt {
        &self.interaction
    }
}
