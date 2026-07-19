use super::artifacts::{
    WorthQueryDeclarationEntryOrchestrationExposureLevel,
    WorthQueryDeclarationEntryOrchestrationProduct,
};
use crate::orchestration_inventory::{
    WorthQueryOrchestrationSurfaceFamily, WorthQueryOrchestrationSurfaceInventory,
    WorthQueryOrchestrationSurfaceVisibility,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationVerbFamily {
    GenericDeclarationEntry,
    RouteFromProgressed,
    ReceiptFromProgressed,
    EnvelopeFromProgressed,
}

impl WorthQueryDeclarationEntryOrchestrationVerbFamily {
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
pub enum WorthQueryDeclarationEntryOrchestrationVerbCeiling {
    Envelope,
}

impl WorthQueryDeclarationEntryOrchestrationVerbCeiling {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Envelope => "envelope",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationEntryOrchestrationVerb {
    public_name: &'static str,
    family: WorthQueryDeclarationEntryOrchestrationVerbFamily,
    exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel,
    ceiling: WorthQueryDeclarationEntryOrchestrationVerbCeiling,
    product: WorthQueryDeclarationEntryOrchestrationProduct,
    canonical_base_name: &'static str,
}

impl WorthQueryDeclarationEntryOrchestrationVerb {
    const fn generic_declaration_entry(
        public_name: &'static str,
        exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel,
    ) -> Self {
        Self {
            public_name,
            family: WorthQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry,
            exposure_level,
            ceiling: WorthQueryDeclarationEntryOrchestrationVerbCeiling::Envelope,
            product: WorthQueryDeclarationEntryOrchestrationProduct::Envelope,
            canonical_base_name: "orchestrate_declaration_entry",
        }
    }

    const fn product_from_progressed(
        public_name: &'static str,
        family: WorthQueryDeclarationEntryOrchestrationVerbFamily,
        exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel,
        product: WorthQueryDeclarationEntryOrchestrationProduct,
        canonical_base_name: &'static str,
    ) -> Self {
        let ceiling = match product {
            WorthQueryDeclarationEntryOrchestrationProduct::RoutePlan => {
                WorthQueryDeclarationEntryOrchestrationVerbCeiling::Envelope
            }
            WorthQueryDeclarationEntryOrchestrationProduct::Receipt => {
                WorthQueryDeclarationEntryOrchestrationVerbCeiling::Envelope
            }
            WorthQueryDeclarationEntryOrchestrationProduct::Envelope => {
                WorthQueryDeclarationEntryOrchestrationVerbCeiling::Envelope
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

    pub fn family(&self) -> WorthQueryDeclarationEntryOrchestrationVerbFamily {
        self.family
    }

    pub fn exposure_level(&self) -> WorthQueryDeclarationEntryOrchestrationExposureLevel {
        self.exposure_level
    }

    pub fn ceiling(&self) -> WorthQueryDeclarationEntryOrchestrationVerbCeiling {
        self.ceiling
    }

    pub fn product(&self) -> WorthQueryDeclarationEntryOrchestrationProduct {
        self.product
    }

    pub fn canonical_base_name(&self) -> &'static str {
        self.canonical_base_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationEntryOrchestrationVerbInventory {
    verbs: Vec<WorthQueryDeclarationEntryOrchestrationVerb>,
}

impl WorthQueryDeclarationEntryOrchestrationVerbInventory {
    pub fn current() -> Self {
        Self {
            verbs: verbs_from_orchestration_inventory(),
        }
    }

    pub fn verbs(&self) -> &[WorthQueryDeclarationEntryOrchestrationVerb] {
        &self.verbs
    }
}

fn verbs_from_orchestration_inventory() -> Vec<WorthQueryDeclarationEntryOrchestrationVerb> {
    WorthQueryOrchestrationSurfaceInventory::current()
        .rows()
        .iter()
        .filter_map(|row| {
            let family = match row.family() {
                WorthQueryOrchestrationSurfaceFamily::DeclarationEntry => {
                    WorthQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry
                }
                WorthQueryOrchestrationSurfaceFamily::RouteFromProgressed => {
                    WorthQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed
                }
                WorthQueryOrchestrationSurfaceFamily::ReceiptFromProgressed => {
                    WorthQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed
                }
                WorthQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed => {
                    WorthQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed
                }
                WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget
                | WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareContext
                | WorthQueryOrchestrationSurfaceFamily::ContinuationExecute
                | WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
                | WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
                | WorthQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration
                | WorthQueryOrchestrationSurfaceFamily::RecoveryBoundary => {
                    return None;
                }
            };
            let exposure_level = match row.visibility() {
                WorthQueryOrchestrationSurfaceVisibility::Ordinary => {
                    WorthQueryDeclarationEntryOrchestrationExposureLevel::Ordinary
                }
                WorthQueryOrchestrationSurfaceVisibility::Checked => {
                    WorthQueryDeclarationEntryOrchestrationExposureLevel::Checked
                }
                WorthQueryOrchestrationSurfaceVisibility::ProofVisible => {
                    WorthQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible
                }
                WorthQueryOrchestrationSurfaceVisibility::OrdinaryOutcome => return None,
            };
            Some(match family {
                WorthQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry => {
                    WorthQueryDeclarationEntryOrchestrationVerb::generic_declaration_entry(
                        row.public_name(),
                        exposure_level,
                    )
                }
                WorthQueryDeclarationEntryOrchestrationVerbFamily::RouteFromProgressed => {
                    WorthQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
                        row.public_name(),
                        family,
                        exposure_level,
                        WorthQueryDeclarationEntryOrchestrationProduct::RoutePlan,
                        row.canonical_base_name(),
                    )
                }
                WorthQueryDeclarationEntryOrchestrationVerbFamily::ReceiptFromProgressed => {
                    WorthQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
                        row.public_name(),
                        family,
                        exposure_level,
                        WorthQueryDeclarationEntryOrchestrationProduct::Receipt,
                        row.canonical_base_name(),
                    )
                }
                WorthQueryDeclarationEntryOrchestrationVerbFamily::EnvelopeFromProgressed => {
                    WorthQueryDeclarationEntryOrchestrationVerb::product_from_progressed(
                        row.public_name(),
                        family,
                        exposure_level,
                        WorthQueryDeclarationEntryOrchestrationProduct::Envelope,
                        row.canonical_base_name(),
                    )
                }
            })
        })
        .collect()
}
