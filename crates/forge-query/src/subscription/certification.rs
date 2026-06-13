use crate::identity::hash_parts;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::acknowledgement::SubscriptionAcknowledgementFrontier;
use super::activation::SubscriptionActivationInput;
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::active_lane::ActiveSubscriptionLaneAdmission;
use super::admission::QuerySubscriptionAdmissionArtifact;
use super::attachment::SubscriptionConsumerAttachment;
use super::closeout::SubscriptionLifecycleCloseout;
use super::closeout::SubscriptionLifecycleCloseoutKind;
use super::continuation::SubscriptionContinuationReport;
use super::delivery_window::QueryDeliveryBatch;
use super::delivery_work_packet::ActiveDeliveryWorkPacket;
use super::input::LiveQueryAdmissionArtifact;
use super::maintenance_delta::{
    QueryMaintenanceDeltaLoweringReport, QuerySubscriptionMaintenanceDelta,
};
use super::preview_isolation::{
    PreviewSubscriptionDiscardCloseout, PreviewSubscriptionIsolationArtifact,
    PreviewSubscriptionPromotionHandoff, PreviewSubscriptionResidueReport,
};
use super::scale::QuerySubscriptionScaleSlopeReport;
use super::selection::QuerySubscriptionFamilySelection;
use super::evidence_identities::{
    certification_activation_bundle_identity, lifecycle_certification_bundle_identity,
    subscription_certification_projection, subscription_certification_sequence_projection,
    typed_identity_drift,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionCertificationDenialKind {
    ActivationAdmissionMismatch,
    ScaleSlopeDrift,
    ScaleSlopeSourceMismatch,
}

impl QuerySubscriptionCertificationDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivationAdmissionMismatch => "activation_admission_mismatch",
            Self::ScaleSlopeDrift => "scale_slope_drift",
            Self::ScaleSlopeSourceMismatch => "scale_slope_source_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionCertificationError {
    denial_kind: QuerySubscriptionCertificationDenialKind,
    message: &'static str,
    failure_digest: String,
}

impl QuerySubscriptionCertificationError {
    pub(super) fn new(
        denial_kind: QuerySubscriptionCertificationDenialKind,
        message: &'static str,
        evidence_parts: &[String],
    ) -> Self {
        let mut parts = vec![
            "query_subscription_certification_error_v1".to_string(),
            denial_kind.as_str().to_string(),
            message.to_string(),
        ];
        parts.extend(evidence_parts.iter().cloned());
        let failure_digest = hash_parts(&parts);
        Self {
            denial_kind,
            message,
            failure_digest,
        }
    }

