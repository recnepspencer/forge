use crate::declaration::stable_text_digest;
use crate::graph::UiGraphNodeIdentity;

use crate::evidence::measurement::{UiMeasurementResult, UiSettledQueryFactReceipt};

#[derive(Clone, Debug, PartialEq)]
pub enum UiChildIntrinsicMeasurementSource {
    SettledQueryFact(UiSettledQueryFactReceipt),
    HostMeasurementResult(UiMeasurementResult),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiChildIntrinsicMeasurementEvidence {
    contributor_graph_node_identity: UiGraphNodeIdentity,
    source: UiChildIntrinsicMeasurementSource,
    identity_digest: u64,
}

impl UiChildIntrinsicMeasurementEvidence {
    pub(crate) fn operationally_matches(&self, other: &Self) -> bool {
        if self.contributor_graph_node_identity != other.contributor_graph_node_identity {
            return false;
        }
        match (&self.source, &other.source) {
            (
                UiChildIntrinsicMeasurementSource::SettledQueryFact(left),
                UiChildIntrinsicMeasurementSource::SettledQueryFact(right),
            ) => left == right,
            (
                UiChildIntrinsicMeasurementSource::HostMeasurementResult(left),
                UiChildIntrinsicMeasurementSource::HostMeasurementResult(right),
            ) => left.operationally_matches(right),
            _ => false,
        }
    }

    pub fn for_query_projection_fact(
        contributor_graph_node_identity: UiGraphNodeIdentity,
        receipt: &UiSettledQueryFactReceipt,
    ) -> Self {
        Self::new(
            contributor_graph_node_identity,
            UiChildIntrinsicMeasurementSource::SettledQueryFact(receipt.clone()),
        )
    }

    pub fn for_host_measurement_result(
        contributor_graph_node_identity: UiGraphNodeIdentity,
        result: &UiMeasurementResult,
    ) -> Self {
        Self::new(
            contributor_graph_node_identity,
            UiChildIntrinsicMeasurementSource::HostMeasurementResult(result.clone()),
        )
    }

    pub fn contributor_graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.contributor_graph_node_identity
    }

    pub fn query_projection_fact(&self) -> Option<&UiSettledQueryFactReceipt> {
        match &self.source {
            UiChildIntrinsicMeasurementSource::SettledQueryFact(receipt) => Some(receipt),
            UiChildIntrinsicMeasurementSource::HostMeasurementResult(_) => None,
        }
    }

    pub fn host_measurement_result(&self) -> Option<&UiMeasurementResult> {
        match &self.source {
            UiChildIntrinsicMeasurementSource::HostMeasurementResult(result) => Some(result),
            UiChildIntrinsicMeasurementSource::SettledQueryFact(_) => None,
        }
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    fn new(
        contributor_graph_node_identity: UiGraphNodeIdentity,
        source: UiChildIntrinsicMeasurementSource,
    ) -> Self {
        let identity_digest = stable_text_digest("worth-ui.child-intrinsic-measurement")
            ^ contributor_graph_node_identity.digest().rotate_left(7)
            ^ source_identity_digest(&source).rotate_left(13);
        Self {
            contributor_graph_node_identity,
            source,
            identity_digest,
        }
    }
}

fn source_identity_digest(source: &UiChildIntrinsicMeasurementSource) -> u64 {
    match source {
        UiChildIntrinsicMeasurementSource::SettledQueryFact(receipt) => {
            stable_text_digest("worth-ui.child-intrinsic-measurement.query")
                ^ receipt.observation_identity_digest().rotate_left(7)
                ^ receipt
                    .declaration_support_authority_generation()
                    .as_u64()
                    .rotate_left(13)
        }
        UiChildIntrinsicMeasurementSource::HostMeasurementResult(result) => {
            stable_text_digest("worth-ui.child-intrinsic-measurement.host")
                ^ result.request_identity().as_u64().rotate_left(7)
                ^ result.evidence_generation().as_u64().rotate_left(13)
        }
    }
}
