use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::remask_posture::ForgeQueryRuntimeRemaskProjection;
use crate::runtime::ForgeQueryRuntimeRemaskPosture;
use crate::subscription::SubscriptionActivationInput;

#[path = "signal_routing_receipt.rs"]
mod signal_routing_receipt;

pub use signal_routing_receipt::SignalInvalidationRoutingReceipt;

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
    receipt_identity: ForgeQueryEvidenceIdentity,
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
        let receipt_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "live-view-declaration-admission-receipt",
        )
        .field_value(ForgeQueryEvidenceTag::new("view"), &view_name)
        .field_value(ForgeQueryEvidenceTag::new("target"), &target_collection)
        .field_shape(ForgeQueryEvidenceTag::new("shape"), &view_shape)
        .field_usize(
            ForgeQueryEvidenceTag::new("query_projection_count"),
            query_projection_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("result_field_count"),
            result_field_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("predicate_filter_count"),
            predicate_filter_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("traversal_count"),
            traversal_count,
        )
        .field_usize(ForgeQueryEvidenceTag::new("ordering_count"), ordering_count)
        .seal();
        let receipt_digest = receipt_identity.as_str().to_string();

        Self {
            view_name,
            target_collection,
            view_shape,
            query_projection_count,
            result_field_count,
            predicate_filter_count,
            traversal_count,
            ordering_count,
            receipt_identity,
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

    pub fn receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.receipt_identity
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
pub struct SubscriptionActivationReceipt {
    view_name: String,
    activation_identity: ForgeQueryEvidenceIdentity,
    query_declaration_identity: ForgeQueryEvidenceIdentity,
    bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    basis_binding_identity: ForgeQueryEvidenceIdentity,
    signal_strategy_identity: ForgeQueryEvidenceIdentity,
    support_identity: ForgeQueryEvidenceIdentity,
    remask_posture: Option<ForgeQueryRuntimeRemaskPosture>,
    receipt_identity: ForgeQueryEvidenceIdentity,
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
        let activation_identity =
            subscription_activation_receipt_source_identity("activation", &activation_digest);
        let query_declaration_identity = subscription_activation_receipt_source_identity(
            "query_declaration",
            &query_declaration_digest,
        );
        let bridge_declaration_identity = subscription_activation_receipt_source_identity(
            "bridge_declaration",
            &bridge_declaration_digest,
        );
        let basis_binding_identity =
            subscription_activation_receipt_source_identity("basis_binding", &basis_binding_digest);
        let signal_strategy_identity = subscription_activation_receipt_source_identity(
            "signal_strategy",
            &signal_strategy_digest,
        );
        let support_identity =
            subscription_activation_receipt_source_identity("support", &support_evidence);
        let remask_posture = remask_projection.map(|projection| {
            ForgeQueryRuntimeRemaskPosture::from_activation_projection(
                &projection,
                &support_evidence,
                &basis_binding_digest,
            )
        });
        let receipt_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "subscription-activation-receipt",
        )
        .field_value(ForgeQueryEvidenceTag::new("view"), &view_name)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("activation"),
            &activation_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query_declaration"),
            &query_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            &bridge_declaration_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_binding"),
            &basis_binding_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("signal_strategy"),
            &signal_strategy_identity,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("support"), &support_identity)
        .optional_value(
            ForgeQueryEvidenceTag::new("remask"),
            remask_posture
                .as_ref()
                .map(|posture| posture.remask_digest()),
        )
        .seal();
        let receipt_digest = receipt_identity.as_str().to_string();

        Self {
            view_name,
            activation_identity,
            query_declaration_identity,
            bridge_declaration_identity,
            basis_binding_identity,
            signal_strategy_identity,
            support_identity,
            remask_posture,
            receipt_identity,
            receipt_digest,
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn activation_digest(&self) -> &str {
        self.activation_identity.as_str()
    }

    pub fn query_declaration_digest(&self) -> &str {
        self.query_declaration_identity.as_str()
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        self.bridge_declaration_identity.as_str()
    }

    pub fn basis_binding_digest(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn signal_strategy_digest(&self) -> &str {
        self.signal_strategy_identity.as_str()
    }

    pub fn support_evidence(&self) -> &str {
        self.support_identity.as_str()
    }

    pub fn remask_posture(&self) -> Option<&ForgeQueryRuntimeRemaskPosture> {
        self.remask_posture.as_ref()
    }

    pub fn receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.receipt_identity
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

fn subscription_activation_receipt_source_identity(
    role: &str,
    source_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "subscription_activation_receipt_source_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_identity(ForgeQueryEvidenceTag::new("source_digest"), source_digest)
        .seal()
}

#[cfg(test)]
#[path = "receipts_tests.rs"]
mod tests;