    pub fn denial_kind(&self) -> &QuerySubscriptionCertificationDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionCertificationBundle {
    certification_bundle_identity: ForgeQueryEvidenceIdentity,
    admission_identity: ForgeQueryEvidenceIdentity,
    activation_identity: ForgeQueryEvidenceIdentity,
    query_declaration_for_reporting: String,
    query_declaration_identity: ForgeQueryEvidenceIdentity,
    bridge_declaration_for_reporting: String,
    bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    basis_binding_identity: ForgeQueryEvidenceIdentity,
    signal_strategy_identity: ForgeQueryEvidenceIdentity,
    diagnostics_for_reporting: String,
    support_profile_for_reporting: String,
    admission_counter_for_reporting: String,
    activation_counter_for_reporting: String,
    scale_slope_for_reporting: String,
    scale_activation_for_reporting: String,
    scale_admission_for_reporting: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationContext {
    query_digest: String,
    subscription_family_digest: String,
    subscription_equivalence_digest: String,
    policy_digest: String,
    tenant_basis_digest: String,
    relationship_proof_digest: String,
    view_shape_digest: String,
    basis_digest: String,
}

impl SubscriptionLifecycleCertificationContext {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn admitted(
        query_digest: impl Into<String>,
        subscription_family_digest: impl Into<String>,
        subscription_equivalence_digest: impl Into<String>,
        policy_digest: impl Into<String>,
        tenant_basis_digest: impl Into<String>,
        relationship_proof_digest: impl Into<String>,
        view_shape_digest: impl Into<String>,
        basis_digest: impl Into<String>,
    ) -> Self {
        Self {
            query_digest: query_digest.into(),
            subscription_family_digest: subscription_family_digest.into(),
            subscription_equivalence_digest: subscription_equivalence_digest.into(),
            policy_digest: policy_digest.into(),
            tenant_basis_digest: tenant_basis_digest.into(),
            relationship_proof_digest: relationship_proof_digest.into(),
            view_shape_digest: view_shape_digest.into(),
            basis_digest: basis_digest.into(),
        }
    }

    pub fn from_live_selection(
        live: &LiveQueryAdmissionArtifact,
        selection: &QuerySubscriptionFamilySelection,
    ) -> Self {
        Self::admitted(
            live.query_digest(),
            subscription_certification_projection(
                "subscription_lifecycle_family_v1",
                [("family", selection.family().as_str().to_string())],
            ),
            selection.equivalence_basis().digest().as_str().to_string(),
            live.policy_digest().unwrap_or("none").to_string(),
            live.tenant_digest().unwrap_or("none").to_string(),
            live.relationship_proof_digest()
                .unwrap_or("none")
                .to_string(),
            subscription_certification_projection(
                "subscription_lifecycle_view_shape_v1",
                [(
                    "view",
                    live.view_family()
                        .map(|family| family.as_str().to_string())
                        .unwrap_or_else(|| "none".to_string()),
                )],
            ),
            subscription_certification_projection(
                "subscription_lifecycle_basis_v1",
                [("basis", live.basis_posture().as_str().to_string())],
            ),
        )
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn subscription_family_digest(&self) -> &str {
        &self.subscription_family_digest
    }

    pub fn subscription_equivalence_digest(&self) -> &str {
        &self.subscription_equivalence_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_basis_digest(&self) -> &str {
        &self.tenant_basis_digest
    }

    pub fn relationship_proof_digest(&self) -> &str {
        &self.relationship_proof_digest
    }

    pub fn view_shape_digest(&self) -> &str {
        &self.view_shape_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubscriptionLifecycleCertificationDenialKind {
    ActivationAdmissionMismatch,
    ActiveLaneSourceMismatch,
    AttachmentSourceMismatch,
    DeliveryWindowSourceMismatch,
    MaintenanceDeltaSourceMismatch,
    WorkPacketSourceMismatch,
    DeliveryBatchSourceMismatch,
    ContinuationSourceMismatch,
    PreviewEvidenceMissing,
    PreviewSourceMismatch,
    CloseoutSourceMismatch,
}

impl SubscriptionLifecycleCertificationDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ActivationAdmissionMismatch => "activation_admission_mismatch",
            Self::ActiveLaneSourceMismatch => "active_lane_source_mismatch",
            Self::AttachmentSourceMismatch => "attachment_source_mismatch",
            Self::DeliveryWindowSourceMismatch => "delivery_window_source_mismatch",
            Self::MaintenanceDeltaSourceMismatch => "maintenance_delta_source_mismatch",
            Self::WorkPacketSourceMismatch => "work_packet_source_mismatch",
            Self::DeliveryBatchSourceMismatch => "delivery_batch_source_mismatch",
            Self::ContinuationSourceMismatch => "continuation_source_mismatch",
            Self::PreviewEvidenceMissing => "preview_evidence_missing",
            Self::PreviewSourceMismatch => "preview_source_mismatch",
            Self::CloseoutSourceMismatch => "closeout_source_mismatch",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SubscriptionLifecyclePreviewCertification<'a> {
    None,
    Discard {
        isolation: &'a PreviewSubscriptionIsolationArtifact,
        residue_report: &'a PreviewSubscriptionResidueReport,
        discard_closeout: &'a PreviewSubscriptionDiscardCloseout,
    },
    Promotion {
        isolation: &'a PreviewSubscriptionIsolationArtifact,
        residue_report: &'a PreviewSubscriptionResidueReport,
        promotion_handoff: &'a PreviewSubscriptionPromotionHandoff,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreviewCertificationEvidence {
    preview_isolation_digest: String,
    preview_residue_digest: String,
    counter_evidence: Vec<String>,
    support_evidence: Vec<String>,
    performance_receipt_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationError {
    denial_kind: SubscriptionLifecycleCertificationDenialKind,
    message: &'static str,
    failure_digest: String,
}

impl SubscriptionLifecycleCertificationError {
    fn new(
        denial_kind: SubscriptionLifecycleCertificationDenialKind,
        message: &'static str,
        evidence_parts: &[String],
    ) -> Self {
        let mut parts = vec![
            "subscription_lifecycle_certification_error_v1".to_string(),
            denial_kind.as_str().to_string(),
            message.to_string(),
        ];
        parts.extend(evidence_parts.iter().cloned());
        Self {
            denial_kind,
            message,
            failure_digest: hash_parts(&parts),
        }
    }

    pub fn denial_kind(&self) -> &SubscriptionLifecycleCertificationDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationBundle {
    certification_bundle_identity: ForgeQueryEvidenceIdentity,
    query_digest: String,
    subscription_family_digest: String,
    query_declaration_for_reporting: String,
    subscription_declaration_identity: ForgeQueryEvidenceIdentity,
    subscription_equivalence_digest: String,
    admission_identity: ForgeQueryEvidenceIdentity,
    active_lane_digest: String,
    active_lane_handle_digest: String,
    active_lane_lookup_class_digest: String,
    subscription_budget_digest: String,
    subscription_performance_receipt_digest: String,
    consumer_attachment_digest: String,
    acknowledgement_frontier_digest: String,
    delivery_window_digest: String,
    maintenance_delta_digest: String,
    active_delivery_work_packet_digest: String,
    active_delivery_density_posture_digest: String,
    allocation_posture_digest: String,
    delivery_batch_digest: String,
    patch_group_digest: String,
    delivery_receipt_digest: String,
    continuation_digest: String,
    preview_isolation_digest: String,
    preview_residue_digest: String,
    policy_digest: String,
    tenant_basis_digest: String,
    relationship_proof_digest: String,
    view_shape_digest: String,
    basis_digest: String,
    bridge_declaration_for_reporting: String,
    bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    signal_strategy_identity: ForgeQueryEvidenceIdentity,
    counter_snapshot: String,
    counter_evidence: Vec<String>,
    subscription_lifecycle_scale_slope_digest: String,
    support_matrix_digest: String,
}

impl SubscriptionLifecycleCertificationBundle {
    pub fn certification_bundle_for_reporting(&self) -> &str {
        self.certification_bundle_identity.as_str()
    }

    pub fn certification_bundle_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.certification_bundle_identity
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn subscription_family_digest(&self) -> &str {
        &self.subscription_family_digest
    }

    pub fn query_declaration_for_reporting(&self) -> &str {
        &self.query_declaration_for_reporting
    }

    pub fn subscription_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn subscription_equivalence_digest(&self) -> &str {
        &self.subscription_equivalence_digest
    }

    pub fn admission_for_reporting(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn active_lane_digest(&self) -> &str {
        &self.active_lane_digest
    }

    pub fn active_lane_handle_digest(&self) -> &str {
        &self.active_lane_handle_digest
    }

    pub fn active_lane_lookup_class_digest(&self) -> &str {
        &self.active_lane_lookup_class_digest
    }

    pub fn subscription_budget_digest(&self) -> &str {
        &self.subscription_budget_digest
    }

    pub fn subscription_performance_receipt_digest(&self) -> &str {
        &self.subscription_performance_receipt_digest
    }

    pub fn consumer_attachment_digest(&self) -> &str {
        &self.consumer_attachment_digest
    }

    pub fn acknowledgement_frontier_digest(&self) -> &str {
        &self.acknowledgement_frontier_digest
    }

    pub fn delivery_window_digest(&self) -> &str {
        &self.delivery_window_digest
    }

    pub fn maintenance_delta_digest(&self) -> &str {
        &self.maintenance_delta_digest
    }

    pub fn active_delivery_work_packet_digest(&self) -> &str {
        &self.active_delivery_work_packet_digest
    }

    pub fn active_delivery_density_posture_digest(&self) -> &str {
        &self.active_delivery_density_posture_digest
    }

    pub fn allocation_posture_digest(&self) -> &str {
        &self.allocation_posture_digest
    }

    pub fn delivery_batch_digest(&self) -> &str {
        &self.delivery_batch_digest
    }

    pub fn patch_group_digest(&self) -> &str {
        &self.patch_group_digest
    }

    pub fn delivery_receipt_digest(&self) -> &str {
        &self.delivery_receipt_digest
    }

    pub fn continuation_digest(&self) -> &str {
        &self.continuation_digest
    }

    pub fn preview_isolation_digest(&self) -> &str {
        &self.preview_isolation_digest
    }

    pub fn preview_residue_digest(&self) -> &str {
        &self.preview_residue_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_basis_digest(&self) -> &str {
        &self.tenant_basis_digest
    }

    pub fn relationship_proof_digest(&self) -> &str {
        &self.relationship_proof_digest
    }

    pub fn view_shape_digest(&self) -> &str {
        &self.view_shape_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn bridge_declaration_for_reporting(&self) -> &str {
        &self.bridge_declaration_for_reporting
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn signal_strategy_for_reporting(&self) -> &str {
        self.signal_strategy_identity.as_str()
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn counter_evidence(&self) -> &[String] {
        &self.counter_evidence
    }

    pub fn subscription_lifecycle_scale_slope_digest(&self) -> &str {
        &self.subscription_lifecycle_scale_slope_digest
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }
}

#[allow(clippy::too_many_arguments)]
pub fn certify_subscription_lifecycle(
    context: SubscriptionLifecycleCertificationContext,
    admission: &QuerySubscriptionAdmissionArtifact,
    activation: &SubscriptionActivationInput,
    scale_report: &QuerySubscriptionScaleSlopeReport,
    active_admission: &ActiveSubscriptionLaneAdmission,
    active_lane_handle: &ActiveSubscriptionLaneHandle,
    attachment: &SubscriptionConsumerAttachment,
    delivery_window_digest: impl Into<String>,
    maintenance_delta: &QuerySubscriptionMaintenanceDelta,
    lowering_report: &QueryMaintenanceDeltaLoweringReport,
    work_packet: &ActiveDeliveryWorkPacket,
    delivery_batch: &QueryDeliveryBatch,
    acknowledged_attachment: &SubscriptionConsumerAttachment,
    continuation: Option<&SubscriptionContinuationReport>,
    preview: SubscriptionLifecyclePreviewCertification<'_>,
    lifecycle_closeout: &SubscriptionLifecycleCloseout,
) -> Result<SubscriptionLifecycleCertificationBundle, SubscriptionLifecycleCertificationError> {
    let delivery_window_digest = delivery_window_digest.into();
    let base = certify_query_subscription_activation(
        admission.clone(),
        activation.clone(),
        scale_report.clone(),
    )
    .map_err(|error| {
        SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::ActivationAdmissionMismatch,
            "subscription lifecycle certification requires aligned admission, activation, and scale evidence",
            &[error.failure_digest().to_string()],
        )
    })?;

    if active_admission.activation_digest() != activation.activation_for_reporting()
        || active_admission.admission_digest() != admission.admission_for_reporting()
        || active_admission.query_declaration_digest() != admission.query_declaration_for_reporting()
        || active_admission.bridge_declaration_digest() != admission.bridge_declaration_for_reporting()
        || active_admission.basis_binding_digest() != admission.basis_binding_for_reporting()
        || active_admission.signal_strategy_digest() != admission.signal_strategy_for_reporting()
    {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::ActiveLaneSourceMismatch,
            "active lane admission must certify the same admitted subscription source",
            &[
                format!("lane_activation:{}", active_admission.activation_digest()),
                format!("activation:{}", activation.activation_for_reporting()),
                format!("lane_admission:{}", active_admission.admission_digest()),
                format!("admission:{}", admission.admission_for_reporting()),
            ],
        ));
    }

    if attachment.lane_digest() != active_lane_handle.lane_digest() {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::AttachmentSourceMismatch,
            "consumer attachment must belong to the certified active lane handle",
            &[
                format!("attachment_lane:{}", attachment.lane_digest().as_str()),
                format!("handle_lane:{}", active_lane_handle.lane_digest().as_str()),
            ],
        ));
    }

    if maintenance_delta.active_lane_digest() != active_lane_handle.lane_digest()
        || lowering_report.maintenance_delta_digest()
            != maintenance_delta.maintenance_delta_digest()
    {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::MaintenanceDeltaSourceMismatch,
            "maintenance delta and lowering report must belong to the certified lane",
            &[
                format!(
                    "delta_lane:{}",
                    maintenance_delta.active_lane_digest().as_str()
                ),
                format!("handle_lane:{}", active_lane_handle.lane_digest().as_str()),
                format!(
                    "lowering_delta:{}",
                    lowering_report.maintenance_delta_digest()
                ),
                format!("delta:{}", maintenance_delta.maintenance_delta_digest()),
            ],
        ));
    }

    if work_packet.active_lane_digest() != active_lane_handle.lane_digest()
        || work_packet.attachment_digest() != attachment.attachment_digest()
        || work_packet.maintenance_delta().maintenance_delta_digest()
            != maintenance_delta.maintenance_delta_digest()
        || work_packet.lowering_report().lowering_report_digest()
            != lowering_report.lowering_report_digest()
    {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::WorkPacketSourceMismatch,
            "active delivery work packet must consume the certified lane, attachment, delta, and lowering report",
            &[
                format!("packet_lane:{}", work_packet.active_lane_digest().as_str()),
                format!("handle_lane:{}", active_lane_handle.lane_digest().as_str()),
                format!("packet_attachment:{}", work_packet.attachment_digest().as_str()),
                format!("attachment:{}", attachment.attachment_digest().as_str()),
                format!(
                    "packet_delta:{}",
                    work_packet.maintenance_delta().maintenance_delta_digest()
                ),
                format!("delta:{}", maintenance_delta.maintenance_delta_digest()),
                format!(
                    "packet_lowering:{}",
                    work_packet.lowering_report().lowering_report_digest()
                ),
                format!("lowering:{}", lowering_report.lowering_report_digest()),
            ],
        ));
    }

    if delivery_batch.delivery_window_digest() != delivery_window_digest
        || delivery_batch.attachment_digest() != attachment.attachment_digest()
        || delivery_batch.receipt().attachment_digest() != attachment.attachment_digest()
    {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::DeliveryBatchSourceMismatch,
            "delivery batch and receipt must belong to the certified window and consumer attachment",
            &[
                format!("batch_window:{}", delivery_batch.delivery_window_digest()),
                format!("window:{delivery_window_digest}"),
                format!("batch_attachment:{}", delivery_batch.attachment_digest().as_str()),
                format!("attachment:{}", attachment.attachment_digest().as_str()),
                format!(
                    "receipt_attachment:{}",
                    delivery_batch.receipt().attachment_digest().as_str()
                ),
            ],
        ));
    }

    if acknowledged_attachment.attachment_digest() != attachment.attachment_digest() {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::DeliveryBatchSourceMismatch,
            "acknowledged attachment must advance the frontier for the certified consumer attachment",
            &[
                format!("ack_attachment:{}", acknowledged_attachment.attachment_digest().as_str()),
                format!("attachment:{}", attachment.attachment_digest().as_str()),
            ],
        ));
    }

