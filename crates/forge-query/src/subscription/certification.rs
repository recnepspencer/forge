use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::identity_authority::{
    admit_query_subscription_authority_identity, QuerySubscriptionAuthorityIdentity,
    QuerySubscriptionIdentityKind,
};

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
use super::evidence_identities::{
    certification_activation_bundle_identity, lifecycle_absent_continuation_identity,
    lifecycle_absent_performance_receipt_identity, lifecycle_absent_preview_isolation_identity,
    lifecycle_absent_preview_residue_identity, lifecycle_active_delivery_density_posture_identity,
    lifecycle_active_lane_handle_identity, lifecycle_active_lane_lookup_class_identity,
    lifecycle_allocation_posture_identity, lifecycle_certification_bundle_identity,
    lifecycle_context_basis_posture_identity, lifecycle_context_policy_identity,
    lifecycle_context_query_identity, lifecycle_context_relationship_proof_identity,
    lifecycle_context_tenant_basis_identity, lifecycle_context_view_shape_identity,
    lifecycle_counter_sequence_identity, lifecycle_labeled_counter_identity,
    lifecycle_performance_sequence_identity, lifecycle_preview_promotion_residue_identity,
    lifecycle_subscription_budget_identity, lifecycle_subscription_equivalence_identity,
    lifecycle_subscription_family_identity, lifecycle_support_matrix_identity,
    typed_identity_drift,
};
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
use super::validation_evidence::{
    validation_role_evidence_identity, validation_shape_role_evidence_identity,
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
    failure_identity: ForgeQueryEvidenceIdentity,
}

impl QuerySubscriptionCertificationError {
    pub(super) fn new(
        denial_kind: QuerySubscriptionCertificationDenialKind,
        message: &'static str,
        evidence: &[ForgeQueryEvidenceIdentity],
    ) -> Self {
        Self {
            denial_kind,
            message,
            failure_identity: subscription_certification_failure_identity(
                "query_subscription_certification_error_v1",
                denial_kind.as_str(),
                message,
                evidence,
            ),
        }
    }

    pub fn denial_kind(&self) -> &QuerySubscriptionCertificationDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.failure_identity
    }
}

fn subscription_certification_failure_identity(
    identity_family: &'static str,
    kind: &'static str,
    message: &'static str,
    evidence: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            identity_family,
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind)
        .field_value(ForgeQueryEvidenceTag::new("message"), message)
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("evidence"), evidence.iter())
        .seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionCertificationBundle {
    pub(in crate::subscription) certification_bundle_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) admission_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) activation_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) query_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) basis_binding_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) signal_strategy_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) diagnostics_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) support_profile_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) admission_counter_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) activation_counter_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) scale_slope_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) scale_activation_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) scale_admission_identity: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationContext {
    pub(in crate::subscription) query_scope_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_family_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_equivalence_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) policy_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) tenant_basis_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) relationship_proof_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) view_shape_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) basis_posture_identity: ForgeQueryEvidenceIdentity,
}

impl SubscriptionLifecycleCertificationContext {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn admitted(
        query_scope_identity: ForgeQueryEvidenceIdentity,
        subscription_family_identity: ForgeQueryEvidenceIdentity,
        subscription_equivalence_identity: ForgeQueryEvidenceIdentity,
        policy_identity: ForgeQueryEvidenceIdentity,
        tenant_basis_identity: ForgeQueryEvidenceIdentity,
        relationship_proof_identity: ForgeQueryEvidenceIdentity,
        view_shape_identity: ForgeQueryEvidenceIdentity,
        basis_posture_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            query_scope_identity,
            subscription_family_identity,
            subscription_equivalence_identity,
            policy_identity,
            tenant_basis_identity,
            relationship_proof_identity,
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
            query_scope_identity,
            subscription_family_identity,
            subscription_equivalence_identity,
            live.policy_context_identity().clone(),
            live.tenant_context_identity().clone(),
            live.relationship_proof_context_identity().clone(),
            lifecycle_context_view_shape_identity(live.view_family().map(|family| family.as_str())),
            lifecycle_context_basis_posture_identity(live.basis_posture().as_str()),
        )
    }

    pub fn query_scope_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_scope_identity
    }

    pub fn subscription_family_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_equivalence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_equivalence_identity
    }

    pub fn policy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.policy_identity
    }

    pub fn tenant_basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.tenant_basis_identity
    }

    pub fn relationship_proof_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.relationship_proof_identity
    }

    pub fn view_shape_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.view_shape_identity
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
    preview_isolation_identity: ForgeQueryEvidenceIdentity,
    preview_residue_identity: ForgeQueryEvidenceIdentity,
    counter_identities: Vec<ForgeQueryEvidenceIdentity>,
    support_identities: Vec<ForgeQueryEvidenceIdentity>,
    performance_receipt_identity: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationError {
    denial_kind: SubscriptionLifecycleCertificationDenialKind,
    message: &'static str,
    failure_identity: ForgeQueryEvidenceIdentity,
}

