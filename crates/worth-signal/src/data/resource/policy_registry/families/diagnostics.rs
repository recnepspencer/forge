use super::super::super::policy::ResourceDiagnosticsPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_diagnostics(
        &self,
        policy: &ResourceDiagnosticsPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceDiagnosticsPolicyDeclaration::RetainedOnly => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.retained-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("diagnostics:retained-only"),
            )?,
            ResourceDiagnosticsPolicyDeclaration::BudgetedExpansion {
                max_replay_reconstruction_width,
            } => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.budgeted-expansion",
                if *max_replay_reconstruction_width == u32::MAX {
                    ResourcePolicySelectionBasis::BuiltInDefault
                } else {
                    ResourcePolicySelectionBasis::DeclaredBuiltIn
                },
                diagnostics_parameter_digest(
                    "budgeted-expansion",
                    *max_replay_reconstruction_width,
                    None,
                ),
            )?,
            ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                max_replay_reconstruction_width,
                max_forensic_reconstruction_width,
            } => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.forensic-expansion-budget",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                diagnostics_parameter_digest(
                    "forensic-expansion-budget",
                    *max_replay_reconstruction_width,
                    Some(*max_forensic_reconstruction_width),
                ),
            )?,
            ResourceDiagnosticsPolicyDeclaration::DenyColdExpansion => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.deny-cold-expansion",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("diagnostics:deny-cold-expansion"),
            )?,
            ResourceDiagnosticsPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Diagnostics, name)?
            }
        })
    }
}

pub(super) fn diagnostics_parameter_digest(
    family: &'static str,
    max_replay_reconstruction_width: u32,
    max_forensic_reconstruction_width: Option<u32>,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "diagnostics:{family}:max-replay-reconstruction-width:{max_replay_reconstruction_width}:max-forensic-reconstruction-width:{}",
        max_forensic_reconstruction_width
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_owned())
    ))
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            50,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.retained-only",
            8,
        ),
        (
            51,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.budgeted-expansion",
            8,
        ),
        (
            58,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.forensic-expansion-budget",
            8,
        ),
        (
            52,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.deny-cold-expansion",
            8,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
