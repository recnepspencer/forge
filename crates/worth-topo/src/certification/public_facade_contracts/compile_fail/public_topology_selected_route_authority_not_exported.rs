use topology::facade::TopologyQueryBackedReadFamilySelectedRouteAuthority;

fn main() {
    let _ = std::any::type_name::<&dyn TopologyQueryBackedReadFamilySelectedRouteAuthority>();
}
