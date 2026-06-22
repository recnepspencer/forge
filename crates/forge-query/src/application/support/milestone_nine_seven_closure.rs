use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryConcurrentHostileMatrixArtifact, ForgeQueryConcurrentHostileMatrixPosture,
    ForgeQueryJournalIdentityBoundaryPosture, ForgeQueryJournalReplayBoundaryCertification,
    ForgeQueryMilestoneClosureStatus, ForgeQueryPublicBridgeReaderLaneCertification,
    ForgeQueryPublicBridgeReaderLanePosture, ForgeQuerySharedReadPinningBoundaryClosure,
    ForgeQuerySharedReadPinningBoundaryPosture,
};

const REQUIRED_PHASES: [&str; 4] = [
    "phase-13-shared-read-pinning",
    "phase-15-journal-replay",
    "phase-16-concurrent-hostile-matrix",
    "phase-17-public-bridge-reader-lane",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMilestoneNineSevenPhaseClosure {
    phase: &'static str,
    status: ForgeQueryMilestoneClosureStatus,
    evidence_digest: String,
}

impl ForgeQueryMilestoneNineSevenPhaseClosure {
    pub fn from_shared_read_pinning(closure: &ForgeQuerySharedReadPinningBoundaryClosure) -> Self {
        Self::new(
            REQUIRED_PHASES[0],
            milestone_status_from_pinning_posture(closure.posture()),
            closure.closure_digest(),
        )
    }

    pub fn from_journal_replay_boundary(
        closure: &ForgeQueryJournalReplayBoundaryCertification,
    ) -> Self {
        Self::new(
            REQUIRED_PHASES[1],
            milestone_status_from_journal_posture(closure.journal_boundary_posture()),
            closure.journal_identity_inventory_digest(),
        )
    }

    pub fn from_concurrent_hostile_matrix(
        artifact: &ForgeQueryConcurrentHostileMatrixArtifact,
    ) -> Self {
        Self::new(
            REQUIRED_PHASES[2],
            milestone_status_from_concurrent_posture(artifact.posture()),
            artifact.digest().as_str(),
        )
    }

    pub fn from_public_bridge_reader_lane(
        certification: &ForgeQueryPublicBridgeReaderLaneCertification,
    ) -> Self {
        Self::new(
            REQUIRED_PHASES[3],
            milestone_status_from_public_bridge_posture(certification.posture()),
            certification.digest().as_str(),
        )
    }

    pub(crate) fn new(
        phase: &'static str,
        status: ForgeQueryMilestoneClosureStatus,
        evidence_digest: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            status,
            evidence_digest: evidence_digest.into(),
        }
    }

    pub fn phase(&self) -> &'static str {
        self.phase
    }

    pub fn status(&self) -> ForgeQueryMilestoneClosureStatus {
        self.status
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMilestoneNineSevenDerivedClosure {
    status: ForgeQueryMilestoneClosureStatus,
    phase_closures: Vec<ForgeQueryMilestoneNineSevenPhaseClosure>,
    defended_exclusions: Vec<String>,
    closure_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryMilestoneNineSevenDerivedClosure {
    pub fn derive_from_phase_closures(
        phase_closures: impl IntoIterator<Item = ForgeQueryMilestoneNineSevenPhaseClosure>,
        defended_exclusions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let mut phase_closures = phase_closures.into_iter().collect::<Vec<_>>();
        phase_closures.sort_by_key(ForgeQueryMilestoneNineSevenPhaseClosure::phase);
        let defended_exclusions = defended_exclusions
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let status = derive_milestone_nine_seven_status(&phase_closures);
        let closure_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
                .field_shape(ForgeQueryEvidenceTag::new("milestone"), "forge-query-9.7")
                .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("required_phase"),
                    REQUIRED_PHASES.iter().copied(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("phase_status"),
                    phase_closures.iter().map(|closure| {
                        format!("{}:{}", closure.phase(), closure.status().as_str())
                    }),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("phase_evidence_digest"),
                    phase_closures
                        .iter()
                        .map(ForgeQueryMilestoneNineSevenPhaseClosure::evidence_digest),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("defended_exclusion"),
                    defended_exclusions.iter().map(String::as_str),
                )
                .seal();
        Self {
            status,
            phase_closures,
            defended_exclusions,
            closure_identity,
        }
    }

    pub fn support_profile_publication_contract() -> Self {
        Self::derive_from_phase_closures(
            REQUIRED_PHASES.into_iter().map(|phase| {
                ForgeQueryMilestoneNineSevenPhaseClosure::new(
                    phase,
                    ForgeQueryMilestoneClosureStatus::Partial,
                    format!("{phase}:support-profile-requires-phase-local-artifact"),
                )
            }),
            ["store-backed execution parity belongs to Milestone 10"],
        )
    }

    pub fn status(&self) -> ForgeQueryMilestoneClosureStatus {
        self.status
    }

    pub fn phase_closures(&self) -> &[ForgeQueryMilestoneNineSevenPhaseClosure] {
        &self.phase_closures
    }

    pub fn defended_exclusions(&self) -> &[String] {
        &self.defended_exclusions
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.closure_identity
    }

    pub fn required_phases() -> &'static [&'static str] {
        &REQUIRED_PHASES
    }
}

fn derive_milestone_nine_seven_status(
    phase_closures: &[ForgeQueryMilestoneNineSevenPhaseClosure],
) -> ForgeQueryMilestoneClosureStatus {
    let required_closed = REQUIRED_PHASES.iter().all(|required_phase| {
        phase_closures.iter().any(|closure| {
            closure.phase() == *required_phase
                && closure.status() == ForgeQueryMilestoneClosureStatus::Closed
                && !closure.evidence_digest().is_empty()
        })
    });
    if required_closed {
        return ForgeQueryMilestoneClosureStatus::Closed;
    }
    if phase_closures
        .iter()
        .any(|closure| closure.status() != ForgeQueryMilestoneClosureStatus::Open)
    {
        return ForgeQueryMilestoneClosureStatus::Partial;
    }
    ForgeQueryMilestoneClosureStatus::Open
}

fn milestone_status_from_pinning_posture(
    posture: ForgeQuerySharedReadPinningBoundaryPosture,
) -> ForgeQueryMilestoneClosureStatus {
    match posture {
        ForgeQuerySharedReadPinningBoundaryPosture::Closed => {
            ForgeQueryMilestoneClosureStatus::Closed
        }
        ForgeQuerySharedReadPinningBoundaryPosture::Partial => {
            ForgeQueryMilestoneClosureStatus::Partial
        }
        ForgeQuerySharedReadPinningBoundaryPosture::Open => ForgeQueryMilestoneClosureStatus::Open,
    }
}

fn milestone_status_from_journal_posture(
    posture: ForgeQueryJournalIdentityBoundaryPosture,
) -> ForgeQueryMilestoneClosureStatus {
    match posture {
        ForgeQueryJournalIdentityBoundaryPosture::Closed => {
            ForgeQueryMilestoneClosureStatus::Closed
        }
        ForgeQueryJournalIdentityBoundaryPosture::Partial => {
            ForgeQueryMilestoneClosureStatus::Partial
        }
        ForgeQueryJournalIdentityBoundaryPosture::Open => ForgeQueryMilestoneClosureStatus::Open,
    }
}

fn milestone_status_from_concurrent_posture(
    posture: ForgeQueryConcurrentHostileMatrixPosture,
) -> ForgeQueryMilestoneClosureStatus {
    match posture {
        ForgeQueryConcurrentHostileMatrixPosture::Closed => {
            ForgeQueryMilestoneClosureStatus::Closed
        }
        ForgeQueryConcurrentHostileMatrixPosture::Partial => {
            ForgeQueryMilestoneClosureStatus::Partial
        }
        ForgeQueryConcurrentHostileMatrixPosture::Open => ForgeQueryMilestoneClosureStatus::Open,
    }
}

fn milestone_status_from_public_bridge_posture(
    posture: ForgeQueryPublicBridgeReaderLanePosture,
) -> ForgeQueryMilestoneClosureStatus {
    match posture {
        ForgeQueryPublicBridgeReaderLanePosture::Closed => ForgeQueryMilestoneClosureStatus::Closed,
        ForgeQueryPublicBridgeReaderLanePosture::Open => ForgeQueryMilestoneClosureStatus::Open,
    }
}
