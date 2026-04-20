use crate::authoring::{
    AspectFieldSelector, OrderingSelector, PredicateSelector, TraversalSelector,
};

use super::slot::TemplateParameterSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateBindingSet {
    bindings: Vec<TemplateBindingEntry>,
}

impl TemplateBindingSet {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    pub fn bind_predicate(mut self, slot: &TemplateParameterSlot, value: PredicateSelector) -> Self {
        self.bindings.push(TemplateBindingEntry {
            slot: slot.clone(),
            value: TemplateBindingValue::Predicate(value),
        });
        self
    }

    pub fn bind_ordering(mut self, slot: &TemplateParameterSlot, value: OrderingSelector) -> Self {
        self.bindings.push(TemplateBindingEntry {
            slot: slot.clone(),
            value: TemplateBindingValue::Ordering(value),
        });
        self
    }

    pub fn bind_projection(
        mut self,
        slot: &TemplateParameterSlot,
        value: AspectFieldSelector,
    ) -> Self {
        self.bindings.push(TemplateBindingEntry {
            slot: slot.clone(),
            value: TemplateBindingValue::Projection(value),
        });
        self
    }

    pub fn bind_traversal(
        mut self,
        slot: &TemplateParameterSlot,
        value: TraversalSelector,
    ) -> Self {
        self.bindings.push(TemplateBindingEntry {
            slot: slot.clone(),
            value: TemplateBindingValue::Traversal(value),
        });
        self
    }

    pub(crate) fn bindings(&self) -> &[TemplateBindingEntry] {
        &self.bindings
    }
}

impl Default for TemplateBindingSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TemplateBindingEntry {
    pub(crate) slot: TemplateParameterSlot,
    pub(crate) value: TemplateBindingValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TemplateBindingValue {
    Predicate(PredicateSelector),
    Ordering(OrderingSelector),
    Projection(AspectFieldSelector),
    Traversal(TraversalSelector),
}
