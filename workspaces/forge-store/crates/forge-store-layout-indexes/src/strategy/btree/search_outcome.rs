use crate::production_transition::{
    S8LayoutMachineState as State, S8LayoutMachineTransition as Edge,
    S8LayoutProductionOperation as Operation, S8LayoutProductionTransition,
    S8LayoutStateMachine as Machine, S8OwnerIssuedResult, S8OwnerTransitionContract,
};
use crate::strategy::S8StrategyDenial;

#[derive(Debug, PartialEq, Eq)]
pub struct S8BTreeSearchOutcome<T> {
    issued: S8OwnerIssuedResult<T, S8StrategyDenial>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8BTreeSearchOutcomeView<'a, T> {
    Validated(&'a T),
    Denied(&'a S8StrategyDenial),
}

impl<T> S8BTreeSearchOutcome<T> {
    pub(super) fn issue(result: Result<T, S8StrategyDenial>) -> Self {
        let issued = match result {
            Ok(value) => S8OwnerIssuedResult::admitted(value, Self::validated_transition()),
            Err(denial) => S8OwnerIssuedResult::denied(denial, Self::denied_transition()),
        };
        Self { issued }
    }

    pub fn view(&self) -> S8BTreeSearchOutcomeView<'_, T> {
        match self.issued.result() {
            Ok(value) => S8BTreeSearchOutcomeView::Validated(value),
            Err(denial) => S8BTreeSearchOutcomeView::Denied(denial),
        }
    }

    pub const fn production_transition(&self) -> S8LayoutProductionTransition {
        self.issued.transition()
    }
    pub fn into_result(self) -> Result<T, S8StrategyDenial> {
        self.issued.into_result()
    }
    pub fn unwrap(self) -> T {
        self.into_result().unwrap()
    }
    pub fn unwrap_err(self) -> S8StrategyDenial
    where
        T: core::fmt::Debug,
    {
        self.into_result().unwrap_err()
    }

    const fn validated_transition() -> S8LayoutProductionTransition {
        crate::production_transition::owner_transition(
            Machine::BTreeSearchPathInvariant,
            Operation::VerifyBTreeSearchPath,
            "SeparatorValidated",
            State::CanonicalKeysAdmitted,
            Edge::ValidateSeparator,
            State::SeparatorValidated,
        )
    }

    const fn denied_transition() -> S8LayoutProductionTransition {
        crate::production_transition::owner_transition(
            Machine::BTreeSearchPathInvariant,
            Operation::VerifyBTreeSearchPath,
            "SeparatorDenied",
            State::CanonicalKeysAdmitted,
            Edge::ValidateSeparator,
            State::Denied,
        )
    }

    pub(crate) fn owner_transition_contract() -> S8OwnerTransitionContract {
        static FACTS: [S8LayoutProductionTransition; 2] = [
            S8BTreeSearchOutcome::<()>::validated_transition(),
            S8BTreeSearchOutcome::<()>::denied_transition(),
        ];
        S8OwnerTransitionContract::from_owner_outcomes(
            Machine::BTreeSearchPathInvariant,
            Operation::VerifyBTreeSearchPath,
            &FACTS,
        )
    }
}

impl<T: PartialEq> PartialEq<Result<T, S8StrategyDenial>> for S8BTreeSearchOutcome<T> {
    fn eq(&self, other: &Result<T, S8StrategyDenial>) -> bool {
        self.issued.result() == other.as_ref()
    }
}
