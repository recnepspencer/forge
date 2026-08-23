use super::super::locality::{LocalityMatchClass, LocalityWideningDecision};
use super::super::RegionScopedExecutionReport;
use super::super::{LivePolicyCounters, LiveReplayBundle};
use super::query_contract::DeliveryLocalityOutcome;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryContractReplayRecord {
    pub(in crate::live) digest: String,
    pub(in crate::live) query_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) replay_digest: String,
    pub(in crate::live) locality_outcome: DeliveryLocalityOutcome,
    pub(in crate::live) stream_contract_digest: Option<String>,
}

impl DeliveryContractReplayRecord {
    pub(in crate::live) fn from_region_execution(
        report: &RegionScopedExecutionReport,
        replay_bundle: &LiveReplayBundle,
    ) -> Self {
        let locality_outcome = DeliveryLocalityOutcome::from_region_scoped_report(report);
        Self {
            digest: hash_parts(&[
                format!("query:{}", report.query_digest()),
                format!("delivery:{}", report.delivery_digest()),
                format!("replay:{}", report.replay_digest()),
                format!("locality_outcome:{}", locality_outcome.as_str()),
                "stream_contract:none".to_string(),
            ]),
            query_digest: report.query_digest().to_string(),
            delivery_digest: report.delivery_digest().to_string(),
            replay_digest: replay_bundle.replay_digest().to_string(),
            locality_outcome,
            stream_contract_digest: None,
        }
    }
    pub(in crate::live) fn with_stream_contract_digest(
        &self,
        stream_contract_digest: &str,
    ) -> Self {
        Self {
            digest: hash_parts(&[
                format!("query:{}", self.query_digest),
                format!("delivery:{}", self.delivery_digest),
                format!("replay:{}", self.replay_digest),
                format!("locality_outcome:{}", self.locality_outcome.as_str()),
                format!("stream_contract:{stream_contract_digest}"),
            ]),
            query_digest: self.query_digest.clone(),
            delivery_digest: self.delivery_digest.clone(),
            replay_digest: self.replay_digest.clone(),
            locality_outcome: self.locality_outcome.clone(),
            stream_contract_digest: Some(stream_contract_digest.to_string()),
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn locality_outcome(&self) -> &DeliveryLocalityOutcome {
        &self.locality_outcome
    }

    pub fn stream_contract_digest(&self) -> Option<&str> {
        self.stream_contract_digest.as_deref()
    }
}

impl DeliveryLocalityOutcome {
    pub(in crate::live) fn from_region_scoped_report(report: &RegionScopedExecutionReport) -> Self {
        match (report.locality_match_class(), report.widening_decision()) {
            (LocalityMatchClass::RegionMatch(_), None) => Self::InRegionRegion,
            (
                LocalityMatchClass::RegionMatch(_),
                Some(LocalityWideningDecision::Admitted { peer_scopes, .. }),
            ) => Self::InRegionRegionWithPeerWidening {
                peer_scopes: peer_scopes.clone(),
            },
            (LocalityMatchClass::PartitionMatch(_), None) => Self::InRegionPartition,
            (
                LocalityMatchClass::PartitionMatch(_),
                Some(LocalityWideningDecision::Admitted { peer_scopes, .. }),
            ) => Self::InRegionPartitionWithPeerWidening {
                peer_scopes: peer_scopes.clone(),
            },
            (LocalityMatchClass::OffRegionSuppressed { .. }, None) => Self::OffRegionSuppressed,
            (_, Some(LocalityWideningDecision::Denied { .. })) => {
                unreachable!("widening denials do not produce admitted region-scoped deliveries")
            }
            (LocalityMatchClass::OffRegionSuppressed { .. }, Some(_)) => {
                unreachable!("suppressed deliveries do not carry widening admissions")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedReplayBundle {
    pub(in crate::live) locality_digest: String,
    pub(in crate::live) replay_record: DeliveryContractReplayRecord,
    pub(in crate::live) bundle: LiveReplayBundle,
}

impl RegionScopedReplayBundle {
    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn replay_record(&self) -> &DeliveryContractReplayRecord {
        &self.replay_record
    }

    pub fn live_replay_bundle(&self) -> &LiveReplayBundle {
        &self.bundle
    }

    pub fn counter_snapshot(&self) -> &LivePolicyCounters {
        self.bundle.counter_snapshot()
    }
}
