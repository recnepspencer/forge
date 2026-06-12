use forge_runtime_bridge::facade::BridgeIdentityEvidence;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceIdentityEncoder,
    ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeLowerRuntimeEvidenceKind {
    TruthViewEvaluation,
    ContinuityDelivery,
    SubscriptionAdmission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeLowerRuntimeEvidenceReference {
    kind: BridgeLowerRuntimeEvidenceKind,
    detail: BridgeLowerRuntimeEvidenceDetail,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum BridgeLowerRuntimeEvidenceDetail {
    TruthViewEvaluation {
        record_identity: BridgeIdentityEvidence,
        selector_identity: BridgeIdentityEvidence,
        authority_digest: String,
        snapshot_identity: BridgeIdentityEvidence,
    },
    ContinuityDelivery {
        route_identity: BridgeIdentityEvidence,
        continuity_identity: BridgeIdentityEvidence,
        continuity_resolution_digest: String,
        source_snapshot_identity: BridgeIdentityEvidence,
    },
    SubscriptionAdmission {
        admitted_subscription_identity: BridgeIdentityEvidence,
        basis_identity: BridgeIdentityEvidence,
        strategy_identity: BridgeIdentityEvidence,
        subscription_digest: String,
    },
}

impl BridgeLowerRuntimeEvidenceReference {
    pub fn kind(&self) -> BridgeLowerRuntimeEvidenceKind {
        self.kind
    }

    pub fn record_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                record_identity, ..
            } => Some(record_identity.as_str()),
            _ => None,
        }
    }

    pub fn selector_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                selector_identity, ..
            } => Some(selector_identity.as_str()),
            _ => None,
        }
    }

    pub fn authority_digest(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                authority_digest, ..
            } => Some(authority_digest.as_str()),
            _ => None,
        }
    }

    pub fn snapshot_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                snapshot_identity, ..
            } => Some(snapshot_identity.as_str()),
            _ => None,
        }
    }

    pub fn route_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery { route_identity, .. } => {
                Some(route_identity.as_str())
            }
            _ => None,
        }
    }

    pub fn continuity_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                continuity_identity,
                ..
            } => Some(continuity_identity.as_str()),
            _ => None,
        }
    }

    pub fn continuity_resolution_digest(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                continuity_resolution_digest,
                ..
            } => Some(continuity_resolution_digest.as_str()),
            _ => None,
        }
    }

    pub fn source_snapshot_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                source_snapshot_identity,
                ..
            } => Some(source_snapshot_identity.as_str()),
            _ => None,
        }
    }

    pub fn admitted_subscription_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                admitted_subscription_identity,
                ..
            } => Some(admitted_subscription_identity.as_str()),
            _ => None,
        }
    }

    pub fn basis_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission { basis_identity, .. } => {
                Some(basis_identity.as_str())
            }
            _ => None,
        }
    }

    pub fn strategy_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                strategy_identity, ..
            } => Some(strategy_identity.as_str()),
            _ => None,
        }
    }

    pub fn subscription_digest(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                subscription_digest,
                ..
            } => Some(subscription_digest.as_str()),
            _ => None,
        }
    }

    pub(super) fn truth_view(
        record_identity: BridgeIdentityEvidence,
        selector_identity: BridgeIdentityEvidence,
        authority_digest: String,
        snapshot_identity: BridgeIdentityEvidence,
    ) -> Self {
        Self {
            kind: BridgeLowerRuntimeEvidenceKind::TruthViewEvaluation,
            detail: BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                record_identity,
                selector_identity,
                authority_digest,
                snapshot_identity,
            },
        }
    }

    pub(super) fn continuity(
        route_identity: BridgeIdentityEvidence,
        continuity_identity: BridgeIdentityEvidence,
        continuity_resolution_digest: String,
        source_snapshot_identity: BridgeIdentityEvidence,
    ) -> Self {
        Self {
            kind: BridgeLowerRuntimeEvidenceKind::ContinuityDelivery,
            detail: BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                route_identity,
                continuity_identity,
                continuity_resolution_digest,
                source_snapshot_identity,
            },
        }
    }

    pub(super) fn subscription(
        admitted_subscription_identity: BridgeIdentityEvidence,
        basis_identity: BridgeIdentityEvidence,
        strategy_identity: BridgeIdentityEvidence,
        subscription_digest: String,
    ) -> Self {
        Self {
            kind: BridgeLowerRuntimeEvidenceKind::SubscriptionAdmission,
            detail: BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                admitted_subscription_identity,
                basis_identity,
                strategy_identity,
                subscription_digest,
            },
        }
    }

    fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        let encoder = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::BridgeLowerRuntimeEvidenceReference,
        );
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                record_identity,
                selector_identity,
                authority_digest,
                snapshot_identity,
            } => bridge_identity_field(
                bridge_identity_field(
                    bridge_identity_field(
                        encoder
                            .field_shape(
                                ForgeQueryEvidenceTag::new("kind"),
                                "truth_view_evaluation",
                            )
                            .field_identity(
                                ForgeQueryEvidenceTag::new("authority_digest"),
                                authority_digest,
                            ),
                        ForgeQueryEvidenceTag::new("record_identity"),
                        record_identity,
                    ),
                    ForgeQueryEvidenceTag::new("selector_identity"),
                    selector_identity,
                ),
                ForgeQueryEvidenceTag::new("snapshot_identity"),
                snapshot_identity,
            )
            .seal(),
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                route_identity,
                continuity_identity,
                continuity_resolution_digest,
                source_snapshot_identity,
            } => bridge_identity_field(
                bridge_identity_field(
                    bridge_identity_field(
                        encoder
                            .field_shape(ForgeQueryEvidenceTag::new("kind"), "continuity_delivery")
                            .field_identity(
                                ForgeQueryEvidenceTag::new("continuity_resolution_digest"),
                                continuity_resolution_digest,
                            ),
                        ForgeQueryEvidenceTag::new("route_identity"),
                        route_identity,
                    ),
                    ForgeQueryEvidenceTag::new("continuity_identity"),
                    continuity_identity,
                ),
                ForgeQueryEvidenceTag::new("source_snapshot_identity"),
                source_snapshot_identity,
            )
            .seal(),
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                admitted_subscription_identity,
                basis_identity,
                strategy_identity,
                subscription_digest,
            } => bridge_identity_field(
                bridge_identity_field(
                    bridge_identity_field(
                        encoder
                            .field_shape(
                                ForgeQueryEvidenceTag::new("kind"),
                                "subscription_admission",
                            )
                            .field_identity(
                                ForgeQueryEvidenceTag::new("subscription_digest"),
                                subscription_digest,
                            ),
                        ForgeQueryEvidenceTag::new("admitted_subscription_identity"),
                        admitted_subscription_identity,
                    ),
                    ForgeQueryEvidenceTag::new("basis_identity"),
                    basis_identity,
                ),
                ForgeQueryEvidenceTag::new("strategy_identity"),
                strategy_identity,
            )
            .seal(),
        }
    }
}

fn bridge_identity_field(
    encoder: ForgeQueryEvidenceIdentityEncoder,
    tag: ForgeQueryEvidenceTag,
    identity: &BridgeIdentityEvidence,
) -> ForgeQueryEvidenceIdentityEncoder {
    encoder.field_identity(tag, identity.as_str())
}

pub(super) fn binding_digest(
    capability_digest: &str,
    evidence: &BridgeLowerRuntimeEvidenceReference,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::BridgeLowerRuntimeBasisBinding)
        .field_identity(
            ForgeQueryEvidenceTag::new("capability_digest"),
            capability_digest,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("evidence_identity"),
            &evidence.evidence_identity(),
        )
        .seal()
        .as_str()
        .to_string()
}
