use forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupFamilyQueryPostureKind {
    NotRequired,
    ImportedSupportAdmissionRequired,
    ImportedSupportPinRequired,
    ImportedProjectionConsumptionRequired,
    ImportedLowerRuntimeBoundaryEnvelopeRequired,
    ImportedTypedArtifactIdentityRequired,
    ImportedConsumerKitProofRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilyQueryPosture {
    kind: EvidenceLookupFamilyQueryPostureKind,
    imported_evidence: Option<EvidenceLookupQueryImportEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupProjectionConsumptionSurface {
    ForgeQueryProjectionConsumptionReceipt,
}

impl EvidenceLookupProjectionConsumptionSurface {
    pub const fn type_name(self) -> &'static str {
        match self {
            Self::ForgeQueryProjectionConsumptionReceipt => {
                "forge_query::facade::ProjectionConsumptionReceipt"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupProjectionFactFamily {
    SpatialTouchOperandProjection,
}

impl EvidenceLookupProjectionFactFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpatialTouchOperandProjection => "spatial-touch-operand-projection",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupLowerRuntimeBoundaryEnvelopeSurface {
    ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
}

impl EvidenceLookupLowerRuntimeBoundaryEnvelopeSurface {
    pub const fn trait_name(self) -> &'static str {
        match self {
            Self::ForgeQueryLowerRuntimeBoundaryEnvelopeSource => {
                "forge_query::facade::runtime::ForgeQueryLowerRuntimeBoundaryEnvelopeSource"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQueryImportEvidence {
    ConsumerKitSupportPin {
        support_pin: ForgeQueryGraphObligationSupportPin,
    },
    ProjectionConsumptionReceipt {
        surface: EvidenceLookupProjectionConsumptionSurface,
        fact_family: EvidenceLookupProjectionFactFamily,
        requirement_digest: String,
    },
    LowerRuntimeBoundaryEnvelope {
        surface: EvidenceLookupLowerRuntimeBoundaryEnvelopeSurface,
        requirement_digest: String,
    },
}

impl EvidenceLookupFamilyQueryPosture {
    pub(crate) const fn not_required() -> Self {
        Self {
            kind: EvidenceLookupFamilyQueryPostureKind::NotRequired,
            imported_evidence: None,
        }
    }

    pub(crate) fn imported_support_pin_required(
        support_pin: ForgeQueryGraphObligationSupportPin,
    ) -> Self {
        Self {
            kind: EvidenceLookupFamilyQueryPostureKind::ImportedSupportPinRequired,
            imported_evidence: Some(EvidenceLookupQueryImportEvidence::ConsumerKitSupportPin {
                support_pin,
            }),
        }
    }

    pub(crate) fn imported_projection_consumption_required(
        fact_family: EvidenceLookupProjectionFactFamily,
    ) -> Self {
        let surface =
            EvidenceLookupProjectionConsumptionSurface::ForgeQueryProjectionConsumptionReceipt;
        Self {
            kind: EvidenceLookupFamilyQueryPostureKind::ImportedProjectionConsumptionRequired,
            imported_evidence: Some(
                EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt {
                    surface,
                    fact_family,
                    requirement_digest: query_import_digest(&[
                        "projection-consumption",
                        surface.type_name(),
                        fact_family.as_str(),
                    ]),
                },
            ),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupFamilyQueryPostureKind {
        self.kind
    }

    pub fn imported_evidence_digest(&self) -> Option<&str> {
        self.imported_evidence
            .as_ref()
            .map(EvidenceLookupQueryImportEvidence::evidence_digest)
    }

    pub const fn imported_evidence(&self) -> Option<&EvidenceLookupQueryImportEvidence> {
        self.imported_evidence.as_ref()
    }

    pub const fn requires_query_evidence(&self) -> bool {
        !matches!(self.kind, EvidenceLookupFamilyQueryPostureKind::NotRequired)
    }
}

impl EvidenceLookupQueryImportEvidence {
    pub fn evidence_digest(&self) -> &str {
        match self {
            Self::ConsumerKitSupportPin { support_pin } => support_pin.pin_digest(),
            Self::ProjectionConsumptionReceipt {
                requirement_digest, ..
            } => requirement_digest,
            Self::LowerRuntimeBoundaryEnvelope {
                requirement_digest, ..
            } => requirement_digest,
        }
    }

    pub fn query_surface_type_name(&self) -> &'static str {
        match self {
            Self::ConsumerKitSupportPin { .. } => {
                "forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin"
            }
            Self::ProjectionConsumptionReceipt { surface, .. } => surface.type_name(),
            Self::LowerRuntimeBoundaryEnvelope { surface, .. } => surface.trait_name(),
        }
    }

    pub const fn projection_consumption_surface(
        &self,
    ) -> Option<EvidenceLookupProjectionConsumptionSurface> {
        match self {
            Self::ProjectionConsumptionReceipt { surface, .. } => Some(*surface),
            _ => None,
        }
    }

    pub const fn projection_fact_family(&self) -> Option<EvidenceLookupProjectionFactFamily> {
        match self {
            Self::ProjectionConsumptionReceipt { fact_family, .. } => Some(*fact_family),
            _ => None,
        }
    }
}

fn query_import_digest(parts: &[&str]) -> String {
    let digest_parts = parts
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<_>>();
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &digest_parts)
}
