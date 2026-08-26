#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductReadTransport {
    FlatQuery,
    StructuredQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationBasisKind {
    QueryDerived,
    PrimaryGraphApplication,
    ProductSessionDerived,
    DurableProductDerived,
    FixtureOnly,
}

impl WorthServerProductOperationBasisKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryDerived => "query-derived",
            Self::PrimaryGraphApplication => "primary-graph-application",
            Self::ProductSessionDerived => "product-session-derived",
            Self::DurableProductDerived => "durable-product-derived",
            Self::FixtureOnly => "fixture-only",
        }
    }
}
