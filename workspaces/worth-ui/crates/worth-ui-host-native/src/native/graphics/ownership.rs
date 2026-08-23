use std::ops::{Deref, DerefMut};

use crate::native::{UiNativeResourceClass, UiNativeResourceOwner, UiNativeResourceRegistry};

use super::{basis_changed, UiNativeGraphics, UiNativeGraphicsPort, UiWgpuNativeGraphicsPort};

pub(crate) struct UiNativeOwnedGraphics {
    graphics: UiNativeGraphics,
    core_owners: Vec<UiNativeResourceOwner>,
    retained_target_owner: UiNativeResourceOwner,
}

impl UiNativeOwnedGraphics {
    pub(crate) fn register(
        graphics: UiNativeGraphics,
        registry: &mut UiNativeResourceRegistry,
    ) -> Result<Self, UiNativeGraphics> {
        let mut owners = match registry.reserve(&[
            UiNativeResourceClass::Surface,
            UiNativeResourceClass::Adapter,
            UiNativeResourceClass::Device,
            UiNativeResourceClass::Queue,
            UiNativeResourceClass::RetainedTarget,
        ]) {
            Ok(owners) => owners,
            Err(()) => return Err(graphics),
        };
        let retained_target_owner = owners.pop().expect("retained-target owner");
        Ok(Self {
            graphics,
            core_owners: owners,
            retained_target_owner,
        })
    }

    pub(crate) fn resize(
        &mut self,
        extent: [u32; 2],
        registry: &mut UiNativeResourceRegistry,
    ) -> Result<bool, ()> {
        self.replace_basis(self.graphics.scale_factor, extent, registry)
    }

    pub(crate) fn rebind_scale(
        &mut self,
        scale_factor: f64,
        extent: [u32; 2],
        registry: &mut UiNativeResourceRegistry,
    ) -> Result<bool, ()> {
        self.replace_basis(scale_factor, extent, registry)
    }

    pub(crate) fn replace_retained_target_for_reconstruction(
        &mut self,
        registry: &mut UiNativeResourceRegistry,
    ) -> Result<(), ()> {
        self.replace_target(self.graphics.scale_factor, self.graphics.extent(), registry)
    }

    fn replace_basis(
        &mut self,
        scale_factor: f64,
        extent: [u32; 2],
        registry: &mut UiNativeResourceRegistry,
    ) -> Result<bool, ()> {
        let extent = [extent[0].max(1), extent[1].max(1)];
        if !basis_changed(
            self.graphics.scale_factor,
            self.graphics.extent(),
            scale_factor,
            extent,
        ) {
            return Ok(false);
        }
        self.replace_target(scale_factor, extent, registry)?;
        Ok(true)
    }

    fn replace_target(
        &mut self,
        scale_factor: f64,
        extent: [u32; 2],
        registry: &mut UiNativeResourceRegistry,
    ) -> Result<(), ()> {
        let successor_owner = registry.register(UiNativeResourceClass::RetainedTarget)?;
        let successor =
            UiWgpuNativeGraphicsPort::replacement_target(&mut self.graphics, scale_factor, extent);
        let predecessor = self
            .graphics
            .retained_target
            .replace(successor)
            .expect("live predecessor retained target");
        let predecessor_owner = std::mem::replace(&mut self.retained_target_owner, successor_owner);
        drop(predecessor);
        registry.release(predecessor_owner)?;
        Ok(())
    }

    pub(crate) fn close(mut self, registry: &mut UiNativeResourceRegistry) {
        let target = self
            .graphics
            .retained_target
            .take()
            .expect("live retained target");
        drop(target);
        registry
            .release(self.retained_target_owner)
            .expect("retained-target owner remains exact");
        drop(self.graphics);
        registry
            .release_all(self.core_owners)
            .expect("graphics owners remain exact");
    }
}

impl Deref for UiNativeOwnedGraphics {
    type Target = UiNativeGraphics;

    fn deref(&self) -> &Self::Target {
        &self.graphics
    }
}

impl DerefMut for UiNativeOwnedGraphics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graphics
    }
}
