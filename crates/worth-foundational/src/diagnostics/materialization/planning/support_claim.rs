use crate::diagnostics::{
    FoundationalDiagnosticAvailability, FoundationalDiagnosticMaterializationDenial,
};
use crate::profiles::{
    CertificationPostureProfile, DiagnosticRichnessProfile, FoundationalProfileSet,
    RetentionDeliveryProfile, SupportPostureProfile,
};

use super::super::vocabulary::FoundationalDiagnosticSupportClaimStrength;
use super::inputs::FoundationalDiagnosticSupportInput;

pub(super) fn admit_support_claim(
    input: &FoundationalDiagnosticSupportInput,
    profile: FoundationalProfileSet,
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    match input.support_claim_strength {
        FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly => Ok(()),
        FoundationalDiagnosticSupportClaimStrength::DurableSupportReady => {
            if profile.support_posture() == SupportPostureProfile::InternalOnly {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::InternalSupportCannotClaimDurableSupport,
                );
            }
            admit_visible_evidence(input, profile)
        }
        FoundationalDiagnosticSupportClaimStrength::CertifiedSupportReady => {
            if profile.support_posture() == SupportPostureProfile::InternalOnly {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::InternalSupportCannotClaimCertifiedSupport,
                );
            }
            if profile.certification_posture() != CertificationPostureProfile::ProductionCertified {
                return Err(
                    FoundationalDiagnosticMaterializationDenial::CertifiedSupportRequiresProductionCertifiedProfile,
                );
            }
            admit_visible_evidence(input, profile)
        }
    }
}

fn admit_visible_evidence(
    input: &FoundationalDiagnosticSupportInput,
    profile: FoundationalProfileSet,
) -> Result<(), FoundationalDiagnosticMaterializationDenial> {
    if matches!(
        profile.retention_delivery(),
        RetentionDeliveryProfile::Ephemeral
    ) || !matches!(
        input.availability.availability(),
        FoundationalDiagnosticAvailability::RetainedHot
            | FoundationalDiagnosticAvailability::DeferredCold
            | FoundationalDiagnosticAvailability::Reconstructable
    ) {
        return Err(
            FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleEvidence,
        );
    }
    if !has_visible_rows_for_profile(
        &input.required_rows,
        &input.standard_rows,
        &input.forensic_rows,
        profile,
    ) {
        return Err(
            FoundationalDiagnosticMaterializationDenial::DurableSupportRequiresVisibleRowsAtChosenRichness,
        );
    }

    Ok(())
}

fn has_visible_rows_for_profile(
    required_rows: &[crate::diagnostics::FoundationalDiagnosticRow],
    standard_rows: &[crate::diagnostics::FoundationalDiagnosticRow],
    forensic_rows: &[crate::diagnostics::FoundationalDiagnosticRow],
    profile: FoundationalProfileSet,
) -> bool {
    !required_rows.is_empty()
        || (profile.diagnostic_richness() != DiagnosticRichnessProfile::OperationalMinimal
            && !standard_rows.is_empty())
        || (profile.diagnostic_richness() == DiagnosticRichnessProfile::Forensic
            && !forensic_rows.is_empty())
}
