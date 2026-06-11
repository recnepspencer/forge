use worth_spatial::facade::surface_support::{
    CertifiedPlaneSupport, UnsupportedSurfaceSupport,
};

fn main() {
    let unsupported = unconstructible::<UnsupportedSurfaceSupport>();
    let _ = consume_certified_plane(unsupported);
}

fn consume_certified_plane(_support: CertifiedPlaneSupport) {}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}
