use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use super::{UiNativeResourceCensus, UiNativeResourceClass};

const RESOURCE_CAPACITY: usize =
    crate::UiNativeMechanicsCapacities::QUALIFIED.resource_registry_entries as usize;

#[must_use]
pub(crate) struct UiNativeResourceOwner {
    identity: u64,
    class: UiNativeResourceClass,
}

pub(crate) struct UiNativeResourceRegistry {
    next: u64,
    live: BTreeMap<u64, UiNativeResourceClass>,
    peak: UiNativeResourceCensus,
}

pub(crate) struct UiNativeOwnedResource<T> {
    resource: T,
    owner: UiNativeResourceOwner,
}

impl UiNativeResourceRegistry {
    pub(crate) fn new() -> Self {
        Self {
            next: 1,
            live: BTreeMap::new(),
            peak: UiNativeResourceCensus::default(),
        }
    }

    pub(crate) fn register(
        &mut self,
        class: UiNativeResourceClass,
    ) -> Result<UiNativeResourceOwner, ()> {
        if self.live.len() == RESOURCE_CAPACITY {
            return Err(());
        }
        let identity = self.next;
        self.next = self.next.checked_add(1).ok_or(())?;
        self.live.insert(identity, class);
        self.peak = self.peak.max(self.current());
        Ok(UiNativeResourceOwner { identity, class })
    }

    pub(crate) fn admits(&self, additional: usize) -> bool {
        self.live
            .len()
            .checked_add(additional)
            .is_some_and(|total| total <= RESOURCE_CAPACITY)
    }

    pub(crate) fn reserve(
        &mut self,
        classes: &[UiNativeResourceClass],
    ) -> Result<Vec<UiNativeResourceOwner>, ()> {
        let mut owners = Vec::with_capacity(classes.len());
        for class in classes {
            match self.register(*class) {
                Ok(owner) => owners.push(owner),
                Err(()) => {
                    self.release_all(owners)
                        .expect("fresh reservations must release exactly");
                    return Err(());
                }
            }
        }
        Ok(owners)
    }

    pub(crate) fn release(&mut self, owner: UiNativeResourceOwner) -> Result<(), ()> {
        match self.live.remove(&owner.identity) {
            Some(class) if class == owner.class => Ok(()),
            Some(class) => {
                self.live.insert(owner.identity, class);
                Err(())
            }
            None => Err(()),
        }
    }

    pub(crate) fn release_all(
        &mut self,
        owners: impl IntoIterator<Item = UiNativeResourceOwner>,
    ) -> Result<(), ()> {
        for owner in owners {
            self.release(owner)?;
        }
        Ok(())
    }

    pub(crate) fn current(&self) -> UiNativeResourceCensus {
        self.live
            .values()
            .fold(UiNativeResourceCensus::default(), |mut census, class| {
                census.record(*class);
                census
            })
    }

    pub(crate) const fn peak(&self) -> UiNativeResourceCensus {
        self.peak
    }
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
    use super::{UiNativeOwnedResource, UiNativeResourceClass, UiNativeResourceRegistry};
    use std::cell::Cell;
    use std::rc::Rc;

    struct DropProbe(Rc<Cell<bool>>);

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    #[test]
    fn registry_is_bounded_and_each_live_owner_must_be_released() {
        let mut registry = UiNativeResourceRegistry::new();
        let owners = (0..super::RESOURCE_CAPACITY)
            .map(|_| {
                registry
                    .register(UiNativeResourceClass::ReadbackBuffer)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert!(registry.register(UiNativeResourceClass::Window).is_err());
        registry.release_all(owners).unwrap();
        assert!(registry.current().is_zero());
        assert_eq!(registry.peak().readback_buffers, super::RESOURCE_CAPACITY);
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
        resource.close(&mut registry);
        assert!(dropped.get());
        assert!(registry.current().is_zero());
    }
}
