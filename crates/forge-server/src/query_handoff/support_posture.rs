use forge_query::facade::{
    ForgeQueryLowerRuntimeSupportPosture, ForgeQueryRuntimePublicApiFamilyContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerQuerySupportPosture {
    ProductIndependent {
        label: String,
    },
    QueryReadSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
    },
    DirectReadSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
    },
    DirectStateSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
    },
    DirectInspectionSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
    },
    DirectProjectionSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
    },
    DirectMutationSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
    },
    QueryMutationSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
    },
    DownstreamDeliverySupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
        runtime_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
        durable_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
        contract_digest: String,
    },
    RuntimeBackedResumeSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
        runtime_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
        support_digest: String,
        contract_digest: String,
    },
    DurableResumeSupported {
        family_contract: ForgeQueryRuntimePublicApiFamilyContract,
        durable_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
        support_digest: String,
        contract_digest: String,
    },
}

impl ForgeServerQuerySupportPosture {
    pub fn runtime_resume_support_posture(
        &self,
    ) -> forge_query::facade::ForgeQueryLowerRuntimeSupportPosture {
        match self {
            Self::ProductIndependent { .. } => {
                forge_query::facade::ForgeQueryLowerRuntimeSupportPosture::Forbidden
            }
            Self::DownstreamDeliverySupported {
                runtime_resume_support_posture,
                ..
            }
            | Self::RuntimeBackedResumeSupported {
                runtime_resume_support_posture,
                ..
            } => *runtime_resume_support_posture,
            Self::DurableResumeSupported { .. }
            | Self::QueryReadSupported { .. }
            | Self::DirectReadSupported { .. }
            | Self::DirectStateSupported { .. }
            | Self::DirectInspectionSupported { .. }
            | Self::DirectProjectionSupported { .. }
            | Self::DirectMutationSupported { .. }
            | Self::QueryMutationSupported { .. } => {
                forge_query::facade::ForgeQueryLowerRuntimeSupportPosture::Forbidden
            }
        }
    }

    pub fn durable_resume_support_posture(
        &self,
    ) -> forge_query::facade::ForgeQueryLowerRuntimeSupportPosture {
        match self {
            Self::ProductIndependent { .. } => {
                forge_query::facade::ForgeQueryLowerRuntimeSupportPosture::Forbidden
            }
            Self::DownstreamDeliverySupported {
                durable_resume_support_posture,
                ..
            }
            | Self::DurableResumeSupported {
                durable_resume_support_posture,
                ..
            } => *durable_resume_support_posture,
            Self::RuntimeBackedResumeSupported { .. }
            | Self::QueryReadSupported { .. }
            | Self::DirectReadSupported { .. }
            | Self::DirectStateSupported { .. }
            | Self::DirectInspectionSupported { .. }
            | Self::DirectProjectionSupported { .. }
            | Self::DirectMutationSupported { .. }
            | Self::QueryMutationSupported { .. } => {
                forge_query::facade::ForgeQueryLowerRuntimeSupportPosture::Forbidden
            }
        }
    }

    pub(crate) fn canonical_label(&self) -> String {
        match self {
            Self::ProductIndependent { label } => {
                format!("product-independent:{label}")
            }
            Self::QueryReadSupported { family_contract } => {
                format!("query-read-supported:{}", family_contract.contract_digest())
            }
            Self::DirectReadSupported { family_contract } => {
                format!(
                    "direct-read-supported:{}",
                    family_contract.contract_digest()
                )
            }
            Self::DirectStateSupported { family_contract } => {
                format!(
                    "direct-state-supported:{}",
                    family_contract.contract_digest()
                )
            }
            Self::DirectInspectionSupported { family_contract } => format!(
                "direct-inspection-supported:{}",
                family_contract.contract_digest()
            ),
            Self::DirectProjectionSupported { family_contract } => format!(
                "direct-projection-supported:{}",
                family_contract.contract_digest()
            ),
            Self::DirectMutationSupported { family_contract } => {
                format!(
                    "direct-mutation-supported:{}",
                    family_contract.contract_digest()
                )
            }
            Self::QueryMutationSupported { family_contract } => {
                format!(
                    "query-mutation-supported:{}",
                    family_contract.contract_digest()
                )
            }
            Self::DownstreamDeliverySupported {
                family_contract,
                runtime_resume_support_posture,
                durable_resume_support_posture,
                contract_digest,
            } => format!(
                "downstream-delivery-supported:{}:{}:{}:{contract_digest}",
                family_contract.contract_digest(),
                runtime_resume_support_posture.as_str(),
                durable_resume_support_posture.as_str(),
            ),
            Self::RuntimeBackedResumeSupported {
                family_contract,
                runtime_resume_support_posture,
                support_digest,
                contract_digest,
            } => format!(
                "runtime-backed-resume-supported:{}:{}:{support_digest}:{contract_digest}",
                family_contract.contract_digest(),
                runtime_resume_support_posture.as_str(),
            ),
            Self::DurableResumeSupported {
                family_contract,
                durable_resume_support_posture,
                support_digest,
                contract_digest,
            } => format!(
                "durable-resume-supported:{}:{}:{support_digest}:{contract_digest}",
                family_contract.contract_digest(),
                durable_resume_support_posture.as_str(),
            ),
        }
    }
}
