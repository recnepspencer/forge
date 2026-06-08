use forge_query::facade::{
    ForgeQueryLowerRuntimeSupportPosture, ForgeQueryRuntimePublicApiFamilyContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerQuerySupportPosture {
    QueryReadSupported {
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
    pub(crate) fn canonical_label(&self) -> String {
        match self {
            Self::QueryReadSupported { family_contract } => {
                format!("query-read-supported:{}", family_contract.contract_digest())
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