impl SubscriptionLifecycleCertificationError {
    fn new(
        denial_kind: SubscriptionLifecycleCertificationDenialKind,
        message: &'static str,
        evidence: &[ForgeQueryEvidenceIdentity],
    ) -> Self {
        Self {
            denial_kind,
            message,
            failure_identity: subscription_certification_failure_identity(
                "subscription_lifecycle_certification_error_v1",
                denial_kind.as_str(),
                message,
                evidence,
            ),
        }
    }

    pub fn denial_kind(&self) -> &SubscriptionLifecycleCertificationDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.failure_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionLifecycleCertificationBundle {
    certification_bundle_authority: QuerySubscriptionAuthorityIdentity<
        ForgeQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
    pub(in crate::subscription) query_scope_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_family_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_equivalence_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) admission_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) active_lane_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) active_lane_handle_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) active_lane_lookup_class_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_budget_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_performance_receipt_identity:
        ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) consumer_attachment_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) acknowledgement_frontier_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) delivery_window_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) maintenance_delta_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) active_delivery_work_packet_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) active_delivery_density_posture_identity:
        ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) allocation_posture_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) delivery_batch_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) patch_group_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) delivery_receipt_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) continuation_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) preview_isolation_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) preview_residue_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) policy_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) tenant_basis_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) relationship_proof_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) view_shape_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) basis_posture_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) signal_strategy_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) counter_sequence_identity: ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) subscription_lifecycle_scale_slope_identity:
        ForgeQueryEvidenceIdentity,
    pub(in crate::subscription) support_matrix_identity: ForgeQueryEvidenceIdentity,
}

