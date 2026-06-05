use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
};

struct FakeSource;

impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource for FakeSource {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        unimplemented!()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "fake"
    }
}

fn main() {}
