use super::{
    BaselineLsmLookupAdmission, BaselineLsmLookupAdmissionView, BaselineLsmLookupExecution,
    BaselineLsmLookupSource,
};
use crate::SelectedLsmLookup;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmLookupRuntime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LsmLookupAdmissionDenied {
    Stale(crate::StaleLayoutMaterialization),
    InvariantViolation(crate::strategy::StrategyDenial),
}

pub const fn lsm_lookup_runtime() -> LsmLookupRuntime {
    LsmLookupRuntime
}

impl LsmLookupRuntime {
    pub fn execute(
        self,
        selected: SelectedLsmLookup,
        source: BaselineLsmLookupSource,
        probe_sequence: u64,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> Result<BaselineLsmLookupExecution, LsmLookupAdmissionDenied> {
        let invariants = selected
            .admitted_strategy()
            .expect("LSM lookup selection retains admitted strategy")
            .invariant_suite()
            .require_lsm_suite()
            .map_err(LsmLookupAdmissionDenied::InvariantViolation)?;
        let outcome = BaselineLsmLookupAdmission::admit(selected, frontier);
        match outcome.view() {
            BaselineLsmLookupAdmissionView::Admitted(_) => {
                let admission = outcome
                    .into_admitted()
                    .expect("owner view established admitted LSM lookup readiness");
                let execution = source.execute_latest_visible(admission, probe_sequence);
                invariants
                    .verify_lookup_execution(&execution)
                    .map_err(LsmLookupAdmissionDenied::InvariantViolation)?;
                Ok(execution)
            }
            BaselineLsmLookupAdmissionView::Stale(stale) => {
                Err(LsmLookupAdmissionDenied::Stale(stale.clone()))
            }
        }
    }
}
