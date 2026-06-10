use super::*;
use crate::subscription::SubscriptionActivationInput;

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
        let mut evidence = vec!["runtime-branch-basis-admission".to_string()];
        evidence.extend(branch_support_evidence);
        let basis_admission = ForgeQueryBranchBasisAdmission::new(
            &self.evidence_authority,
            label.clone(),
            options.effect_policy(),
            evidence,
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
        let activation_digest = activation_receipt.activation_digest().to_string();
        let support_evidence = activation_receipt.support_evidence().to_string();
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
        let active_lane_digest = active_lane_handle.lane_digest().as_str().to_string();
        let consumer_attachment = attach_subscription_consumer(
            &mut self.active_subscriptions,
            &active_lane_handle,
            SubscriptionConsumerAttachmentRequest::admitted(
                format!("runtime-live-view:{view_name}"),
                activation_digest.clone(),
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
            lowered_subscription.query_digest.as_str(),
            lowered_subscription.live_view_digest.as_str(),
            lowered_subscription.subscription_family,
            lowered_subscription.subscription_declaration_digest,
            lowered_subscription.bridge_declaration_digest,
            lowered_subscription.admission_digest,
            activation_digest,
            lowered_subscription.basis_binding_digest,
            lowered_subscription.signal_strategy_digest,
            active_lane_digest,
            &consumer_attachment,
            runtime_subscription_budget_policy(),
            RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY,
            RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY,
            active_lane_counters,
            consumer_attachment_counters,
            support_evidence,
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

struct LoweredRuntimeLiveSubscriptionRequest {
    query_digest: String,
    live_view_digest: String,
    subscription_family: String,
    subscription_declaration_digest: String,
    admission_digest: String,
    bridge_declaration_digest: String,
    basis_binding_digest: String,
    signal_strategy_digest: String,
    activation: SubscriptionActivationInput,
}

fn lower_runtime_live_subscription_request(
    backend: &dyn ForgeQueryRuntimeBackend,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
) -> Result<LoweredRuntimeLiveSubscriptionRequest, ForgeQueryRuntimeError> {
    let session = declare_runtime_live_query_session_with_grouped_baseline(
        request.clone(),
        schema_view,
        backend.snapshot_token(),
        grouped_baseline_members_or_error(backend, view_name, request)?,
    )
    .map_err(|error| live_subscription_error(view_name, "live-lowering", error))?;
    let view_family = session.live_view().lowering().family();
    let dimensions = subscription_dimensions_for_request(request, view_family)?;
    let live_admission =
        crate::subscription::LiveQueryAdmissionArtifact::from_live_promotion_with_view(
            session.live_view().core_live_plan().descriptor(),
            crate::subscription::QuerySubscriptionBasisPosture::CurrentHead,
            view_family,
            dimensions,
        );
    let selection = select_runtime_subscription_family(view_name, live_admission)?;
    let subscription_family = selection.family().as_str().to_string();
    let declaration =
        declare_query_subscription(selection, runtime_slice_budget()).map_err(|error| {
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "declaration",
                message: format!("{error:?}"),
            }
        })?;
    let subscription_declaration_digest = declaration.declaration_digest().as_str().to_string();
    let lowering =
        lower_query_subscription_to_bridge(declaration, runtime_bridge_lowering_budget()).map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "bridge-lowering",
                message: format!("{error:?}"),
            },
        )?;
    let admission = admit_query_subscription(lowering, runtime_subscription_admission_budget())
        .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "subscription-admission",
                message: format!("{error:?}"),
            },
        )?;

    Ok(LoweredRuntimeLiveSubscriptionRequest {
        query_digest: session.canonical().query().digest().as_str().to_string(),
        live_view_digest: session.live_view().lowering().digest().to_string(),
        subscription_family,
        subscription_declaration_digest,
        admission_digest: admission.admission_digest().to_string(),
        bridge_declaration_digest: admission.bridge_declaration_digest().to_string(),
        basis_binding_digest: admission.basis_binding_digest().to_string(),
        signal_strategy_digest: admission.signal_strategy_digest().to_string(),
        activation: prepare_subscription_activation(admission),
    })
}

fn grouped_baseline_members_or_error(
    backend: &dyn ForgeQueryRuntimeBackend,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
) -> Result<Option<Vec<(String, String)>>, ForgeQueryRuntimeError> {
    backend.grouped_baseline_members(request).map_err(|error| {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "grouped-baseline",
            message: error.to_string(),
        }
    })
}

fn select_runtime_subscription_family(
    view_name: &str,
    live_admission: crate::subscription::LiveQueryAdmissionArtifact,
) -> Result<crate::subscription::QuerySubscriptionFamilySelection, ForgeQueryRuntimeError> {
    select_query_subscription_family(live_admission, runtime_family_budget()).map_err(|error| {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "family-selection",
            message: format!("{error:?}"),
        }
    })
}

fn install_live_subscription_activation(
    backend: &mut dyn ForgeQueryRuntimeBackend,
    view_name: &str,
    activation: &SubscriptionActivationInput,
) -> Result<SubscriptionActivationReceipt, ForgeQueryRuntimeError> {
    let activation_receipt = backend
        .install_live_subscription(view_name, activation)
        .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "activation-admission",
                message: error.to_string(),
            },
        )?;
    if let Some(message) = activation_receipt.drift_from_activation(view_name, activation) {
        return Err(ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "activation-receipt",
            message,
        });
    }
    Ok(activation_receipt)
}

fn admit_session_label_for_lane(
    admitted_labels: &mut std::collections::BTreeMap<String, ForgeQuerySessionLabel>,
    authority_lane: ForgeQueryAuthorityLane,
    label: &ForgeQuerySessionLabel,
) -> Result<(), ForgeQueryRuntimeError> {
    let identity = label.identity_digest().to_string();
    if admitted_labels.contains_key(&identity) {
        return Err(ForgeQueryRuntimeError::SessionLabelCollision {
            authority_lane,
            label: label.clone(),
        });
    }
    admitted_labels.insert(identity, label.clone());
    Ok(())
}
