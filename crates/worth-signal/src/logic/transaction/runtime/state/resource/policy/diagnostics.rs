use super::super::ResourceRuntimeState;
use crate::data::resource::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct EffectiveResourceDiagnosticsPolicy {
    class: ResourceDiagnosticsDecisionClass,
    max_replay_reconstruction_width: Option<u32>,
    max_forensic_reconstruction_width: Option<u32>,
    decision_digest: ResourcePolicyDigest,
    descriptor_width: u32,
}

impl EffectiveResourceDiagnosticsPolicy {
    pub(in crate::logic::transaction::runtime) const fn class(
        &self,
    ) -> ResourceDiagnosticsDecisionClass {
        self.class
    }

    pub(in crate::logic::transaction::runtime) const fn max_replay_reconstruction_width(
        &self,
    ) -> Option<u32> {
        self.max_replay_reconstruction_width
    }

    pub(in crate::logic::transaction::runtime) const fn max_forensic_reconstruction_width(
        &self,
    ) -> Option<u32> {
        self.max_forensic_reconstruction_width
    }

    pub(in crate::logic::transaction::runtime) fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }

    pub(in crate::logic::transaction::runtime) const fn descriptor_width(&self) -> u32 {
        self.descriptor_width
    }
}

impl ResourceRuntimeState {
    pub fn effective_diagnostics_policy(&self) -> EffectiveResourceDiagnosticsPolicy {
        let mut class = ResourceDiagnosticsDecisionClass::BudgetedExpansion;
        let mut max_replay_reconstruction_width = Some(u32::MAX);
        let mut max_forensic_reconstruction_width = Some(u32::MAX);
        let mut decision_rows = Vec::new();

        for descriptor in self.descriptors.values() {
            let plan = descriptor.diagnostics_decision_plan();
            decision_rows.push(plan.decision_digest().as_str().to_owned());
            match plan.class() {
                ResourceDiagnosticsDecisionClass::DenyColdExpansion => {
                    class = ResourceDiagnosticsDecisionClass::DenyColdExpansion;
                    max_replay_reconstruction_width = None;
                    max_forensic_reconstruction_width = None;
                    break;
                }
                ResourceDiagnosticsDecisionClass::RetainedOnly => {
                    if class != ResourceDiagnosticsDecisionClass::DenyColdExpansion {
                        class = ResourceDiagnosticsDecisionClass::RetainedOnly;
                        max_replay_reconstruction_width = None;
                        max_forensic_reconstruction_width = None;
                    }
                }
                ResourceDiagnosticsDecisionClass::BudgetedExpansion => {
                    if matches!(class, ResourceDiagnosticsDecisionClass::BudgetedExpansion) {
                        max_replay_reconstruction_width = Some(
                            max_replay_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_replay_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                        max_forensic_reconstruction_width = Some(
                            max_forensic_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_forensic_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                    }
                }
                ResourceDiagnosticsDecisionClass::ForensicExpansionBudget => {
                    if matches!(
                        class,
                        ResourceDiagnosticsDecisionClass::BudgetedExpansion
                            | ResourceDiagnosticsDecisionClass::ForensicExpansionBudget
                    ) {
                        class = ResourceDiagnosticsDecisionClass::ForensicExpansionBudget;
                        max_replay_reconstruction_width = Some(
                            max_replay_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_replay_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                        max_forensic_reconstruction_width = Some(
                            max_forensic_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_forensic_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                    }
                }
            }
        }

        decision_rows.sort();
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-diagnostics-effective-policy:{}:{}:{}:{}",
            class.as_str(),
            max_replay_reconstruction_width
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned()),
            max_forensic_reconstruction_width
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned()),
            decision_rows.join("|")
        ));

        EffectiveResourceDiagnosticsPolicy {
            class,
            max_replay_reconstruction_width,
            max_forensic_reconstruction_width,
            decision_digest,
            descriptor_width: self.descriptors.len().min(u32::MAX as usize) as u32,
        }
    }
}
