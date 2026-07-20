use super::{
    WorthQueryOnDemandTriggerFamily, WorthQueryTemporalWake, WorthQueryTypedFamilyIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalTrigger {
    DependencyChange,
    OnDemand(WorthQueryTypedFamilyIdentity),
    Temporal(WorthQueryTemporalWake),
}

impl WorthQueryConditionalTrigger {
    pub fn on_demand<Owner: WorthQueryOnDemandTriggerFamily>() -> Self {
        Self::OnDemand(WorthQueryTypedFamilyIdentity::declared(
            Owner::PORTABLE_IDENTITY,
        ))
    }
}

pub(crate) fn trigger_token(trigger: &WorthQueryConditionalTrigger) -> String {
    match trigger {
        WorthQueryConditionalTrigger::DependencyChange => "dependency-change".to_string(),
        WorthQueryConditionalTrigger::OnDemand(owner) => {
            format!("on-demand#{}:{}", owner.as_str().len(), owner.as_str())
        }
        WorthQueryConditionalTrigger::Temporal(wake) => {
            format!("temporal:{}", super::temporal::temporal_wake_token(*wake))
        }
    }
}
