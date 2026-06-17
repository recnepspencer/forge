use super::evidence_identities::runtime_live_view_consumer_attachment_identity;
use super::live_subscription::live_subscription_source_identity;
use super::runtime_session_lowering::{
    install_live_subscription_activation, lower_runtime_live_subscription_request,
};
use super::*;

impl ForgeQueryRuntime {
    pub fn preview<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.preview_with_options(label, ForgeQueryPreviewOptions::default())
    }

    pub fn branch<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.branch_with_options(label, ForgeQueryBranchOptions::default())
    }

    pub fn branch_with_options<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
        options: ForgeQueryBranchOptions,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.try_branch_with_options(label, options)
    }

    pub fn try_branch<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.try_branch_with_options(label, ForgeQueryBranchOptions::default())
    }

    pub fn try_branch_with_options<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
        options: ForgeQueryBranchOptions,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)?;
        self.admit_branch_session_label(&label)?;
        let branch_support_evidence = self
            .backend
            .support_profile()
            .support_for(ForgeQueryRuntimeFacadeFamily::BranchPreview)
            .map(|support| support.evidence().to_vec())
            .unwrap_or_default();
        let evidence_rows = std::iter::once(ForgeQueryBasisAdmissionEvidenceRow::tagged(
            "runtime-branch-basis-admission",
            "runtime-branch-basis-admission",
        ))
        .chain(
            branch_support_evidence
                .into_iter()
                .map(ForgeQueryBasisAdmissionEvidenceRow::support_profile_token),
        )
        .collect::<Vec<_>>();
        let basis_admission = ForgeQueryBranchBasisAdmission::new(
            &self.evidence_authority,
            label.clone(),
            options.effect_policy(),
            evidence_rows,
        );
        Ok(ForgeQueryBranchSession::new(
            label,
            self,
            options,
            basis_admission,
        ))
    }

    pub fn preview_with_options<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
        options: ForgeQueryPreviewOptions,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.try_preview_with_options(label, options)
    }

    pub fn try_preview<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.try_preview_with_options(label, ForgeQueryPreviewOptions::default())
    }

    pub fn try_preview_with_options<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
        options: ForgeQueryPreviewOptions,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)?;
        self.admit_preview_session_label(&label)?;
        let basis_admission = self.backend.admit_preview_basis(
            &label,
            options.effect_policy(),
            &self.evidence_authority,
        )?;
        Ok(ForgeQueryPreviewSession::new(
            label,
            self,
            options.effect_policy(),
            basis_admission,
        ))
    }

    pub fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.backend.support_profile()
    }

    pub(super) fn admit_facade_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.backend
            .support_profile()
            .admit(family)
            .map_err(ForgeQueryRuntimeError::UnsupportedFacadeFamily)
    }

    pub(in crate::runtime) fn admit_facade_family_lane(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
        authority_lane: ForgeQueryAuthorityLane,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.admit_facade_family(family)?;
        let support_profile = self.backend.support_profile();
        let Some(row) = support_profile.support_for(family) else {
            return Err(ForgeQueryRuntimeError::UnsupportedFacadeFamily(
                ForgeQueryRuntimeSupportDenial::unsupported(
                    family,
                    "backend support profile does not declare this facade family",
                ),
            ));
        };
        if row.authority_lanes().contains(&authority_lane) {
            Ok(())
        } else {
            Err(ForgeQueryRuntimeError::UnsupportedFacadeFamily(
                ForgeQueryRuntimeSupportDenial::new(
                    family,
                    row.status(),
                    Some(row.teaching_posture()),
                    format!(
                        "backend support profile does not admit `{}` lane for `{}` facade family",
                        authority_lane, family
                    ),
                ),
            ))
        }
    }

    fn admit_preview_session_label(
        &mut self,
        label: &ForgeQuerySessionLabel,
    ) -> Result<(), ForgeQueryRuntimeError> {
        admit_session_label_for_lane(
            &mut self.preview_session_labels,
            ForgeQueryAuthorityLane::PreviewTruth,
            label,
        )
    }

    fn admit_branch_session_label(
        &mut self,
        label: &ForgeQuerySessionLabel,
    ) -> Result<(), ForgeQueryRuntimeError> {
        admit_session_label_for_lane(
            &mut self.branch_session_labels,
            ForgeQueryAuthorityLane::BranchLocalTruth,
            label,
        )
    }

    pub(super) fn install_live_subscription_for_request(
        &mut self,
        view_name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryRuntimeLiveSubscriptionActivation, ForgeQueryRuntimeError> {
        let lowered_subscription = lower_runtime_live_subscription_request(
            &*self.backend,
            view_name,
            request,
            schema_view,
        )?;
        let activation = lowered_subscription.activation.clone();
        let counters = activation.counters().clone();
        let activation_receipt =
            install_live_subscription_activation(&mut *self.backend, view_name, &activation)?;
        let remask_posture = activation_receipt.remask_posture().cloned();
        let active_lane_admission =
            admit_active_subscription_lane(activation.clone(), runtime_active_lifecycle_budget())
                .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: view_name.to_string(),
                    stage: "active-lane-admission",
                    message: format!("{error:?}"),
                },
            )?;
        let active_lane_handle =
            open_active_subscription_lane(&mut self.active_subscriptions, active_lane_admission)
                .map_err(
                    |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                        view_name: view_name.to_string(),
                        stage: "active-lane-open",
                        message: format!("{error:?}"),
                    },
                )?;
        let active_lane_counters = self.active_subscriptions.counters().clone();
        let consumer_attachment = attach_subscription_consumer(
            &mut self.active_subscriptions,
            &active_lane_handle,
            SubscriptionConsumerAttachmentRequest::from_consumer_identity(
                runtime_live_view_consumer_attachment_identity(view_name),
                activation_receipt.activation_identity().clone(),
            ),
            runtime_consumer_attachment_budget(),
        )
        .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "consumer-attachment",
                message: format!("{error:?}"),
            },
        )?;
        let consumer_attachment_counters = self.active_subscriptions.counters().clone();

        let installation = ForgeQueryRuntimeLiveSubscriptionInstallation::new(
            view_name,
            lowered_subscription.query_identity,
            lowered_subscription.live_view_identity,
            lowered_subscription.canonical_result_shape_digest,
            lowered_subscription.subscription_family,
            lowered_subscription.subscription_declaration_identity,
            lowered_subscription.bridge_declaration_identity,
            lowered_subscription.admission_identity,
            live_subscription_source_identity(
                "activation",
                activation_receipt.activation_identity(),
            ),
            lowered_subscription.basis_binding_identity,
            lowered_subscription.signal_strategy_identity,
            live_subscription_source_identity(
                "active_lane",
                active_lane_handle.lane_digest().evidence_identity(),
            ),
            &consumer_attachment,
            runtime_subscription_budget_policy(),
            runtime_active_lifecycle_budget_policy(),
            runtime_consumer_attachment_budget_policy(),
            active_lane_counters,
            consumer_attachment_counters,
            live_subscription_source_identity("support", activation_receipt.support_identity()),
            counters,
        );

        Ok(ForgeQueryRuntimeLiveSubscriptionActivation {
            installation,
            active_lane_handle,
            consumer_attachment,
            request: request.clone(),
            remask_posture,
        })
    }
}

fn admit_session_label_for_lane(
    admitted_labels: &mut std::collections::BTreeSet<ForgeQuerySessionLabel>,
    authority_lane: ForgeQueryAuthorityLane,
    label: &ForgeQuerySessionLabel,
) -> Result<(), ForgeQueryRuntimeError> {
    if admitted_labels.contains(label) {
        return Err(ForgeQueryRuntimeError::SessionLabelCollision {
            authority_lane,
            label: label.clone(),
        });
    }
    admitted_labels.insert(label.clone());
    Ok(())
}
