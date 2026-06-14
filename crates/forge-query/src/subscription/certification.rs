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
    certification_activation_bundle_identity, lifecycle_absent_continuation_identity,
    lifecycle_absent_performance_receipt_identity, lifecycle_active_lane_handle_identity,
    lifecycle_active_delivery_density_posture_identity, lifecycle_active_lane_lookup_class_identity,
    lifecycle_allocation_posture_identity, lifecycle_certification_bundle_identity,
    lifecycle_context_basis_posture_identity, lifecycle_context_query_identity,
    lifecycle_context_view_shape_identity, lifecycle_counter_sequence_identity,
    lifecycle_labeled_counter_identity, lifecycle_performance_sequence_identity,
    lifecycle_preview_promotion_residue_identity, lifecycle_subscription_budget_identity, lifecycle_subscription_equivalence_identity,
    lifecycle_subscription_family_identity, lifecycle_support_matrix_identity, typed_identity_drift,
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
    query_scope_identity: ForgeQueryEvidenceIdentity,
    subscription_family_identity: ForgeQueryEvidenceIdentity,
    subscription_equivalence_identity: ForgeQueryEvidenceIdentity,
    policy_digest: String,
    tenant_basis_digest: String,
    relationship_proof_digest: String,
    view_shape_identity: ForgeQueryEvidenceIdentity,
    basis_posture_identity: ForgeQueryEvidenceIdentity,
}

