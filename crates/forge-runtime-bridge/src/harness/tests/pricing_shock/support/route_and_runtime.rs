use super::*;

pub(in crate::harness::tests::pricing_shock) fn pricing_mapping(
    component: &str,
    signal_scope: SignalInvalidationScope,
) -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new(format!(
            "pricing:{component}:{}",
            signal_scope.as_str().replace(':', "-")
        )),
        TruthPatchScope::for_entity_field(
            MappingSelector::exact(format!("component:{component}")),
            forge_foundational::facade::AspectKey::new("cost").expect("valid native aspect key"),
            forge_foundational::facade::FieldKey::new("usd".to_owned())
                .expect("valid native field key"),
        ),
        crate::snapshot::SnapshotReadContract::scalar(
            forge_foundational::facade::AspectKey::new("cost").expect("valid native aspect key"),
            forge_foundational::facade::ScalarAspectType::String,
        ),
        signal_scope,
        CoarseRoutingMode::Direct,
    )
}

pub(in crate::harness::tests::pricing_shock) fn pricing_patch(
    envelope_identity: crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity,
    component: &str,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        envelope_identity,
        vec![BridgeCommittedPatchItem::with_target(
            format!("component:{component}"),
            crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                forge_foundational::facade::AspectLocator::new(
                    forge_foundational::facade::LocatorAuthority::Authoritative,
                    forge_foundational::facade::AspectKey::new("cost")
                        .expect("valid bridge patch aspect key"),
                ),
                forge_foundational::facade::CanonicalFieldPath::single(
                    forge_foundational::facade::FieldKey::new("usd".to_owned())
                        .expect("valid foundational field key"),
                ),
            ),
        )],
    )
    .expect("pricing committed patch envelope should construct")
}

pub(in crate::harness::tests::pricing_shock) fn pricing_patch_items(
    envelope_identity: crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity,
    items: Vec<BridgeCommittedPatchItem>,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(envelope_identity, items)
        .expect("pricing committed patch envelope should construct")
}

pub(in crate::harness::tests::pricing_shock) fn pricing_patch_envelope_identity(
    branch_identity: TruthBranchIdentity,
    commit_identity: TruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
) -> crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity {
    crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
        commit_identity,
        patch_identity,
        snapshot_identity,
        branch_identity,
    )
}

pub(in crate::harness::tests::pricing_shock) fn build_pricing_runtime(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
) -> RuntimeBridge {
    build_pricing_runtime_with_policy(source, sink, BridgeRuntimePolicy::development())
}

pub(in crate::harness::tests::pricing_shock) fn build_pricing_runtime_with_policy(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    policy: BridgeRuntimePolicy,
) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new("price:bicycle"),
        ))
        .register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new("price:wheelbarrow"),
        ))
        .register_mapping(pricing_mapping(
            "rubber",
            SignalInvalidationScope::new("price:scooter"),
        ))
        .build()
        .expect("pricing runtime should build")
}

pub(in crate::harness::tests::pricing_shock) fn build_pricing_runtime_with_policy_and_writeback_authority<
    A,
>(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    policy: BridgeRuntimePolicy,
    authority: A,
) -> RuntimeBridge
where
    A: TruthWritebackAuthority,
{
    RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .with_writeback_authority(authority)
        .register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new("price:bicycle"),
        ))
        .register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new("price:wheelbarrow"),
        ))
        .register_mapping(pricing_mapping(
            "rubber",
            SignalInvalidationScope::new("price:scooter"),
        ))
        .build()
        .expect("pricing runtime with writeback authority should build")
}

pub(in crate::harness::tests::pricing_shock) fn build_pricing_runtime_with_aspects(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    policy: BridgeRuntimePolicy,
) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new("price:bicycle"),
        ))
        .register_mapping(pricing_mapping(
            "rubber",
            SignalInvalidationScope::new("price:scooter"),
        ))
        .register_aspect_mapping(pricing_field_aspect_registration("steel"))
        .register_aspect_mapping(pricing_field_aspect_registration("rubber"))
        .build()
        .expect("pricing runtime with aspects should build")
}

pub(in crate::harness::tests::pricing_shock) fn build_pricing_runtime_with_merge(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    policy: BridgeRuntimePolicy,
) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new("price:bicycle"),
        ))
        .register_mapping(pricing_mapping(
            "rubber",
            SignalInvalidationScope::new("price:scooter"),
        ))
        .register_aspect_mapping(pricing_field_aspect_registration("steel"))
        .register_aspect_mapping(pricing_field_aspect_registration("rubber"))
        .register_merge(pricing_merge_declaration())
        .register_merge(pricing_topology_denial_merge_declaration())
        .build()
        .expect("pricing runtime with merge should build")
}

