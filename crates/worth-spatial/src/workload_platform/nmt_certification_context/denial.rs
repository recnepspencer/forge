use topology::facade::NmtTopologyScopeKind;

use super::NmtCertifiedScopeSet;
use crate::workload_platform::user_response::{WorthUserOutcome, WorthUserOutcomeKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NmtCertificationDenialKind {
    MissingTopologyConstruction,
    MismatchedTopologyConstruction,
    MissingReceiptBackedStage,
    MissingScopeGeometry,
    MissingScopeProjection,
    MissingScopeReplay,
    AggregateReceiptWithoutScopeProof,
    CrossScopeProjection,
    CrossScopeRetainedReplay,
    CrossScopeSurfaceSupport,
    CrossScopeParity,
    MissingScopeEvidence,
    LabelOnlyMotion,
    FalseClosure,
    StormOverlapSmuggling,
    UnsupportedSurface,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NmtScopeAttackCounters {
    touched_scopes: usize,
    scope_entities_read: usize,
    projection_entities_read: usize,
    retained_checkpoints_read: usize,
    parity_lanes_read: usize,
    diagnostic_scopes_materialized: usize,
}

impl NmtScopeAttackCounters {
    pub(crate) fn new(
        touched_scopes: usize,
        scope_entities_read: usize,
        projection_entities_read: usize,
        retained_checkpoints_read: usize,
        parity_lanes_read: usize,
        diagnostic_scopes_materialized: usize,
    ) -> Self {
        Self {
            touched_scopes,
            scope_entities_read,
            projection_entities_read,
            retained_checkpoints_read,
            parity_lanes_read,
            diagnostic_scopes_materialized,
        }
    }

    pub fn touched_scopes(self) -> usize {
        self.touched_scopes
    }

    pub fn scope_entities_read(self) -> usize {
        self.scope_entities_read
    }

    pub fn projection_entities_read(self) -> usize {
        self.projection_entities_read
    }

    pub fn retained_checkpoints_read(self) -> usize {
        self.retained_checkpoints_read
    }

    pub fn parity_lanes_read(self) -> usize {
        self.parity_lanes_read
    }

    pub fn diagnostic_scopes_materialized(self) -> usize {
        self.diagnostic_scopes_materialized
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtCertificationDenial {
    kind: NmtCertificationDenialKind,
    target_scope_identity: Option<String>,
    source_scope_identity: Option<String>,
    target_scope_kind: Option<NmtTopologyScopeKind>,
    consumed_evidence_digest: String,
    human_reason: String,
    counters: NmtScopeAttackCounters,
}

impl NmtCertificationDenial {
    pub(crate) fn new(input: NmtCertificationDenialInput) -> Self {
        Self {
            kind: input.kind,
            target_scope_identity: input.target_scope_identity,
            source_scope_identity: input.source_scope_identity,
            target_scope_kind: input.target_scope_kind,
            consumed_evidence_digest: input.consumed_evidence_digest,
            human_reason: input.human_reason,
            counters: input.counters,
        }
    }

    pub fn kind(&self) -> &NmtCertificationDenialKind {
        &self.kind
    }

    pub fn target_scope_identity(&self) -> Option<&str> {
        self.target_scope_identity.as_deref()
    }

    pub fn source_scope_identity(&self) -> Option<&str> {
        self.source_scope_identity.as_deref()
    }

    pub fn target_scope_kind(&self) -> Option<NmtTopologyScopeKind> {
        self.target_scope_kind
    }

    pub fn consumed_evidence_digest(&self) -> &str {
        &self.consumed_evidence_digest
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn counters(&self) -> NmtScopeAttackCounters {
        self.counters
    }
}

pub(crate) struct NmtCertificationDenialInput {
    pub kind: NmtCertificationDenialKind,
    pub target_scope_identity: Option<String>,
    pub source_scope_identity: Option<String>,
    pub target_scope_kind: Option<NmtTopologyScopeKind>,
    pub consumed_evidence_digest: String,
    pub human_reason: String,
    pub counters: NmtScopeAttackCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NmtBossId {
    OpenRadialFan,
    MixedSurfaceKillBox,
    OpenClassTriadParity,
    GrazingBasketStack,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtBossOutcomeMatrixEvidence {
    outcomes: Vec<WorthUserOutcome>,
}

impl NmtBossOutcomeMatrixEvidence {
    pub fn from_outcomes(outcomes: Vec<WorthUserOutcome>) -> Self {
        Self { outcomes }
    }

    pub fn outcomes(&self) -> &[WorthUserOutcome] {
        &self.outcomes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NmtBossCloseoutDenial {
    MissingCertifiedScopeSet,
    WrongCertifiedScopeSet {
        boss: NmtBossId,
        expected: &'static str,
        actual_scope_kinds: Vec<NmtTopologyScopeKind>,
    },
    MissingOutcomeKind(WorthUserOutcomeKind),
    MissingHumanReadableReason,
    MissingEvidenceDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtBossCloseoutReceipt {
    boss: NmtBossId,
    certified_scope_set_identity: String,
    outcome_count: usize,
}

impl NmtBossCloseoutReceipt {
    pub fn from_certified_scope_set(
        boss: NmtBossId,
        scopes: &NmtCertifiedScopeSet,
        matrix: &NmtBossOutcomeMatrixEvidence,
    ) -> Result<Self, NmtBossCloseoutDenial> {
        if scopes.scopes().is_empty() {
            return Err(NmtBossCloseoutDenial::MissingCertifiedScopeSet);
        }
        require_scope_shape(boss, scopes)?;
        require_outcome(matrix, WorthUserOutcomeKind::Admitted)?;
        require_outcome(matrix, WorthUserOutcomeKind::Unsupported)?;
        require_outcome(matrix, WorthUserOutcomeKind::Denied)?;
        require_outcome(matrix, WorthUserOutcomeKind::IntegrityMismatch)?;
        require_outcome(matrix, WorthUserOutcomeKind::NoOptions)?;
        for outcome in matrix.outcomes() {
            if outcome.evidence().digest().trim().is_empty() {
                return Err(NmtBossCloseoutDenial::MissingEvidenceDigest);
            }
            if outcome
                .human_response()
                .summary()
                .split_whitespace()
                .count()
                < 3
            {
                return Err(NmtBossCloseoutDenial::MissingHumanReadableReason);
            }
        }
        Ok(Self {
            boss,
            certified_scope_set_identity: scopes.parent_construction_identity().to_string(),
            outcome_count: matrix.outcomes().len(),
        })
    }

    pub fn boss(&self) -> NmtBossId {
        self.boss
    }

    pub fn certified_scope_set_identity(&self) -> &str {
        &self.certified_scope_set_identity
    }

    pub fn outcome_count(&self) -> usize {
        self.outcome_count
    }
}

fn require_scope_shape(
    boss: NmtBossId,
    scopes: &NmtCertifiedScopeSet,
) -> Result<(), NmtBossCloseoutDenial> {
    let kinds = scopes
        .scopes()
        .iter()
        .map(|scope| scope.topology_scope().kind())
        .collect::<Vec<_>>();
    let valid = match boss {
        NmtBossId::OpenRadialFan => kinds == [NmtTopologyScopeKind::OpenRadialFan],
        NmtBossId::MixedSurfaceKillBox => kinds == [NmtTopologyScopeKind::OpenSheet],
        NmtBossId::OpenClassTriadParity => {
            kinds.contains(&NmtTopologyScopeKind::OpenWire)
                && kinds.contains(&NmtTopologyScopeKind::OpenSheet)
                && kinds.contains(&NmtTopologyScopeKind::OpenRadialFan)
        }
        NmtBossId::GrazingBasketStack => kinds
            .iter()
            .all(|kind| *kind == NmtTopologyScopeKind::OpenLayer),
    };
    if valid {
        Ok(())
    } else {
        Err(NmtBossCloseoutDenial::WrongCertifiedScopeSet {
            boss,
            expected: expected_scope_shape(boss),
            actual_scope_kinds: kinds,
        })
    }
}

fn expected_scope_shape(boss: NmtBossId) -> &'static str {
    match boss {
        NmtBossId::OpenRadialFan => "exactly one open radial fan scope",
        NmtBossId::MixedSurfaceKillBox => "exactly one open sheet scope",
        NmtBossId::OpenClassTriadParity => "open wire, open sheet, and open radial fan scopes",
        NmtBossId::GrazingBasketStack => "one or more open layer scopes",
    }
}

fn require_outcome(
    matrix: &NmtBossOutcomeMatrixEvidence,
    kind: WorthUserOutcomeKind,
) -> Result<(), NmtBossCloseoutDenial> {
    if matrix
        .outcomes()
        .iter()
        .any(|outcome| outcome.kind() == kind)
    {
        Ok(())
    } else {
        Err(NmtBossCloseoutDenial::MissingOutcomeKind(kind))
    }
}
