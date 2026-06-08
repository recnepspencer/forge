#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerPipelineIntent {
    QueryRead { operation_name: String },
    QueryMutation { operation_name: String },
}

impl ForgeServerPipelineIntent {
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
            Self::QueryRead { operation_name } | Self::QueryMutation { operation_name } => {
                operation_name
            }
        }
    }
}
