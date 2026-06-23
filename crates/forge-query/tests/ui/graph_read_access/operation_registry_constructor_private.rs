use forge_query::facade::runtime::ForgeQueryGraphReadOperationRegistry;

fn main() {
    let _ = ForgeQueryGraphReadOperationRegistry {
        registrations: Vec::new(),
        required_capabilities: Vec::new(),
        unsupported_shapes: Vec::new(),
    };
}

