use super::*;

pub(super) fn admit_authority_requirements(
    requirements: &std::collections::BTreeSet<ForgeQueryAuthorityRequirement>,
) -> Result<(), ForgeQueryRuntimeError> {
    for requirement in requirements {
        match requirement {
            ForgeQueryAuthorityRequirement::ReadOnly
            | ForgeQueryAuthorityRequirement::Live
            | ForgeQueryAuthorityRequirement::BranchLocal
            | ForgeQueryAuthorityRequirement::Previewable
            | ForgeQueryAuthorityRequirement::Writeback
            | ForgeQueryAuthorityRequirement::ReplayRequired => {}
            ForgeQueryAuthorityRequirement::Merge | ForgeQueryAuthorityRequirement::Destructive => {
                return Err(ForgeQueryRuntimeError::UnsupportedAuthority(
                    requirement.as_str().to_string(),
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn live_subscription_error(
    view_name: &str,
    stage: &'static str,
    error: DeclarativeLiveQueryError,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: view_name.to_string(),
        stage,
        message: format!("{error:?}"),
    }
}

pub(super) fn attach_symbolic_target_reference_to_receipt(
    mut receipt: ForgeQueryMutationReceipt,
    reference: BridgeSymbolicTargetReferenceBundle,
) -> ForgeQueryMutationReceipt {
    receipt.bridge_authority = receipt
        .bridge_authority
        .take()
        .map(|bundle| bundle.with_symbolic_target_reference(reference));
    receipt
}

pub(super) fn attach_naming_mutation_to_receipt(
    mut receipt: ForgeQueryMutationReceipt,
    naming: BridgeNamingMutationBundle,
) -> ForgeQueryMutationReceipt {
    receipt.bridge_authority = receipt
        .bridge_authority
        .take()
        .map(|bundle| bundle.with_naming_mutation(naming));
    receipt
}

pub(super) fn attach_continuity_mutation_to_receipt(
    mut receipt: ForgeQueryMutationReceipt,
    continuity: BridgeContinuityMutationBundle,
) -> ForgeQueryMutationReceipt {
    receipt.bridge_authority = receipt
        .bridge_authority
        .take()
        .map(|bundle| bundle.with_continuity_mutation(continuity));
    receipt
}

pub(super) fn record_same_batch_symbolic_target(
    symbolic_targets: &mut BTreeMap<String, (String, Option<String>)>,
    reference: Option<&ForgeQuerySymbolicTargetReference>,
    receipt: &ForgeQueryMutationReceipt,
) {
    if receipt
        .deltas
        .first()
        .is_none_or(|delta| delta.kind != ForgeQueryMutationKind::Created)
    {
        return;
    }
    let Some(reference) = reference else {
        return;
    };
    let (_, target_collection, target_entity_identity) = classify_receipt_mutation_summary(receipt);
    let Some(target_entity_identity) = target_entity_identity else {
        return;
    };
    symbolic_targets.insert(
        reference.symbol().to_string(),
        (target_entity_identity, target_collection),
    );
}

pub(super) fn resolve_same_batch_symbolic_target(
    symbolic_targets: &BTreeMap<String, (String, Option<String>)>,
    reference: &ForgeQuerySymbolicTargetReference,
) -> Result<(String, Option<String>), ForgeQueryRuntimeError> {
    let Some((resolved_entity_identity, resolved_collection)) =
        symbolic_targets.get(reference.symbol())
    else {
        return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
            ForgeQuerySymbolicTargetReferenceDenial::new(
                reference,
                ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
                format!(
                    "same-batch symbolic target `{}` was not declared earlier in this mutation batch",
                    reference.symbol()
                ),
            ),
        ));
    };
    if let Some(expected_collection) = reference.target_collection() {
        if resolved_collection.as_deref() != Some(expected_collection) {
            return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                ForgeQuerySymbolicTargetReferenceDenial::new(
                    reference,
                    ForgeQuerySymbolicTargetReferenceDenialKind::CollectionMismatch,
                    format!(
                        "same-batch symbolic target `{}` resolved to collection `{}`, not `{expected_collection}`",
                        reference.symbol(),
                        resolved_collection.as_deref().unwrap_or("unknown"),
                    ),
                ),
            ));
        }
    }
    Ok((
        resolved_entity_identity.clone(),
        resolved_collection.clone(),
    ))
}

pub(super) fn combined_batch_mutation_receipt(
    receipts: &[ForgeQueryMutationReceipt],
) -> Result<ForgeQueryMutationReceipt, ForgeQueryRuntimeError> {
    let Some(last_receipt) = receipts.last() else {
        return Err(ForgeQueryRuntimeError::Workspace(
            ForgeQueryWorkspaceError::new("mutation batch must produce at least one write receipt"),
        ));
    };
    let commit_identity = format!(
        "batch:{}",
        crate::identity::hash_parts(
            &std::iter::once("forge_query_batch_mutation_receipt_v1".to_string())
                .chain(
                    receipts
                        .iter()
                        .map(|receipt| format!("commit:{}", receipt.commit_identity)),
                )
                .collect::<Vec<_>>(),
        )
    );
    let deltas = receipts
        .iter()
        .flat_map(|receipt| receipt.deltas.iter().cloned())
        .collect::<Vec<_>>();
    Ok(ForgeQueryMutationReceipt {
        commit_identity,
        snapshot_token: last_receipt.snapshot_token.clone(),
        deltas,
        bridge_authority: None,
    })
}

pub(super) fn classify_receipt_mutation_summary(
    receipt: &ForgeQueryMutationReceipt,
) -> (ForgeQueryMutationFamily, Option<String>, Option<String>) {
    let mutation_family = receipt
        .deltas
        .first()
        .map(|delta| match delta.kind {
            ForgeQueryMutationKind::Created => ForgeQueryMutationFamily::Insert,
            ForgeQueryMutationKind::Updated => ForgeQueryMutationFamily::Update,
            ForgeQueryMutationKind::Deleted => ForgeQueryMutationFamily::Delete,
        })
        .unwrap_or(ForgeQueryMutationFamily::Update);
    let mut collections = receipt
        .deltas
        .iter()
        .map(|delta| delta.collection.clone())
        .collect::<Vec<_>>();
    collections.sort();
    collections.dedup();
    let mut entity_identities = receipt
        .deltas
        .iter()
        .map(|delta| delta.entity_identity.clone())
        .collect::<Vec<_>>();
    entity_identities.sort();
    entity_identities.dedup();
    (
        mutation_family,
        (collections.len() == 1).then(|| collections[0].clone()),
        (entity_identities.len() == 1).then(|| entity_identities[0].clone()),
    )
}

pub(super) fn subscription_dimensions_for_request(
    request: &DeclarativeLiveQueryRequest,
    view_family: LiveViewShapeFamily,
) -> Result<QuerySubscriptionAdmissionDimensions, ForgeQueryRuntimeError> {
    let projection_width = NonZeroUsize::new(request.projection().len().max(1))
        .expect("projection width is forced non-zero");
    let ordering_width = NonZeroUsize::new(1).expect("ordering width literal is non-zero");
    let metadata_width = NonZeroUsize::new(1).expect("metadata width literal is non-zero");

    match (request.view_shape(), view_family) {
        (DeclarativeLiveViewShape::ListSplice | DeclarativeLiveViewShape::Table, _) => {
            Ok(QuerySubscriptionAdmissionDimensions::collection_membership(
                projection_width,
                ordering_width,
            ))
        }
        (DeclarativeLiveViewShape::Detail, _) => Ok(
            QuerySubscriptionAdmissionDimensions::detail_exact(projection_width),
        ),
        (
            DeclarativeLiveViewShape::InspectorObserved
            | DeclarativeLiveViewShape::InspectorFocused { .. }
            | DeclarativeLiveViewShape::IdentityAwareInspectorFocused { .. },
            _,
        ) => Ok(
            QuerySubscriptionAdmissionDimensions::inspector_detail_exact(
                projection_width,
                metadata_width,
            ),
        ),
        (DeclarativeLiveViewShape::KanbanGrouped { .. }, _) => Ok(
            QuerySubscriptionAdmissionDimensions::grouped_collection_membership(
                projection_width,
                ordering_width,
                NonZeroUsize::new(1).expect("grouping width literal is non-zero"),
                metadata_width,
            ),
        ),
    }
}

pub(super) fn runtime_family_budget() -> QuerySubscriptionWorkBudget {
    QuerySubscriptionWorkBudget::scratch_buffer_only(64, 64, 64, 512, 1)
}

pub(super) fn runtime_slice_budget() -> QuerySubscriptionSliceBudget {
    QuerySubscriptionSliceBudget::scratch_buffer_only(64, 64, 64, 64, 64, 64, 64, 64)
}

pub(super) fn runtime_bridge_lowering_budget() -> QuerySubscriptionBridgeLoweringBudget {
    QuerySubscriptionBridgeLoweringBudget::admitted(1, 64, 64, 64, 64)
}

pub(super) fn runtime_subscription_admission_budget() -> QuerySubscriptionAdmissionBudget {
    QuerySubscriptionAdmissionBudget::admitted(64, 64, 64, 64, 64)
}

pub(super) fn runtime_active_lifecycle_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::LifecycleArena,
    )
}

pub(super) fn runtime_consumer_attachment_budget() -> SubscriptionConsumerAttachmentBudget {
    SubscriptionConsumerAttachmentBudget::admitted(
        ActiveFanoutWidth::measured(1),
        ConsumerDeliveryPacingWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

pub(super) fn runtime_subscription_budget_policy() -> String {
    [
        RUNTIME_SUBSCRIPTION_FAMILY_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_SLICE_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_BRIDGE_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_ADMISSION_BUDGET_POLICY,
    ]
    .join("|")
}

#[cfg(test)]
pub(super) fn runtime_subscription_budget_digest() -> String {
    crate::identity::hash_parts(&[
        "runtime_live_subscription_budget_policy_v1".to_string(),
        runtime_subscription_budget_policy(),
        RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY.to_string(),
        RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY.to_string(),
    ])
}
