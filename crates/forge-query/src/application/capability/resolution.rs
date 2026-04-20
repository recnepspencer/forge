use crate::application::capability::errors::{
    CapabilityAdmissionError, CapabilityAdmissionFailureClass, ForgeQueryFacadeCounters,
};
use crate::application::support::{ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityAdmissionDecision {
    descriptor: ForgeQueryCapabilityDescriptor,
    validated_config_digest: String,
    counters: ForgeQueryFacadeCounters,
    decision_digest: String,
}

impl CapabilityAdmissionDecision {
    pub(crate) fn admitted(
        descriptor: ForgeQueryCapabilityDescriptor,
        validated_config_digest: &str,
    ) -> Self {
        let counters = ForgeQueryFacadeCounters::admitted_lookup();
        let decision_digest = hash_parts(&[
            format!("family:{}", descriptor.family().as_str()),
            format!("status:{}", descriptor.status().as_str()),
            format!("section:{}", descriptor.config_section().as_str()),
            format!("owner:{}", descriptor.owner().as_str()),
            format!("reason:{}", descriptor.reason()),
            format!("validated_config:{validated_config_digest}"),
            format!("lookups:{}", counters.capability_lookup_count()),
            format!(
                "section_resolutions:{}",
                counters.configuration_section_resolution_count()
            ),
        ]);
        Self {
            descriptor,
            validated_config_digest: validated_config_digest.to_string(),
            counters,
            decision_digest,
        }
    }

    pub fn descriptor(&self) -> &ForgeQueryCapabilityDescriptor {
        &self.descriptor
    }

    pub fn counters(&self) -> &ForgeQueryFacadeCounters {
        &self.counters
    }

    pub fn validated_config_digest(&self) -> &str {
        &self.validated_config_digest
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryCapabilityResolution<C> {
    capability: C,
    admission: CapabilityAdmissionDecision,
}

impl<C> ForgeQueryCapabilityResolution<C> {
    pub(crate) fn new(capability: C, admission: CapabilityAdmissionDecision) -> Self {
        Self {
            capability,
            admission,
        }
    }

    pub fn capability(&self) -> &C {
        &self.capability
    }

    pub fn descriptor(&self) -> &ForgeQueryCapabilityDescriptor {
        self.admission.descriptor()
    }

    pub fn counters(&self) -> &ForgeQueryFacadeCounters {
        self.admission.counters()
    }

    pub fn admission(&self) -> &CapabilityAdmissionDecision {
        &self.admission
    }
}

pub(crate) fn deny_capability(
    descriptor: &ForgeQueryCapabilityDescriptor,
    config: &crate::application::config::ValidatedForgeQueryConfig,
) -> CapabilityAdmissionError {
    let failure_class = match descriptor.family() {
        ForgeQueryCapabilityFamily::DurableArtifacts => {
            CapabilityAdmissionFailureClass::DeferredCapabilityFamily
        }
        _ if !config
            .resolve_section(descriptor.config_section())
            .enabled() =>
        {
            CapabilityAdmissionFailureClass::MissingOwningSection
        }
        ForgeQueryCapabilityFamily::WorkflowOrchestration
        | ForgeQueryCapabilityFamily::HistoricalEvaluation => {
            CapabilityAdmissionFailureClass::InvalidComposedSupportPosture
        }
        _ => CapabilityAdmissionFailureClass::UnsupportedCapabilityFamily,
    };
    CapabilityAdmissionError::capability_denied(descriptor, failure_class)
}
