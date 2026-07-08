use forge_store_physical_integrity::{LogicalDecodeGate, S3LogicalDecoder};

#[derive(Default)]
pub(crate) struct CountingSemanticDecoder {
    pub(crate) invocations: u32,
    pub(crate) semantic_index_lookups: u32,
    pub(crate) domain_constructors: u32,
}

impl<'a> S3LogicalDecoder<'a> for CountingSemanticDecoder {
    type Output = ();

    fn decode(&mut self, _gate: LogicalDecodeGate<'a>) -> Self::Output {
        self.invocations += 1;
        self.semantic_index_lookups += 1;
        self.domain_constructors += 1;
    }
}
