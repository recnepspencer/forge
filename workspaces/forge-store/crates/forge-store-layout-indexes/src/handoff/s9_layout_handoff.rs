use super::{
    S8HazardEvidenceRequirement, S8LayoutHazardInventory, S9DownstreamProtocolTargetInventory,
    S9FormalModelTarget, S9LayoutMachineContract, S9LayoutStateMachine,
    S9LayoutStateMachineInventory,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S9LayoutHandoffDenial {
    IncompleteHazardInventory,
    MissingStateMachine(S9LayoutStateMachine),
    HazardTransitionOutsideMachine,
    MissingRuntimeEvidenceObligation,
    IncompleteDownstreamProtocolDestinations,
}

/// S.9's explicit Store-owned layout grammar. This is deliberately not a
/// runtime or test-success witness; proof lanes remain owned by their actual
/// compile-fail, simulation, formal-model, and runtime producers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageFoundationS9LayoutHandoff {
    hazards: S8LayoutHazardInventory,
    machines: S9LayoutStateMachineInventory,
    downstream_protocols: S9DownstreamProtocolTargetInventory,
}

pub(crate) fn admit_s9_layout_handoff(
    inventory: S8LayoutHazardInventory,
) -> Result<StorageFoundationS9LayoutHandoff, S9LayoutHandoffDenial> {
    if !inventory.is_complete() {
        return Err(S9LayoutHandoffDenial::IncompleteHazardInventory);
    }
    let machines = S9LayoutStateMachineInventory::canonical();
    let downstream_protocols = S9DownstreamProtocolTargetInventory::canonical();
    if !machines.is_complete() || !downstream_protocols.declares_all_destinations() {
        return Err(S9LayoutHandoffDenial::IncompleteDownstreamProtocolDestinations);
    }
    for row in inventory.rows() {
        let Some(contract) = machines.contract(row.machine()) else {
            return Err(S9LayoutHandoffDenial::MissingStateMachine(row.machine()));
        };
        if !contract.permits_edge(row.transition_from(), row.transition(), row.transition_to()) {
            return Err(S9LayoutHandoffDenial::HazardTransitionOutsideMachine);
        }
    }
    if !inventory.rows().iter().any(|row| {
        row.evidence_requirement() == S8HazardEvidenceRequirement::OwnerBoundExactCounters
    }) {
        return Err(S9LayoutHandoffDenial::MissingRuntimeEvidenceObligation);
    }
    Ok(StorageFoundationS9LayoutHandoff {
        hazards: inventory,
        machines,
        downstream_protocols,
    })
}

impl StorageFoundationS9LayoutHandoff {
    pub const fn inventory(&self) -> S8LayoutHazardInventory {
        self.hazards
    }
    pub const fn machine_inventory(&self) -> S9LayoutStateMachineInventory {
        self.machines
    }
    pub const fn downstream_protocol_targets(&self) -> S9DownstreamProtocolTargetInventory {
        self.downstream_protocols
    }
    pub fn requires(&self, machine: S9LayoutStateMachine) -> bool {
        self.machines.requires(machine)
    }

    pub fn machine_contract(
        &self,
        machine: S9LayoutStateMachine,
    ) -> Option<S9LayoutMachineContract> {
        self.machines.contract(machine)
    }

    pub fn declares_pending_protocol_target(&self, target: S9FormalModelTarget) -> bool {
        self.downstream_protocols.contains(target)
    }

    /// The complete, lower-owned transition obligations for one S.9 machine.
    /// Consumers model these rows directly rather than reconstructing a
    /// generic summary from counters, certificates, or runtime labels.
    pub fn obligations_for(
        &self,
        machine: S9LayoutStateMachine,
    ) -> impl Iterator<Item = crate::handoff::S8LayoutHazardRow> {
        self.hazards
            .rows()
            .iter()
            .copied()
            .filter(move |row| row.machine() == machine)
    }

    pub fn compaction_cutover_owner_transitions(
        &self,
    ) -> impl Iterator<Item = forge_store_physical_isolation::CompactionCutoverTransition> {
        forge_store_physical_isolation::compaction_cutover_outcome_facts()
    }
}