impl SubscriptionLifecycleCertificationContext {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn admitted(
        query_digest: impl Into<String>,
        query_scope_identity: ForgeQueryEvidenceIdentity,
        subscription_family_identity: ForgeQueryEvidenceIdentity,
        subscription_equivalence_identity: ForgeQueryEvidenceIdentity,
        policy_digest: impl Into<String>,
        tenant_basis_digest: impl Into<String>,
        relationship_proof_digest: impl Into<String>,
        view_shape_identity: ForgeQueryEvidenceIdentity,
        basis_posture_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            query_digest: query_digest.into(),
            query_scope_identity,
            subscription_family_identity,
            subscription_equivalence_identity,
            policy_digest: policy_digest.into(),
            tenant_basis_digest: tenant_basis_digest.into(),
            relationship_proof_digest: relationship_proof_digest.into(),
            view_shape_identity,
            basis_posture_identity,
        }
    }

    pub fn from_live_selection(
        live: &LiveQueryAdmissionArtifact,
        selection: &QuerySubscriptionFamilySelection,
    ) -> Self {
        let subscription_family_identity =
            lifecycle_subscription_family_identity(selection.family());
        let subscription_equivalence_identity =
            lifecycle_subscription_equivalence_identity(selection.equivalence_basis());
        let query_scope_identity = lifecycle_context_query_identity(live);
        Self::admitted(
            live.query_digest(),
            query_scope_identity,
            subscription_family_identity,
            subscription_equivalence_identity,
            live.policy_digest().unwrap_or("none").to_string(),
            live.tenant_digest().unwrap_or("none").to_string(),
            live.relationship_proof_digest()
                .unwrap_or("none")
                .to_string(),
            lifecycle_context_view_shape_identity(
                live.view_family().map(|family| family.as_str()),
            ),
            lifecycle_context_basis_posture_identity(live.basis_posture().as_str()),
        )
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn query_scope_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_scope_identity
    }

    pub fn subscription_family_for_reporting(&self) -> &str {
        self.subscription_family_identity.as_str()
    }

    pub fn subscription_family_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_equivalence_for_reporting(&self) -> &str {
        self.subscription_equivalence_identity.as_str()
    }

    pub fn subscription_equivalence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_equivalence_identity
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

    pub fn view_shape_for_reporting(&self) -> &str {
        self.view_shape_identity.as_str()
    }

    pub fn view_shape_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.view_shape_identity
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_posture_identity.as_str()
    }

    pub fn basis_posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_posture_identity
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
    counter_identities: Vec<ForgeQueryEvidenceIdentity>,
    support_identities: Vec<ForgeQueryEvidenceIdentity>,
    performance_receipt_digest: String,
    performance_receipt_identity: ForgeQueryEvidenceIdentity,
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
    subscription_family_for_reporting: String,
    query_declaration_for_reporting: String,
    subscription_declaration_identity: ForgeQueryEvidenceIdentity,
    subscription_equivalence_for_reporting: String,
    admission_identity: ForgeQueryEvidenceIdentity,
    active_lane_for_reporting: String,
    active_lane_handle_identity: ForgeQueryEvidenceIdentity,
    active_lane_lookup_class_identity: ForgeQueryEvidenceIdentity,
    subscription_budget_identity: ForgeQueryEvidenceIdentity,
    subscription_performance_receipt_identity: ForgeQueryEvidenceIdentity,
    consumer_attachment_for_reporting: String,
    acknowledgement_frontier_for_reporting: String,
    delivery_window_for_reporting: String,
    maintenance_delta_for_reporting: String,
    active_delivery_work_packet_for_reporting: String,
    active_delivery_density_posture_identity: ForgeQueryEvidenceIdentity,
    allocation_posture_identity: ForgeQueryEvidenceIdentity,
    delivery_batch_for_reporting: String,
    patch_group_for_reporting: String,
    delivery_receipt_for_reporting: String,
    continuation_identity: ForgeQueryEvidenceIdentity,
    preview_isolation_for_reporting: String,
    preview_residue_for_reporting: String,
    policy_digest: String,
    tenant_basis_digest: String,
    relationship_proof_digest: String,
    view_shape_for_reporting: String,
    basis_for_reporting: String,
    bridge_declaration_for_reporting: String,
    bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    signal_strategy_identity: ForgeQueryEvidenceIdentity,
    counter_sequence_identity: ForgeQueryEvidenceIdentity,
    subscription_lifecycle_scale_slope_for_reporting: String,
    support_matrix_identity: ForgeQueryEvidenceIdentity,
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

    pub fn subscription_family_for_reporting(&self) -> &str {
        &self.subscription_family_for_reporting
    }

    pub fn query_declaration_for_reporting(&self) -> &str {
        &self.query_declaration_for_reporting
    }

    pub fn subscription_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn subscription_equivalence_for_reporting(&self) -> &str {
        &self.subscription_equivalence_for_reporting
    }

    pub fn admission_for_reporting(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn active_lane_for_reporting(&self) -> &str {
        &self.active_lane_for_reporting
    }

    pub fn active_lane_handle_for_reporting(&self) -> &str {
        self.active_lane_handle_identity.as_str()
    }

    pub fn active_lane_handle_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_lane_handle_identity
    }

    pub fn active_lane_lookup_class_for_reporting(&self) -> &str {
        self.active_lane_lookup_class_identity.as_str()
    }

    pub fn active_lane_lookup_class_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_lane_lookup_class_identity
    }

    pub fn subscription_budget_for_reporting(&self) -> &str {
        self.subscription_budget_identity.as_str()
    }

    pub fn subscription_budget_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_budget_identity
    }

    pub fn subscription_performance_receipt_for_reporting(&self) -> &str {
        self.subscription_performance_receipt_identity.as_str()
    }

    pub fn subscription_performance_receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_performance_receipt_identity
    }

    pub fn consumer_attachment_for_reporting(&self) -> &str {
        &self.consumer_attachment_for_reporting
    }

    pub fn acknowledgement_frontier_for_reporting(&self) -> &str {
        &self.acknowledgement_frontier_for_reporting
    }

    pub fn delivery_window_for_reporting(&self) -> &str {
        &self.delivery_window_for_reporting
    }

    pub fn maintenance_delta_for_reporting(&self) -> &str {
        &self.maintenance_delta_for_reporting
    }

    pub fn active_delivery_work_packet_for_reporting(&self) -> &str {
        &self.active_delivery_work_packet_for_reporting
    }

    pub fn active_delivery_density_posture_for_reporting(&self) -> &str {
        self.active_delivery_density_posture_identity.as_str()
    }

    pub fn active_delivery_density_posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_delivery_density_posture_identity
    }

    pub fn allocation_posture_for_reporting(&self) -> &str {
        self.allocation_posture_identity.as_str()
    }

    pub fn allocation_posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.allocation_posture_identity
    }

    pub fn delivery_batch_for_reporting(&self) -> &str {
        &self.delivery_batch_for_reporting
    }

    pub fn patch_group_for_reporting(&self) -> &str {
        &self.patch_group_for_reporting
    }

    pub fn delivery_receipt_for_reporting(&self) -> &str {
        &self.delivery_receipt_for_reporting
    }

    pub fn continuation_for_reporting(&self) -> &str {
        self.continuation_identity.as_str()
    }

    pub fn continuation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.continuation_identity
    }

    pub fn preview_isolation_for_reporting(&self) -> &str {
        &self.preview_isolation_for_reporting
    }

    pub fn preview_residue_for_reporting(&self) -> &str {
        &self.preview_residue_for_reporting
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

    pub fn view_shape_for_reporting(&self) -> &str {
        &self.view_shape_for_reporting
    }

    pub fn basis_for_reporting(&self) -> &str {
        &self.basis_for_reporting
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

    pub fn counter_snapshot_for_reporting(&self) -> &str {
        self.counter_sequence_identity.as_str()
    }

    pub fn counter_sequence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.counter_sequence_identity
    }

    pub fn subscription_lifecycle_scale_slope_for_reporting(&self) -> &str {
        &self.subscription_lifecycle_scale_slope_for_reporting
    }

    pub fn support_matrix_for_reporting(&self) -> &str {
        self.support_matrix_identity.as_str()
    }

    pub fn support_matrix_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.support_matrix_identity
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
        || active_admission.basis_binding_for_reporting() != admission.basis_binding_for_reporting()
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

    let active_lane_handle_identity = lifecycle_active_lane_handle_identity(
        active_admission.lane_digest().evidence_identity(),
        active_lane_handle,
    );
    let active_lane_lookup_class_identity = lifecycle_active_lane_lookup_class_identity(
        active_admission.lookup_class().as_str(),
    );
    let subscription_budget_identity = lifecycle_subscription_budget_identity(
        active_admission.budget().registry_lookup_width(),
        active_admission.budget().fanout_width(),
        active_admission
            .budget()
            .allocation_scope_width(),
        active_admission.budget().lookup_class().as_str(),
        active_admission
            .budget()
            .allocation_posture()
            .as_str(),
        active_admission.budget().durable_checkpoint_requested(),
        active_admission
            .budget()
            .store_backed_restart_requested(),
    );
    let absent_performance = lifecycle_absent_performance_receipt_identity();
    let preview_performance_identity = if preview_evidence.performance_receipt_digest == "none" {
        absent_performance.clone()
    } else {
        preview_evidence.performance_receipt_identity.clone()
    };
    let performance_receipt_identities = [
        active_admission
            .performance_receipt()
            .performance_receipt_identity(),
        attachment
            .performance_receipt()
            .performance_receipt_identity(),
        continuation
            .map(|report| report.performance_receipt().performance_receipt_identity())
            .unwrap_or(&absent_performance),
        work_packet
            .performance_receipt()
            .performance_receipt_identity(),
        lifecycle_closeout
            .performance_receipt()
            .performance_receipt_identity(),
        &preview_performance_identity,
    ];
    let subscription_performance_receipt_identity =
        lifecycle_performance_sequence_identity(performance_receipt_identities);
    let continuation_identity = continuation
        .map(|report| report.evidence_identity().clone())
        .unwrap_or_else(lifecycle_absent_continuation_identity);
    let allocation_posture_identity = lifecycle_allocation_posture_identity(
        work_packet.allocation_posture().as_str(),
        work_packet.allocation_scope_width(),
    );
    let active_delivery_density_posture_identity =
        lifecycle_active_delivery_density_posture_identity(work_packet.density_posture().as_str());
    let counter_identities = lifecycle_counter_identities(
        &admission.counters().evidence_identity(),
        &active_admission.counters().evidence_identity(),
        acknowledged_attachment.acknowledgement_frontier(),
        &delivery_batch.counters().evidence_identity(),
        &lifecycle_closeout.counters().evidence_identity(),
        continuation.map(|report| report.evidence_identity()),
        &preview_evidence.counter_identities,
    );
    let counter_sequence_identity = {
        let refs: Vec<&ForgeQueryEvidenceIdentity> = counter_identities.iter().collect();
        lifecycle_counter_sequence_identity(refs)
    };
    let mut support_identities: Vec<&ForgeQueryEvidenceIdentity> = vec![
        admission.support_profile().profile_identity(),
        lifecycle_closeout.support_profile().profile_identity(),
        lifecycle_closeout.evidence_identity(),
    ];
    support_identities.extend(preview_evidence.support_identities.iter());
    let support_matrix_identity = lifecycle_support_matrix_identity(support_identities);
    let delivery_window_identity = delivery_batch.delivery_window_identity();
    let certification_bundle_identity = lifecycle_certification_bundle_identity(
        base.certification_bundle_identity(),
        admission.evidence_identity(),
        admission.query_declaration_identity(),
        admission.bridge_declaration_identity(),
        admission.signal_strategy_identity(),
        context.query_scope_identity(),
        context.subscription_family_identity(),
        context.subscription_equivalence_identity(),
        active_admission.lane_digest().evidence_identity(),
        &active_lane_handle_identity,
        &subscription_performance_receipt_identity,
        attachment.attachment_digest().evidence_identity(),
        delivery_window_identity,
        maintenance_delta.evidence_identity(),
        work_packet.evidence_identity(),
        delivery_batch.evidence_identity(),
        delivery_batch.receipt().evidence_identity(),
        &continuation_identity,
        lifecycle_closeout.evidence_identity(),
        &support_matrix_identity,
        &counter_sequence_identity,
    );

    Ok(SubscriptionLifecycleCertificationBundle {
        certification_bundle_identity,
        query_digest: context.query_digest().to_string(),
        subscription_family_for_reporting: context.subscription_family_for_reporting().to_string(),
        query_declaration_for_reporting: admission.query_declaration_for_reporting().to_string(),
        subscription_declaration_identity: admission.query_declaration_identity().clone(),
        subscription_equivalence_for_reporting: context.subscription_equivalence_for_reporting().to_string(),
        admission_identity: admission.evidence_identity().clone(),
        active_lane_for_reporting: active_admission.lane_digest().as_str().to_string(),
        active_lane_handle_identity,
        active_lane_lookup_class_identity,
        subscription_budget_identity,
        subscription_performance_receipt_identity,
        consumer_attachment_for_reporting: attachment.attachment_digest().as_str().to_string(),
        acknowledgement_frontier_for_reporting: acknowledged_attachment
            .acknowledgement_frontier()
            .frontier_for_reporting()
            .to_string(),
        delivery_window_for_reporting: delivery_window_digest,
        maintenance_delta_for_reporting: maintenance_delta.maintenance_delta_digest().to_string(),
        active_delivery_work_packet_for_reporting: work_packet.work_packet_digest().to_string(),
        active_delivery_density_posture_identity,
        allocation_posture_identity,
        delivery_batch_for_reporting: delivery_batch.delivery_batch_digest().to_string(),
        patch_group_for_reporting: delivery_batch
            .patch_group()
            .patch_group_for_reporting()
            .to_string(),
        delivery_receipt_for_reporting: delivery_batch.receipt().receipt_digest().to_string(),
        continuation_identity,
        preview_isolation_for_reporting: preview_evidence.preview_isolation_digest,
        preview_residue_for_reporting: preview_evidence.preview_residue_digest,
        policy_digest: context.policy_digest().to_string(),
        tenant_basis_digest: context.tenant_basis_digest().to_string(),
        relationship_proof_digest: context.relationship_proof_digest().to_string(),
        view_shape_for_reporting: context.view_shape_for_reporting().to_string(),
        basis_for_reporting: context.basis_for_reporting().to_string(),
        bridge_declaration_for_reporting: admission.bridge_declaration_for_reporting().to_string(),
        bridge_declaration_identity: admission.bridge_declaration_identity().clone(),
        signal_strategy_identity: admission.signal_strategy_identity().clone(),
        counter_sequence_identity,
        subscription_lifecycle_scale_slope_for_reporting: scale_report.digest().to_string(),
        support_matrix_identity,
    })
}