impl SubscriptionLifecycleCertificationBundle {
    pub fn certification_bundle_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.certification_bundle_authority.value()
    }

    pub fn certification_bundle_authority(
        &self,
    ) -> &QuerySubscriptionAuthorityIdentity<
        ForgeQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    > {
        &self.certification_bundle_authority
    }

    pub fn query_scope_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_scope_identity
    }

    pub fn subscription_family_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_family_identity
    }

    pub fn subscription_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_declaration_identity
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn active_lane_handle_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_lane_handle_identity
    }

    pub fn active_lane_lookup_class_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_lane_lookup_class_identity
    }

    pub fn subscription_budget_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_budget_identity
    }

    pub fn subscription_performance_receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.subscription_performance_receipt_identity
    }

    pub fn active_delivery_density_posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.active_delivery_density_posture_identity
    }

    pub fn allocation_posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.allocation_posture_identity
    }

    pub fn continuation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.continuation_identity
    }

    pub fn policy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.policy_identity
    }

    pub fn tenant_basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.tenant_basis_identity
    }

    pub fn relationship_proof_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.relationship_proof_identity
    }

    pub fn view_shape_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.view_shape_identity
    }

    pub fn basis_posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_posture_identity
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn counter_sequence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.counter_sequence_identity
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
    delivery_window_identity: &ForgeQueryEvidenceIdentity,
    maintenance_delta: &QuerySubscriptionMaintenanceDelta,
    lowering_report: &QueryMaintenanceDeltaLoweringReport,
    work_packet: &ActiveDeliveryWorkPacket,
    delivery_batch: &QueryDeliveryBatch,
    acknowledged_attachment: &SubscriptionConsumerAttachment,
    continuation: Option<&SubscriptionContinuationReport>,
    preview: SubscriptionLifecyclePreviewCertification<'_>,
    lifecycle_closeout: &SubscriptionLifecycleCloseout,
) -> Result<SubscriptionLifecycleCertificationBundle, SubscriptionLifecycleCertificationError> {
    let base = certify_query_subscription_activation(
        admission.clone(),
        activation.clone(),
        scale_report.clone(),
    )
    .map_err(|error| {
        SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::ActivationAdmissionMismatch,
            "subscription lifecycle certification requires aligned admission, activation, and scale evidence",
            &[validation_role_evidence_identity(
                "activation_certification",
                error.failure_identity(),
            )],
        )
    })?;

    if typed_identity_drift(
        active_admission.activation_identity(),
        activation.evidence_identity(),
    ) || typed_identity_drift(
        active_admission.admission_identity(),
        admission.evidence_identity(),
    ) || typed_identity_drift(
        active_admission.query_declaration_identity(),
        admission.query_declaration_identity(),
    ) || typed_identity_drift(
        active_admission.bridge_declaration_identity(),
        admission.bridge_declaration_identity(),
    ) || typed_identity_drift(
        active_admission.basis_binding_identity(),
        admission.basis_binding_identity(),
    ) || typed_identity_drift(
        active_admission.signal_strategy_identity(),
        admission.signal_strategy_identity(),
    ) {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::ActiveLaneSourceMismatch,
            "active lane admission must certify the same admitted subscription source",
            &[
                validation_role_evidence_identity(
                    "lane_activation",
                    active_admission.activation_identity(),
                ),
                validation_role_evidence_identity("activation", activation.evidence_identity()),
                validation_role_evidence_identity(
                    "lane_admission",
                    active_admission.admission_identity(),
                ),
                validation_role_evidence_identity("admission", admission.evidence_identity()),
            ],
        ));
    }

    if typed_identity_drift(
        attachment.lane_digest().evidence_identity(),
        active_lane_handle.lane_digest().evidence_identity(),
    ) {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::AttachmentSourceMismatch,
            "consumer attachment must belong to the certified active lane handle",
            &[
                validation_role_evidence_identity(
                    "attachment_lane",
                    attachment.lane_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "handle_lane",
                    active_lane_handle.lane_digest().evidence_identity(),
                ),
            ],
        ));
    }

    if typed_identity_drift(
        maintenance_delta.active_lane_digest().evidence_identity(),
        active_lane_handle.lane_digest().evidence_identity(),
    ) || typed_identity_drift(
        lowering_report.maintenance_delta_identity(),
        maintenance_delta.evidence_identity(),
    ) {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::MaintenanceDeltaSourceMismatch,
            "maintenance delta and lowering report must belong to the certified lane",
            &[
                validation_role_evidence_identity(
                    "delta_lane",
                    maintenance_delta.active_lane_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "handle_lane",
                    active_lane_handle.lane_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "lowering_delta",
                    lowering_report.maintenance_delta_identity(),
                ),
                validation_role_evidence_identity("delta", maintenance_delta.evidence_identity()),
            ],
        ));
    }

    if typed_identity_drift(
        work_packet.active_lane_digest().evidence_identity(),
        active_lane_handle.lane_digest().evidence_identity(),
    ) || typed_identity_drift(
        work_packet.attachment_digest().evidence_identity(),
        attachment.attachment_digest().evidence_identity(),
    ) || typed_identity_drift(
        work_packet.maintenance_delta().evidence_identity(),
        maintenance_delta.evidence_identity(),
    ) || typed_identity_drift(
        work_packet.lowering_report().evidence_identity(),
        lowering_report.evidence_identity(),
    ) {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::WorkPacketSourceMismatch,
            "active delivery work packet must consume the certified lane, attachment, delta, and lowering report",
            &[
                validation_role_evidence_identity(
                    "packet_lane",
                    work_packet.active_lane_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "handle_lane",
                    active_lane_handle.lane_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "packet_attachment",
                    work_packet.attachment_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "attachment",
                    attachment.attachment_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "packet_delta",
                    work_packet.maintenance_delta().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "delta",
                    maintenance_delta.evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "packet_lowering",
                    work_packet.lowering_report().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "lowering",
                    lowering_report.evidence_identity(),
                ),
            ],
        ));
    }

    if typed_identity_drift(
        delivery_batch.delivery_window_identity(),
        delivery_window_identity,
    ) || typed_identity_drift(
        delivery_batch.attachment_digest().evidence_identity(),
        attachment.attachment_digest().evidence_identity(),
    ) || typed_identity_drift(
        delivery_batch
            .receipt()
            .attachment_digest()
            .evidence_identity(),
        attachment.attachment_digest().evidence_identity(),
    ) {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::DeliveryBatchSourceMismatch,
            "delivery batch and receipt must belong to the certified window and consumer attachment",
            &[
                validation_role_evidence_identity(
                    "batch_window",
                    delivery_batch.delivery_window_identity(),
                ),
                validation_role_evidence_identity("window", delivery_window_identity),
                validation_role_evidence_identity(
                    "batch_attachment",
                    delivery_batch.attachment_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "attachment",
                    attachment.attachment_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "receipt_attachment",
                    delivery_batch
                        .receipt()
                        .attachment_digest()
                        .evidence_identity(),
                ),
            ],
        ));
    }

    if typed_identity_drift(
        acknowledged_attachment
            .attachment_digest()
            .evidence_identity(),
        attachment.attachment_digest().evidence_identity(),
    ) {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::DeliveryBatchSourceMismatch,
            "acknowledged attachment must advance the frontier for the certified consumer attachment",
            &[
                validation_role_evidence_identity(
                    "ack_attachment",
                    acknowledged_attachment
                        .attachment_digest()
                        .evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "attachment",
                    attachment.attachment_digest().evidence_identity(),
                ),
            ],
        ));
    }

    if let Some(report) = continuation {
        if typed_identity_drift(
            report.active_lane_digest().evidence_identity(),
            active_lane_handle.lane_digest().evidence_identity(),
        ) {
            return Err(SubscriptionLifecycleCertificationError::new(
                SubscriptionLifecycleCertificationDenialKind::ContinuationSourceMismatch,
                "continuation report must belong to the certified active lane",
                &[
                    validation_role_evidence_identity(
                        "continuation_lane",
                        report.active_lane_digest().evidence_identity(),
                    ),
                    validation_role_evidence_identity(
                        "handle_lane",
                        active_lane_handle.lane_digest().evidence_identity(),
                    ),
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

    if typed_identity_drift(
        lifecycle_closeout.active_lane_digest().evidence_identity(),
        active_lane_handle.lane_digest().evidence_identity(),
    ) || typed_identity_drift(
        lifecycle_closeout.attachment_digest().evidence_identity(),
        attachment.attachment_digest().evidence_identity(),
    ) {
        return Err(SubscriptionLifecycleCertificationError::new(
            SubscriptionLifecycleCertificationDenialKind::CloseoutSourceMismatch,
            "lifecycle closeout must terminate the certified lane and attachment",
            &[
                validation_role_evidence_identity(
                    "closeout_lane",
                    lifecycle_closeout.active_lane_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "handle_lane",
                    active_lane_handle.lane_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "closeout_attachment",
                    lifecycle_closeout.attachment_digest().evidence_identity(),
                ),
                validation_role_evidence_identity(
                    "attachment",
                    attachment.attachment_digest().evidence_identity(),
                ),
            ],
        ));
    }

    let active_lane_handle_identity = lifecycle_active_lane_handle_identity(
        active_admission.lane_digest().evidence_identity(),
        active_lane_handle,
    );
    let active_lane_lookup_class_identity =
        lifecycle_active_lane_lookup_class_identity(active_admission.lookup_class().as_str());
    let subscription_budget_identity = lifecycle_subscription_budget_identity(
        active_admission.budget().registry_lookup_width(),
        active_admission.budget().fanout_width(),
        active_admission.budget().allocation_scope_width(),
        active_admission.budget().lookup_class().as_str(),
        active_admission.budget().allocation_posture().as_str(),
        active_admission.budget().durable_checkpoint_requested(),
        active_admission.budget().store_backed_restart_requested(),
    );
    let absent_performance = lifecycle_absent_performance_receipt_identity();
    let preview_performance_identity = preview_evidence.performance_receipt_identity.clone();
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
        context.policy_identity(),
        context.tenant_basis_identity(),
        context.relationship_proof_identity(),
        context.view_shape_identity(),
        context.basis_posture_identity(),
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
        certification_bundle_authority: admit_query_subscription_authority_identity(
            certification_bundle_identity,
        ),
        query_scope_identity: context.query_scope_identity().clone(),
        subscription_family_identity: context.subscription_family_identity().clone(),
        subscription_declaration_identity: admission.query_declaration_identity().clone(),
        subscription_equivalence_identity: context.subscription_equivalence_identity().clone(),
        admission_identity: admission.evidence_identity().clone(),
        active_lane_identity: active_admission.lane_digest().evidence_identity().clone(),
        active_lane_handle_identity,
        active_lane_lookup_class_identity,
        subscription_budget_identity,
        subscription_performance_receipt_identity,
        consumer_attachment_identity: attachment.attachment_digest().evidence_identity().clone(),
        acknowledgement_frontier_identity: acknowledged_attachment
            .acknowledgement_frontier()
            .evidence_identity()
            .clone(),
        delivery_window_identity: delivery_window_identity.clone(),
        maintenance_delta_identity: maintenance_delta.evidence_identity().clone(),
        active_delivery_work_packet_identity: work_packet.evidence_identity().clone(),
        active_delivery_density_posture_identity,
        allocation_posture_identity,
        delivery_batch_identity: delivery_batch.evidence_identity().clone(),
        patch_group_identity: delivery_batch.patch_group().patch_group_identity().clone(),
        delivery_receipt_identity: delivery_batch.receipt().evidence_identity().clone(),
        continuation_identity,
        preview_isolation_identity: preview_evidence.preview_isolation_identity,
        preview_residue_identity: preview_evidence.preview_residue_identity,
        policy_identity: context.policy_identity().clone(),
        tenant_basis_identity: context.tenant_basis_identity().clone(),
        relationship_proof_identity: context.relationship_proof_identity().clone(),
        view_shape_identity: context.view_shape_identity().clone(),
        basis_posture_identity: context.basis_posture_identity().clone(),
        bridge_declaration_identity: admission.bridge_declaration_identity().clone(),
        signal_strategy_identity: admission.signal_strategy_identity().clone(),
        counter_sequence_identity,
        subscription_lifecycle_scale_slope_identity: scale_report.evidence_identity_ref().clone(),
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
                    &[validation_shape_role_evidence_identity(
                        "closeout_kind",
                        lifecycle_closeout.closeout_kind().as_str(),
                    )],
                ));
            }

            Ok(PreviewCertificationEvidence {
                preview_isolation_identity: lifecycle_absent_preview_isolation_identity(),
                preview_residue_identity: lifecycle_absent_preview_residue_identity(),
                counter_identities: Vec::new(),
                support_identities: vec![ForgeQueryEvidenceIdentity::compose(
                    crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
                )
                .field_shape(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
                    "subscription_preview_support_absent_v1",
                )
                .seal()],
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
                || typed_identity_drift(
                    lifecycle_closeout.basis_binding_identity(),
                    isolation.basis_binding_identity(),
                )
                || typed_identity_drift(
                    lifecycle_closeout.checkpoint_identity(),
                    isolation.checkpoint_identity(),
                )
                || discard_closeout.active_lane_digest() != active_lane_handle.lane_digest()
                || discard_closeout.attachment_digest() != attachment.attachment_digest()
                || discard_closeout.future_selection() != isolation.future_selection()
                || typed_identity_drift(
                    discard_closeout.basis_binding_identity(),
                    isolation.basis_binding_identity(),
                )
                || typed_identity_drift(
                    discard_closeout.checkpoint_identity(),
                    isolation.checkpoint_identity(),
                )
                || typed_identity_drift(
                    discard_closeout.preview_epoch_identity(),
                    isolation.preview_epoch_identity(),
                )
                || discard_closeout.residue_report_identity() != residue_report.report_identity()
                || typed_identity_drift(
                    lifecycle_closeout.source_identity(),
                    discard_closeout.closeout_identity(),
                )
            {
                return Err(SubscriptionLifecycleCertificationError::new(
                    SubscriptionLifecycleCertificationDenialKind::PreviewSourceMismatch,
                    "preview discard certification requires aligned isolation, residue, closeout, and lifecycle closeout evidence",
                    &[
                        validation_shape_role_evidence_identity(
                            "closeout_kind",
                            lifecycle_closeout.closeout_kind().as_str(),
                        ),
                        validation_role_evidence_identity(
                            "lane",
                            active_lane_handle.lane_digest().evidence_identity(),
                        ),
                        validation_role_evidence_identity(
                            "attachment",
                            attachment.attachment_digest().evidence_identity(),
                        ),
                        validation_role_evidence_identity(
                            "preview_lane",
                            isolation.active_lane_digest().evidence_identity(),
                        ),
                        validation_role_evidence_identity(
                            "preview_attachment",
                            isolation.attachment_digest().evidence_identity(),
                        ),
                        validation_role_evidence_identity(
                            "discard_attachment",
                            discard_closeout.attachment_digest().evidence_identity(),
                        ),
                    ],
                ));
            }

            Ok(PreviewCertificationEvidence {
                preview_isolation_identity: isolation.isolation_identity().clone(),
                preview_residue_identity: residue_report.report_identity().clone(),
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
                    .field_evidence_identity(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("residue"),
                        residue_report.report_identity(),
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
                || typed_identity_drift(
                    lifecycle_closeout.basis_binding_identity(),
                    isolation.basis_binding_identity(),
                )
                || typed_identity_drift(
                    lifecycle_closeout.checkpoint_identity(),
                    isolation.checkpoint_identity(),
                )
                || promotion_handoff.preview_lane_digest() != active_lane_handle.lane_digest()
                || promotion_handoff.attachment_digest() != attachment.attachment_digest()
                || promotion_handoff.future_selection() != isolation.future_selection()
                || typed_identity_drift(
                    promotion_handoff.preview_basis_binding_identity(),
                    isolation.basis_binding_identity(),
                )
                || typed_identity_drift(
                    promotion_handoff.preview_checkpoint_identity(),
                    isolation.checkpoint_identity(),
                )
                || typed_identity_drift(
                    promotion_handoff.preview_epoch_identity(),
                    isolation.preview_epoch_identity(),
                )
                || promotion_handoff.residue_report_identity() != residue_report.report_identity()
                || typed_identity_drift(
                    lifecycle_closeout.source_identity(),
                    promotion_handoff.handoff_identity(),
                )
            {
                return Err(SubscriptionLifecycleCertificationError::new(
                    SubscriptionLifecycleCertificationDenialKind::PreviewSourceMismatch,
                    "preview promotion certification requires aligned isolation, residue, handoff, and lifecycle closeout evidence",
                    &[
                        validation_shape_role_evidence_identity(
                            "closeout_kind",
                            lifecycle_closeout.closeout_kind().as_str(),
                        ),
                        validation_role_evidence_identity(
                            "lane",
                            active_lane_handle.lane_digest().evidence_identity(),
                        ),
                        validation_role_evidence_identity(
                            "attachment",
                            attachment.attachment_digest().evidence_identity(),
                        ),
                        validation_role_evidence_identity(
                            "promotion_lane",
                            promotion_handoff.preview_lane_digest().evidence_identity(),
                        ),
                    ],
                ));
            }

            Ok(PreviewCertificationEvidence {
                preview_isolation_identity: isolation.isolation_identity().clone(),
                preview_residue_identity: lifecycle_preview_promotion_residue_identity(
                    residue_report.report_identity(),
                    promotion_handoff.handoff_identity(),
                    promotion_handoff
                        .authoritative_active_lane_digest()
                        .evidence_identity(),
                ),
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
                    .field_evidence_identity(
                        crate::evidence_identity::ForgeQueryEvidenceTag::new("residue"),
                        residue_report.report_identity(),
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
                performance_receipt_identity: promotion_handoff
                    .performance_receipt()
                    .performance_receipt_identity()
                    .clone(),
            })
        }
    }
}

pub fn certify_query_subscription_activation(
    admission: QuerySubscriptionAdmissionArtifact,
    activation: SubscriptionActivationInput,
    scale_report: QuerySubscriptionScaleSlopeReport,
) -> Result<QuerySubscriptionCertificationBundle, QuerySubscriptionCertificationError> {
    if typed_identity_drift(
        activation.admission_identity(),
        admission.evidence_identity(),
    ) || typed_identity_drift(
        activation.query_declaration_identity(),
        admission.query_declaration_identity(),
    ) || typed_identity_drift(
        activation.bridge_declaration_identity(),
        admission.bridge_declaration_identity(),
    ) || typed_identity_drift(
        activation.basis_binding_identity(),
        admission.basis_binding_identity(),
    ) || typed_identity_drift(
        activation.signal_strategy_identity(),
        admission.signal_strategy_identity(),
    ) {
        return Err(QuerySubscriptionCertificationError::new(
            QuerySubscriptionCertificationDenialKind::ActivationAdmissionMismatch,
            "subscription activation input does not match the admitted subscription artifact",
            &[
                validation_role_evidence_identity("admission", admission.evidence_identity()),
                validation_role_evidence_identity(
                    "activation_admission",
                    activation.admission_identity(),
                ),
                validation_role_evidence_identity(
                    "admission_query",
                    admission.query_declaration_identity(),
                ),
                validation_role_evidence_identity(
                    "activation_query",
                    activation.query_declaration_identity(),
                ),
            ],
        ));
    }

    if typed_identity_drift(
        scale_report.activation_identity(),
        activation.evidence_identity(),
    ) || typed_identity_drift(
        scale_report.admission_identity(),
        activation.admission_identity(),
    ) {
        return Err(QuerySubscriptionCertificationError::new(
            QuerySubscriptionCertificationDenialKind::ScaleSlopeSourceMismatch,
            "subscription scale slope report does not certify this activation source",
            &[
                validation_role_evidence_identity("activation", activation.evidence_identity()),
                validation_role_evidence_identity(
                    "scale_activation",
                    scale_report.activation_identity(),
                ),
                validation_role_evidence_identity(
                    "activation_admission",
                    activation.admission_identity(),
                ),
                validation_role_evidence_identity(
                    "scale_admission",
                    scale_report.admission_identity(),
                ),
            ],
        ));
    }

    let admission_counter_identity = admission.counters().evidence_identity();
    let activation_counter_identity = activation.counters().evidence_identity();
    let diagnostics_identity = admission.diagnostics().diagnostics_identity().clone();
    let support_profile_identity = admission.support_profile().profile_identity().clone();
    let scale_slope_identity = scale_report.evidence_identity_ref().clone();
    let scale_activation_identity = scale_report.activation_identity().clone();
    let scale_admission_identity = scale_report.admission_identity().clone();
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
        scale_report.evidence_identity_ref(),
    );
    Ok(QuerySubscriptionCertificationBundle {
        certification_bundle_identity,
        admission_identity: admission.evidence_identity().clone(),
        activation_identity: activation.evidence_identity().clone(),
        query_declaration_identity: admission.query_declaration_identity().clone(),
        bridge_declaration_identity: admission.bridge_declaration_identity().clone(),
        basis_binding_identity: admission.basis_binding_identity().clone(),
        signal_strategy_identity: admission.signal_strategy_identity().clone(),
        diagnostics_identity,
        support_profile_identity,
        admission_counter_identity,
        activation_counter_identity,
        scale_slope_identity,
        scale_activation_identity,
        scale_admission_identity,
    })
}
