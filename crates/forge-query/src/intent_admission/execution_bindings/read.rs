use crate::identity::hash_parts;
use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{ForgeQueryReadFamily, ForgeQueryRuntimeLiveSubscriptionInstallation};

use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily,
};
use crate::intent_admission::ForgeQueryLiveReadExecutionHandoff;
use crate::intent_admission::ForgeQueryReadExecutionHandoff;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadExecutionBinding {
    handoff: ForgeQueryReadExecutionHandoff,
    binding_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveReadExecutionBinding {
    handoff: ForgeQueryLiveReadExecutionHandoff,
    binding_digest: String,
}

impl ForgeQueryReadExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryReadExecutionHandoff) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_read_execution_binding_v1".to_string(),
            format!("handoff:{}", handoff.handoff_digest()),
        ]);
        Self {
            handoff,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn read_family(&self) -> &ForgeQueryReadFamily {
        self.handoff.read_family()
    }

    pub fn basis_context(&self) -> Option<&AdmittedQueryBasisContext> {
        self.handoff.basis_context()
    }

    pub fn handoff(&self) -> &ForgeQueryReadExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

impl ForgeQueryLiveReadExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryLiveReadExecutionHandoff) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_live_read_execution_binding_v1".to_string(),
            format!("handoff:{}", handoff.handoff_digest()),
        ]);
        Self {
            handoff,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        self.handoff.installation()
    }

    pub fn handoff(&self) -> &ForgeQueryLiveReadExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}
