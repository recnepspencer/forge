use super::super::resolution::{
    FoundationalProfileResolutionFamily, FoundationalProfileResolutionLedger,
    FoundationalProfileResolutionRelation,
};
use super::super::FoundationalProfileSet;
use super::FoundationalProfileProgressionDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileNarrowingRecord {
    kind: FoundationalProfileNarrowingKind,
    reason: &'static str,
}

impl FoundationalProfileNarrowingRecord {
    pub const fn new(kind: FoundationalProfileNarrowingKind, reason: &'static str) -> Self {
        Self { kind, reason }
    }

    pub const fn kind(&self) -> FoundationalProfileNarrowingKind {
        self.kind
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileNarrowingKind {
    RichnessReduced,
    RetentionNarrowed,
    SupportPostureReduced,
    CertificationPostureReduced,
    CompatibilityRestricted,
}

pub(super) fn legacy_narrowing_projection(
    ledger: FoundationalProfileResolutionLedger,
) -> Option<FoundationalProfileNarrowingRecord> {
    let record = ledger
        .records()
        .find(|record| record.relation() == FoundationalProfileResolutionRelation::Narrowing)?;
    let kind = match record.family() {
        FoundationalProfileResolutionFamily::DiagnosticRichness => {
            FoundationalProfileNarrowingKind::RichnessReduced
        }
        FoundationalProfileResolutionFamily::RetentionDelivery => {
            FoundationalProfileNarrowingKind::RetentionNarrowed
        }
        FoundationalProfileResolutionFamily::SupportPosture => {
            FoundationalProfileNarrowingKind::SupportPostureReduced
        }
        FoundationalProfileResolutionFamily::CertificationPosture => {
            FoundationalProfileNarrowingKind::CertificationPostureReduced
        }
        FoundationalProfileResolutionFamily::CompatibilityPosture => {
            FoundationalProfileNarrowingKind::CompatibilityRestricted
        }
        FoundationalProfileResolutionFamily::AdmissionReadiness
        | FoundationalProfileResolutionFamily::ExecutionObjective
        | FoundationalProfileResolutionFamily::ObservationActivation => return None,
    };
    Some(FoundationalProfileNarrowingRecord::new(
        kind,
        record.reason(),
    ))
}

pub(super) fn classify_profile_narrowing(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
) -> Result<Option<FoundationalProfileNarrowingRecord>, FoundationalProfileProgressionDenial> {
    let Some(expected_kind) = detect_profile_narrowing_kind(stronger, weaker)? else {
        return Ok(None);
    };

    let Some(record) = narrowing else {
        return Err(FoundationalProfileProgressionDenial::MissingExplicitNarrowingRecord);
    };

    if record.kind() != expected_kind {
        return Err(FoundationalProfileProgressionDenial::NarrowingRecordKindMismatch);
    }

    Ok(Some(record))
}

/// Validate a resolution-ledger transition with the same monotonic and
/// readiness rules used by the legacy narrowing front door. Resolution
/// records are an additional explanation; they are not a second authority.
pub(crate) fn classify_profile_narrowing_for_resolution(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
) -> Result<Option<FoundationalProfileNarrowingRecord>, FoundationalProfileProgressionDenial> {
    let Some(kind) = detect_profile_narrowing_kind(stronger, weaker)? else {
        return Ok(None);
    };

    Ok(Some(FoundationalProfileNarrowingRecord::new(
        kind,
        narrowing_reason(kind),
    )))
}

const fn narrowing_reason(kind: FoundationalProfileNarrowingKind) -> &'static str {
    match kind {
        FoundationalProfileNarrowingKind::RichnessReduced => "diagnostic richness narrowed",
        FoundationalProfileNarrowingKind::RetentionNarrowed => "retention delivery narrowed",
        FoundationalProfileNarrowingKind::SupportPostureReduced => "support posture narrowed",
        FoundationalProfileNarrowingKind::CertificationPostureReduced => {
            "certification posture narrowed"
        }
        FoundationalProfileNarrowingKind::CompatibilityRestricted => {
            "compatibility posture narrowed"
        }
    }
}

fn detect_profile_narrowing_kind(
    stronger: FoundationalProfileSet,
    weaker: FoundationalProfileSet,
) -> Result<Option<FoundationalProfileNarrowingKind>, FoundationalProfileProgressionDenial> {
    if stronger.admission_readiness() != weaker.admission_readiness() {
        return Err(
            FoundationalProfileProgressionDenial::AdmissionReadinessCannotChangeAcrossProfileProgression,
        );
    }

    let mut changed_kind = None;
    record_family_narrowing(
        stronger.diagnostic_richness(),
        weaker.diagnostic_richness(),
        FoundationalProfileNarrowingKind::RichnessReduced,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.retention_delivery(),
        weaker.retention_delivery(),
        FoundationalProfileNarrowingKind::RetentionNarrowed,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.support_posture(),
        weaker.support_posture(),
        FoundationalProfileNarrowingKind::SupportPostureReduced,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.certification_posture(),
        weaker.certification_posture(),
        FoundationalProfileNarrowingKind::CertificationPostureReduced,
        &mut changed_kind,
    )?;
    record_family_narrowing(
        stronger.compatibility_posture(),
        weaker.compatibility_posture(),
        FoundationalProfileNarrowingKind::CompatibilityRestricted,
        &mut changed_kind,
    )?;

    Ok(changed_kind)
}

fn record_family_narrowing<T: Copy + Ord>(
    stronger: T,
    weaker: T,
    kind: FoundationalProfileNarrowingKind,
    changed_kind: &mut Option<FoundationalProfileNarrowingKind>,
) -> Result<(), FoundationalProfileProgressionDenial> {
    if stronger == weaker {
        return Ok(());
    }
    if weaker > stronger {
        return Err(
            FoundationalProfileProgressionDenial::RequestedAndAdmittedProfilesMayOnlyNarrow,
        );
    }
    if changed_kind.replace(kind).is_some() {
        return Err(
            FoundationalProfileProgressionDenial::RequestedAndAdmittedProfilesMayDifferInOnlyOneFamily,
        );
    }
    Ok(())
}
