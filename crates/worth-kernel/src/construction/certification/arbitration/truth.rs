use crate::construction::certification::{
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionRow,
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionIntentArbitrationDxSurface,
    PrimitiveConstructionIntentArbitrationDxSurfaceRow,
    PrimitiveConstructionIntentArbitrationPolicyRow, PrimitiveConstructionObservedIntentRelation,
    PrimitiveConstructionPreservedIntentResolutionRow, PrimitiveConstructionPreservedIntentTruth,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::{
    PrimitiveConstructionIntentArbitrationReplayParityReport,
    PrimitiveConstructionIntentChosenTruth,
    PrimitiveConstructionQueryIntentArbitrationParityReport,
};
use worth_spatial::facade::arbitration::{
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialIntentCandidate,
    SpatialIntentEscalation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationCanonicalTruth {
    authored_act: SpatialAuthoredActKind,
    observed_relations: Vec<PrimitiveConstructionObservedIntentRelation>,
    conflict_class: PrimitiveConstructionIntentArbitrationConflictClass,
    escalation: SpatialIntentEscalation,
    candidates: Vec<SpatialIntentCandidate>,
    blocked_candidates: Vec<(SpatialIntentCandidate, SpatialBlockedCapability)>,
    preserved_truth: PrimitiveConstructionPreservedIntentTruth,
    truth_digest: String,
}

impl PrimitiveConstructionIntentArbitrationCanonicalTruth {
    pub fn from_preserved_row(report: &PrimitiveConstructionPreservedIntentResolutionRow) -> Self {
        let authored_act = report.authored_act();
        let observed_relations = report.observed_relations().to_vec();
        let conflict_class = report.conflict_class();
        let escalation = report.escalation();
        let candidates = report.candidates().to_vec();
        let blocked_candidates = report.blocked_candidates().to_vec();
        let preserved_truth = report.preserved_truth();
        let truth_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &[
                authored_act.as_str().to_string(),
                format!("{observed_relations:?}"),
                format!("{conflict_class:?}"),
                format!("{escalation:?}"),
                format!("{candidates:?}"),
                format!("{blocked_candidates:?}"),
                format!("{preserved_truth:?}"),
            ],
        );
        Self {
            authored_act,
            observed_relations,
            conflict_class,
            escalation,
            candidates,
            blocked_candidates,
            preserved_truth,
            truth_digest,
        }
    }

    pub fn authored_act(&self) -> SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn observed_relations(&self) -> &[PrimitiveConstructionObservedIntentRelation] {
        &self.observed_relations
    }

    pub fn conflict_class(&self) -> PrimitiveConstructionIntentArbitrationConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn candidates(&self) -> &[SpatialIntentCandidate] {
        &self.candidates
    }

    pub fn blocked_candidates(&self) -> &[(SpatialIntentCandidate, SpatialBlockedCapability)] {
        &self.blocked_candidates
    }

    pub fn preserved_truth(&self) -> PrimitiveConstructionPreservedIntentTruth {
        self.preserved_truth
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }

    pub fn matches_preserved_row(
        &self,
        row: &PrimitiveConstructionPreservedIntentResolutionRow,
    ) -> bool {
        self.authored_act == row.authored_act()
            && self.observed_relations == row.observed_relations()
            && self.conflict_class == row.conflict_class()
            && self.escalation == row.escalation()
            && self.candidates == row.candidates()
            && self.blocked_candidates == row.blocked_candidates()
            && self.preserved_truth == row.preserved_truth()
    }

    pub fn policy_matches(&self, row: &PrimitiveConstructionIntentArbitrationPolicyRow) -> bool {
        self.authored_act == row.authored_act()
            && self.observed_relations == row.observed_relations()
            && self.conflict_class == row.conflict_class()
            && self.escalation == row.escalation()
            && self.candidates == row.candidates()
            && self.blocked_candidates == row.blocked_candidates()
    }

    pub fn policy_resolution_surface_consistent(
        &self,
        row: &PrimitiveConstructionIntentArbitrationPolicyRow,
    ) -> bool {
        match self.preserved_truth {
            PrimitiveConstructionPreservedIntentTruth::Unresolved { .. } => {
                row.chosen_candidate().is_none()
            }
            PrimitiveConstructionPreservedIntentTruth::Resolved {
                candidate,
                authority,
            } => match authority {
                PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve => {
                    row.chosen_candidate() == Some(candidate)
                }
                PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice => {
                    row.chosen_candidate().is_none() && row.candidates().contains(&candidate)
                }
            },
        }
    }

    pub fn chosen_row_matches(
        &self,
        row: Option<&PrimitiveConstructionChosenIntentResolutionRow>,
    ) -> bool {
        match (self.preserved_truth, row) {
            (
                PrimitiveConstructionPreservedIntentTruth::Resolved {
                    candidate,
                    authority,
                },
                Some(row),
            ) => {
                self.authored_act == row.authored_act()
                    && self.observed_relations == row.observed_relations()
                    && self.conflict_class == row.conflict_class()
                    && candidate == row.chosen_candidate()
                    && authority == row.authority()
            }
            (
                PrimitiveConstructionPreservedIntentTruth::Resolved {
                    authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
                    ..
                },
                None,
            ) => false,
            _ => row.is_none(),
        }
    }

    pub fn dx_matches(&self, row: &PrimitiveConstructionIntentArbitrationDxSurfaceRow) -> bool {
        self.conflict_class == row.conflict_class()
            && self.escalation == row.escalation()
            && self.candidates.len() == row.candidate_count()
            && self.blocked_candidates.len() == row.blocked_candidate_count()
    }

    pub fn dx_surface_consistent(
        &self,
        row: &PrimitiveConstructionIntentArbitrationDxSurfaceRow,
    ) -> bool {
        let expected_surface = match self.escalation {
            SpatialIntentEscalation::AutoResolve(_) => {
                PrimitiveConstructionIntentArbitrationDxSurface::CommonPath
            }
            SpatialIntentEscalation::PreserveCandidates
            | SpatialIntentEscalation::BlockedByMissingCapability(_) => {
                PrimitiveConstructionIntentArbitrationDxSurface::AdvancedPath
            }
            SpatialIntentEscalation::AskForClarification => {
                PrimitiveConstructionIntentArbitrationDxSurface::HumanEscalation
            }
        };
        row.dx_surface() == expected_surface
            && row.chosen_candidate()
                == match self.preserved_truth {
                    PrimitiveConstructionPreservedIntentTruth::Resolved {
                        candidate,
                        authority:
                            PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
                    } => Some(candidate),
                    _ => None,
                }
    }

    pub fn replay_matches(
        &self,
        report: &PrimitiveConstructionIntentArbitrationReplayParityReport,
    ) -> bool {
        report.parity_verified()
            && self.matches_preserved_row(report.direct_row())
            && self.matches_preserved_row(report.replay_row())
    }

    pub fn query_matches(
        &self,
        report: &PrimitiveConstructionQueryIntentArbitrationParityReport,
    ) -> bool {
        report.parity_verified()
            && self.authored_act == report.authored_act()
            && self.observed_relations == report.observed_relations()
            && self.conflict_class == report.conflict_class()
            && self.escalation == report.escalation()
            && self.candidates == report.candidates()
            && self.blocked_candidates == report.blocked_candidates()
            && report.chosen_truth()
                == match self.preserved_truth {
                    PrimitiveConstructionPreservedIntentTruth::Unresolved { .. } => {
                        PrimitiveConstructionIntentChosenTruth::Unresolved
                    }
                    PrimitiveConstructionPreservedIntentTruth::Resolved {
                        candidate,
                        authority,
                    } => PrimitiveConstructionIntentChosenTruth::Resolved {
                        candidate,
                        authority,
                    },
                }
    }
}
