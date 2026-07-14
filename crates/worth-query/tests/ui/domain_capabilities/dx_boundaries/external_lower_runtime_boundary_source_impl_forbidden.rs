use worth_query::facade::runtime::{WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryEnvelopeSource};

struct FakeSource;

impl WorthQueryLowerRuntimeBoundaryEnvelopeSource for FakeSource {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope {
        unimplemented!()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "fake"
    }
}

fn main() {}
