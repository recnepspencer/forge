use crate::runtime::WorthUiExecutionPlanEquivalenceBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanDigest {
    raw: u64,
    basis: WorthUiExecutionPlanEquivalenceBasis,
}

impl WorthUiExecutionPlanDigest {
    pub(crate) fn new(raw: u64, basis: WorthUiExecutionPlanEquivalenceBasis) -> Self {
        Self { raw, basis }
    }

    pub fn raw(self) -> u64 {
        self.raw
    }

    pub fn basis(self) -> WorthUiExecutionPlanEquivalenceBasis {
        self.basis
    }
}
