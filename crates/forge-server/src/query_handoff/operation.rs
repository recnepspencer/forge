use forge_query::facade::ForgeQueryWriteCommand;

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeServerQueryOperation {
    SingleMutation {
        operation_name: String,
        command: ForgeQueryWriteCommand,
    },
    BatchMutation {
        operation_name: String,
        commands: Vec<ForgeQueryWriteCommand>,
    },
}

impl ForgeServerQueryOperation {
    pub fn single_mutation(
        operation_name: impl Into<String>,
        command: ForgeQueryWriteCommand,
    ) -> Self {
        Self::SingleMutation {
            operation_name: operation_name.into(),
            command,
        }
    }

    pub fn batch_mutation(
        operation_name: impl Into<String>,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Self {
        Self::BatchMutation {
            operation_name: operation_name.into(),
            commands,
        }
    }

    pub fn operation_name(&self) -> &str {
        match self {
            Self::SingleMutation { operation_name, .. }
            | Self::BatchMutation { operation_name, .. } => operation_name,
        }
    }

    pub fn is_batch(&self) -> bool {
        matches!(self, Self::BatchMutation { .. })
    }

    pub fn mutation_count(&self) -> usize {
        match self {
            Self::SingleMutation { .. } => 1,
            Self::BatchMutation { commands, .. } => commands.len(),
        }
    }

    pub fn as_single_command(&self) -> Option<&ForgeQueryWriteCommand> {
        match self {
            Self::SingleMutation { command, .. } => Some(command),
            Self::BatchMutation { .. } => None,
        }
    }

    pub fn as_batch_commands(&self) -> Option<&[ForgeQueryWriteCommand]> {
        match self {
            Self::SingleMutation { .. } => None,
            Self::BatchMutation { commands, .. } => Some(commands),
        }
    }

    pub(crate) fn handoff_operation(&self) -> ForgeServerQueryHandoffOperation {
        ForgeServerQueryHandoffOperation::direct_mutation(self.operation_name())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryHandoffOperation {
    QueryRead {
        operation_name: String,
    },
    DirectRead {
        operation_name: String,
    },
    DirectState {
        target_label: String,
    },
    DirectInspection {
        target_label: String,
    },
    DirectProjection {
        target_label: String,
    },
    DirectMutation {
        operation_name: String,
    },
    QueryMutation {
        operation_name: String,
    },
    DownstreamDelivery {
        view_name: String,
        freshness_mode: crate::ForgeServerDirectFreshnessMode,
        delivery_class: crate::ForgeServerDirectDeliveryClass,
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

    pub fn direct_read(operation_name: impl Into<String>) -> Self {
        Self::DirectRead {
            operation_name: operation_name.into(),
        }
    }

    pub fn direct_state(target_label: impl Into<String>) -> Self {
        Self::DirectState {
            target_label: target_label.into(),
        }
    }

    pub fn direct_inspection(target_label: impl Into<String>) -> Self {
        Self::DirectInspection {
            target_label: target_label.into(),
        }
    }

    pub fn direct_projection(target_label: impl Into<String>) -> Self {
        Self::DirectProjection {
            target_label: target_label.into(),
        }
    }

    pub fn direct_mutation(operation_name: impl Into<String>) -> Self {
        Self::DirectMutation {
            operation_name: operation_name.into(),
        }
    }

    pub fn downstream_delivery(
        view_name: impl Into<String>,
        freshness_mode: crate::ForgeServerDirectFreshnessMode,
        delivery_class: crate::ForgeServerDirectDeliveryClass,
        requested_resume: ForgeServerQueryRequestedResume,
    ) -> Self {
        Self::DownstreamDelivery {
            view_name: view_name.into(),
            freshness_mode,
            delivery_class,
            requested_resume,
        }
    }

    pub(crate) fn canonical_label(&self) -> String {
        match self {
            Self::QueryRead { operation_name } => format!("query-read:{operation_name}"),
            Self::DirectRead { operation_name } => format!("direct-read:{operation_name}"),
            Self::DirectState { target_label } => format!("direct-state:{target_label}"),
            Self::DirectInspection { target_label } => {
                format!("direct-inspection:{target_label}")
            }
            Self::DirectProjection { target_label } => {
                format!("direct-projection:{target_label}")
            }
            Self::DirectMutation { operation_name } => {
                format!("direct-mutation:{operation_name}")
            }
            Self::QueryMutation { operation_name } => format!("query-mutation:{operation_name}"),
            Self::DownstreamDelivery {
                view_name,
                freshness_mode,
                delivery_class,
                requested_resume,
            } => format!(
                "downstream-delivery:{view_name}:{}:{}:{}",
                freshness_mode.as_str(),
                delivery_class.as_str(),
                requested_resume.canonical_label()
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryOperationKind {
    QueryRead,
    DirectRead,
    DirectState,
    DirectInspection,
    DirectProjection,
    DirectMutation,
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
