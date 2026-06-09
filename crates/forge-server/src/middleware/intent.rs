#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerPipelineIntent {
    ForgeNativeSession { operation_name: String },
    QueryRead { operation_name: String },
    QueryMutation { operation_name: String },
}

impl ForgeServerPipelineIntent {
    pub fn forge_native_session(operation_name: impl Into<String>) -> Self {
        Self::ForgeNativeSession {
            operation_name: operation_name.into(),
        }
    }

    pub fn query_read(operation_name: impl Into<String>) -> Self {
        Self::QueryRead {
            operation_name: operation_name.into(),
        }
    }

    pub fn query_mutation(operation_name: impl Into<String>) -> Self {
        Self::QueryMutation {
            operation_name: operation_name.into(),
        }
    }

    pub fn operation_name(&self) -> &str {
        match self {
            Self::ForgeNativeSession { operation_name }
            | Self::QueryRead { operation_name }
            | Self::QueryMutation { operation_name } => operation_name,
        }
    }
}
