use forge_query::facade::ForgeQueryRuntimeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerSchedulerRuntimeFailure {
    DirectMutationAssertionDenied { detail: String },
    DirectMutationBindingDenied { detail: String },
    DirectMutationContinuityDenied { detail: String },
    DirectMutationNamingDenied { detail: String },
    DirectMutationTargetReferenceDenied { detail: String },
    Opaque { detail: String },
}

impl ForgeServerSchedulerRuntimeFailure {
    pub(crate) fn from_mutation_runtime_error(error: ForgeQueryRuntimeError) -> Self {
        match error {
            ForgeQueryRuntimeError::ExistingTruthAssertionDenied(_) => {
                Self::DirectMutationAssertionDenied {
                    detail: error.to_string(),
                }
            }
            ForgeQueryRuntimeError::MutationBindingDenied(_) => Self::DirectMutationBindingDenied {
                detail: error.to_string(),
            },
            ForgeQueryRuntimeError::MutationContinuityDenied(_) => {
                Self::DirectMutationContinuityDenied {
                    detail: error.to_string(),
                }
            }
            ForgeQueryRuntimeError::MutationNamingDenied(_) => Self::DirectMutationNamingDenied {
                detail: error.to_string(),
            },
            ForgeQueryRuntimeError::MutationTargetReferenceDenied(_) => {
                Self::DirectMutationTargetReferenceDenied {
                    detail: error.to_string(),
                }
            }
            other => Self::Opaque {
                detail: other.to_string(),
            },
        }
    }

    pub(crate) fn opaque(detail: impl Into<String>) -> Self {
        Self::Opaque {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::DirectMutationAssertionDenied { detail }
            | Self::DirectMutationBindingDenied { detail }
            | Self::DirectMutationContinuityDenied { detail }
            | Self::DirectMutationNamingDenied { detail }
            | Self::DirectMutationTargetReferenceDenied { detail }
            | Self::Opaque { detail } => detail,
        }
    }
}