fn lifecycle_counter_identities(
    admission_counters: &ForgeQueryEvidenceIdentity,
    active_counters: &ForgeQueryEvidenceIdentity,
    frontier: &SubscriptionAcknowledgementFrontier,
    batch_counters: &ForgeQueryEvidenceIdentity,
    closeout_counters: &ForgeQueryEvidenceIdentity,
    continuation_report_identity: Option<&ForgeQueryEvidenceIdentity>,
    preview_counter_identities: &[ForgeQueryEvidenceIdentity],
) -> Vec<ForgeQueryEvidenceIdentity> {
    let mut identities = vec![
        lifecycle_labeled_counter_identity("admission", admission_counters),
        lifecycle_labeled_counter_identity("active", active_counters),
        lifecycle_labeled_counter_identity(
            "frontier",
            &ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
            )
            .field_shape(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                "subscription_acknowledgement_frontier_v1",
            )
            .field_evidence_identity(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("attachment"),
                frontier.attachment_digest().evidence_identity(),
            )
            .field_usize(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("sequence"),
                frontier.acknowledged_sequence().get() as usize,
            )
            .seal(),
        ),
        lifecycle_labeled_counter_identity("batch", batch_counters),
        lifecycle_labeled_counter_identity("closeout", closeout_counters),
    ];
    if let Some(report_identity) = continuation_report_identity {
        identities.push(lifecycle_labeled_counter_identity(
            "continuation",
            report_identity,
        ));
    }
    identities.extend(preview_counter_identities.iter().cloned());
    identities
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
                counter_identities: Vec::new(),
                support_identities: vec![ForgeQueryEvidenceIdentity::compose(
                    crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                )
                .field_shape(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                    "subscription_preview_support_absent_v1",
                )
                .seal()],
                performance_receipt_digest: "none".to_string(),
                performance_receipt_identity: lifecycle_absent_performance_receipt_identity(),
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
                || lifecycle_closeout.basis_binding_for_reporting()
                    != isolation.basis_binding_for_reporting()
                || lifecycle_closeout.checkpoint_for_reporting()
                    != isolation.checkpoint_for_reporting()
                || discard_closeout.active_lane_digest() != active_lane_handle.lane_digest()
                || discard_closeout.attachment_digest() != attachment.attachment_digest()
                || discard_closeout.future_selection() != isolation.future_selection()
                || discard_closeout.basis_binding_for_reporting()
                    != isolation.basis_binding_for_reporting()
                || discard_closeout.checkpoint_for_reporting()
                    != isolation.checkpoint_for_reporting()
                || discard_closeout.preview_epoch_for_reporting()
                    != isolation.preview_epoch_for_reporting()
                || discard_closeout.residue_report_digest() != residue_report.report_digest()
                || lifecycle_closeout.source_digest() != discard_closeout.closeout_for_reporting()
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
                preview_isolation_digest: isolation.isolation_for_reporting().to_string(),
                preview_residue_digest: residue_report.report_digest().to_string(),
                counter_identities: vec![
                    lifecycle_labeled_counter_identity(
                        "preview_isolation",
                        &isolation.counters().evidence_identity(),
                    ),
                    lifecycle_labeled_counter_identity(
                        "preview_discard",
                        &discard_closeout.counters().evidence_identity(),
                    ),
                ],
                support_identities: vec![
                    ForgeQueryEvidenceIdentity::compose(
                        crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                        "subscription_preview_isolation_support_v1",
                    )
                    .field_evidence_identity(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("isolation"),
                        isolation.isolation_identity(),
                    )
                    .seal(),
                    ForgeQueryEvidenceIdentity::compose(
                        crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                        "subscription_preview_residue_support_v1",
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("residue"),
                        residue_report.report_digest(),
                    )
                    .seal(),
                    ForgeQueryEvidenceIdentity::compose(
                        crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                        "subscription_preview_discard_support_v1",
                    )
                    .field_evidence_identity(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("discard"),
                        discard_closeout.closeout_identity(),
                    )
                    .seal(),
                ],
                performance_receipt_digest: discard_closeout
                    .performance_receipt()
                    .performance_receipt_for_reporting()
                    .to_string(),
                performance_receipt_identity: discard_closeout
                    .performance_receipt()
                    .performance_receipt_identity()
                    .clone(),
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
                || lifecycle_closeout.basis_binding_for_reporting()
                    != isolation.basis_binding_for_reporting()
                || lifecycle_closeout.checkpoint_for_reporting()
                    != isolation.checkpoint_for_reporting()
                || promotion_handoff.preview_lane_digest() != active_lane_handle.lane_digest()
                || promotion_handoff.attachment_digest() != attachment.attachment_digest()
                || promotion_handoff.future_selection() != isolation.future_selection()
                || promotion_handoff.preview_basis_binding_for_reporting()
                    != isolation.basis_binding_for_reporting()
                || promotion_handoff.preview_checkpoint_for_reporting()
                    != isolation.checkpoint_for_reporting()
                || promotion_handoff.preview_epoch_for_reporting()
                    != isolation.preview_epoch_for_reporting()
                || promotion_handoff.residue_report_digest() != residue_report.report_digest()
                || lifecycle_closeout.source_digest() != promotion_handoff.handoff_for_reporting()
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
                preview_isolation_digest: isolation.isolation_for_reporting().to_string(),
                preview_residue_digest: lifecycle_preview_promotion_residue_identity(
                    residue_report.report_digest(),
                    promotion_handoff.handoff_for_reporting(),
                    promotion_handoff
                        .authoritative_active_lane_digest()
                        .as_str(),
                )
                .as_str()
                .to_string(),
                counter_identities: vec![
                    lifecycle_labeled_counter_identity(
                        "preview_isolation",
                        &isolation.counters().evidence_identity(),
                    ),
                    lifecycle_labeled_counter_identity(
                        "preview_promotion",
                        &promotion_handoff.counters().evidence_identity(),
                    ),
                ],
                support_identities: vec![
                    ForgeQueryEvidenceIdentity::compose(
                        crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                        "subscription_preview_isolation_support_v1",
                    )
                    .field_evidence_identity(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("isolation"),
                        isolation.isolation_identity(),
                    )
                    .seal(),
                    ForgeQueryEvidenceIdentity::compose(
                        crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                        "subscription_preview_residue_support_v1",
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("residue"),
                        residue_report.report_digest(),
                    )
                    .seal(),
                    ForgeQueryEvidenceIdentity::compose(
                        crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                    )
                    .field_shape(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                        "subscription_preview_promotion_support_v1",
                    )
                    .field_evidence_identity(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("promotion"),
                        promotion_handoff.handoff_identity(),
                    )
                    .seal(),
                ],
                performance_receipt_digest: promotion_handoff
                    .performance_receipt()
                    .performance_receipt_for_reporting()
                    .to_string(),
                performance_receipt_identity: promotion_handoff
                    .performance_receipt()
                    .performance_receipt_identity()
                    .clone(),
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
        admission.query_declaration_identity(),
        admission.bridge_declaration_identity(),
        admission.basis_binding_identity(),
        admission.signal_strategy_identity(),
        admission.diagnostics().diagnostics_identity(),
        admission.support_profile().profile_identity(),
        &admission.counters().evidence_identity(),
        &activation.counters().evidence_identity(),
        &scale_report.evidence_identity(),
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
