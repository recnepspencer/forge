use std::sync::Arc;

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityValidityTimeline;

pub(in crate::domain_computation) struct WorthQueryCapabilityCurrentnessAuthority {
    capability_authority_identity: Arc<str>,
    timeline: ApplicationCapabilityValidityTimeline,
    sampled_value: AspectValue,
}

impl WorthQueryCapabilityCurrentnessAuthority {
    pub(super) fn new(
        capability_authority_identity: Arc<str>,
        timeline: ApplicationCapabilityValidityTimeline,
        sampled_value: AspectValue,
    ) -> Self {
        Self {
            capability_authority_identity,
            timeline,
            sampled_value,
        }
    }

    pub(super) fn capability_authority_identity(&self) -> &str {
        &self.capability_authority_identity
    }

    pub(super) const fn timeline(&self) -> ApplicationCapabilityValidityTimeline {
        self.timeline
    }

    pub(super) const fn sampled_value(&self) -> &AspectValue {
        &self.sampled_value
    }

    pub(super) fn replace_sample(
        &mut self,
        timeline: ApplicationCapabilityValidityTimeline,
        sampled_value: AspectValue,
    ) -> bool {
        if self.timeline != timeline {
            return false;
        }
        self.sampled_value = sampled_value;
        true
    }
}
