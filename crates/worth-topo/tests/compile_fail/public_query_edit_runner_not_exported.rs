use topology::facade::TopologyQueryEditRunner;

fn main() {
    let _ = std::any::type_name::<TopologyQueryEditRunner<'static, 'static>>();
}
