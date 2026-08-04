use super::{
    PersistedPhysicalMutationFate, PhysicalMutationIdempotencyBindingState,
    PhysicalMutationIdempotencyRegistry, PhysicalNamespaceDurableCheckpointGeneration,
    RebuiltPhysicalMutationBindingState,
};
use crate::physical_runtime::durability::closeout::{
    PhysicalRecoveryAttemptBindingFact, PhysicalRecoveryCompletedMutationFact,
    PhysicalRecoveryOperationFact, PhysicalRecoveryOperationFate,
    PhysicalRecoveryWalAttemptBinding,
};
use crate::physical_runtime::durability::mutation::idempotency::fate::PersistedIndeterminatePhysicalMutationBasis;

impl PhysicalMutationIdempotencyRegistry {
    pub(in crate::physical_runtime::durability) fn into_closeout_facts(
        self,
    ) -> (
        PhysicalNamespaceDurableCheckpointGeneration,
        Box<[PhysicalRecoveryOperationFact]>,
    ) {
        let facts = self
            .bindings
            .into_values()
            .map(closeout_fact)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        (self.generation, facts)
    }
}

fn closeout_fact(state: PhysicalMutationIdempotencyBindingState) -> PhysicalRecoveryOperationFact {
    let (basis, attempt, fate, last_compacted) = match state {
        PhysicalMutationIdempotencyBindingState::Unsealed(basis) => (
            basis,
            PhysicalRecoveryAttemptBindingFact::Unsealed,
            PhysicalRecoveryOperationFate::Unresolved,
            None,
        ),
        PhysicalMutationIdempotencyBindingState::GroupSealed { basis, group } => (
            basis,
            PhysicalRecoveryAttemptBindingFact::GroupSealed(group),
            PhysicalRecoveryOperationFate::Unresolved,
            None,
        ),
        PhysicalMutationIdempotencyBindingState::RebuiltUnresolved { basis, prior } => {
            let attempt = match prior {
                RebuiltPhysicalMutationBindingState::Unsealed => {
                    PhysicalRecoveryAttemptBindingFact::Unsealed
                }
                RebuiltPhysicalMutationBindingState::GroupSealed(group) => {
                    PhysicalRecoveryAttemptBindingFact::GroupSealed(group)
                }
            };
            (
                basis,
                attempt,
                PhysicalRecoveryOperationFate::Unresolved,
                None,
            )
        }
        PhysicalMutationIdempotencyBindingState::WalBound { basis, persisted } => {
            let attempt = PhysicalRecoveryAttemptBindingFact::WalBound(
                PhysicalRecoveryWalAttemptBinding::from_persisted(&persisted),
            );
            (
                basis,
                attempt,
                PhysicalRecoveryOperationFate::Unresolved,
                None,
            )
        }
        PhysicalMutationIdempotencyBindingState::Terminal {
            basis,
            fate,
            last_compacted,
        } => {
            let (attempt, fate) = terminal_fact(fate);
            (basis, attempt, fate, last_compacted)
        }
    };
    PhysicalRecoveryOperationFact::new(
        basis.key().identity(),
        basis.key().lease(),
        basis.fingerprint(),
        basis.mutation(),
        attempt,
        fate,
        last_compacted,
    )
}

fn terminal_fact(
    fate: PersistedPhysicalMutationFate,
) -> (
    PhysicalRecoveryAttemptBindingFact,
    PhysicalRecoveryOperationFate,
) {
    match fate {
        PersistedPhysicalMutationFate::ProvenNoEffect(fate) => (
            PhysicalRecoveryAttemptBindingFact::Unsealed,
            PhysicalRecoveryOperationFate::ProvenNoEffect(fate),
        ),
        PersistedPhysicalMutationFate::Completed(completed) => {
            let (binding, fact) = completed.into_parts();
            (
                PhysicalRecoveryAttemptBindingFact::WalBound(
                    PhysicalRecoveryWalAttemptBinding::from_persisted(&binding),
                ),
                PhysicalRecoveryOperationFate::Completed(
                    PhysicalRecoveryCompletedMutationFact::from_completed(&fact),
                ),
            )
        }
        PersistedPhysicalMutationFate::Indeterminate(indeterminate) => {
            let (basis, fate) = indeterminate.into_parts();
            let attempt = match basis {
                PersistedIndeterminatePhysicalMutationBasis::Unsealed => {
                    PhysicalRecoveryAttemptBindingFact::Unsealed
                }
                PersistedIndeterminatePhysicalMutationBasis::GroupSealed(group) => {
                    PhysicalRecoveryAttemptBindingFact::GroupSealed(group)
                }
                PersistedIndeterminatePhysicalMutationBasis::WalBound(binding) => {
                    PhysicalRecoveryAttemptBindingFact::WalBound(
                        PhysicalRecoveryWalAttemptBinding::from_persisted(&binding),
                    )
                }
            };
            (attempt, PhysicalRecoveryOperationFate::Indeterminate(fate))
        }
    }
}
