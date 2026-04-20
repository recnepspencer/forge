#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompositionCounters {
    scope_expansion_count: usize,
    scope_expansion_width: usize,
    template_slot_count: usize,
    template_binding_width: usize,
    template_defaulting_count: usize,
    scope_rediscovery_count: usize,
    template_rediscovery_count: usize,
}

impl CompositionCounters {
    pub fn scope_expansion_count(&self) -> usize {
        self.scope_expansion_count
    }

    pub fn scope_expansion_width(&self) -> usize {
        self.scope_expansion_width
    }

    pub fn template_slot_count(&self) -> usize {
        self.template_slot_count
    }

    pub fn template_binding_width(&self) -> usize {
        self.template_binding_width
    }

    pub fn template_defaulting_count(&self) -> usize {
        self.template_defaulting_count
    }

    pub fn scope_rediscovery_count(&self) -> usize {
        self.scope_rediscovery_count
    }

    pub fn template_rediscovery_count(&self) -> usize {
        self.template_rediscovery_count
    }

    pub(crate) fn for_scope_expansion(count: usize, width: usize) -> Self {
        Self {
            scope_expansion_count: count,
            scope_expansion_width: width,
            template_slot_count: 0,
            template_binding_width: 0,
            template_defaulting_count: 0,
            scope_rediscovery_count: 0,
            template_rediscovery_count: 0,
        }
    }

    pub(crate) fn for_template_instantiation(slot_count: usize, binding_width: usize) -> Self {
        Self {
            scope_expansion_count: 0,
            scope_expansion_width: 0,
            template_slot_count: slot_count,
            template_binding_width: binding_width,
            template_defaulting_count: 0,
            scope_rediscovery_count: 0,
            template_rediscovery_count: 0,
        }
    }
}