    if let Some(report) = continuation {
        if report.active_lane_digest() != active_lane_handle.lane_digest() {
            return Err(SubscriptionLifecycleCertificationError::new(
                SubscriptionLifecycleCertificationDenialKind::ContinuationSourceMismatch,
                "continuation report must belong to the certified active lane",
                &[
                    format!("continuation_lane:{}", report.active_lane_digest().as_str()),
                    format!("handle_lane:{}", active_lane_handle.lane_digest().as_str()),
                ],
            ));
        }
    }

    let preview_evidence = preview_certification_evidence(
        preview,
        active_lane_handle,
        attachment,
        lifecycle_closeout,
    )?;

    if lifecycle_closeout.active_lane_digest() != active_lane_handle.lane_digest()
        || lifecycle_closeout.attachment_digest() != attachment.attachment_digest()
    {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::CloseoutSourceMismatch,
            "lifecycle closeout must terminate the certified lane and attachment",
            &[
                format!(
                    "closeout_lane:{}",
                    lifecycle_closeout.active_lane_digest().as_str()
                ),
                format!("handle_lane:{}", active_lane_handle.lane_digest().as_str()),
                format!(
                    "closeout_attachment:{}",
                    lifecycle_closeout.attachment_digest().as_str()
                ),
                format!("attachment:{}", attachment.attachment_digest().as_str()),
            ],
        ));
    }

    let active_lane_handle_digest = subscription_certification_projection(
        "subscription_active_lane_handle_v1",
        [
            ("lane", active_lane_handle.lane_digest().as_str().to_string()),
            ("index", active_lane_handle.lane_index().to_string()),
            (
                "generation",
                active_lane_handle.registry_generation().to_string(),
            ),
        ],
    );
    let active_lane_lookup_class_digest = subscription_certification_projection(
        "subscription_active_lane_lookup_class_v1",
        [(
            "lookup_class",
            active_admission.lookup_class().as_str().to_string(),
        )],
    );
    let subscription_budget_digest = subscription_certification_projection(
        "active_subscription_budget_v1",
        [
            (
                "lookup_width",
                active_admission.budget().registry_lookup_width().to_string(),
            ),
            (
                "fanout_width",
                active_admission.budget().fanout_width().to_string(),
            ),
            (
                "allocation_scope_width",
                active_admission
                    .budget()
                    .allocation_scope_width()
                    .to_string(),
            ),
            (
                "lookup_class",
                active_admission.budget().lookup_class().as_str().to_string(),
            ),
            (
                "allocation_posture",
                active_admission
                    .budget()
                    .allocation_posture()
                    .as_str()
                    .to_string(),
            ),
            (
                "durable_checkpoint_requested",
                active_admission
                    .budget()
                    .durable_checkpoint_requested()
                    .to_string(),
            ),
            (
                "store_backed_restart_requested",
                active_admission
                    .budget()
                    .store_backed_restart_requested()
                    .to_string(),
            ),
        ],
    );
    let continuation_digest = continuation
        .map(|report| report.continuation_digest().to_string())
        .unwrap_or_else(|| "none".to_string());
    let subscription_performance_receipt_digest = subscription_certification_sequence_projection(
        "subscription_performance_receipt_v1",
        "subscription_performance_receipt_element_v1",
        &[
            active_admission
                .performance_receipt()
                .performance_receipt_for_reporting()
                .to_string(),
            attachment
                .performance_receipt()
                .performance_receipt_for_reporting()
                .to_string(),
            continuation
                .map(|report| {
                    report
                        .performance_receipt()
                        .performance_receipt_for_reporting()
                        .to_string()
                })
                .unwrap_or_else(|| "none".to_string()),
            work_packet
                .performance_receipt()
                .performance_receipt_for_reporting()
                .to_string(),
            lifecycle_closeout
                .performance_receipt()
                .performance_receipt_for_reporting()
                .to_string(),
            preview_evidence.performance_receipt_digest.clone(),
        ],
    );
    let allocation_posture_digest = subscription_certification_projection(
        "subscription_allocation_posture_v1",
        [
            (
                "posture",
                work_packet.allocation_posture().as_str().to_string(),
            ),
            (
                "allocation_scope_width",
                work_packet.allocation_scope_width().to_string(),
            ),
        ],
    );
    let counter_evidence = lifecycle_counter_evidence(
        admission.counters().digest(),
        active_admission.counters().digest(),
        acknowledged_attachment.acknowledgement_frontier(),
        delivery_batch.counters().digest(),
        lifecycle_closeout.counters().digest(),
        continuation.map(|report| report.report_digest()),
        &preview_evidence.counter_evidence,
    );
    let counter_snapshot =
        subscription_certification_sequence_projection("subscription_counter_snapshot_v1", "counter", &counter_evidence);
    let mut support_parts = vec![
        admission.support_profile().digest().to_string(),
        lifecycle_closeout.support_profile().digest().to_string(),
        lifecycle_closeout.closeout_digest().to_string(),
    ];
    support_parts.extend(preview_evidence.support_evidence.iter().cloned());
    let support_matrix_digest = subscription_certification_sequence_projection(
        "subscription_support_matrix_v1",
        "support",
        &support_parts,
    );
    let certification_bundle_identity = lifecycle_certification_bundle_identity(
        base.certification_bundle_identity(),
        admission.evidence_identity(),
        admission.query_declaration_identity(),
        admission.bridge_declaration_identity(),
        admission.signal_strategy_identity(),
        context.query_digest(),
        context.subscription_family_digest(),
        context.subscription_equivalence_digest(),
        active_admission.lane_digest().as_str(),
        &active_lane_handle_digest,
        &subscription_performance_receipt_digest,
        attachment.attachment_digest().as_str(),
        &delivery_window_digest,
        maintenance_delta.maintenance_delta_digest(),
        work_packet.work_packet_digest(),
        delivery_batch.delivery_batch_digest(),
        delivery_batch.receipt().receipt_digest(),
        &continuation_digest,
        lifecycle_closeout.closeout_digest(),
        &support_matrix_digest,
        &counter_snapshot,
    );

    Ok(SubscriptionLifecycleCertificationBundle {
        certification_bundle_identity,
        query_digest: context.query_digest().to_string(),
        subscription_family_digest: context.subscription_family_digest().to_string(),
        query_declaration_for_reporting: admission.query_declaration_for_reporting().to_string(),
        subscription_declaration_identity: admission.query_declaration_identity().clone(),
        subscription_equivalence_digest: context.subscription_equivalence_digest().to_string(),
        admission_identity: admission.evidence_identity().clone(),
        active_lane_digest: active_admission.lane_digest().as_str().to_string(),
        active_lane_handle_digest,
        active_lane_lookup_class_digest,
        subscription_budget_digest,
        subscription_performance_receipt_digest,
        consumer_attachment_digest: attachment.attachment_digest().as_str().to_string(),
        acknowledgement_frontier_digest: acknowledged_attachment
            .acknowledgement_frontier()
            .frontier_digest()
            .to_string(),
        delivery_window_digest,
        maintenance_delta_digest: maintenance_delta.maintenance_delta_digest().to_string(),
        active_delivery_work_packet_digest: work_packet.work_packet_digest().to_string(),
        active_delivery_density_posture_digest: subscription_certification_projection(
            "subscription_active_delivery_density_posture_v1",
            [(
                "posture",
                work_packet.density_posture().as_str().to_string(),
            )],
        ),
        allocation_posture_digest,
        delivery_batch_digest: delivery_batch.delivery_batch_digest().to_string(),
        patch_group_digest: delivery_batch
            .patch_group()
            .patch_group_digest()
            .to_string(),
        delivery_receipt_digest: delivery_batch.receipt().receipt_digest().to_string(),
        continuation_digest,
        preview_isolation_digest: preview_evidence.preview_isolation_digest,
        preview_residue_digest: preview_evidence.preview_residue_digest,
        policy_digest: context.policy_digest().to_string(),
        tenant_basis_digest: context.tenant_basis_digest().to_string(),
        relationship_proof_digest: context.relationship_proof_digest().to_string(),
        view_shape_digest: context.view_shape_digest().to_string(),
        basis_digest: context.basis_digest().to_string(),
        bridge_declaration_for_reporting: admission.bridge_declaration_for_reporting().to_string(),
        bridge_declaration_identity: admission.bridge_declaration_identity().clone(),
        signal_strategy_identity: admission.signal_strategy_identity().clone(),
        counter_snapshot,
        counter_evidence,
        subscription_lifecycle_scale_slope_digest: scale_report.digest().to_string(),
        support_matrix_digest,
    })
}

