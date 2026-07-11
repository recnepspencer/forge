use crate::strategy::S8StrategyDenial;

#[derive(Debug, PartialEq, Eq)]
pub struct S8BTreeSearchOutcome<T> {
    result: Result<T, S8StrategyDenial>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8BTreeSearchOutcomeView<'a, T> {
    Validated(&'a T),
    Denied(&'a S8StrategyDenial),
}

impl<T> S8BTreeSearchOutcome<T> {
    pub(super) fn issue(result: Result<T, S8StrategyDenial>) -> Self {
        Self { result }
    }

    pub fn view(&self) -> S8BTreeSearchOutcomeView<'_, T> {
        match self.result.as_ref() {
            Ok(value) => S8BTreeSearchOutcomeView::Validated(value),
            Err(denial) => S8BTreeSearchOutcomeView::Denied(denial),
        }
    }

    pub fn into_result(self) -> Result<T, S8StrategyDenial> {
        self.result
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
}

impl<T: PartialEq> PartialEq<Result<T, S8StrategyDenial>> for S8BTreeSearchOutcome<T> {
    fn eq(&self, other: &Result<T, S8StrategyDenial>) -> bool {
        self.result.as_ref() == other.as_ref()
    }
}
