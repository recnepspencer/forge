#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryHandoffOperation {
    QueryRead {
        operation_name: String,
    },
    QueryMutation {
        operation_name: String,
    },
    DownstreamDelivery {
        view_name: String,
        requested_resume: ForgeServerQueryRequestedResume,
    },
}

impl ForgeServerQueryHandoffOperation {
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

    pub fn downstream_delivery(
        view_name: impl Into<String>,
        requested_resume: ForgeServerQueryRequestedResume,
    ) -> Self {
        Self::DownstreamDelivery {
            view_name: view_name.into(),
            requested_resume,
        }
    }

    pub(crate) fn canonical_label(&self) -> String {
        match self {
            Self::QueryRead { operation_name } => format!("query-read:{operation_name}"),
            Self::QueryMutation { operation_name } => format!("query-mutation:{operation_name}"),
            Self::DownstreamDelivery {
                view_name,
                requested_resume,
            } => format!(
                "downstream-delivery:{view_name}:{}",
                requested_resume.canonical_label()
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryOperationKind {
    QueryRead,
    QueryMutation,
    DownstreamDelivery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryRequestedResume {
    None,
    RuntimeBacked { basis_digest: Option<String> },
    Durable,
}

impl ForgeServerQueryRequestedResume {
    pub fn none() -> Self {
        Self::None
    }

    pub fn runtime_backed(basis_digest: Option<impl Into<String>>) -> Self {
        Self::RuntimeBacked {
            basis_digest: basis_digest.map(Into::into),
        }
    }

    pub fn durable() -> Self {
        Self::Durable
    }

    pub fn kind(&self) -> ForgeServerQueryRequestedResumeKind {
        match self {
            Self::None => ForgeServerQueryRequestedResumeKind::None,
            Self::RuntimeBacked { .. } => ForgeServerQueryRequestedResumeKind::RuntimeBacked,
            Self::Durable => ForgeServerQueryRequestedResumeKind::Durable,
        }
    }

    pub(crate) fn canonical_label(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::RuntimeBacked { basis_digest } => {
                format!(
                    "runtime-backed:{}",
                    basis_digest.as_deref().unwrap_or("none")
                )
            }
            Self::Durable => "durable".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryRequestedResumeKind {
    None,
    RuntimeBacked,
    Durable,
}