fn lifecycle_counter_evidence(
    admission_counter_digest: String,
    active_counter_digest: String,
    frontier: &SubscriptionAcknowledgementFrontier,
    batch_counter_digest: String,
    closeout_counter_digest: String,
    continuation_report_digest: Option<&str>,
    preview_counter_evidence: &[String],
) -> Vec<String> {
    let mut evidence = vec![
        format!("admission:{admission_counter_digest}"),
        format!("active:{active_counter_digest}"),
        format!("frontier:{}", frontier.frontier_digest()),
        format!("batch:{batch_counter_digest}"),
        format!("closeout:{closeout_counter_digest}"),
    ];
    if let Some(report_digest) = continuation_report_digest {
        evidence.push(format!("continuation:{report_digest}"));
    }
    evidence.extend(preview_counter_evidence.iter().cloned());
    evidence
}

fn preview_certification_evidence(
    preview: SubscriptionLifecyclePreviewCertification<'_>,
    active_lane_handle: &ActiveSubscriptionLaneHandle,
    attachment: &SubscriptionConsumerAttachment,
    lifecycle_closeout: &SubscriptionLifecycleCloseout,
) -> Result<PreviewCertificationEvidence, SubscriptionLifecycleCertificationError> {
    match preview {
        SubscriptionLifecyclePreviewCertification::None => {
            if matches!(
                lifecycle_closeout.closeout_kind(),
                SubscriptionLifecycleCloseoutKind::PreviewDiscarded
                    | SubscriptionLifecycleCloseoutKind::PreviewPromoted
            ) {
                return Err(SubscriptionLifecycleCertificationError::new(
                    SubscriptionLifecycleCertificationDenialKind::PreviewEvidenceMissing,
                    "preview lifecycle closeout requires explicit preview certification evidence",
                    &[format!(
                        "closeout_kind:{}",
                        lifecycle_closeout.closeout_kind().as_str()
                    )],
                ));
            }

            Ok(PreviewCertificationEvidence {
                preview_isolation_digest: "none".to_string(),
                preview_residue_digest: "none".to_string(),
                counter_evidence: Vec::new(),
                support_evidence: vec!["preview:none".to_string()],
                performance_receipt_digest: "none".to_string(),
            })
        }
        SubscriptionLifecyclePreviewCertification::Discard {
            isolation,
            residue_report,
            discard_closeout,
        } => {
            if lifecycle_closeout.closeout_kind()
                != &SubscriptionLifecycleCloseoutKind::PreviewDiscarded
                || isolation.active_lane_digest() != active_lane_handle.lane_digest()
                || isolation.attachment_digest() != attachment.attachment_digest()
                || lifecycle_closeout.future_selection() != isolation.future_selection()
                || lifecycle_closeout.basis_binding_digest() != isolation.basis_binding_digest()
                || lifecycle_closeout.checkpoint_identity_digest()
                    != isolation.checkpoint_identity_digest()
                || discard_closeout.active_lane_digest() != active_lane_handle.lane_digest()
                || discard_closeout.attachment_digest() != attachment.attachment_digest()
                || discard_closeout.future_selection() != isolation.future_selection()
                || discard_closeout.basis_binding_digest() != isolation.basis_binding_digest()
                || discard_closeout.checkpoint_identity_digest()
                    != isolation.checkpoint_identity_digest()
                || discard_closeout.preview_epoch_digest() != isolation.preview_epoch_digest()
                || discard_closeout.residue_report_digest() != residue_report.report_digest()
                || lifecycle_closeout.source_digest() != discard_closeout.closeout_digest()
            {
                return Err(SubscriptionLifecycleCertificationError::new(
                    SubscriptionLifecycleCertificationDenialKind::PreviewSourceMismatch,
                    "preview discard certification requires aligned isolation, residue, closeout, and lifecycle closeout evidence",
                    &[
                        format!("closeout_kind:{}", lifecycle_closeout.closeout_kind().as_str()),
                        format!("lane:{}", active_lane_handle.lane_digest().as_str()),
                        format!("attachment:{}", attachment.attachment_digest().as_str()),
                        format!("preview_lane:{}", isolation.active_lane_digest().as_str()),
                        format!(
                            "preview_attachment:{}",
                            isolation.attachment_digest().as_str()
                        ),
                        format!(
                            "discard_attachment:{}",
                            discard_closeout.attachment_digest().as_str()
                        ),
                    ],
                ));
            }

            Ok(PreviewCertificationEvidence {
                preview_isolation_digest: isolation.isolation_digest().to_string(),
                preview_residue_digest: residue_report.report_digest().to_string(),
                counter_evidence: vec![
                    format!("preview_isolation:{}", isolation.counters().digest()),
                    format!("preview_discard:{}", discard_closeout.counters().digest()),
                    format!(
                        "preview_authoritative_residue:{}",
                        residue_report.authoritative_residue_width()
                    ),
                    format!(
                        "preview_residue_width:{}",
                        residue_report.preview_residue_width()
                    ),
                ],
                support_evidence: vec![
                    format!("preview_isolation:{}", isolation.isolation_digest()),
                    format!("preview_residue:{}", residue_report.report_digest()),
                    format!("preview_discard:{}", discard_closeout.closeout_digest()),
                ],
                performance_receipt_digest: discard_closeout
                    .performance_receipt()
                    .performance_receipt_for_reporting()
                    .to_string(),
            })
        }
        SubscriptionLifecyclePreviewCertification::Promotion {
            isolation,
            residue_report,
            promotion_handoff,
        } => {
            if lifecycle_closeout.closeout_kind()
                != &SubscriptionLifecycleCloseoutKind::PreviewPromoted
                || isolation.active_lane_digest() != active_lane_handle.lane_digest()
                || isolation.attachment_digest() != attachment.attachment_digest()
                || lifecycle_closeout.future_selection() != isolation.future_selection()
                || lifecycle_closeout.basis_binding_digest() != isolation.basis_binding_digest()
                || lifecycle_closeout.checkpoint_identity_digest()
                    != isolation.checkpoint_identity_digest()
                || promotion_handoff.preview_lane_digest() != active_lane_handle.lane_digest()
                || promotion_handoff.attachment_digest() != attachment.attachment_digest()
                || promotion_handoff.future_selection() != isolation.future_selection()
                || promotion_handoff.preview_basis_binding_digest()
                    != isolation.basis_binding_digest()
                || promotion_handoff.preview_checkpoint_identity_digest()
                    != isolation.checkpoint_identity_digest()
                || promotion_handoff.preview_epoch_digest() != isolation.preview_epoch_digest()
                || promotion_handoff.residue_report_digest() != residue_report.report_digest()
                || lifecycle_closeout.source_digest() != promotion_handoff.handoff_digest()
            {
                return Err(SubscriptionLifecycleCertificationError::new(
                    SubscriptionLifecycleCertificationDenialKind::PreviewSourceMismatch,
                    "preview promotion certification requires aligned isolation, residue, handoff, and lifecycle closeout evidence",
                    &[
                        format!("closeout_kind:{}", lifecycle_closeout.closeout_kind().as_str()),
                        format!("lane:{}", active_lane_handle.lane_digest().as_str()),
                        format!("attachment:{}", attachment.attachment_digest().as_str()),
                        format!(
                            "promotion_lane:{}",
                            promotion_handoff.preview_lane_digest().as_str()
                        ),
                    ],
                ));
            }

            Ok(PreviewCertificationEvidence {
                preview_isolation_digest: isolation.isolation_digest().to_string(),
                preview_residue_digest: subscription_certification_projection(
                    "subscription_preview_residue_v1",
                    [
                        (
                            "residue",
                            residue_report.report_digest().to_string(),
                        ),
                        (
                            "handoff",
                            promotion_handoff.handoff_digest().to_string(),
                        ),
                        (
                            "authoritative_lane",
                            promotion_handoff
                                .authoritative_active_lane_digest()
                                .as_str()
                                .to_string(),
                        ),
                    ],
                ),
                counter_evidence: vec![
                    format!("preview_isolation:{}", isolation.counters().digest()),
                    format!(
                        "preview_promotion:{}",
                        promotion_handoff.counters().digest()
                    ),
                    format!(
                        "preview_authoritative_residue:{}",
                        residue_report.authoritative_residue_width()
                    ),
                    "promotion_authority_boundary_crossed:true".to_string(),
                ],
                support_evidence: vec![
                    format!("preview_isolation:{}", isolation.isolation_digest()),
                    format!("preview_residue:{}", residue_report.report_digest()),
                    format!("preview_promotion:{}", promotion_handoff.handoff_digest()),
                ],
                performance_receipt_digest: promotion_handoff
                    .performance_receipt()
                    .performance_receipt_for_reporting()
                    .to_string(),
            })
        }
    }
}

