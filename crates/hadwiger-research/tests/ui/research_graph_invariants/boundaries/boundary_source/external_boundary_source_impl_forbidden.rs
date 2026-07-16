use worth_query::facade::runtime::{
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryEnvelopeSource,
};

struct FakeBoundarySource;

impl WorthQueryLowerRuntimeBoundaryEnvelopeSource for FakeBoundarySource {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope {
        panic!("fake source must not compile")
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "fake"
    }
}

fn main() {}
