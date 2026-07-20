use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphObservationPosture {
    NotRequired,
    Snapshot,
    Live,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphProjectionPosture {
    NotRequired,
    NativeProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphMutationPosture {
    NotRequired,
    TouchAndEffect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphIdentityPosture {
    Opaque,
    PreservedLineage,
    EvolvingLineage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphLocalityPosture {
    InProcess,
    ExternalBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphBudgetPosture {
    ConstantAdmission,
    DeclaredBreadth,
    ExternalBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphCommitPosture {
    ReadOnly,
    AtomicAuthorityRequired,
    CompensationRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphFailureTopology {
    Local,
    BoundaryFailure,
    PartialCommitPossible,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryGraphParticipationContract {
    pub observation: WorthQueryGraphObservationPosture,
    pub projection: WorthQueryGraphProjectionPosture,
    pub mutation: WorthQueryGraphMutationPosture,
    pub identity: WorthQueryGraphIdentityPosture,
    pub locality: WorthQueryGraphLocalityPosture,
    pub budget: WorthQueryGraphBudgetPosture,
    pub commit: WorthQueryGraphCommitPosture,
    pub failure: WorthQueryGraphFailureTopology,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphParticipationDefinition<G> {
    role: String,
    contract: WorthQueryGraphParticipationContract,
    _marker: PhantomData<fn() -> G>,
}

impl<G> WorthQueryGraphParticipationDefinition<G> {
    pub fn new(role: impl Into<String>, contract: WorthQueryGraphParticipationContract) -> Self {
        Self {
            role: role.into(),
            contract,
            _marker: PhantomData,
        }
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn contract(&self) -> &WorthQueryGraphParticipationContract {
        &self.contract
    }

    pub(crate) fn validate(&self) -> Result<(), &'static str> {
        if self.role.trim().is_empty() {
            return Err("graph participation role must not be empty");
        }
        if self.contract.commit == WorthQueryGraphCommitPosture::ReadOnly
            && self.contract.mutation != WorthQueryGraphMutationPosture::NotRequired
        {
            return Err("a mutating graph participation cannot declare read-only commit posture");
        }
        if self.contract.commit == WorthQueryGraphCommitPosture::AtomicAuthorityRequired
            && self.contract.failure == WorthQueryGraphFailureTopology::PartialCommitPossible
        {
            return Err("atomic commit authority cannot declare partial-commit failure topology");
        }
        Ok(())
    }
}
