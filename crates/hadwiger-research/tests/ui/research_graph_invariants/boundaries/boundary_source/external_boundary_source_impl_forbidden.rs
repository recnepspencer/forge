use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
};

struct FakeBoundarySource;

impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource for FakeBoundarySource {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        panic!("fake source must not compile")
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "fake"
    }
}

fn main() {}
