use crate::capability::PluginSlotId;

use super::{
    PluginCapabilityPermission, PluginContributionFamily, PluginSlotContributionReference,
    PluginSlotDiagnostics, PluginSlotGlobalMutationHook, PluginSlotOrdering,
    PluginSlotSupportPosture,
};

/// Declarative extension point through which plugins may contribute capabilities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginSlotDescriptor {
    id: PluginSlotId,
    allowed_families: Vec<PluginContributionFamily>,
    permission: Option<PluginCapabilityPermission>,
    ordering: Option<PluginSlotOrdering>,
    diagnostics: Option<PluginSlotDiagnostics>,
    support: Option<PluginSlotSupportPosture>,
    contribution_reference: Option<PluginSlotContributionReference>,
    global_mutation_hooks: Vec<PluginSlotGlobalMutationHook>,
}

impl PluginSlotDescriptor {
    pub fn new(id: PluginSlotId) -> Self {
        Self {
            id,
            allowed_families: Vec::new(),
            permission: None,
            ordering: None,
            diagnostics: None,
            support: None,
            contribution_reference: None,
            global_mutation_hooks: Vec::new(),
        }
    }

    pub fn allow_family(mut self, family: PluginContributionFamily) -> Self {
        self.allowed_families.push(family);
        self
    }

    pub fn with_permission(mut self, permission: PluginCapabilityPermission) -> Self {
        self.permission = Some(permission);
        self
    }

    pub fn with_ordering(mut self, ordering: PluginSlotOrdering) -> Self {
        self.ordering = Some(ordering);
        self
    }

    pub fn with_diagnostics(mut self, diagnostics: PluginSlotDiagnostics) -> Self {
        self.diagnostics = Some(diagnostics);
        self
    }

    pub fn with_support(mut self, support: PluginSlotSupportPosture) -> Self {
        self.support = Some(support);
        self
    }

    pub fn with_contribution_reference(
        mut self,
        contribution_reference: PluginSlotContributionReference,
    ) -> Self {
        self.contribution_reference = Some(contribution_reference);
        self
    }

    pub fn with_global_mutation_hook_for_diagnostics(
        mut self,
        hook: PluginSlotGlobalMutationHook,
    ) -> Self {
        self.global_mutation_hooks.push(hook);
        self
    }

    pub fn id(&self) -> &PluginSlotId {
        &self.id
    }

    pub fn allowed_families(&self) -> &[PluginContributionFamily] {
        &self.allowed_families
    }

    pub fn permission(&self) -> Option<PluginCapabilityPermission> {
        self.permission
    }

    pub fn ordering(&self) -> Option<PluginSlotOrdering> {
        self.ordering
    }

    pub fn diagnostics(&self) -> Option<PluginSlotDiagnostics> {
        self.diagnostics
    }

    pub fn support(&self) -> Option<PluginSlotSupportPosture> {
        self.support
    }

    pub fn contribution_reference(&self) -> Option<&PluginSlotContributionReference> {
        self.contribution_reference.as_ref()
    }

    pub(crate) fn global_mutation_hooks(&self) -> &[PluginSlotGlobalMutationHook] {
        &self.global_mutation_hooks
    }

    pub(crate) fn canonicalized_for_freeze(mut self) -> Self {
        self.allowed_families.sort();
        self.allowed_families.dedup();
        self.global_mutation_hooks.sort();
        self.global_mutation_hooks.dedup();
        self
    }
}
