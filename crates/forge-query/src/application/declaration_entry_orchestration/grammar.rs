use super::artifacts::{
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationProduct,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationVerbFamily {
    GenericDeclarationEntry,
    RouteFromProgressed,
    ReceiptFromProgressed,
    EnvelopeFromProgressed,
}

impl ForgeQueryDeclarationEntryOrchestrationVerbFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GenericDeclarationEntry => "generic_declaration_entry",
            Self::RouteFromProgressed => "route_from_progressed",
            Self::ReceiptFromProgressed => "receipt_from_progressed",
            Self::EnvelopeFromProgressed => "envelope_from_progressed",
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
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
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
            product: ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
            canonical_base_name: "orchestrate_declaration_entry",
        }
    }

    const fn product_from_progressed(
        public_name: &'static str,
        family: ForgeQueryDeclarationEntryOrchestrationVerbFamily,
        exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
        product: ForgeQueryDeclarationEntryOrchestrationProduct,
        canonical_base_name: &'static str,
    ) -> Self {
        let ceiling = match product {
            ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => {
                ForgeQueryDeclarationEntryOrchestrationVerbCeiling::Envelope
            }
            ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => {
                ForgeQueryDeclarationEntryOrchestrationVerbCeiling::Envelope
            }
            ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => {
                ForgeQueryDeclarationEntryOrchestrationVerbCeiling::Envelope
            }
        };
        Self {
            public_name,
            family,
            exposure_level,
            ceiling,
            product,
            canonical_base_name,
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

    pub fn product(&self) -> ForgeQueryDeclarationEntryOrchestrationProduct {
        self.product
    }

    pub fn canonical_base_name(&self) -> &'static str {
        self.canonical_base_name
    }
}

const CURRENT_VERBS: [ForgeQueryDeclarationEntryOrchestrationVerb; 21] = [
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
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_routes_from_progressed",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
        "orchestrate_routes_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_routes_from_progressed_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
        "orchestrate_routes_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_routes_from_progressed_checked",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
        "orchestrate_routes_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_routes_from_progressed_checked_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
        "orchestrate_routes_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_routes_from_progressed_proof",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
        "orchestrate_routes_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_routes_from_progressed_proof_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
        "orchestrate_routes_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_receipt_from_progressed",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
        "orchestrate_receipt_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_receipt_from_progressed_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
        "orchestrate_receipt_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_receipt_from_progressed_checked",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
        "orchestrate_receipt_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_receipt_from_progressed_checked_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
        "orchestrate_receipt_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_receipt_from_progressed_proof",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
        "orchestrate_receipt_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_receipt_from_progressed_proof_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
        "orchestrate_receipt_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_envelope_from_progressed",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
        "orchestrate_envelope_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_envelope_from_progressed_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
        "orchestrate_envelope_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_envelope_from_progressed_checked",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
        "orchestrate_envelope_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_envelope_from_progressed_checked_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
        "orchestrate_envelope_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_envelope_from_progressed_proof",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
        "orchestrate_envelope_from_progressed",
    ),
    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
        "orchestrate_envelope_from_progressed_proof_with_intent",
        ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
        "orchestrate_envelope_from_progressed",
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
