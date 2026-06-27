mod catalog;
mod closeout;
mod declaration;
mod error;
mod family_identity;
mod posture;
mod selection;
mod source_pressure;
mod stage_applicability;
mod stage_receipt_identity;

#[cfg(test)]
mod tests;

pub use closeout::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyCatalogCloseout,
    EvidenceLookupFamilyCatalogCounters, EvidenceLookupTopologyRequirementReport,
};
pub use declaration::{
    EvidenceLookupFamilyDeclaration, EvidenceLookupProductPosture,
    EvidenceLookupSpatialTouchAuthorityRequirement,
};
pub use error::{EvidenceLookupFamilyCatalogError, EvidenceLookupFamilyCatalogErrorKind};
pub use family_identity::EvidenceLookupFamilyIdentity;
pub use posture::{
    EvidenceLookupDiagnosticWitnessKind, EvidenceLookupDiagnosticWitnessShape,
    EvidenceLookupEvidenceClass, EvidenceLookupEvidenceClassSet, EvidenceLookupFamilyIndexPosture,
    EvidenceLookupFamilyIndexPostureKind, EvidenceLookupFamilyQueryPosture,
    EvidenceLookupFamilyQueryPostureKind, EvidenceLookupLowerRuntimeBoundaryEnvelopeSurface,
    EvidenceLookupProjectionConsumptionSurface, EvidenceLookupProjectionFactFamily,
    EvidenceLookupQueryImportEvidence, EvidenceLookupTopologyInputPosture,
    EvidenceLookupTopologyInputState,
};
pub use selection::{
    EvidenceLookupFamilyStageSelection, EvidenceLookupFamilyStageSelectionCounters,
};
pub use source_pressure::EvidenceLookupFamilySourceInventoryPressure;
pub use stage_applicability::EvidenceLookupStageApplicability;
pub use stage_receipt_identity::EvidenceLookupStageReceiptFamilyIdentity;

#[cfg(test)]
pub(crate) use closeout::EvidenceLookupFamilyCatalogCloseout as TestCatalogCloseout;
#[cfg(test)]
pub(crate) use declaration::EvidenceLookupFamilyDeclarationBuilder;
