use std::ops::{Deref, DerefMut};

use super::{UiNativeResourceClass, UiNativeResourceOwner, UiNativeResourceRegistry};

pub(crate) struct UiNativeOwnedResource<T> {
    resource: T,
    owner: UiNativeResourceOwner,
}

impl<T> UiNativeOwnedResource<T> {
    pub(crate) fn register(
        resource: T,
        class: UiNativeResourceClass,
        registry: &mut UiNativeResourceRegistry,
    ) -> Result<Self, T> {
        match registry.register(class) {
            Ok(owner) => Ok(Self { resource, owner }),
            Err(()) => Err(resource),
        }
    }

    pub(crate) fn close(self, registry: &mut UiNativeResourceRegistry) {
        let Self { resource, owner } = self;
        drop(resource);
        registry
            .release(owner)
            .expect("resource owner must remain exact until its resource closes");
    }
}

impl<T> Deref for UiNativeOwnedResource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<T> DerefMut for UiNativeOwnedResource<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}

#[cfg(test)]
mod tests {
    use super::UiNativeOwnedResource;
    use crate::native::{UiNativeResourceClass, UiNativeResourceRegistry};
    use std::cell::Cell;
    use std::rc::Rc;

    struct DropProbe(Rc<Cell<bool>>);

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    #[test]
    fn actual_resource_drops_before_its_exact_registry_owner_releases() {
        let mut registry = UiNativeResourceRegistry::new();
        let dropped = Rc::new(Cell::new(false));
        let resource = UiNativeOwnedResource::register(
            DropProbe(Rc::clone(&dropped)),
            UiNativeResourceClass::Window,
            &mut registry,
        )
        .unwrap_or_else(|_| panic!("resource capacity"));
        assert_eq!(registry.current().windows, 1);
        resource.close(&mut registry);
        assert!(dropped.get());
        assert!(registry.current().is_zero());
    }
}
