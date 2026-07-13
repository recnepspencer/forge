use crate::facade::foundation::{
    IdentityBindingDescriptor, NonIdentityBindingMetadata, QueryBindingDescriptor,
    QueryBindingSlot, QueryBindingSubject,
};

pub fn ordered_bindings() -> QueryBindingDescriptor {
    QueryBindingDescriptor::new()
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ))
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap())
}

pub fn reordered_bindings() -> QueryBindingDescriptor {
    QueryBindingDescriptor::new()
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap())
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ))
}
