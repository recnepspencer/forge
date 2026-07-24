macro_rules! resource_family {
    ($name:ident, $empty:literal) => {
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err($empty);
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

resource_family!(
    WorthQueryExecutionProviderFamily,
    "empty-execution-provider-family"
);
resource_family!(
    WorthQueryExecutionAccessProductFamily,
    "empty-execution-access-product-family"
);
resource_family!(
    WorthQueryExecutionAllocatorFamily,
    "empty-execution-allocator-family"
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionProviderRequirements {
    provider: WorthQueryExecutionProviderFamily,
    access_product: WorthQueryExecutionAccessProductFamily,
    allocator: WorthQueryExecutionAllocatorFamily,
}

impl WorthQueryExecutionProviderRequirements {
    pub fn new(
        provider: WorthQueryExecutionProviderFamily,
        access_product: WorthQueryExecutionAccessProductFamily,
        allocator: WorthQueryExecutionAllocatorFamily,
    ) -> Self {
        Self {
            provider,
            access_product,
            allocator,
        }
    }

    pub fn provider(&self) -> &WorthQueryExecutionProviderFamily {
        &self.provider
    }

    pub fn access_product(&self) -> &WorthQueryExecutionAccessProductFamily {
        &self.access_product
    }

    pub fn allocator(&self) -> &WorthQueryExecutionAllocatorFamily {
        &self.allocator
    }
}
