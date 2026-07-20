use crate::binding::resolve_bindings;
use crate::facade::foundation::{
    derive_binding_requirements, BoundBinding, BoundBindings, QueryBindingSlot, QueryBindingSubject,
};
use crate::facade::runtime::ValidatedQueryBundle;

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
) -> crate::facade::foundation::BindingResolution {
    resolve_bindings(
        derive_binding_requirements(bundle),
        root_bound_bindings(value),
    )
    .unwrap()
}
