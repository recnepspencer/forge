use worth_schema::facade::WorthTopologyAuthority;

fn main() {
    let _ = std::mem::size_of::<WorthTopologyAuthority<'static>>();
}
