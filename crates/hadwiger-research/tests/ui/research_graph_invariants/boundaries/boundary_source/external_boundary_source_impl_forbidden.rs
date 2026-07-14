use worth_query::facade::runtime::{
    WORTHQueryLowerRuntimeBoundaryEnvelope, WORTHQueryLowerRuntimeBoundaryEnvelopeSource,
};

struct FakeBoundarySource;

impl WORTHQueryLowerRuntimeBoundaryEnvelopeSource for FakeBoundarySource {
    fn lower_runtime_boundary_envelope(&self) -> &WORTHQueryLowerRuntimeBoundaryEnvelope {
        panic!("fake source must not compile")
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "fake"
    }
}

fn main() {}
