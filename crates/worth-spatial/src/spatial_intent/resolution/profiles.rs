#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialThresholdPosture {
    Strict,
    Balanced,
    Generous,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPreviewRichness {
    Compact,
    Standard,
    HighFidelity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationPosture {
    AskFirst,
    PreserveAmbiguity,
    PreferSnap,
    PreferHostRelationships,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialIntentPolicyProfile {
    name: &'static str,
    proximity_posture: SpatialThresholdPosture,
    alignment_posture: SpatialThresholdPosture,
    arbitration_posture: SpatialArbitrationPosture,
    preview_richness: SpatialPreviewRichness,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialIntentPolicyProfileOverride {
    name: Option<&'static str>,
    proximity_posture: Option<SpatialThresholdPosture>,
    alignment_posture: Option<SpatialThresholdPosture>,
    arbitration_posture: Option<SpatialArbitrationPosture>,
    preview_richness: Option<SpatialPreviewRichness>,
}

impl SpatialIntentPolicyProfileOverride {
    pub const fn new() -> Self {
        Self {
            name: None,
            proximity_posture: None,
            alignment_posture: None,
            arbitration_posture: None,
            preview_richness: None,
        }
    }

    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub const fn with_proximity_posture(mut self, posture: SpatialThresholdPosture) -> Self {
        self.proximity_posture = Some(posture);
        self
    }

    pub const fn with_alignment_posture(mut self, posture: SpatialThresholdPosture) -> Self {
        self.alignment_posture = Some(posture);
        self
    }

    pub const fn with_arbitration_posture(mut self, posture: SpatialArbitrationPosture) -> Self {
        self.arbitration_posture = Some(posture);
        self
    }

    pub const fn with_preview_richness(mut self, richness: SpatialPreviewRichness) -> Self {
        self.preview_richness = Some(richness);
        self
    }
}

impl SpatialIntentPolicyProfile {
    pub const fn conservative_exact_modeling() -> Self {
        Self {
            name: "conservative_exact_modeling",
            proximity_posture: SpatialThresholdPosture::Strict,
            alignment_posture: SpatialThresholdPosture::Strict,
            arbitration_posture: SpatialArbitrationPosture::AskFirst,
            preview_richness: SpatialPreviewRichness::Standard,
        }
    }

    pub const fn bim_host_friendly() -> Self {
        Self {
            name: "bim_host_friendly",
            proximity_posture: SpatialThresholdPosture::Balanced,
            alignment_posture: SpatialThresholdPosture::Balanced,
            arbitration_posture: SpatialArbitrationPosture::PreferHostRelationships,
            preview_richness: SpatialPreviewRichness::Standard,
        }
    }

    pub const fn ask_first_arbitration() -> Self {
        Self {
            name: "ask_first_arbitration",
            proximity_posture: SpatialThresholdPosture::Balanced,
            alignment_posture: SpatialThresholdPosture::Balanced,
            arbitration_posture: SpatialArbitrationPosture::AskFirst,
            preview_richness: SpatialPreviewRichness::Standard,
        }
    }

    pub const fn aggressive_snap() -> Self {
        Self {
            name: "aggressive_snap",
            proximity_posture: SpatialThresholdPosture::Generous,
            alignment_posture: SpatialThresholdPosture::Balanced,
            arbitration_posture: SpatialArbitrationPosture::PreferSnap,
            preview_richness: SpatialPreviewRichness::Standard,
        }
    }

    pub const fn high_fidelity_preview() -> Self {
        Self {
            name: "high_fidelity_preview",
            proximity_posture: SpatialThresholdPosture::Balanced,
            alignment_posture: SpatialThresholdPosture::Balanced,
            arbitration_posture: SpatialArbitrationPosture::AskFirst,
            preview_richness: SpatialPreviewRichness::HighFidelity,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn proximity_posture(&self) -> SpatialThresholdPosture {
        self.proximity_posture
    }

    pub fn alignment_posture(&self) -> SpatialThresholdPosture {
        self.alignment_posture
    }

    pub fn arbitration_posture(&self) -> SpatialArbitrationPosture {
        self.arbitration_posture
    }

    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.preview_richness
    }

    pub fn derive(
        self,
        override_spec: SpatialIntentPolicyProfileOverride,
    ) -> SpatialIntentPolicyProfile {
        SpatialIntentPolicyProfile {
            name: override_spec.name.unwrap_or(self.name),
            proximity_posture: override_spec
                .proximity_posture
                .unwrap_or(self.proximity_posture),
            alignment_posture: override_spec
                .alignment_posture
                .unwrap_or(self.alignment_posture),
            arbitration_posture: override_spec
                .arbitration_posture
                .unwrap_or(self.arbitration_posture),
            preview_richness: override_spec
                .preview_richness
                .unwrap_or(self.preview_richness),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SpatialArbitrationPosture, SpatialIntentPolicyProfile, SpatialIntentPolicyProfileOverride,
        SpatialPreviewRichness, SpatialThresholdPosture,
    };

    #[test]
    fn policy_profile_derive_preserves_base_and_overrides_named_fields() {
        let derived = SpatialIntentPolicyProfile::bim_host_friendly().derive(
            SpatialIntentPolicyProfileOverride::new()
                .with_name("bim_host_friendly_high_fidelity")
                .with_preview_richness(SpatialPreviewRichness::HighFidelity)
                .with_arbitration_posture(SpatialArbitrationPosture::AskFirst),
        );

        assert_eq!(derived.name(), "bim_host_friendly_high_fidelity");
        assert_eq!(
            derived.proximity_posture(),
            SpatialThresholdPosture::Balanced
        );
        assert_eq!(
            derived.alignment_posture(),
            SpatialThresholdPosture::Balanced
        );
        assert_eq!(
            derived.preview_richness(),
            SpatialPreviewRichness::HighFidelity
        );
        assert_eq!(
            derived.arbitration_posture(),
            SpatialArbitrationPosture::AskFirst
        );
    }
}
