use crate::identity::hash_parts;

use super::support_matrix::{EffectSupportCause, EffectSupportPosture};
use super::taxonomy::{DeniedEffectEligibilityKind, EffectFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectDeferredNeighborFamily {
    StoreBackedExecutionParity,
    DurableReplayAndRestartStableEnvelope,
}

impl EffectDeferredNeighborFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StoreBackedExecutionParity => "store_backed_execution_parity",
            Self::DurableReplayAndRestartStableEnvelope => {
                "durable_replay_and_restart_stable_envelope"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectDeferredResiduePosture {
    ZeroOperationalResidue,
}

impl EffectDeferredResiduePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ZeroOperationalResidue => "zero_operational_residue",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectDeferredSupportContract {
    neighbor_family: EffectDeferredNeighborFamily,
    denial_kind: DeniedEffectEligibilityKind,
    residue_posture: EffectDeferredResiduePosture,
    contract_digest: String,
}

impl EffectDeferredSupportContract {
    fn new(
        neighbor_family: EffectDeferredNeighborFamily,
        denial_kind: DeniedEffectEligibilityKind,
    ) -> Self {
        let residue_posture = EffectDeferredResiduePosture::ZeroOperationalResidue;
        let contract_digest = hash_parts(&[
            "effect_deferred_support_contract_v1".to_string(),
            format!("neighbor:{}", neighbor_family.as_str()),
            format!("denial_kind:{}", denial_kind.as_str()),
            format!("residue:{}", residue_posture.as_str()),
        ]);
        Self {
            neighbor_family,
            denial_kind,
            residue_posture,
            contract_digest,
        }
    }

    pub fn neighbor_family(&self) -> EffectDeferredNeighborFamily {
        self.neighbor_family
    }

    pub fn denial_kind(&self) -> DeniedEffectEligibilityKind {
        self.denial_kind
    }

    pub fn residue_posture(&self) -> EffectDeferredResiduePosture {
        self.residue_posture
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn leaves_zero_operational_residue(&self) -> bool {
        self.residue_posture == EffectDeferredResiduePosture::ZeroOperationalResidue
    }
}

const EMPTY_DENIAL_KINDS: &[DeniedEffectEligibilityKind] = &[];
const UNSUPPORTED_DENIAL_KINDS: &[DeniedEffectEligibilityKind] =
    &[DeniedEffectEligibilityKind::UnsupportedForBasisFamily];
const BRANCH_AUTHORITY_DENIAL_KINDS: &[DeniedEffectEligibilityKind] =
    &[DeniedEffectEligibilityKind::BranchAuthorityRequired];
const PREVIEW_REBIND_DENIAL_KINDS: &[DeniedEffectEligibilityKind] =
    &[DeniedEffectEligibilityKind::PreviewRebindRequired];
const STORE_BACKED_DEFERRED_DENIAL_KINDS: &[DeniedEffectEligibilityKind] =
    &[DeniedEffectEligibilityKind::StoreBackedExecutionDeferred];
const DURABLE_REPLAY_DEFERRED_DENIAL_KINDS: &[DeniedEffectEligibilityKind] =
    &[DeniedEffectEligibilityKind::DurableReplayDeferred];

const EMPTY_DEFERRED_NEIGHBORS: &[EffectDeferredNeighborFamily] = &[];
const WRITEBACK_DEFERRED_NEIGHBORS: &[EffectDeferredNeighborFamily] = &[
    EffectDeferredNeighborFamily::StoreBackedExecutionParity,
    EffectDeferredNeighborFamily::DurableReplayAndRestartStableEnvelope,
];

pub(crate) fn support_denial_kinds(
    posture: EffectSupportPosture,
    cause: EffectSupportCause,
) -> &'static [DeniedEffectEligibilityKind] {
    match (posture, cause) {
        (EffectSupportPosture::Unsupported, EffectSupportCause::UnsupportedForBasisFamily) => {
            UNSUPPORTED_DENIAL_KINDS
        }
        (EffectSupportPosture::Denied, EffectSupportCause::BranchAuthorityRequired) => {
            BRANCH_AUTHORITY_DENIAL_KINDS
        }
        (EffectSupportPosture::RebindRequired, EffectSupportCause::PreviewRebindRequired) => {
            PREVIEW_REBIND_DENIAL_KINDS
        }
        (EffectSupportPosture::Deferred, EffectSupportCause::StoreBackedExecutionDeferred) => {
            STORE_BACKED_DEFERRED_DENIAL_KINDS
        }
        (EffectSupportPosture::Deferred, EffectSupportCause::DurableReplayDeferred) => {
            DURABLE_REPLAY_DEFERRED_DENIAL_KINDS
        }
        _ => EMPTY_DENIAL_KINDS,
    }
}

pub(crate) fn support_deferred_neighbors(
    effect_family: EffectFamily,
) -> &'static [EffectDeferredNeighborFamily] {
    match effect_family {
        EffectFamily::Writeback => WRITEBACK_DEFERRED_NEIGHBORS,
        EffectFamily::Mutation | EffectFamily::Merge => EMPTY_DEFERRED_NEIGHBORS,
    }
}

pub(crate) fn deferred_support_contract(
    cause: EffectSupportCause,
) -> Option<EffectDeferredSupportContract> {
    match cause {
        EffectSupportCause::StoreBackedExecutionDeferred => {
            Some(EffectDeferredSupportContract::new(
                EffectDeferredNeighborFamily::StoreBackedExecutionParity,
                DeniedEffectEligibilityKind::StoreBackedExecutionDeferred,
            ))
        }
        EffectSupportCause::DurableReplayDeferred => Some(EffectDeferredSupportContract::new(
            EffectDeferredNeighborFamily::DurableReplayAndRestartStableEnvelope,
            DeniedEffectEligibilityKind::DurableReplayDeferred,
        )),
        _ => None,
    }
}
