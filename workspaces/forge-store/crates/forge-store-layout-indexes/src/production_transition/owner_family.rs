use super::{S8LayoutProductionOperation, S8LayoutProductionTransition, S8LayoutStateMachine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8OwnerOutcomeFamilyContract {
    machine: S8LayoutStateMachine,
    production_operation: S8LayoutProductionOperation,
    transitions: &'static [S8LayoutProductionTransition],
}

impl S8OwnerOutcomeFamilyContract {
    const fn new(
        machine: S8LayoutStateMachine,
        production_operation: S8LayoutProductionOperation,
        transitions: &'static [S8LayoutProductionTransition],
    ) -> Self {
        Self {
            machine,
            production_operation,
            transitions,
        }
    }

    pub const fn machine(self) -> S8LayoutStateMachine {
        self.machine
    }
    pub const fn production_operation(self) -> S8LayoutProductionOperation {
        self.production_operation
    }
    pub const fn transitions(self) -> &'static [S8LayoutProductionTransition] {
        self.transitions
    }
    pub fn contains(self, fact: S8LayoutProductionTransition) -> bool {
        self.transitions.contains(&fact)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutMachineContract {
    machine: S8LayoutStateMachine,
    owner_families: Box<[S8OwnerOutcomeFamilyContract]>,
    transitions: Box<[S8LayoutProductionTransition]>,
}

impl S8LayoutMachineContract {
    pub(crate) fn aggregate(
        machine: S8LayoutStateMachine,
        owner_families: impl IntoIterator<Item = S8OwnerOutcomeFamilyContract>,
    ) -> Self {
        let owner_families: Box<[_]> = owner_families.into_iter().collect();
        let transitions = owner_families
            .iter()
            .flat_map(|family| family.transitions().iter().copied())
            .collect();
        Self {
            machine,
            owner_families,
            transitions,
        }
    }

    pub(crate) fn single(owner_family: S8OwnerOutcomeFamilyContract) -> Self {
        Self::aggregate(owner_family.machine(), [owner_family])
    }

    pub const fn machine(&self) -> S8LayoutStateMachine {
        self.machine
    }
    pub fn owner_families(&self) -> &[S8OwnerOutcomeFamilyContract] {
        &self.owner_families
    }
    pub fn transitions(&self) -> &[S8LayoutProductionTransition] {
        &self.transitions
    }
    pub fn contains(&self, fact: S8LayoutProductionTransition) -> bool {
        fact.machine() == self.machine && self.owner_families.iter().any(|f| f.contains(fact))
    }
    pub fn production_operation(&self) -> S8LayoutProductionOperation {
        self.owner_families
            .first()
            .expect("machine contract requires an owner family")
            .production_operation()
    }
    pub fn permits_edge(
        &self,
        from: super::S8LayoutMachineState,
        transition: super::S8LayoutMachineTransition,
        to: super::S8LayoutMachineState,
    ) -> bool {
        self.transitions.iter().copied().any(|fact| {
            let edge = fact.edge();
            edge.from() == from && edge.transition() == transition && edge.to() == to
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8OwnerTransitionContract(S8OwnerOutcomeFamilyContract);

impl S8OwnerTransitionContract {
    pub(crate) const fn from_owner_outcomes(
        machine: S8LayoutStateMachine,
        production_operation: S8LayoutProductionOperation,
        transitions: &'static [S8LayoutProductionTransition],
    ) -> Self {
        Self(S8OwnerOutcomeFamilyContract::new(
            machine,
            production_operation,
            transitions,
        ))
    }

    pub(crate) const fn owner_family(self) -> S8OwnerOutcomeFamilyContract {
        self.0
    }

    pub(crate) fn handoff_contract(self) -> S8LayoutMachineContract {
        S8LayoutMachineContract::single(self.owner_family())
    }
}
