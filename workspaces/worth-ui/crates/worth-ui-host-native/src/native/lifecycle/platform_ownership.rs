use crate::native::graphics::{UiNativeDeviceOwners, UiNativeDeviceState, UiNativeOwnedDevice};
use crate::native::presentation::{
    UiNativeOwnedPresentationSurface, UiNativePresentationSurface,
    UiNativePresentationSurfaceOwners,
};
use crate::native::{UiNativeResourceClass, UiNativeResourceRegistry};

pub(crate) fn register_platform_owners(
    device: UiNativeDeviceState,
    surface: UiNativePresentationSurface,
    registry: &mut UiNativeResourceRegistry,
) -> Result<
    (UiNativeOwnedDevice, UiNativeOwnedPresentationSurface),
    Box<(UiNativeDeviceState, UiNativePresentationSurface)>,
> {
    let retains_target = surface.has_retained_target();
    let Some((device_owners, surface_owners)) = reserve_owners(registry, retains_target) else {
        return Err(Box::new((device, surface)));
    };
    Ok((
        UiNativeOwnedDevice::new(device, device_owners),
        UiNativeOwnedPresentationSurface::new(surface, surface_owners),
    ))
}

pub(crate) fn close_platform_owners(
    mut device: UiNativeOwnedDevice,
    surface: UiNativeOwnedPresentationSurface,
    registry: &mut UiNativeResourceRegistry,
) -> Result<(), Box<(UiNativeOwnedDevice, UiNativeOwnedPresentationSurface)>> {
    if device.collect_settled(registry).is_err() || !device.can_close() {
        return Err(Box::new((device, surface)));
    }
    surface.close(registry);
    if device.close(registry).is_err() {
        unreachable!("preflighted device owner closes after its presentation surface");
    }
    Ok(())
}

fn reserve_owners(
    registry: &mut UiNativeResourceRegistry,
    retains_target: bool,
) -> Option<(UiNativeDeviceOwners, UiNativePresentationSurfaceOwners)> {
    let mut classes = vec![
        UiNativeResourceClass::Surface,
        UiNativeResourceClass::Adapter,
        UiNativeResourceClass::Device,
        UiNativeResourceClass::Queue,
    ];
    if retains_target {
        classes.push(UiNativeResourceClass::RetainedTarget);
    }
    let mut owners = registry.reserve(&classes).ok()?.into_iter();
    let surface = owners.next()?;
    let adapter = owners.next()?;
    let device = owners.next()?;
    let queue = owners.next()?;
    let retained_target = owners.next();
    Some((
        UiNativeDeviceOwners {
            adapter,
            device,
            queue,
        },
        UiNativePresentationSurfaceOwners {
            surface,
            retained_target,
        },
    ))
}
