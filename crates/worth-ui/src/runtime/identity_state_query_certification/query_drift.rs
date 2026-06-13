use crate::runtime::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryDriftCertification {
    plans: Vec<WorthUiQueryLiveRebindPlan>,
    typed_denials: Vec<WorthUiQueryBindingDriftDenial>,
    ui_local_recovery_denial_count: usize,
}

impl WorthUiQueryDriftCertification {
    pub(crate) fn new(plans: Vec<WorthUiQueryLiveRebindPlan>) -> Self {
        let typed_denials = plans
            .iter()
            .flat_map(|plan| plan.entries())
            .filter_map(|entry| match entry.outcome() {
                WorthUiQueryLiveRebindOutcome::Deny(denial) => Some(denial.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let ui_local_recovery_denial_count = typed_denials
            .iter()
            .filter(|denial| {
                denial.reason()
                    == WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery
            })
            .count();
        Self {
            plans,
            typed_denials,
            ui_local_recovery_denial_count,
        }
    }

    pub fn plans(&self) -> &[WorthUiQueryLiveRebindPlan] {
        &self.plans
    }

    pub fn typed_denials(&self) -> &[WorthUiQueryBindingDriftDenial] {
        &self.typed_denials
    }

    pub fn typed_denial_kinds(&self) -> Vec<WorthUiQueryBindingDriftDenialKind> {
        self.typed_denials
            .iter()
            .map(WorthUiQueryBindingDriftDenial::reason)
            .collect()
    }

    pub fn ui_local_recovery_denial_count(&self) -> usize {
        self.ui_local_recovery_denial_count
    }
}
