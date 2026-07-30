use super::{WorthUiHostCapability, WorthUiHostCapabilityPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostCapabilityObservationGeneration {
    value: u64,
}

impl WorthUiHostCapabilityObservationGeneration {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn as_u64(self) -> u64 {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHostCapabilityReport {
    observation_generation: WorthUiHostCapabilityObservationGeneration,
    posture: WorthUiHostCapabilityPosture,
    observed_capabilities: Box<[WorthUiHostCapability]>,
}

impl WorthUiHostCapabilityReport {
    pub fn available(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::Available, capabilities)
    }

    pub fn missing(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::Missing, capabilities)
    }

    pub fn ambiguous(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::Ambiguous, capabilities)
    }

    pub fn diagnostic_only(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::DiagnosticOnly, capabilities)
    }

    fn new(
        posture: WorthUiHostCapabilityPosture,
        mut observed_capabilities: Vec<WorthUiHostCapability>,
    ) -> Self {
        observed_capabilities.sort_by_key(|capability| capability.as_str());
        observed_capabilities.dedup();

        Self {
            observation_generation: WorthUiHostCapabilityObservationGeneration::new(0),
            posture,
            observed_capabilities: observed_capabilities.into_boxed_slice(),
        }
    }

    pub fn with_observation_generation(
        mut self,
        observation_generation: WorthUiHostCapabilityObservationGeneration,
    ) -> Self {
        self.observation_generation = observation_generation;
        self
    }

    pub fn observation_generation(&self) -> WorthUiHostCapabilityObservationGeneration {
        self.observation_generation
    }

    pub fn posture(&self) -> WorthUiHostCapabilityPosture {
        self.posture
    }

    pub fn observed_capabilities(&self) -> &[WorthUiHostCapability] {
        &self.observed_capabilities
    }

    pub fn profile_identity_digest(&self) -> u64 {
        let posture_label = match self.posture {
            WorthUiHostCapabilityPosture::Available => "available",
            WorthUiHostCapabilityPosture::Missing => "missing",
            WorthUiHostCapabilityPosture::Ambiguous => "ambiguous",
            WorthUiHostCapabilityPosture::DiagnosticOnly => "diagnostic-only",
        };
        self.observed_capabilities.iter().fold(
            stable_text_digest("worth-ui-host-capability-report")
                ^ stable_text_digest(posture_label).rotate_left(7),
            |digest, capability| digest ^ stable_text_digest(capability.as_str()).rotate_left(17),
        )
    }

    pub fn supports(&self, capability: WorthUiHostCapability) -> bool {
        self.observed_capabilities.contains(&capability)
    }
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}

#[cfg(test)]
mod tests {
    use super::{WorthUiHostCapability, WorthUiHostCapabilityReport};

    #[test]
    fn profile_digest_uses_constructor_canonical_order_without_input_order_leakage() {
        let first = WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::Ime,
            WorthUiHostCapability::Accessibility,
            WorthUiHostCapability::Ime,
        ]);
        let second = WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::Accessibility,
            WorthUiHostCapability::Ime,
        ]);

        assert_eq!(first, second);
        assert_eq!(
            first.profile_identity_digest(),
            second.profile_identity_digest()
        );
    }
}
