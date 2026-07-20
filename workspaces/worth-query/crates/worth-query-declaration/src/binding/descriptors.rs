use crate::binding::metadata::NonIdentityBindingMetadata;

use super::{QueryBindingSlot, QueryBindingSubject};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IdentityBindingDescriptor {
    slot: QueryBindingSlot,
    subject: QueryBindingSubject,
}

impl IdentityBindingDescriptor {
    pub fn new(slot: QueryBindingSlot, subject: QueryBindingSubject) -> Self {
        Self { slot, subject }
    }

    pub fn slot(&self) -> &QueryBindingSlot {
        &self.slot
    }

    pub fn subject(&self) -> &QueryBindingSubject {
        &self.subject
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBindingDescriptor {
    identity: Vec<IdentityBindingDescriptor>,
    non_identity: Vec<NonIdentityBindingMetadata>,
}

impl QueryBindingDescriptor {
    pub fn new() -> Self {
        Self {
            identity: Vec::new(),
            non_identity: Vec::new(),
        }
    }

    pub fn with_identity(mut self, descriptor: IdentityBindingDescriptor) -> Self {
        self.identity.push(descriptor);
        self
    }

    pub fn with_non_identity(mut self, metadata: NonIdentityBindingMetadata) -> Self {
        self.non_identity.push(metadata);
        self
    }

    pub fn identity(&self) -> &[IdentityBindingDescriptor] {
        &self.identity
    }

    pub fn non_identity(&self) -> &[NonIdentityBindingMetadata] {
        &self.non_identity
    }
}

impl Default for QueryBindingDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
