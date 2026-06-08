use std::collections::BTreeSet;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryMutationReceipt;
use crate::runtime::remask_posture::ForgeQueryRuntimeRemaskProjection;
use crate::runtime::ForgeQueryRuntimeRemaskPosture;
use crate::subscription::SubscriptionActivationInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveViewDeclarationAdmissionReceipt {
    view_name: String,
    target_collection: String,
    view_shape: String,
    query_projection_count: usize,
    result_field_count: usize,
    predicate_filter_count: usize,
    traversal_count: usize,
    ordering_count: usize,
    receipt_digest: String,
}

impl LiveViewDeclarationAdmissionReceipt {
    pub(crate) fn from_request(
        view_name: impl Into<String>,
        request: &DeclarativeLiveQueryRequest,
    ) -> Self {
        let view_name = view_name.into();
        let target_collection = request.target().to_string();
        let view_shape = request.view_shape().as_str().to_string();
        let query_projection_count = request.query_projection().len();
        let result_field_count = request.result_fields().len();
        let predicate_filter_count = request.predicate_filters().len();
        let traversal_count = request.traversal().len();
        let ordering_count = request.ordering().len();
        let receipt_digest = hash_parts(&[
            "live_view_declaration_admission_receipt_v1".to_string(),
            format!("view:{view_name}"),
            format!("target:{target_collection}"),
            format!("shape:{view_shape}"),
            format!("query_projection_count:{query_projection_count}"),
            format!("result_field_count:{result_field_count}"),
            format!("predicate_filter_count:{predicate_filter_count}"),
            format!("traversal_count:{traversal_count}"),
            format!("ordering_count:{ordering_count}"),
        ]);

        Self {
            view_name,
            target_collection,
            view_shape,
            query_projection_count,
            result_field_count,
            predicate_filter_count,
            traversal_count,
            ordering_count,
            receipt_digest,
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn target_collection(&self) -> &str {
        &self.target_collection
    }

    pub fn view_shape(&self) -> &str {
        &self.view_shape
    }

    pub fn query_projection_count(&self) -> usize {
        self.query_projection_count
    }

    pub fn result_field_count(&self) -> usize {
        self.result_field_count
    }

    pub fn predicate_filter_count(&self) -> usize {
        self.predicate_filter_count
    }

    pub fn traversal_count(&self) -> usize {
        self.traversal_count
    }

    pub fn ordering_count(&self) -> usize {
        self.ordering_count
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub(crate) fn drift_from_request(
        &self,
        expected_view_name: &str,
        request: &DeclarativeLiveQueryRequest,
    ) -> Option<String> {
        if self.view_name() != expected_view_name {
            return Some(format!(
                "live-view admission receipt view drifted: expected `{expected_view_name}`, found `{}`",
                self.view_name()
            ));
        }
        if self.target_collection() != request.target() {
            return Some(format!(
                "live-view admission receipt target drifted: expected `{}`, found `{}`",
                request.target(),
                self.target_collection()
            ));
        }
        if self.view_shape() != request.view_shape().as_str()
            || self.query_projection_count() != request.query_projection().len()
            || self.result_field_count() != request.result_fields().len()
            || self.predicate_filter_count() != request.predicate_filters().len()
            || self.traversal_count() != request.traversal().len()
            || self.ordering_count() != request.ordering().len()
        {
            return Some(
                "live-view admission receipt shape drifted from the declared request".to_string(),
            );
        }
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalInvalidationRoutingReceipt {
    commit_identity: String,
    snapshot_token: String,
    delta_count: usize,
    routed_collection_count: usize,
    receipt_digest: String,
}

impl SignalInvalidationRoutingReceipt {
    pub(crate) fn from_mutation_receipt(receipt: &ForgeQueryMutationReceipt) -> Self {
        let routed_collection_count = receipt
            .deltas
            .iter()
            .map(|delta| delta.collection.clone())
            .collect::<BTreeSet<_>>()
            .len();
        let delta_count = receipt.deltas.len();
        let commit_identity = receipt.commit_identity.clone();
        let snapshot_token = receipt.snapshot_token.clone();
        let receipt_digest = hash_parts(&[
            "signal_invalidation_routing_receipt_v1".to_string(),
            format!("commit:{commit_identity}"),
            format!("snapshot:{snapshot_token}"),
            format!("delta_count:{delta_count}"),
            format!("routed_collection_count:{routed_collection_count}"),
        ]);

        Self {
            commit_identity,
            snapshot_token,
            delta_count,
            routed_collection_count,
            receipt_digest,
        }
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn delta_count(&self) -> usize {
        self.delta_count
    }

    pub fn routed_collection_count(&self) -> usize {
        self.routed_collection_count
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub(crate) fn drift_from_mutation_receipt(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Option<String> {
        if self.commit_identity() != receipt.commit_identity
            || self.snapshot_token() != receipt.snapshot_token
        {
            return Some(format!(
                "signal invalidation routing receipt drifted from write receipt: expected commit `{}` / snapshot `{}`, found commit `{}` / snapshot `{}`",
                receipt.commit_identity,
                receipt.snapshot_token,
                self.commit_identity(),
                self.snapshot_token()
            ));
        }
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionActivationReceipt {
    view_name: String,
    activation_digest: String,
    query_declaration_digest: String,
    bridge_declaration_digest: String,
    basis_binding_digest: String,
    signal_strategy_digest: String,
    support_evidence: String,
    remask_posture: Option<ForgeQueryRuntimeRemaskPosture>,
    receipt_digest: String,
}

impl SubscriptionActivationReceipt {
    pub(crate) fn from_activation(
        view_name: impl Into<String>,
        activation: &SubscriptionActivationInput,
        support_evidence: impl Into<String>,
        remask_projection: Option<ForgeQueryRuntimeRemaskProjection>,
    ) -> Self {
        let view_name = view_name.into();
        let activation_digest = activation.activation_digest().to_string();
        let query_declaration_digest = activation.query_declaration_digest().to_string();
        let bridge_declaration_digest = activation.bridge_declaration_digest().to_string();
        let basis_binding_digest = activation.basis_binding_digest().to_string();
        let signal_strategy_digest = activation.signal_strategy_digest().to_string();
        let support_evidence = support_evidence.into();
        let remask_posture = remask_projection.map(|projection| {
            ForgeQueryRuntimeRemaskPosture::from_activation_projection(
                &projection,
                &support_evidence,
                &basis_binding_digest,
            )
        });
        let receipt_digest = hash_parts(&[
            "subscription_activation_receipt_v1".to_string(),
            format!("view:{view_name}"),
            format!("activation:{activation_digest}"),
            format!("query_declaration:{query_declaration_digest}"),
            format!("bridge_declaration:{bridge_declaration_digest}"),
            format!("basis_binding:{basis_binding_digest}"),
            format!("signal_strategy:{signal_strategy_digest}"),
            format!("support:{support_evidence}"),
            format!(
                "remask:{}",
                remask_posture
                    .as_ref()
                    .map_or("none", |posture| posture.remask_digest())
            ),
        ]);

        Self {
            view_name,
            activation_digest,
            query_declaration_digest,
            bridge_declaration_digest,
            basis_binding_digest,
            signal_strategy_digest,
            support_evidence,
            remask_posture,
            receipt_digest,
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
    }

    pub fn support_evidence(&self) -> &str {
        &self.support_evidence
    }

    pub fn remask_posture(&self) -> Option<&ForgeQueryRuntimeRemaskPosture> {
        self.remask_posture.as_ref()
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub(crate) fn drift_from_activation(
        &self,
        expected_view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Option<String> {
        if self.view_name() != expected_view_name {
            return Some(format!(
                "subscription activation receipt view drifted: expected `{expected_view_name}`, found `{}`",
                self.view_name()
            ));
        }
        if self.activation_digest() != activation.activation_digest()
            || self.query_declaration_digest() != activation.query_declaration_digest()
            || self.bridge_declaration_digest() != activation.bridge_declaration_digest()
            || self.basis_binding_digest() != activation.basis_binding_digest()
            || self.signal_strategy_digest() != activation.signal_strategy_digest()
        {
            return Some(
                "subscription activation receipt drifted from activation input".to_string(),
            );
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
    use crate::memory_workspace::{ForgeQueryMutationDelta, ForgeQueryMutationKind};

    #[test]
    fn live_view_declaration_receipt_captures_request_shape() {
        let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table());
        let receipt = LiveViewDeclarationAdmissionReceipt::from_request("tasks.table", &request);

        assert_eq!(receipt.view_name(), "tasks.table");
        assert_eq!(receipt.target_collection(), "Task");
        assert_eq!(receipt.view_shape(), "table");
        assert!(!receipt.receipt_digest().is_empty());
    }

    #[test]
    fn signal_invalidation_routing_receipt_summarizes_delta_width() {
        let receipt = ForgeQueryMutationReceipt {
            commit_identity: "commit-1".to_string(),
            snapshot_token: "snapshot-1".to_string(),
            deltas: vec![
                ForgeQueryMutationDelta {
                    collection: "Task".to_string(),
                    entity_identity: "task-1".to_string(),
                    kind: ForgeQueryMutationKind::Created,
                    aspect_paths: vec!["title.value".to_string()],
                },
                ForgeQueryMutationDelta {
                    collection: "Task".to_string(),
                    entity_identity: "task-2".to_string(),
                    kind: ForgeQueryMutationKind::Updated,
                    aspect_paths: vec!["status.value".to_string()],
                },
            ],
            bridge_authority: None,
        };

        let routed = SignalInvalidationRoutingReceipt::from_mutation_receipt(&receipt);

        assert_eq!(routed.commit_identity(), "commit-1");
        assert_eq!(routed.delta_count(), 2);
        assert_eq!(routed.routed_collection_count(), 1);
        assert!(!routed.receipt_digest().is_empty());
    }
}
