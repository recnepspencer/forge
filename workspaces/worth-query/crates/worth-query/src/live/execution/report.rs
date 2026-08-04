use super::super::patches::LivePatchEnvelope;
use super::super::promotion::LiveQueryFamily;
use super::super::telemetry::LivePolicyCounters;
use super::replay::LiveReplayBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveExecutionReport {
    pub(in crate::live) query_digest: String,
    pub(in crate::live) result_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) replay_digest: String,
    pub(in crate::live) family: LiveQueryFamily,
    pub(in crate::live) outcome_kind: String,
    pub(in crate::live) outcome_digest: String,
    pub(in crate::live) basis_digest: String,
    pub(in crate::live) subscription_digest: String,
}

impl LiveExecutionReport {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn outcome_kind(&self) -> &str {
        &self.outcome_kind
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn subscription_digest(&self) -> &str {
        &self.subscription_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveExecutionEnvelope {
    pub(in crate::live) report: LiveExecutionReport,
    pub(in crate::live) patch_envelope: LivePatchEnvelope,
    pub(in crate::live) replay_bundle: LiveReplayBundle,
    pub(in crate::live) counters: LivePolicyCounters,
}

impl LiveExecutionEnvelope {
    pub fn report(&self) -> &LiveExecutionReport {
        &self.report
    }

    pub fn patch_envelope(&self) -> &LivePatchEnvelope {
        &self.patch_envelope
    }

    pub fn replay_bundle(&self) -> &LiveReplayBundle {
        &self.replay_bundle
    }

    pub fn counters(&self) -> &LivePolicyCounters {
        &self.counters
    }
}
