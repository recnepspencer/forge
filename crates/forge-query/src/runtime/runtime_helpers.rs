use super::*;
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity,
};

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
                return Err(ForgeQueryRuntimeError::UnsupportedAuthorityRequirement(
                    requirement.clone(),
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

pub(super) fn synthetic_existing_assertion_receipt(
    binding: &ForgeQueryExistingTruthTargetBinding,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
    declared_aspect_value_digest: Option<&crate::evidence_identity::ForgeQueryEvidenceIdentity>,
) -> ForgeQueryMutationReceipt {
    let assertion_identity = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
    )
    .field_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("binding"),
        binding.binding_digest(),
    )
    .field_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("snapshot_identity"),
        &snapshot_identity.evidence_identity(),
    )
    .optional_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("aspect_digest"),
        declared_aspect_value_digest,
    )
    .seal();
    ForgeQueryMutationReceipt {
        commit_identity: ForgeQueryCommitIdentity::preview(assertion_identity),
        snapshot_identity: snapshot_identity.clone(),
        deltas: Vec::new(),
        bridge_authority: None,
    }
}

pub(super) fn record_same_batch_symbolic_target(
    symbolic_targets: &mut BTreeMap<String, (ForgeQueryEntityIdentity, Option<String>)>,
    reference: Option<&ForgeQuerySymbolicTargetReference>,
    declared_collection: Option<&str>,
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
    let resolved_target = declared_collection
        .and_then(|collection| {
            let mut matches = receipt
                .deltas
                .iter()
                .filter(|delta| {
                    delta.kind == ForgeQueryMutationKind::Created && delta.collection == collection
                })
                .map(|delta| (delta.entity_identity.clone(), Some(collection.to_string())))
                .collect::<Vec<_>>();
            (matches.len() == 1).then(|| matches.remove(0))
        })
        .or_else(|| {
            let (_, target_collection, target_entity_identity) =
                classify_receipt_mutation_summary(receipt);
            target_entity_identity.map(|identity| (identity, target_collection))
        });
    let Some((target_entity_identity, target_collection)) = resolved_target else {
        return;
    };
    symbolic_targets.insert(
        reference.symbol().to_string(),
        (target_entity_identity, target_collection),
    );
}

pub(super) fn resolve_same_batch_symbolic_target(
    symbolic_targets: &BTreeMap<String, (ForgeQueryEntityIdentity, Option<String>)>,
    reference: &ForgeQuerySymbolicTargetReference,
) -> Result<(ForgeQueryEntityIdentity, Option<String>), ForgeQueryRuntimeError> {
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

pub(super) fn resolve_symbolic_aspect_references(
    symbolic_targets: &BTreeMap<String, (ForgeQueryEntityIdentity, Option<String>)>,
    mut aspects: Vec<ForgeQueryAspectValue>,
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
) -> Result<Vec<ForgeQueryAspectValue>, ForgeQueryRuntimeError> {
    for reference in symbolic_aspect_references {
        let (resolved_entity_identity, _) =
            resolve_same_batch_symbolic_target(symbolic_targets, reference.reference())?;
        let resolved_entity_evidence_identity = resolved_entity_identity.evidence_identity();
        aspects.push(ForgeQueryAspectValue::new_set_evidence_identity(
            reference.aspect_path().to_string(),
            &resolved_entity_evidence_identity,
        )?);
    }
    Ok(aspects)
}

pub(super) fn combined_batch_mutation_receipt(
    receipts: &[ForgeQueryMutationReceipt],
) -> Result<ForgeQueryMutationReceipt, ForgeQueryRuntimeError> {
    let Some(last_receipt) = receipts.last() else {
        return Err(ForgeQueryRuntimeError::Workspace(
            ForgeQueryWorkspaceError::new("mutation batch must produce at least one write receipt"),
        ));
    };
    let commit_identity = ForgeQueryCommitIdentity::preview(
        crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::BatchWriteReceipt,
        )
        .field_identity_sequence(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("component_commit_identity"),
            receipts
                .iter()
                .map(|receipt| receipt.commit_identity.evidence_identity()),
        )
        .seal(),
    );
    let deltas = receipts
        .iter()
        .flat_map(|receipt| receipt.deltas.iter().cloned())
        .collect::<Vec<_>>();
    Ok(ForgeQueryMutationReceipt {
        commit_identity,
        snapshot_identity: last_receipt.snapshot_identity.clone(),
        deltas,
        bridge_authority: None,
    })
}

pub(super) fn classify_receipt_mutation_summary(
    receipt: &ForgeQueryMutationReceipt,
) -> (
    ForgeQueryMutationFamily,
    Option<String>,
    Option<ForgeQueryEntityIdentity>,
) {
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

pub(super) fn runtime_subscription_budget_policy(
) -> ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
    ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity::subscription_policy(
        [
            RUNTIME_SUBSCRIPTION_FAMILY_BUDGET_POLICY,
            RUNTIME_SUBSCRIPTION_SLICE_BUDGET_POLICY,
            RUNTIME_SUBSCRIPTION_BRIDGE_BUDGET_POLICY,
            RUNTIME_SUBSCRIPTION_ADMISSION_BUDGET_POLICY,
        ]
        .join(" / "),
    )
}

pub(super) fn runtime_active_lifecycle_budget_policy(
) -> ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
    ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity::active_lifecycle_policy(
        RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY,
    )
}

pub(super) fn runtime_consumer_attachment_budget_policy(
) -> ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity {
    ForgeQueryRuntimeLiveSubscriptionBudgetPolicyIdentity::consumer_attachment_policy(
        RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY,
    )
}

#[cfg(test)]
pub(super) fn runtime_subscription_budget_digest() -> String {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
        "runtime_live_subscription_budget_policy_v1",
    )
    .field_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("subscription_budget_policy"),
        runtime_subscription_budget_policy().evidence_identity(),
    )
    .field_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("active_lifecycle_budget_policy"),
        runtime_active_lifecycle_budget_policy().evidence_identity(),
    )
    .field_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("consumer_attachment_budget_policy"),
        runtime_consumer_attachment_budget_policy().evidence_identity(),
    )
    .seal()
    .as_str()
    .to_string()
}
