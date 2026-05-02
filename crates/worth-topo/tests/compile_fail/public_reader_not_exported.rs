use worth_topo::facade::WorthTopologyReader;

fn main() {
    let _ = std::mem::size_of::<WorthTopologyReader<'static>>();
}
