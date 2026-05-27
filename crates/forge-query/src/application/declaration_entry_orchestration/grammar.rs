use super::artifacts::{
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationProduct,
};
use crate::orchestration_inventory::{
    ForgeQueryOrchestrationSurfaceFamily, ForgeQueryOrchestrationSurfaceInventory,
    ForgeQueryOrchestrationSurfaceVisibility,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryOrchestrationVerbInventory {
    verbs: Vec<ForgeQueryDeclarationEntryOrchestrationVerb>,
}

impl ForgeQueryDeclarationEntryOrchestrationVerbInventory {
    pub fn current() -> Self {
        Self {
            verbs: verbs_from_orchestration_inventory(),
        }
    }

    pub fn verbs(&self) -> &[ForgeQueryDeclarationEntryOrchestrationVerb] {
        &self.verbs
    }
}

fn verbs_from_orchestration_inventory() -> Vec<ForgeQueryDeclarationEntryOrchestrationVerb> {
    ForgeQueryOrchestrationSurfaceInventory::current()
        .rows()
        .iter()
        .filter_map(|row| {
            let family = match row.family() {
                ForgeQueryOrchestrationSurfaceFamily::DeclarationEntry => {
                    ForgeQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry
                }
                ForgeQueryOrchestrationSurfaceFamily::RouteFromProgressed => {
                    ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed
                }
                ForgeQueryOrchestrationSurfaceFamily::ReceiptFromProgressed => {
                    ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed
                }
                ForgeQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed => {
                    ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed
                }
                ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget
                | ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareContext
                | ForgeQueryOrchestrationSurfaceFamily::ContinuationExecute
                | ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
                | ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
                | ForgeQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration => {
                    return None;
                }
            };
            let exposure_level = match row.visibility() {
                ForgeQueryOrchestrationSurfaceVisibility::Ordinary => {
                    ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary
                }
                ForgeQueryOrchestrationSurfaceVisibility::Checked => {
                    ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked
                }
                ForgeQueryOrchestrationSurfaceVisibility::ProofVisible => {
                    ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible
                }
                ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome => return None,
            };
            Some(match family {
                ForgeQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry => {
                    ForgeQueryDeclarationEntryOrchestrationVerb::generic_declaration_entry(
                        row.public_name(),
                        exposure_level,
                    )
                }
                ForgeQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed => {
                    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
                        row.public_name(),
                        family,
                        exposure_level,
                        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
                        row.canonical_base_name(),
                    )
                }
                ForgeQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed => {
                    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
                        row.public_name(),
                        family,
                        exposure_level,
                        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
                        row.canonical_base_name(),
                    )
                }
                ForgeQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed => {
                    ForgeQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
                        row.public_name(),
                        family,
                        exposure_level,
                        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
                        row.canonical_base_name(),
                    )
                }
            })
        })
        .collect()
}
