use super::model::{
    FoundationalBoundaryMaterializationAttachment, FoundationalBoundaryMaterializationDecisionRow,
    FoundationalBoundaryMaterializationInput, FoundationalBoundaryMaterializationPlan,
};
use super::vocabulary::{
    FoundationalBoundaryAttachmentPoint, FoundationalBoundaryAvailability,
    FoundationalBoundaryDecisionCause, FoundationalBoundaryDecisionSubject,
    FoundationalBoundaryDeliveryClass, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource, FoundationalBoundaryPlanningDenial,
    FoundationalBoundarySurfaceDisposition, FoundationalBoundarySurfaceDispositionDenial,
    FoundationalBoundarySurfaceDispositionLegality,
};
use crate::boundary_artifacts::FoundationalBoundaryArtifactRole;
use crate::profiles::{
    DiagnosticRichnessProfile, MaterializedFoundationalProfileSet, RetentionDeliveryProfile,
};

pub(crate) fn build_boundary_materialization_plan<Surface>(
    input: FoundationalBoundaryMaterializationInput<Surface>,
    seam: FoundationalBoundaryMaterializationSeam,
    profile: MaterializedFoundationalProfileSet,
) -> Result<FoundationalBoundaryMaterializationPlan<Surface>, FoundationalBoundaryPlanningDenial> {
    validate_source_and_seam(input.source(), seam)?;
    let disposition = derive_disposition(&input, seam)?;
    let attachments = derive_attachments(&input, &profile);
    let decision_rows = derive_decision_rows(&input, seam, &disposition, &attachments);

    Ok(FoundationalBoundaryMaterializationPlan::new(
        input,
        seam,
        disposition,
        attachments,
        decision_rows,
        profile,
    ))
}

pub fn evaluate_boundary_surface_disposition_legality(
    delivery_class: FoundationalBoundaryDeliveryClass,
    availability: FoundationalBoundaryAvailability,
) -> Result<
    FoundationalBoundarySurfaceDispositionLegality,
    FoundationalBoundarySurfaceDispositionDenial,
> {
    use FoundationalBoundaryAvailability::{
        Deferred as DeferredAvailability, Present, Reconstructable, Unavailable,
    };
    use FoundationalBoundaryDeliveryClass::{
        CanDefer, MustBeHot, ReconstructableFromRetainedBasis,
    };

    let denial = match (delivery_class, availability) {
        (MustBeHot, Present) | (CanDefer, Present) | (CanDefer, DeferredAvailability) => None,
        (MustBeHot, DeferredAvailability) => {
            Some(FoundationalBoundarySurfaceDispositionDenial::MustBeHotCannotDefer)
        }
        (MustBeHot, Reconstructable) => {
            Some(FoundationalBoundarySurfaceDispositionDenial::MustBeHotCannotReconstruct)
        }
        (MustBeHot, Unavailable) => {
            Some(FoundationalBoundarySurfaceDispositionDenial::MustBeHotCannotBeUnavailable)
        }
        (CanDefer, Reconstructable) => {
            Some(FoundationalBoundarySurfaceDispositionDenial::DeferredDeliveryCannotReconstruct)
        }
        (CanDefer, Unavailable) => None,
        (ReconstructableFromRetainedBasis, Present) => Some(
            FoundationalBoundarySurfaceDispositionDenial::ReconstructableDeliveryCannotAppearPresent,
        ),
        (ReconstructableFromRetainedBasis, DeferredAvailability) => Some(
            FoundationalBoundarySurfaceDispositionDenial::ReconstructableDeliveryCannotDefer,
        ),
        (ReconstructableFromRetainedBasis, Reconstructable)
        | (ReconstructableFromRetainedBasis, Unavailable) => None,
    };

    match denial {
        Some(denial) => Err(denial),
        None => Ok(FoundationalBoundarySurfaceDispositionLegality::new(
            FoundationalBoundarySurfaceDisposition::new(delivery_class, availability),
        )),
    }
}

