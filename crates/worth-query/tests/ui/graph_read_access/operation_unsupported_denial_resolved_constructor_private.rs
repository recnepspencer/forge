use worth_query::facade::runtime::WorthQueryGraphReadOperationUnsupportedDenial;

fn main() {
    let _ = WorthQueryGraphReadOperationUnsupportedDenial::unsupported_shape(
        "unsupported-shape",
        "cannot lower",
    );
}

