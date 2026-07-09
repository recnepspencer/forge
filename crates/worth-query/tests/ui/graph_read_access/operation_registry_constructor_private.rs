use worth_query::facade::runtime::WorthQueryGraphReadOperationRegistry;

fn main() {
    let _ = WorthQueryGraphReadOperationRegistry {
        registrations: Vec::new(),
        required_capabilities: Vec::new(),
        unsupported_shapes: Vec::new(),
    };
}