fn validate_source_and_seam(
    source: FoundationalBoundaryMaterializationSource,
    seam: FoundationalBoundaryMaterializationSeam,
) -> Result<(), FoundationalBoundaryPlanningDenial> {
    use FoundationalBoundaryMaterializationSeam::{
        BoundaryExchange, PersistenceExport, SupportMaterialization,
    };
    use FoundationalBoundaryMaterializationSource::{
        CompatibilityLowered, DerivedSupport, NativeAuthority,
    };

    match (source, seam) {
        (NativeAuthority, SupportMaterialization) => {
            Err(FoundationalBoundaryPlanningDenial::NativeAuthorityCannotUseSupportMaterialization)
        }
        (CompatibilityLowered, SupportMaterialization) => Err(
            FoundationalBoundaryPlanningDenial::CompatibilityLoweredCannotUseSupportMaterialization,
        ),
        (DerivedSupport, BoundaryExchange) => {
            Err(FoundationalBoundaryPlanningDenial::DerivedSupportCannotUseBoundaryExchange)
        }
        (DerivedSupport, PersistenceExport) => {
            Err(FoundationalBoundaryPlanningDenial::DerivedSupportCannotUsePersistenceExport)
        }
        _ => Ok(()),
    }
}

fn derive_disposition<Surface>(
    input: &FoundationalBoundaryMaterializationInput<Surface>,
    seam: FoundationalBoundaryMaterializationSeam,
) -> Result<FoundationalBoundarySurfaceDisposition, FoundationalBoundaryPlanningDenial> {
    use FoundationalBoundaryAvailability::{Deferred, Present, Reconstructable, Unavailable};
    use FoundationalBoundaryDeliveryClass::{
        CanDefer, MustBeHot, ReconstructableFromRetainedBasis,
    };
    use FoundationalBoundaryMaterializationSeam::{
        BoundaryExchange, PersistenceExport, SupportMaterialization,
    };
    use FoundationalBoundaryMaterializationSource::{
        CompatibilityLowered, DerivedSupport, NativeAuthority,
    };

    let legality = match (input.source(), seam) {
        (NativeAuthority, BoundaryExchange | PersistenceExport) => {
            evaluate_boundary_surface_disposition_legality(MustBeHot, Present)
        }
        (CompatibilityLowered, BoundaryExchange) => {
            evaluate_boundary_surface_disposition_legality(CanDefer, Present)
        }
        (CompatibilityLowered, PersistenceExport) => {
            evaluate_boundary_surface_disposition_legality(
                ReconstructableFromRetainedBasis,
                Reconstructable,
            )
        }
        (DerivedSupport, SupportMaterialization)
            if input.role() == FoundationalBoundaryArtifactRole::SupportOnly =>
        {
            evaluate_boundary_surface_disposition_legality(CanDefer, Deferred)
        }
        (DerivedSupport, SupportMaterialization)
            if input.role() == FoundationalBoundaryArtifactRole::PlannedWork =>
        {
            evaluate_boundary_surface_disposition_legality(CanDefer, Unavailable)
        }
        (DerivedSupport, SupportMaterialization) => {
            evaluate_boundary_surface_disposition_legality(CanDefer, Present)
        }
        _ => unreachable!("source and seam validation closed illegal combinations earlier"),
    };

    legality
        .map(|legality| legality.disposition())
        .map_err(FoundationalBoundaryPlanningDenial::IllegalSurfaceDisposition)
}

fn derive_attachments<Surface>(
    input: &FoundationalBoundaryMaterializationInput<Surface>,
    profile: &MaterializedFoundationalProfileSet,
) -> Vec<FoundationalBoundaryMaterializationAttachment> {
    let materialized = profile.materialized();
    let include_diagnostics = matches!(
        materialized.diagnostic_richness(),
        DiagnosticRichnessProfile::Standard | DiagnosticRichnessProfile::Forensic
    ) && matches!(
        input.role(),
        FoundationalBoundaryArtifactRole::DerivedProjection
            | FoundationalBoundaryArtifactRole::SupportOnly
            | FoundationalBoundaryArtifactRole::ReceiptEvidence
    );
    let include_provenance = materialized.retention_delivery()
        != RetentionDeliveryProfile::Ephemeral
        || input.is_authority_claim();
    let include_profile_decisions = profile.requested_to_admitted_narrowing().is_some()
        || profile.admitted_to_materialized_narrowing().is_some();

    vec![
        FoundationalBoundaryMaterializationAttachment::included(
            FoundationalBoundaryAttachmentPoint::ProfileMeaning,
        ),
        if include_profile_decisions {
            FoundationalBoundaryMaterializationAttachment::included(
                FoundationalBoundaryAttachmentPoint::ProfileDecisions,
            )
        } else {
            FoundationalBoundaryMaterializationAttachment::omitted(
                FoundationalBoundaryAttachmentPoint::ProfileDecisions,
            )
        },
        FoundationalBoundaryMaterializationAttachment::omitted(
            FoundationalBoundaryAttachmentPoint::CanonicalBasis,
        ),
        if include_diagnostics {
            FoundationalBoundaryMaterializationAttachment::included(
                FoundationalBoundaryAttachmentPoint::DiagnosticsAttachment,
            )
        } else {
            FoundationalBoundaryMaterializationAttachment::omitted(
                FoundationalBoundaryAttachmentPoint::DiagnosticsAttachment,
            )
        },
        if include_provenance {
            FoundationalBoundaryMaterializationAttachment::included(
                FoundationalBoundaryAttachmentPoint::ProvenanceAttachment,
            )
        } else {
            FoundationalBoundaryMaterializationAttachment::omitted(
                FoundationalBoundaryAttachmentPoint::ProvenanceAttachment,
            )
        },
        FoundationalBoundaryMaterializationAttachment::included(
            FoundationalBoundaryAttachmentPoint::PerformanceAccounting,
        ),
        FoundationalBoundaryMaterializationAttachment::omitted(
            FoundationalBoundaryAttachmentPoint::SameFamilyResolutionAttachment,
        ),
    ]
}

