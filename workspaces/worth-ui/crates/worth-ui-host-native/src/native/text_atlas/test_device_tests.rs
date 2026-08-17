use crate::native::graphics::adapter_selection::{select_eligible_adapter, AdapterCandidate};

pub(crate) fn qualified_test_device() -> (wgpu::Device, wgpu::Queue, wgpu::AdapterInfo) {
    let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    descriptor.backends = wgpu::Backends::DX12;
    let instance = wgpu::Instance::new(descriptor);
    let observed = pollster::block_on(instance.enumerate_adapters(wgpu::Backends::DX12))
        .into_iter()
        .map(|adapter| {
            let info = adapter.get_info();
            (
                AdapterCandidate {
                    surface_supported: true,
                    device_type: info.device_type,
                    limits: adapter.limits(),
                    vendor: info.vendor,
                    device: info.device,
                    name: info.name,
                    driver_info: info.driver_info,
                },
                adapter,
            )
        })
        .collect();
    let (_, adapter) = select_eligible_adapter(observed)
        .expect("qualified DX12 atlas test requires a production-eligible adapter");
    let info = adapter.get_info();
    assert_eq!(info.backend, wgpu::Backend::Dx12);
    assert!(!matches!(
        info.device_type,
        wgpu::DeviceType::Cpu | wgpu::DeviceType::Other
    ));
    let required_limits = wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits());
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("worth-ui-qualified-dx12-atlas-test-device"),
        required_features: wgpu::Features::empty(),
        required_limits,
        ..Default::default()
    }))
    .expect("qualified DX12 atlas test requires a device");
    (device, queue, info)
}
