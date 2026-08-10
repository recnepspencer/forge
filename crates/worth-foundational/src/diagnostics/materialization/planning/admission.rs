use crate::diagnostics::{
    evaluate_diagnostic_materialization_legality, FoundationalDiagnosticAbsenceCause,
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticAvailability,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticMaterializationDenial,
};

use super::super::vocabulary::{
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticAssemblyDebtClass,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticSurfaceAvailability,
};

pub(super) fn admit_common_materialization(
    availability: FoundationalDiagnosticSurfaceAvailability,
    partiality: &FoundationalDiagnosticPartiality,
    assembly_debts: &[FoundationalDiagnosticAssemblyDebt],
    delivery_class: FoundationalDiagnosticDeliveryClass,
    kind: FoundationalDiagnosticArtifactKind,
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    validate_availability_shape(availability)?;
    validate_materialization_legality(availability, delivery_class, kind)?;
    validate_partiality(partiality)?;
    validate_named_assembly_debts(assembly_debts)
}

fn validate_availability_shape(
    availability: FoundationalDiagnosticSurfaceAvailability,
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    match availability.availability() {
        FoundationalDiagnosticAvailability::RetainedHot
        | FoundationalDiagnosticAvailability::DeferredCold
        | FoundationalDiagnosticAvailability::Reconstructable => {
            if availability.absence_cause().is_some() {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::UnavailableAvailabilityRequiresCause,
                );
            }
        }
        FoundationalDiagnosticAvailability::Redacted => {
            if availability.absence_cause() != Some(FoundationalDiagnosticAbsenceCause::Redacted) {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::RedactedAvailabilityMustUseRedactedCause,
                );
            }
        }
        FoundationalDiagnosticAvailability::Unavailable => {
            if availability.absence_cause().is_none() {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::UnavailableAvailabilityRequiresCause,
                );
            }
        }
    }

    Ok(())
}

fn validate_materialization_legality(
    availability: FoundationalDiagnosticSurfaceAvailability,
    delivery_class: FoundationalDiagnosticDeliveryClass,
    kind: FoundationalDiagnosticArtifactKind,
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    evaluate_diagnostic_materialization_legality(kind, delivery_class, availability.availability())
        .map_err(|_| {
            FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleEvidence
        })
}

fn validate_partiality(
    partiality: &FoundationalDiagnosticPartiality,
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    match partiality {
        FoundationalDiagnosticPartiality::Complete => {}
        FoundationalDiagnosticPartiality::PartialWithNamedGaps(gaps) => {
            if gaps.is_empty() {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::PartialityRequiresNamedGaps,
                );
            }
        }
    }

    Ok(())
}

fn validate_named_assembly_debts(
    assembly_debts: &[FoundationalDiagnosticAssemblyDebt],
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    for debt in assembly_debts {
        match debt.class() {
            FoundationalDiagnosticAssemblyDebtClass::RowScanFallback if debt.count() == 0 => {
                return Err(FoundationalDiagnosticMaterializationDenial::RowScanFallbackMustRemainExplicitDebt);
            }
            FoundationalDiagnosticAssemblyDebtClass::WholeViewFallback if debt.count() == 0 => {
                return Err(FoundationalDiagnosticMaterializationDenial::WholeViewFallbackMustRemainExplicitDebt);
            }
            FoundationalDiagnosticAssemblyDebtClass::RepeatedRediscovery if debt.count() == 0 => {
                return Err(FoundationalDiagnosticMaterializationDenial::RepeatedRediscoveryMustRemainExplicitDebt);
            }
            _ => {}
        }
    }

    Ok(())
}

impl From<FoundationalDiagnosticSurfaceAvailability> for FoundationalDiagnosticAvailability {
    fn from(value: FoundationalDiagnosticSurfaceAvailability) -> Self {
        value.availability()
    }
}
