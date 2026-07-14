use super::{LayoutCourtroomTranscriptIdentity, LayoutEvidenceBundle};
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutHazard {
    HiddenBroadScan,
    StaleAcceptedAsExact,
    PartialCoverageFalseAbsence,
    ProjectionUsedAsAuthority,
    CrossScopeIndexReuse,
    CorruptionConvertedToEmpty,
    DerivedRollbackAuthority,
    BTreeSeparatorMisrouting,
    LsmTombstoneLoss,
    CacheAdmissionBypass,
    CompatibilityPathReadiness,
    CopiedCounterEvidence,
    StaticCaseSubstitution,
    CertificationAuthoredLowerAuthority,
}

impl LayoutHazard {
    pub const fn all() -> [Self; 14] {
        [
            Self::HiddenBroadScan,
            Self::StaleAcceptedAsExact,
            Self::PartialCoverageFalseAbsence,
            Self::ProjectionUsedAsAuthority,
            Self::CrossScopeIndexReuse,
            Self::CorruptionConvertedToEmpty,
            Self::DerivedRollbackAuthority,
            Self::BTreeSeparatorMisrouting,
            Self::LsmTombstoneLoss,
            Self::CacheAdmissionBypass,
            Self::CompatibilityPathReadiness,
            Self::CopiedCounterEvidence,
            Self::StaticCaseSubstitution,
            Self::CertificationAuthoredLowerAuthority,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCompileFailBoundary {
    ProjectionCannotSatisfyAuthority,
    CounterReceiptCannotBeCopiedOrPaired,
    CertificationCannotIssueOwnerOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutHazardEvidencePosture {
    ExecutedOwnerEvidence,
    ExactCoverageEvidence,
    ExternalCompileFailRequirement(LayoutCompileFailBoundary),
    FoundationalEvidenceAndExternalCompileFailRequirement(LayoutCompileFailBoundary),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutHazardRow {
    transcript_identity: LayoutCourtroomTranscriptIdentity,
    hazard: LayoutHazard,
    evidence_posture: LayoutHazardEvidencePosture,
    detection: &'static str,
    containment: &'static str,
    recovery: &'static str,
    residual_risk: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutHazardInventory {
    transcript_identity: LayoutCourtroomTranscriptIdentity,
    rows: Vec<LayoutHazardRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutHazardAdjudicationDenial {
    MissingExecutedEvidence(LayoutHazard),
    FoundationalEvidenceIncomplete(LayoutHazard),
}

pub fn adjudicate_layout_hazards(
    bundle: &LayoutEvidenceBundle,
) -> Result<LayoutHazardInventory, LayoutHazardAdjudicationDenial> {
    let rows = LayoutHazard::all()
        .into_iter()
        .map(|hazard| adjudicate_hazard(bundle, hazard))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(LayoutHazardInventory {
        transcript_identity: bundle.transcript_identity(),
        rows,
    })
}

fn adjudicate_hazard(
    bundle: &LayoutEvidenceBundle,
    hazard: LayoutHazard,
) -> Result<LayoutHazardRow, LayoutHazardAdjudicationDenial> {
    let contract = contract(hazard);
    match contract.requirement {
        EvidenceRequirement::Executed(required) => {
            if !required
                .iter()
                .all(|evidence| bundle.coverage().executed_evidence().contains(*evidence))
            {
                return Err(LayoutHazardAdjudicationDenial::MissingExecutedEvidence(
                    hazard,
                ));
            }
        }
        EvidenceRequirement::FoundationalAndCompileFail(_) => {
            if bundle.foundational().counter_row_count() == 0
                || bundle.foundational().support_row_count() == 0
                || bundle.foundational().source_entry_count() == 0
            {
                return Err(LayoutHazardAdjudicationDenial::FoundationalEvidenceIncomplete(hazard));
            }
        }
        EvidenceRequirement::ExactCoverage | EvidenceRequirement::CompileFail(_) => {}
    }
    Ok(LayoutHazardRow {
        transcript_identity: bundle.transcript_identity(),
        hazard,
        evidence_posture: contract.requirement.posture(),
        detection: contract.detection,
        containment: contract.containment,
        recovery: contract.recovery,
        residual_risk: None,
    })
}

#[derive(Clone, Copy)]
enum EvidenceRequirement {
    Executed(&'static [Evidence]),
    ExactCoverage,
    CompileFail(LayoutCompileFailBoundary),
    FoundationalAndCompileFail(LayoutCompileFailBoundary),
}

impl EvidenceRequirement {
    const fn posture(self) -> LayoutHazardEvidencePosture {
        match self {
            Self::Executed(_) => LayoutHazardEvidencePosture::ExecutedOwnerEvidence,
            Self::ExactCoverage => LayoutHazardEvidencePosture::ExactCoverageEvidence,
            Self::CompileFail(boundary) => {
                LayoutHazardEvidencePosture::ExternalCompileFailRequirement(boundary)
            }
            Self::FoundationalAndCompileFail(boundary) => {
                LayoutHazardEvidencePosture::FoundationalEvidenceAndExternalCompileFailRequirement(
                    boundary,
                )
            }
        }
    }
}

struct HazardContract {
    requirement: EvidenceRequirement,
    detection: &'static str,
    containment: &'static str,
    recovery: &'static str,
}

const fn contract(hazard: LayoutHazard) -> HazardContract {
    use LayoutCompileFailBoundary as CompileFail;
    use LayoutHazard::*;
    match hazard {
        HiddenBroadScan => executed(
            &[Evidence::HiddenBroadScanDenied],
            "typed hidden-scan denial and bounded-access counters",
            "deny before execution",
            "admit an explicit exact scan",
        ),
        StaleAcceptedAsExact => executed(
            &[Evidence::BTreeReadinessStale],
            "owner-issued stale readiness",
            "prevent execution",
            "rebind against current catalog and source",
        ),
        PartialCoverageFalseAbsence => executed(
            &[Evidence::BTreePhysicalReadDenied],
            "incomplete physical read denial",
            "deny absence publication",
            "restore the admitted physical source",
        ),
        ProjectionUsedAsAuthority => compile_fail(
            CompileFail::ProjectionCannotSatisfyAuthority,
            "compile-time authority mismatch",
            "reject projection input",
            "return to the owning operation",
        ),
        CrossScopeIndexReuse => executed(
            &[Evidence::CrossTenantScopeDenied],
            "typed tenant-scope denial",
            "deny key-domain admission",
            "readmit under current security scope",
        ),
        CorruptionConvertedToEmpty => executed(
            &[Evidence::CorruptionQuarantined],
            "owner-issued quarantine classification",
            "quarantine instead of returning absence",
            "rebuild from authoritative state",
        ),
        DerivedRollbackAuthority => executed(
            &[Evidence::RollbackPublicationSourceDenied],
            "rollback publication-source binding",
            "deny copied rollback evidence",
            "execute rollback through its owner",
        ),
        BTreeSeparatorMisrouting => executed(
            &[
                Evidence::BTreeLeafOrderDenied,
                Evidence::BTreeLeftPartitionDenied,
                Evidence::BTreeRightPartitionDenied,
            ],
            "separator-partition verification",
            "deny before found or absent observation",
            "reopen an admitted root and child set",
        ),
        LsmTombstoneLoss => executed(
            &[Evidence::LsmTombstoneRequired],
            "named compaction membership roles",
            "deny compaction selection",
            "replay durable membership records",
        ),
        CacheAdmissionBypass => executed(
            &[Evidence::LsmCacheArtifactInvalid],
            "WAL artifact readmission",
            "discard invalid cached membership",
            "rederive membership from durable artifacts",
        ),
        CompatibilityPathReadiness => executed(
            &[Evidence::CompatibilityWindowMismatch],
            "typed compatibility-window denial",
            "deny stronger readiness",
            "migrate or readmit the layout",
        ),
        CopiedCounterEvidence => HazardContract {
            requirement: EvidenceRequirement::FoundationalAndCompileFail(
                CompileFail::CounterReceiptCannotBeCopiedOrPaired,
            ),
            detection: "counter-backed report source binding",
            containment: "deny independently paired performance evidence",
            recovery: "re-execute and recertify the bound counter receipt",
        },
        StaticCaseSubstitution => HazardContract {
            requirement: EvidenceRequirement::ExactCoverage,
            detection: "exact declared-versus-observed set equality",
            containment: "reject missing, duplicate, or unexpected cases",
            recovery: "execute the ordinary owner facade",
        },
        CertificationAuthoredLowerAuthority => compile_fail(
            CompileFail::CertificationCannotIssueOwnerOutcome,
            "compile-time owner-constructor privacy",
            "reject certification-authored outcomes",
            "return to the permanent production owner",
        ),
    }
}

const fn executed(
    evidence: &'static [Evidence],
    detection: &'static str,
    containment: &'static str,
    recovery: &'static str,
) -> HazardContract {
    HazardContract {
        requirement: EvidenceRequirement::Executed(evidence),
        detection,
        containment,
        recovery,
    }
}

const fn compile_fail(
    boundary: LayoutCompileFailBoundary,
    detection: &'static str,
    containment: &'static str,
    recovery: &'static str,
) -> HazardContract {
    HazardContract {
        requirement: EvidenceRequirement::CompileFail(boundary),
        detection,
        containment,
        recovery,
    }
}

impl LayoutHazardInventory {
    pub const fn transcript_identity(&self) -> LayoutCourtroomTranscriptIdentity {
        self.transcript_identity
    }

    pub fn rows(&self) -> &[LayoutHazardRow] {
        &self.rows
    }
}

impl LayoutHazardRow {
    pub const fn transcript_identity(&self) -> LayoutCourtroomTranscriptIdentity {
        self.transcript_identity
    }

    pub const fn hazard(&self) -> LayoutHazard {
        self.hazard
    }

    pub const fn evidence_posture(&self) -> LayoutHazardEvidencePosture {
        self.evidence_posture
    }

    pub const fn detection(&self) -> &'static str {
        self.detection
    }

    pub const fn containment(&self) -> &'static str {
        self.containment
    }

    pub const fn recovery(&self) -> &'static str {
        self.recovery
    }

    pub const fn residual_risk(&self) -> Option<&'static str> {
        self.residual_risk
    }
}
