use crate::physical_runtime::record_serving::ServingHealth;
use crate::physical_runtime::{
    PhysicalDurabilityGroupBasis, PhysicalRootPublicationMemberIdentity,
    PhysicalRootPublicationWorkAction, PhysicalRootPublicationWorkFailureCause,
    PhysicalRootPublicationWorkScope, PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition,
    PhysicalWorkSettlementEvidence, RootPublicationPreparedPhysicalMutationMembers,
    RootReplacedPhysicalMutationMembers, SettledPhysicalWork,
};

use super::{PhysicalRootPublicationWorkFailure, PhysicalRootPublicationWorkPort};

pub enum PhysicalRootReplacementOutcome {
    Replaced(RootReplacedPhysicalMutationMembers),
    NotStarted(PhysicalRootReplacementNotStarted),
    InspectionRequired(IndeterminatePhysicalRootReplacement),
}

pub struct PhysicalRootReplacementNotStarted {
    prepared: RootPublicationPreparedPhysicalMutationMembers,
    cause: PhysicalRootReplacementFailureCause,
}

pub struct IndeterminatePhysicalRootReplacement {
    core: super::super::mutation::RootPublicationPreparedCore,
    settlement: SettledPhysicalWork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRootReplacementFailureCause {
    Work(PhysicalRootPublicationWorkFailureCause),
    ProvenNoEffect {
        recovery: PhysicalWorkRecoveryDisposition,
    },
}

pub(in crate::physical_runtime) fn replace_root_candidate(
    prepared: RootPublicationPreparedPhysicalMutationMembers,
    work: &PhysicalRootPublicationWorkPort,
    health: &ServingHealth,
) -> PhysicalRootReplacementOutcome {
    let scope = PhysicalRootPublicationWorkScope::new(
        prepared.identity(),
        PhysicalRootPublicationWorkAction::ReplaceBootstrapCatalog,
    )
    .expect("bootstrap catalog replacement is a valid root-publication action");
    let settlement = match work.execute(scope) {
        Ok(settlement) => settlement,
        Err(failure) => return replacement_not_started(prepared, failure),
    };
    match settlement.evidence() {
        PhysicalWorkSettlementEvidence::PublicationEffect { .. }
            if settlement.evidence().fate() == PhysicalWorkEffectFate::PublicationCompleted =>
        {
            PhysicalRootReplacementOutcome::Replaced(RootReplacedPhysicalMutationMembers::new(
                prepared.into_core(),
                settlement,
            ))
        }
        PhysicalWorkSettlementEvidence::NoEffect(_) => {
            PhysicalRootReplacementOutcome::NotStarted(PhysicalRootReplacementNotStarted {
                prepared,
                cause: PhysicalRootReplacementFailureCause::ProvenNoEffect {
                    recovery: settlement.recovery_disposition(),
                },
            })
        }
        _ => {
            let mut core = prepared.into_core();
            core.require_inspection();
            health.revoke();
            PhysicalRootReplacementOutcome::InspectionRequired(
                IndeterminatePhysicalRootReplacement { core, settlement },
            )
        }
    }
}

fn replacement_not_started(
    prepared: RootPublicationPreparedPhysicalMutationMembers,
    failure: PhysicalRootPublicationWorkFailure,
) -> PhysicalRootReplacementOutcome {
    PhysicalRootReplacementOutcome::NotStarted(PhysicalRootReplacementNotStarted {
        prepared,
        cause: PhysicalRootReplacementFailureCause::Work(failure.cause()),
    })
}

impl PhysicalRootReplacementNotStarted {
    pub const fn cause(&self) -> PhysicalRootReplacementFailureCause {
        self.cause
    }

    pub fn prepared(&self) -> &RootPublicationPreparedPhysicalMutationMembers {
        &self.prepared
    }

    pub fn into_prepared(self) -> RootPublicationPreparedPhysicalMutationMembers {
        self.prepared
    }
}

impl PhysicalRootReplacementOutcome {
    pub(in crate::physical_runtime) fn runtime_released(
        prepared: RootPublicationPreparedPhysicalMutationMembers,
    ) -> Self {
        Self::NotStarted(PhysicalRootReplacementNotStarted {
            prepared,
            cause: PhysicalRootReplacementFailureCause::Work(
                PhysicalRootPublicationWorkFailureCause::RuntimeReleased,
            ),
        })
    }
}

impl IndeterminatePhysicalRootReplacement {
    pub const fn group_basis(&self) -> PhysicalDurabilityGroupBasis {
        self.core.group()
    }

    pub fn members(&self) -> &[PhysicalRootPublicationMemberIdentity] {
        self.core.members()
    }

    pub fn effect_fate(&self) -> PhysicalWorkEffectFate {
        self.settlement.evidence().fate()
    }

    pub const fn recovery_disposition(&self) -> PhysicalWorkRecoveryDisposition {
        self.settlement.recovery_disposition()
    }
}
