use topology::facade::TopologyMutationApplicationRunner;

fn main() {
    let _ = std::any::type_name::<TopologyMutationApplicationRunner<'static, 'static>>();
}


