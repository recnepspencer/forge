use std::marker::PhantomData;

use super::TypedMutationIntent;

impl<Schema, Operation, Input> Clone for TypedMutationIntent<Schema, Operation, Input>
where
    Input: Clone,
{
    fn clone(&self) -> Self {
        Self {
            operation: self.operation,
            binding: self.binding.clone(),
            input: self.input.clone(),
            creates: self.creates.clone(),
            deletes: self.deletes.clone(),
            links: self.links.clone(),
            unlinks: self.unlinks.clone(),
            writes: self.writes.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Schema, Operation, Input> std::fmt::Debug for TypedMutationIntent<Schema, Operation, Input>
where
    Input: std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TypedMutationIntent")
            .field("operation", &self.operation)
            .field("binding", &self.binding)
            .field("input", &self.input)
            .field("creates", &self.creates)
            .field("deletes", &self.deletes)
            .field("links", &self.links)
            .field("unlinks", &self.unlinks)
            .field("writes", &self.writes)
            .finish_non_exhaustive()
    }
}

impl<Schema, Operation, Input> PartialEq for TypedMutationIntent<Schema, Operation, Input>
where
    Input: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation
            && self.binding == other.binding
            && self.input == other.input
            && self.creates == other.creates
            && self.deletes == other.deletes
            && self.links == other.links
            && self.unlinks == other.unlinks
            && self.writes == other.writes
    }
}
