use super::{QueryViewBindingKey, ViewBindingDescriptor};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenViewBindingEntry {
    descriptor: ViewBindingDescriptor,
    query_binding_key: QueryViewBindingKey,
}

impl FrozenViewBindingEntry {
    pub(crate) fn new(
        descriptor: ViewBindingDescriptor,
        query_binding_key: QueryViewBindingKey,
    ) -> Self {
        Self {
            descriptor,
            query_binding_key,
        }
    }

    pub fn descriptor(&self) -> &ViewBindingDescriptor {
        &self.descriptor
    }

    pub fn query_binding_key(&self) -> &QueryViewBindingKey {
        &self.query_binding_key
    }
}
