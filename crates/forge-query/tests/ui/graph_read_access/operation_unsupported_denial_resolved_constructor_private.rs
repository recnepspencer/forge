use forge_query::facade::runtime::ForgeQueryGraphReadOperationUnsupportedDenial;

fn main() {
    let _ = ForgeQueryGraphReadOperationUnsupportedDenial::unsupported_shape(
        "unsupported-shape",
        "cannot lower",
    );
}

