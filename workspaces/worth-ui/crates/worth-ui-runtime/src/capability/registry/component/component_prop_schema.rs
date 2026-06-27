/// Component prop schema metadata declared before UI lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentPropSchema {
    schema_key: String,
    typed: bool,
}

impl ComponentPropSchema {
    pub fn named(schema_key: impl Into<String>) -> Self {
        Self {
            schema_key: schema_key.into(),
            typed: true,
        }
    }

    pub fn untyped_for_diagnostics(schema_key: impl Into<String>) -> Self {
        Self {
            schema_key: schema_key.into(),
            typed: false,
        }
    }

    pub fn schema_key(&self) -> &str {
        &self.schema_key
    }

    pub fn is_typed(&self) -> bool {
        self.typed
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("{}:{}", self.schema_key, self.typed)
    }
}