impl QuerySubscriptionCertificationBundle {
    pub fn certification_bundle_for_reporting(&self) -> &str {
        self.certification_bundle_identity.as_str()
    }

    pub fn certification_bundle_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.certification_bundle_identity
    }

    pub fn admission_for_reporting(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn activation_for_reporting(&self) -> &str {
        self.activation_identity.as_str()
    }

    pub fn activation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn query_declaration_for_reporting(&self) -> &str {
        &self.query_declaration_for_reporting
    }

    pub fn query_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_declaration_identity
    }

    pub fn bridge_declaration_for_reporting(&self) -> &str {
        &self.bridge_declaration_for_reporting
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn signal_strategy_for_reporting(&self) -> &str {
        self.signal_strategy_identity.as_str()
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn diagnostics_for_reporting(&self) -> &str {
        &self.diagnostics_for_reporting
    }

    pub fn support_profile_for_reporting(&self) -> &str {
        &self.support_profile_for_reporting
    }

    pub fn admission_counter_for_reporting(&self) -> &str {
        &self.admission_counter_for_reporting
    }

    pub fn activation_counter_for_reporting(&self) -> &str {
        &self.activation_counter_for_reporting
    }

    pub fn scale_slope_for_reporting(&self) -> &str {
        &self.scale_slope_for_reporting
    }

    pub fn scale_activation_for_reporting(&self) -> &str {
        &self.scale_activation_for_reporting
    }

    pub fn scale_admission_for_reporting(&self) -> &str {
        &self.scale_admission_for_reporting
    }
}

pub fn certify_query_subscription_activation(
    admission: QuerySubscriptionAdmissionArtifact,
    activation: SubscriptionActivationInput,
    scale_report: QuerySubscriptionScaleSlopeReport,
) -> Result<QuerySubscriptionCertificationBundle, QuerySubscriptionCertificationError> {
    if typed_identity_drift(activation.admission_identity(), admission.evidence_identity())
        || activation.query_declaration_for_reporting() != admission.query_declaration_for_reporting()
        || typed_identity_drift(
            activation.bridge_declaration_identity(),
            admission.bridge_declaration_identity(),
        )
        || typed_identity_drift(
            activation.basis_binding_identity(),
            admission.basis_binding_identity(),
        )
        || typed_identity_drift(
            activation.signal_strategy_identity(),
            admission.signal_strategy_identity(),
        )
    {
        return Err(QuerySubscriptionCertificationError::new(
            QuerySubscriptionCertificationDenialKind::ActivationAdmissionMismatch,
            "subscription activation input does not match the admitted subscription artifact",
            &[
                format!("admission:{}", admission.admission_for_reporting()),
                format!("activation_admission:{}", activation.admission_for_reporting()),
                format!(
                    "admission_query:{}",
                    admission.query_declaration_for_reporting()
                ),
                format!(
                    "activation_query:{}",
                    activation.query_declaration_for_reporting()
                ),
            ],
        ));
    }

    if scale_report.activation_digest() != activation.activation_for_reporting()
        || scale_report.admission_digest() != activation.admission_for_reporting()
    {
        return Err(QuerySubscriptionCertificationError::new(
            QuerySubscriptionCertificationDenialKind::ScaleSlopeSourceMismatch,
            "subscription scale slope report does not certify this activation source",
            &[
                format!("activation:{}", activation.activation_for_reporting()),
                format!("scale_activation:{}", scale_report.activation_digest()),
                format!("activation_admission:{}", activation.admission_for_reporting()),
                format!("scale_admission:{}", scale_report.admission_digest()),
            ],
        ));
    }

    let admission_counter_digest = admission.counters().digest();
    let activation_counter_digest = activation.counters().digest();
    let diagnostics_digest = admission.diagnostics().digest().to_string();
    let support_profile_digest = admission.support_profile().digest().to_string();
    let scale_slope_digest = scale_report.digest().to_string();
    let scale_activation_digest = scale_report.activation_digest().to_string();
    let scale_admission_digest = scale_report.admission_digest().to_string();
    let certification_bundle_identity = certification_activation_bundle_identity(
        admission.evidence_identity(),
        activation.evidence_identity(),
        admission.query_declaration_for_reporting(),
        admission.bridge_declaration_identity(),
        admission.basis_binding_identity(),
        admission.signal_strategy_identity(),
        admission.diagnostics().diagnostics_identity(),
        admission.support_profile().profile_identity(),
        &admission.counters().evidence_identity(),
        &activation.counters().evidence_identity(),
        &scale_report.evidence_identity(),
        scale_report.activation_digest(),
        scale_report.admission_digest(),
    );
    Ok(QuerySubscriptionCertificationBundle {
        certification_bundle_identity,
        admission_identity: admission.evidence_identity().clone(),
        activation_identity: activation.evidence_identity().clone(),
        query_declaration_for_reporting: admission.query_declaration_for_reporting().to_string(),
        query_declaration_identity: admission.query_declaration_identity().clone(),
        bridge_declaration_for_reporting: admission.bridge_declaration_for_reporting().to_string(),
        bridge_declaration_identity: admission.bridge_declaration_identity().clone(),
        basis_binding_identity: admission.basis_binding_identity().clone(),
        signal_strategy_identity: admission.signal_strategy_identity().clone(),
        diagnostics_for_reporting: diagnostics_digest,
        support_profile_for_reporting: support_profile_digest,
        admission_counter_for_reporting: admission_counter_digest,
        activation_counter_for_reporting: activation_counter_digest,
        scale_slope_for_reporting: scale_slope_digest,
        scale_activation_for_reporting: scale_activation_digest,
        scale_admission_for_reporting: scale_admission_digest,
    })
}
