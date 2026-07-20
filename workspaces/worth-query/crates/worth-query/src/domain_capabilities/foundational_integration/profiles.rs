use worth_foundational::{
    admit_requested_foundational_profile, foundational_profile_progression_authority,
    materialize_admitted_foundational_profile, request_foundational_profile_set,
    DiagnosticRichnessProfile, FoundationalProfileNarrowingKind,
    FoundationalProfileNarrowingRecord, FoundationalProfileSet, FoundationalProfileSetInput,
    SupportPostureProfile,
};
use worth_proof::TransitionOutcome;

use super::super::materialization::{
    WorthQueryDomainCapabilityDescriptiveArtifactKind,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
    WorthQueryDomainCapabilityProfileProgression,
};
use super::super::targets::WorthQueryDomainCapabilityTargetBinding;
use super::super::{
    WorthQueryDomainCapabilityPayload, WorthQueryMaterializationReadyDomainCapabilityContribution,
};

pub(crate) fn materialize_profile_progression<P, T>(
    contribution: &WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
    artifact_kind: WorthQueryDomainCapabilityDescriptiveArtifactKind,
) -> Result<
    WorthQueryDomainCapabilityProfileProgression,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    let category = contribution.payload().category();
    let requested = request_foundational_profile_set(requested_profile);
    let admitted_profile = admitted_profile_for(contribution, requested_profile, artifact_kind);
    let admitted_narrowing = profile_narrowing(
        requested_profile,
        admitted_profile,
        artifact_kind,
        "descriptive admission posture narrows support visibility",
    );
    let admitted = match admit_requested_foundational_profile(
        requested.clone(),
        admitted_profile,
        admitted_narrowing,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => {
            return Err(
                WorthQueryDomainCapabilityDescriptiveMaterializationDenial::ProfileAdmission {
                    category,
                    artifact_kind,
                    denial,
                },
            )
        }
        outcome => panic!("unexpected foundational profile admission outcome: {outcome:?}"),
    };

    let materialized_profile =
        materialized_profile_for(contribution, admitted_profile, artifact_kind);
    let materialized_narrowing = profile_narrowing(
        admitted_profile,
        materialized_profile,
        artifact_kind,
        "descriptive materialization narrows retained forensic breadth",
    );
    let materialized = match materialize_admitted_foundational_profile(
        admitted.clone(),
        materialized_profile,
        materialized_narrowing,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => return Err(
            WorthQueryDomainCapabilityDescriptiveMaterializationDenial::ProfileMaterialization {
                category,
                artifact_kind,
                denial,
            },
        ),
        outcome => panic!("unexpected foundational profile materialization outcome: {outcome:?}"),
    };

    Ok(WorthQueryDomainCapabilityProfileProgression::new(
        requested,
        admitted,
        materialized,
    ))
}

fn admitted_profile_for<P, T>(
    contribution: &WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested: FoundationalProfileSet,
    artifact_kind: WorthQueryDomainCapabilityDescriptiveArtifactKind,
) -> FoundationalProfileSet
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    if matches!(
        artifact_kind,
        WorthQueryDomainCapabilityDescriptiveArtifactKind::ExplanationBundle
            | WorthQueryDomainCapabilityDescriptiveArtifactKind::TraceArtifact
    ) && requested.support_posture() != SupportPostureProfile::InternalOnly
    {
        return with_support_posture(requested, SupportPostureProfile::InternalOnly);
    }

    contribution.payload();
    requested
}

fn materialized_profile_for<P, T>(
    contribution: &WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    admitted: FoundationalProfileSet,
    artifact_kind: WorthQueryDomainCapabilityDescriptiveArtifactKind,
) -> FoundationalProfileSet
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    if matches!(
        artifact_kind,
        WorthQueryDomainCapabilityDescriptiveArtifactKind::Summary
    ) && admitted.diagnostic_richness() == DiagnosticRichnessProfile::Forensic
    {
        return with_richness(admitted, DiagnosticRichnessProfile::Standard);
    }

    let _ = contribution;
    admitted
}

fn profile_narrowing(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
    artifact_kind: WorthQueryDomainCapabilityDescriptiveArtifactKind,
    reason: &'static str,
) -> Option<FoundationalProfileNarrowingRecord> {
    if stronger == weaker {
        return None;
    }
    if stronger.support_posture() != weaker.support_posture() {
        return Some(FoundationalProfileNarrowingRecord::new(
            FoundationalProfileNarrowingKind::SupportPostureReduced,
            reason,
        ));
    }
    if stronger.diagnostic_richness() != weaker.diagnostic_richness() {
        return Some(FoundationalProfileNarrowingRecord::new(
            FoundationalProfileNarrowingKind::RichnessReduced,
            reason,
        ));
    }

    panic!(
        "unexpected foundational profile narrowing for {} without a supported family",
        artifact_kind.as_str()
    );
}

fn with_support_posture(
    profile: FoundationalProfileSet,
    support_posture: SupportPostureProfile,
) -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: profile.diagnostic_richness(),
        support_posture,
        compatibility_posture: profile.compatibility_posture(),
        admission_readiness: profile.admission_readiness(),
        retention_delivery: profile.retention_delivery(),
        certification_posture: profile.certification_posture(),
    })
    .expect("support posture narrowing should preserve profile legality")
}

fn with_richness(
    profile: FoundationalProfileSet,
    diagnostic_richness: DiagnosticRichnessProfile,
) -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness,
        support_posture: profile.support_posture(),
        compatibility_posture: profile.compatibility_posture(),
        admission_readiness: profile.admission_readiness(),
        retention_delivery: profile.retention_delivery(),
        certification_posture: profile.certification_posture(),
    })
    .expect("richness narrowing should preserve profile legality")
}
