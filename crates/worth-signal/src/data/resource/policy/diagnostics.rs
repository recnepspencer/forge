use serde::{Deserialize, Serialize};

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::ResourceDiagnosticsPolicyDeclaration;

const RETAINED_ONLY_NAME: &str = "signal.resource.diagnostics.retained-only";
const BUDGETED_EXPANSION_NAME: &str = "signal.resource.diagnostics.budgeted-expansion";
const FORENSIC_EXPANSION_BUDGET_NAME: &str =
    "signal.resource.diagnostics.forensic-expansion-budget";
const DENY_COLD_EXPANSION_NAME: &str = "signal.resource.diagnostics.deny-cold-expansion";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceDiagnosticsDecisionClass {
    RetainedOnly,
    BudgetedExpansion,
    ForensicExpansionBudget,
    DenyColdExpansion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceDiagnosticsDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceDiagnosticsDecisionClass,
    max_replay_reconstruction_width: Option<u32>,
    max_forensic_reconstruction_width: Option<u32>,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceDiagnosticsDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceDiagnosticsPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceDiagnosticsPolicyDeclaration::RetainedOnly => {
                ensure_descriptor_name(
                    frozen,
                    RETAINED_ONLY_NAME,
                    "retained-only diagnostics policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceDiagnosticsDecisionClass::RetainedOnly,
                    None,
                    None,
                ))
            }
            ResourceDiagnosticsPolicyDeclaration::BudgetedExpansion {
                max_replay_reconstruction_width,
            } => {
                ensure_descriptor_name(
                    frozen,
                    BUDGETED_EXPANSION_NAME,
                    "budgeted-expansion diagnostics policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceDiagnosticsDecisionClass::BudgetedExpansion,
                    Some(*max_replay_reconstruction_width),
                    Some(*max_replay_reconstruction_width),
                ))
            }
            ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                max_replay_reconstruction_width,
                max_forensic_reconstruction_width,
            } => {
                ensure_descriptor_name(
                    frozen,
                    FORENSIC_EXPANSION_BUDGET_NAME,
                    "forensic-expansion-budget diagnostics policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceDiagnosticsDecisionClass::ForensicExpansionBudget,
                    Some(*max_replay_reconstruction_width),
                    Some(*max_forensic_reconstruction_width),
                ))
            }
            ResourceDiagnosticsPolicyDeclaration::DenyColdExpansion => {
                ensure_descriptor_name(
                    frozen,
                    DENY_COLD_EXPANSION_NAME,
                    "deny-cold-expansion diagnostics policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceDiagnosticsDecisionClass::DenyColdExpansion,
                    None,
                    None,
                ))
            }
            ResourceDiagnosticsPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Diagnostics,
                    name: name.clone(),
                    reason:
                        "named diagnostics policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceDiagnosticsDecisionClass,
        max_replay_reconstruction_width: Option<u32>,
        max_forensic_reconstruction_width: Option<u32>,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-diagnostics-plan:{}:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str(),
            max_replay_reconstruction_width
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned()),
            max_forensic_reconstruction_width
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned())
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            max_replay_reconstruction_width,
            max_forensic_reconstruction_width,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceDiagnosticsDecisionClass {
        self.class
    }

    pub fn max_replay_reconstruction_width(&self) -> Option<u32> {
        self.max_replay_reconstruction_width
    }

    pub fn max_forensic_reconstruction_width(&self) -> Option<u32> {
        self.max_forensic_reconstruction_width
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }

    pub fn denies_cold_reconstruction(&self) -> bool {
        matches!(
            self.class,
            ResourceDiagnosticsDecisionClass::RetainedOnly
                | ResourceDiagnosticsDecisionClass::DenyColdExpansion
        )
    }
}

impl ResourceDiagnosticsDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RetainedOnly => "retained-only",
            Self::BudgetedExpansion => "budgeted-expansion",
            Self::ForensicExpansionBudget => "forensic-expansion-budget",
            Self::DenyColdExpansion => "deny-cold-expansion",
        }
    }
}

fn ensure_descriptor_name(
    frozen: &FrozenResourcePolicyDescriptor,
    expected: &str,
    reason: &'static str,
) -> Result<(), ResourcePolicyResolutionError> {
    if frozen.descriptor().semantic_name().as_str() == expected {
        return Ok(());
    }
    Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
        kind: ResourcePolicyKind::Diagnostics,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
