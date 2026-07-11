pub use crate::production_transition::{
    S8LayoutMachineContract as S9LayoutMachineContract, S8LayoutMachineEdge as S9LayoutMachineEdge,
    S8LayoutMachineState as S9LayoutMachineState,
    S8LayoutMachineTransition as S9LayoutMachineTransition,
    S8LayoutProductionOperation as S9LayoutProductionOperation,
    S8LayoutProductionTransition as S9LayoutProductionTransition,
    S8LayoutStateMachine as S9LayoutStateMachine,
};

#[cfg(test)]
impl S9LayoutMachineContract {
    pub(crate) fn for_machine(machine: S9LayoutStateMachine) -> Self {
        S9LayoutStateMachineInventory::canonical()
            .contract(machine)
            .expect("required owner contract")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S9FormalModelTarget {
    WalCheckpointPageFlushOrdering,
    RecoverySourcePrecedence,
    CompactionCutover,
    PhysicalLeaseReclaimBarrier,
    RepairQuarantine,
    ReplicationImportAdmission,
}

/// Protocol destinations S.8 identifies for later owner publication. Their
/// presence is a handoff obligation, never proof that S.9 has modeled them.
pub const S9_DOWNSTREAM_PROTOCOL_DESTINATIONS: [S9FormalModelTarget; 6] = [
    S9FormalModelTarget::WalCheckpointPageFlushOrdering,
    S9FormalModelTarget::RecoverySourcePrecedence,
    S9FormalModelTarget::CompactionCutover,
    S9FormalModelTarget::PhysicalLeaseReclaimBarrier,
    S9FormalModelTarget::RepairQuarantine,
    S9FormalModelTarget::ReplicationImportAdmission,
];

macro_rules! owner_registry {
    ($( $machine:ident => [$( $owner:expr ),+ $(,)?] ),+ $(,)?) => {
        pub const S9_REQUIRED_LAYOUT_MACHINES: [S9LayoutStateMachine; owner_registry!(@count $($machine),+)] = [
            $(S9LayoutStateMachine::$machine),+
        ];

        fn owner_machine_contract(
            machine: S9LayoutStateMachine,
        ) -> Option<S9LayoutMachineContract> {
            match machine {
                $(S9LayoutStateMachine::$machine => Some(
                    S9LayoutMachineContract::aggregate(
                        S9LayoutStateMachine::$machine,
                        [$($owner.owner_family()),+],
                    )
                )),+
            }
        }
    };
    (@count $head:ident $(, $tail:ident)*) => {
        1usize $(+ owner_registry!(@one $tail))*
    };
    (@one $item:ident) => { 1usize };
}

owner_registry!(
    ArtifactDeclaration => [crate::PhysicalArtifactFamilyDeclaration::owner_transition_contract()],
    KeyDomainAdmission => [crate::S8KeyDomainAdmissionOutcome::owner_transition_contract()],
    StrategyInvariantAdmission => [crate::strategy::S8StrategyInvariantAdmissionOutcome::owner_transition_contract()],
    LayoutAdmission => [crate::strategy_registry::S8LayoutAdmissionOutcome::owner_transition_contract()],
    AccessSelectionAndBudgetAdmission => [crate::planning::S8AccessPlanSelectionOutcome::indexed_contract()],
    AccessLowering => [crate::S8AccessLoweringOutcome::indexed_contract()],
    ExecutionReadiness => [crate::S8ExecutionReadinessOutcome::indexed_contract()],
    ExecutedEvidence => [
        crate::S8ExecutedCounterAdmissionOutcome::indexed_contract(),
        crate::S8ExecutedEvidenceOutcome::indexed_contract(),
    ],
    DerivedRebuildParity => [
        crate::S8DerivedIndexRebuildOutcome::owner_transition_contract(),
        crate::S8DerivedIndexParityOutcome::owner_transition_contract()
    ],
    LiveMaintenanceAdmissionAndLowering => [
        crate::S8LayoutMutationAdmissionOutcome::owner_transition_contract(),
        crate::S8IndexMaintenanceTransitionOutcome::owner_transition_contract()
    ],
    MigrationRollbackPlanning => [
        crate::S8MigrationPlanningOutcome::owner_transition_contract(),
        crate::S8RollbackPlanningOutcome::owner_transition_contract()
    ],
    StaleRebindReadmission => [crate::S8StaleReadmissionOutcome::contract()],
    CorruptionQuarantine => [
        crate::S8LayoutCorruptionOutcome::owner_transition_contract(),
        crate::S8LayoutReadmissionOutcome::owner_transition_contract()
    ],
    BootstrapCatalogDiscovery => [crate::S8BootstrapCatalogReadOutcome::owner_transition_contract()],
    FullDeclaredScanAdmission => [crate::access_shape::S8FullDeclaredScanOutcome::owner_transition_contract()],
    DegradedExactScan => [
        crate::planning::S8AccessPlanSelectionOutcome::degraded_contract(),
        crate::S8AccessLoweringOutcome::degraded_contract(),
        crate::S8ExecutionReadinessOutcome::degraded_contract(),
        crate::S8ExecutedCounterAdmissionOutcome::degraded_contract(),
        crate::S8ExecutedEvidenceOutcome::degraded_contract(),
    ],
    MaterializationCoverageAbsence => [crate::S8PhysicalAbsenceOutcome::owner_transition_contract()],
    BTreeSearchPathInvariant => [crate::strategy::S8BTreeSearchOutcome::<()>::owner_transition_contract()],
    CompactionCutover => [
        crate::handoff::compaction_cutover::lower_rewrite_contract(),
        crate::handoff::compaction_cutover::tombstone_retention_contract(),
        crate::handoff::compaction_cutover::publication_contract(),
        crate::handoff::compaction_cutover::recovery_visibility_contract(),
        crate::handoff::compaction_cutover::reclaim_deferral_contract(),
        crate::handoff::compaction_cutover::reclaim_drain_contract(),
        crate::handoff::compaction_cutover::mutation_denial_contract(),
    ],
    LegacyDisposition => [crate::LegacySurfaceDispositionOutcome::owner_transition_contract()],
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S9LayoutStateMachineInventory {
    _seal: (),
}

impl S9LayoutStateMachineInventory {
    pub(crate) const fn canonical() -> Self {
        Self { _seal: () }
    }

    pub fn contracts(self) -> impl ExactSizeIterator<Item = S9LayoutMachineContract> {
        S9_REQUIRED_LAYOUT_MACHINES.into_iter().map(move |machine| {
            self.contract(machine)
                .expect("required owner outcome contract")
        })
    }

    pub fn contract(self, machine: S9LayoutStateMachine) -> Option<S9LayoutMachineContract> {
        owner_machine_contract(machine)
    }

    pub fn requires(self, machine: S9LayoutStateMachine) -> bool {
        self.contract(machine).is_some()
    }

    pub fn is_complete(self) -> bool {
        self.contracts().len() == S9_REQUIRED_LAYOUT_MACHINES.len()
            && S9_REQUIRED_LAYOUT_MACHINES
                .into_iter()
                .all(|machine| self.requires(machine))
            && self.contracts().all(|contract| {
                !contract.transitions().is_empty()
                    && contract.owner_families().iter().all(|family| {
                        family.machine() == contract.machine()
                            && family.transitions().iter().all(|transition| {
                                transition.machine() == contract.machine()
                                    && transition.production_operation()
                                        == family.production_operation()
                            })
                    })
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8HazardProofLane {
    CompileFail,
    Runtime,
    Simulation,
    FormalModel,
}