fn derive_decision_rows<Surface>(
    input: &FoundationalBoundaryMaterializationInput<Surface>,
    seam: FoundationalBoundaryMaterializationSeam,
    disposition: &FoundationalBoundarySurfaceDisposition,
    attachments: &[FoundationalBoundaryMaterializationAttachment],
) -> Vec<FoundationalBoundaryMaterializationDecisionRow> {
    let category_decision = if input.is_authority_claim() {
        FoundationalBoundaryMaterializationDecisionRow::new(
            Some(input.category()),
            FoundationalBoundaryDecisionSubject::CategoryRoleAdmission,
            FoundationalBoundaryDecisionCause::NarrowedByAuthority,
            seam,
            None,
        )
    } else {
        FoundationalBoundaryMaterializationDecisionRow::new(
            Some(input.category()),
            FoundationalBoundaryDecisionSubject::CategoryRoleAdmission,
            FoundationalBoundaryDecisionCause::RequestedAsAdmitted,
            seam,
            None,
        )
    };

    let availability_cause = match disposition.availability() {
        FoundationalBoundaryAvailability::Present => {
            FoundationalBoundaryDecisionCause::RequestedAsAdmitted
        }
        FoundationalBoundaryAvailability::Deferred => {
            FoundationalBoundaryDecisionCause::DeferredBySupportPosture
        }
        FoundationalBoundaryAvailability::Reconstructable => {
            FoundationalBoundaryDecisionCause::ReconstructableFromRetainedBasis
        }
        FoundationalBoundaryAvailability::Unavailable => {
            FoundationalBoundaryDecisionCause::DeniedByBudget
        }
    };

    let mut rows = vec![
        category_decision,
        FoundationalBoundaryMaterializationDecisionRow::new(
            Some(input.category()),
            FoundationalBoundaryDecisionSubject::DeliveryAvailabilityResolution,
            availability_cause,
            seam,
            None,
        ),
    ];

    for attachment in attachments {
        rows.push(FoundationalBoundaryMaterializationDecisionRow::new(
            Some(input.category()),
            if attachment.is_included() {
                FoundationalBoundaryDecisionSubject::AttachmentInclusion
            } else {
                FoundationalBoundaryDecisionSubject::AttachmentElision
            },
            attachment_cause(*attachment),
            seam,
            Some(attachment.point()),
        ));
    }

    rows
}

fn attachment_cause(
    attachment: FoundationalBoundaryMaterializationAttachment,
) -> FoundationalBoundaryDecisionCause {
    if attachment.is_included() {
        return FoundationalBoundaryDecisionCause::RequestedAsAdmitted;
    }

    match attachment.point() {
        FoundationalBoundaryAttachmentPoint::CanonicalBasis
        | FoundationalBoundaryAttachmentPoint::SameFamilyResolutionAttachment => {
            FoundationalBoundaryDecisionCause::DeniedByMilestoneBoundary
        }
        FoundationalBoundaryAttachmentPoint::DiagnosticsAttachment => {
            FoundationalBoundaryDecisionCause::ElidedByProfile
        }
        FoundationalBoundaryAttachmentPoint::ProvenanceAttachment => {
            FoundationalBoundaryDecisionCause::UnavailableByRetention
        }
        FoundationalBoundaryAttachmentPoint::ProfileDecisions => {
            FoundationalBoundaryDecisionCause::DeniedByBudget
        }
        FoundationalBoundaryAttachmentPoint::ProfileMeaning
        | FoundationalBoundaryAttachmentPoint::PerformanceAccounting => {
            FoundationalBoundaryDecisionCause::RequestedAsAdmitted
        }
    }
}
