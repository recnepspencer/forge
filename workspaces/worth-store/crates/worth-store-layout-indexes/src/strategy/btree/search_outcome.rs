use crate::strategy::StrategyDenial;

#[derive(Debug, PartialEq, Eq)]
pub struct BTreeSearchOutcome<T> {
    result: Result<T, StrategyDenial>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTreeSearchOutcomeView<'a, T> {
    Validated(&'a T),
    Denied(&'a StrategyDenial),
}

impl<T> BTreeSearchOutcome<T> {
    pub(super) fn issue(result: Result<T, StrategyDenial>) -> Self {
        Self { result }
    }

    pub fn view(&self) -> BTreeSearchOutcomeView<'_, T> {
        match self.result.as_ref() {
            Ok(value) => BTreeSearchOutcomeView::Validated(value),
            Err(denial) => BTreeSearchOutcomeView::Denied(denial),
        }
    }

    pub fn into_result(self) -> Result<T, StrategyDenial> {
        self.result
    }

    pub fn unwrap(self) -> T {
        self.into_result().unwrap()
    }

    pub fn unwrap_err(self) -> StrategyDenial
    where
        T: core::fmt::Debug,
    {
        self.into_result().unwrap_err()
    }
}

impl<T: PartialEq> PartialEq<Result<T, StrategyDenial>> for BTreeSearchOutcome<T> {
    fn eq(&self, other: &Result<T, StrategyDenial>) -> bool {
        self.result.as_ref() == other.as_ref()
    }
}