pub(in crate::harness::tests::pricing_shock) fn build_high_fanout_pricing_runtime(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    steel_product_count: usize,
) -> RuntimeBridge {
    build_high_fanout_pricing_runtime_with_policy(
        source,
        sink,
        steel_product_count,
        BridgeRuntimePolicy::development(),
    )
}

pub(in crate::harness::tests::pricing_shock) fn build_high_fanout_pricing_runtime_with_policy(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    steel_product_count: usize,
    policy: BridgeRuntimePolicy,
) -> RuntimeBridge {
    let mut builder = RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new("price:product-000"),
        ));

    for product_idx in 1..steel_product_count {
        builder = builder.register_mapping(pricing_mapping(
            "steel",
            SignalInvalidationScope::new(format!("price:product-{product_idx:03}")),
        ));
    }

    builder
        .register_mapping(pricing_mapping(
            "rubber",
            SignalInvalidationScope::new("price:scooter"),
        ))
        .build()
        .expect("high-fanout pricing runtime should build")
}

pub(in crate::harness::tests::pricing_shock) fn pricing_preview_declaration(
) -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new("pricing:preview-declaration"),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new("pricing:binding"),
            crate::truth_identity_fixtures::truth_branch_fixture("pricing-shock"),
            BridgeSignalBranchIdentity::new("signal:pricing-shock"),
        ),
        crate::facade::BridgePreviewSessionBasis::new(
            BridgeTruthViewSelector::branch_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("pricing-shock"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot:pricing-shock"),
            ),
            BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ]),
            crate::facade::BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        ),
    )
}

pub(in crate::harness::tests::pricing_shock) fn pricing_merge_declaration(
) -> MergeHistoryDeclaration {
    MergeHistoryDeclaration::new(
        MergeHistoryDeclarationIdentity::new("merge:pricing-shock"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        BridgeMergeAuthorityBasis::new(
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            "merge-artifact:pricing-shock",
            "rel-merge-v1",
            "schema-policy-v1",
            BridgeMergeParentOrderProof::new(vec![
                crate::truth_identity_fixtures::truth_commit_fixture("commit:rubber-main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit:rubber-shock"),
            ]),
        ),
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent)
}

pub(in crate::harness::tests::pricing_shock) fn pricing_topology_denial_merge_declaration(
) -> MergeHistoryDeclaration {
    MergeHistoryDeclaration::new(
        MergeHistoryDeclarationIdentity::new("merge:pricing-topology-denial"),
        BridgeMergeConsumptionClass::TopologyRewireMerge,
        BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        BridgeMergeAuthorityBasis::new(
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            "merge-artifact:pricing-topology-denial",
            "rel-merge-v1",
            "schema-policy-v1",
            BridgeMergeParentOrderProof::new(vec![
                crate::truth_identity_fixtures::truth_commit_fixture("commit:rubber-main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit:rubber-shock"),
            ]),
        ),
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent)
}

pub(in crate::harness::tests::pricing_shock) fn pricing_component_read_packet(
    component: &str,
) -> SnapshotReadPacket {
    SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
        format!("component:{component}"),
        crate::snapshot::SnapshotReadContract::scalar(
            forge_foundational::facade::AspectKey::new("cost").expect("valid snapshot aspect key"),
            forge_foundational::facade::ScalarAspectType::String,
        ),
    )])
}

pub(in crate::harness::tests::pricing_shock) fn pricing_provenance_read_request(
    component: &str,
    field: &str,
) -> SnapshotReadRequest {
    SnapshotReadRequest::for_coarse(
        format!("component:{component}"),
        crate::snapshot::SnapshotReadContract::scalar(
            forge_foundational::facade::AspectKey::new(format!("provenance:{field}"))
                .expect("valid provenance aspect key"),
            forge_foundational::facade::ScalarAspectType::String,
        ),
    )
}

pub(in crate::harness::tests::pricing_shock) fn pricing_provenance_read_packet(
    component: &str,
) -> SnapshotReadPacket {
    SnapshotReadPacket::new(vec![
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("provenance:regime")
                    .expect("valid snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("provenance:external-factor")
                    .expect("valid snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("provenance:factor-delta")
                    .expect("valid snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("provenance:trend-delta")
                    .expect("valid snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("provenance:jump-delta")
                    .expect("valid snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("provenance:shock-delta")
                    .expect("valid snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("provenance:shock-multiplier")
                    .expect("valid snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
    ])
}
