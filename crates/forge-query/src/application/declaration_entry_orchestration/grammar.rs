use super::artifacts::ForgeQueryDeclarationEntryOrchestrationExposureLevel;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationVerbFamily {
    GenericDeclarationEntry,
}

impl ForgeQueryDeclarationEntryOrchestrationVerbFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GenericDeclarationEntry => "generic_declaration_entry",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationVerbCeiling {
    Envelope,
}

impl ForgeQueryDeclarationEntryOrchestrationVerbCeiling {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Envelope => "envelope",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryOrchestrationVerb {
    public_name: &'static str,
    family: ForgeQueryDeclarationEntryOrchestrationVerbFamily,
    exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ceiling: ForgeQueryDeclarationEntryOrchestrationVerbCeiling,
    canonical_base_name: &'static str,
}

impl ForgeQueryDeclarationEntryOrchestrationVerb {
    const fn generic_declaration_entry(
        public_name: &'static str,
        exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ) -> Self {
        Self {
            public_name,
            family: ForgeQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry,
            exposure_level,
            ceiling: ForgeQueryDeclarationEntryOrchestrationVerbCeiling::Envelope,
            canonical_base_name: "orchestrate_declaration_entry",
        }
    }

    pub fn public_name(&self) -> &'static str {
        self.public_name
    }

    pub fn family(&self) -> ForgeQueryDeclarationEntryOrchestrationVerbFamily {
        self.family
    }

    pub fn exposure_level(&self) -> ForgeQueryDeclarationEntryOrchestrationExposureLevel {
        self.exposure_level
    }

    pub fn ceiling(&self) -> ForgeQueryDeclarationEntryOrchestrationVerbCeiling {
        self.ceiling
    }

    pub fn canonical_base_name(&self) -> &'static str {
        self.canonical_base_name
    }
}

const CURRENT_VERBS: [ForgeQueryDeclarationEntryOrchestrationVerb; 3] = [
    ForgeQueryDeclarationEntryOrchestrationVerb::generic_declaration_entry(
        "orchestrate_declaration_entry",
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::generic_declaration_entry(
        "orchestrate_declaration_entry_checked",
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::generic_declaration_entry(
        "orchestrate_declaration_entry_proof",
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
    ),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryOrchestrationVerbInventory {
    verbs: &'static [ForgeQueryDeclarationEntryOrchestrationVerb],
}

impl ForgeQueryDeclarationEntryOrchestrationVerbInventory {
    pub fn current() -> Self {
        Self {
            verbs: &CURRENT_VERBS,
        }
    }

    pub fn verbs(&self) -> &[ForgeQueryDeclarationEntryOrchestrationVerb] {
        self.verbs
    }
}
