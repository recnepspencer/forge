use super::observation::FoundationalObservationDisposition;
use super::vocabulary::{
    FoundationalDescriptiveSurface, FoundationalSurfaceAbsenceCause,
    FoundationalSurfaceAvailabilityDecision,
};
use crate::profiles::progression::MaterializedFoundationalProfileSet;
use crate::profiles::{
    CompatibilityPostureProfile, DiagnosticRichnessProfile,
    FoundationalProfileAttachmentTargetKind, FoundationalProfileSet, RetentionDeliveryProfile,
    SupportPostureProfile,
};

pub(super) fn availability_decision(
    profile: &MaterializedFoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
    selected: &[FoundationalDescriptiveSurface],
    disposition: FoundationalObservationDisposition,
) -> FoundationalSurfaceAvailabilityDecision {
    if !selected.contains(&surface) {
        return FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::DeniedByBudget,
        );
    }

    if !disposition.is_active() {
        return FoundationalSurfaceAvailabilityDecision::unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::ObservationNotActivated,
        );
    }

    let materialized = profile.materialized();
    match surface {
        FoundationalDescriptiveSurface::History => history_decision(materialized, surface),
        FoundationalDescriptiveSurface::Replay => replay_decision(materialized, surface),
        FoundationalDescriptiveSurface::Lineage => lineage_decision(materialized, surface),
        FoundationalDescriptiveSurface::Provenance => {
            provenance_decision(materialized, target_kind, surface)
        }
        FoundationalDescriptiveSurface::ForensicDiagnostics => {
            forensic_decision(materialized, target_kind, surface)
        }
    }
}

fn history_decision(
    materialized: &FoundationalProfileSet,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() == DiagnosticRichnessProfile::OperationalMinimal {
        unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral {
        unavailable(surface, FoundationalSurfaceAbsenceCause::NotRetained)
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn replay_decision(
    materialized: &FoundationalProfileSet,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() == DiagnosticRichnessProfile::OperationalMinimal {
        unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral {
        unavailable(surface, FoundationalSurfaceAbsenceCause::NotRetained)
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Retained
        || materialized.compatibility_posture() == CompatibilityPostureProfile::NativeOnly
    {
        unavailable(surface, FoundationalSurfaceAbsenceCause::NotReconstructable)
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn lineage_decision(
    materialized: &FoundationalProfileSet,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() == DiagnosticRichnessProfile::OperationalMinimal {
        unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral {
        unavailable(surface, FoundationalSurfaceAbsenceCause::NotRetained)
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn provenance_decision(
    materialized: &FoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if target_kind == FoundationalProfileAttachmentTargetKind::SupportArtifact
        && materialized.support_posture() != SupportPostureProfile::CertificationReady
    {
        unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::DeferredBySupportPosture,
        )
    } else if target_kind != FoundationalProfileAttachmentTargetKind::ProofBearingArtifact
        && materialized.retention_delivery() == RetentionDeliveryProfile::Ephemeral
    {
        unavailable(surface, FoundationalSurfaceAbsenceCause::NotRetained)
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn forensic_decision(
    materialized: &FoundationalProfileSet,
    target_kind: FoundationalProfileAttachmentTargetKind,
    surface: FoundationalDescriptiveSurface,
) -> FoundationalSurfaceAvailabilityDecision {
    if materialized.diagnostic_richness() != DiagnosticRichnessProfile::Forensic {
        unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::OmittedByActiveRichness,
        )
    } else if target_kind == FoundationalProfileAttachmentTargetKind::SupportArtifact
        && materialized.certification_posture()
            != crate::profiles::CertificationPostureProfile::ProductionCertified
    {
        unavailable(
            surface,
            FoundationalSurfaceAbsenceCause::UncertifiedForRequestedPosture,
        )
    } else {
        FoundationalSurfaceAvailabilityDecision::available(surface)
    }
}

fn unavailable(
    surface: FoundationalDescriptiveSurface,
    cause: FoundationalSurfaceAbsenceCause,
) -> FoundationalSurfaceAvailabilityDecision {
    FoundationalSurfaceAvailabilityDecision::unavailable(surface, cause)
}
