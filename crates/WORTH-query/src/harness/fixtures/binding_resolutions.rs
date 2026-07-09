use crate::facade::{
    derive_binding_requirements, resolve_bindings, BoundBinding, BoundBindings, QueryBindingSlot,
    QueryBindingSubject, ValidatedQueryBundle,
};

pub fn root_bound_bindings(value: &str) -> BoundBindings {
    BoundBindings::new(vec![BoundBinding::new(
        QueryBindingSlot::new("root").unwrap(),
        QueryBindingSubject::RootEntity,
        value,
    )])
}

pub fn resolved_root_binding(
    bundle: &ValidatedQueryBundle,
    value: &str,
) -> crate::facade::BindingResolution {
    resolve_bindings(
        derive_binding_requirements(bundle),
        root_bound_bindings(value),
    )
    .unwrap()
}
