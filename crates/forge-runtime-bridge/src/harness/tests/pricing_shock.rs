use super::pricing_support::*;
use super::pricing_domain::{
    MaterialPriceAttribution, MaterialTickWave, PricingCommitAttribution, PricingDomainWorld,
    PricingMaterial, ProductPriceBreakdown,
};
use std::collections::BTreeMap;
use forge_harness::facade::{FeedStreamEventKind, FeedVolatilityRegime};
use crate::adapter::{
    TruthWritebackAuthority, TruthWritebackAuthorityError, TruthWritebackReceipt,
    TruthWritebackRequest,
};
use crate::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyDeclaration,
    BridgePolicyDeclarationIdentity,
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeCommittedPatchItem,
    BridgeFailureClass, BridgeMappingId, BridgeMappingRegistration,
    BridgePreviewResidueClass, BridgePreviewSessionDeclaration,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, BridgeReplayErrorKind,
    BridgeRequestKind, BridgeRuntimePolicy, BridgeSignalBranchIdentity,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity,
    BridgeSpeculativePromotionRequest, BridgeSpeculativeSessionRequest,
    BridgeStandardRouteError, BridgeTruthViewEvaluationRequest, CoarseRoutingMode,
    FineGrainedMatchStatus, MappingSelector, SliceFallbackPolicy, SubscriptionSliceKind,
    RawCommittedPatchEnvelope, RuntimeBridge, RuntimeBridgeBuilder, SignalInvalidationScope,
    SnapshotReadRecord, TruthBranchIdentity, TruthCommitIdentity, TruthDeltaSurfaceKind,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
    BridgeWritebackCausalityBasis, BridgeWritebackCausalityIdentity,
    BridgeWritebackDeclaration, BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass,
    BridgeWritebackEffectIdentity, BridgeWritebackErrorKind, BridgeWritebackFailureClass,
    BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackRequestMode,
    BridgeWritebackStrategyClass,
    BridgeMergeAuthorityBasis, BridgeMergeAuthorityBasisKind, BridgeMergeConsumptionClass,
    BridgeMergeOntologyMappingSurface, BridgeMergeParentOrderProof,
    BridgeMergeStructuralAdvisoryDisposition, MergeHistoryDeclaration,
    MergeHistoryDeclarationIdentity,
};
use crate::harness::fixtures::{
    InMemoryRelationalBridgeSource, RecordingSignalBridgeSink, RecordingTruthWritebackAuthority,
    SnapshotFixture,
};
use crate::error::BridgeDeliveryErrorKind;
use crate::snapshot::{SnapshotReadPacket, SnapshotReadRequest};
use crate::speculation::BridgePreviewLifecycleStateKind;
use serde_json::json;

#[derive(Clone)]
struct RejectingPricingWritebackAuthority {
    failure_class: BridgeWritebackFailureClass,
}

#[derive(Clone)]
struct GeneratedPricingScenario {
    main_snapshot: SnapshotFixture,
    speculative_snapshot: SnapshotFixture,
    live_main_snapshot: SnapshotFixture,
    interleaved_main_snapshot: SnapshotFixture,
    fanout_first_snapshot: SnapshotFixture,
    fanout_second_snapshot: SnapshotFixture,
    main_steel_cost: i64,
    main_rubber_cost: i64,
    speculative_rubber_cost: i64,
    live_main_steel_cost: i64,
    interleaved_main_rubber_cost: i64,
    fanout_second_steel_cost: i64,
    main_portfolio: Vec<ProductPriceBreakdown>,
    speculative_portfolio: Vec<ProductPriceBreakdown>,
    crisis_portfolio: Vec<ProductPriceBreakdown>,
    main_material_prices: BTreeMap<PricingMaterial, i64>,
    crisis_overrides: BTreeMap<PricingMaterial, i64>,
    crisis_family_tariff_bps: BTreeMap<String, i64>,
    commit_attributions: BTreeMap<String, PricingCommitAttribution>,
}

fn generated_pricing_scenario() -> GeneratedPricingScenario {
    let mut world = PricingDomainWorld::new(1_337);
    let _warmup_wave = world.advance_material_streams();
    let main_wave = world.advance_material_streams();

    let main_steel_cost = world.current_material_price_microunits(PricingMaterial::Steel);
    let main_rubber_cost = world.current_material_price_microunits(PricingMaterial::Rubber);
    let speculative_rubber_cost =
        world.shocked_material_price_microunits(PricingMaterial::Rubber, 4_000);
    let crisis_overrides = BTreeMap::from([
        (
            PricingMaterial::Steel,
            world.shocked_material_price_microunits(PricingMaterial::Steel, 1_450),
        ),
        (
            PricingMaterial::Rubber,
            world.shocked_material_price_microunits(PricingMaterial::Rubber, 4_000),
        ),
        (
            PricingMaterial::Fuel,
            world.shocked_material_price_microunits(PricingMaterial::Fuel, 2_800),
        ),
        (
            PricingMaterial::Copper,
            world.shocked_material_price_microunits(PricingMaterial::Copper, 1_650),
        ),
        (
            PricingMaterial::Electronics,
            world.shocked_material_price_microunits(PricingMaterial::Electronics, 1_250),
        ),
    ]);
    let crisis_family_tariff_bps = BTreeMap::from([
        ("washer".to_owned(), 650),
        ("dryer".to_owned(), 650),
        ("e-bike".to_owned(), 420),
    ]);
    let main_portfolio = world.price_matrix();
    let main_material_prices = BTreeMap::from([
        (PricingMaterial::Steel, world.current_material_price_microunits(PricingMaterial::Steel)),
        (
            PricingMaterial::Aluminum,
            world.current_material_price_microunits(PricingMaterial::Aluminum),
        ),
        (PricingMaterial::Copper, world.current_material_price_microunits(PricingMaterial::Copper)),
        (PricingMaterial::Rubber, world.current_material_price_microunits(PricingMaterial::Rubber)),
        (
            PricingMaterial::PlasticResin,
            world.current_material_price_microunits(PricingMaterial::PlasticResin),
        ),
        (
            PricingMaterial::Electronics,
            world.current_material_price_microunits(PricingMaterial::Electronics),
        ),
        (
            PricingMaterial::Packaging,
            world.current_material_price_microunits(PricingMaterial::Packaging),
        ),
        (PricingMaterial::Labor, world.current_material_price_microunits(PricingMaterial::Labor)),
        (PricingMaterial::Fuel, world.current_material_price_microunits(PricingMaterial::Fuel)),
    ]);
    let speculative_portfolio =
        world.price_matrix_with_overrides([(PricingMaterial::Rubber, speculative_rubber_cost)]);
    let crisis_portfolio =
        world.price_matrix_with_scenario(crisis_overrides.clone(), crisis_family_tariff_bps.clone());

    let main_snapshot_base = world.snapshot_fixture("snapshot:pricing-main");
    let speculative_snapshot_base = world.snapshot_fixture_with_overrides(
        "snapshot:pricing-shock",
        [(PricingMaterial::Rubber, speculative_rubber_cost)],
    );
    let fanout_first_snapshot_base = world.snapshot_fixture("snapshot:pricing-fanout-a");
    let mut commit_attributions = BTreeMap::new();
    commit_attributions.insert(
        "commit:steel-main".to_owned(),
        pricing_commit_attribution(
            &world,
            "commit:steel-main",
            "snapshot:pricing-main",
            "main",
            PricingMaterial::Steel,
            attribution_for(&main_wave, PricingMaterial::Steel),
            0,
            1000,
            "bicycle-000",
        ),
    );
    commit_attributions.insert(
        "commit:rubber-main".to_owned(),
        pricing_commit_attribution(
            &world,
            "commit:rubber-main",
            "snapshot:pricing-main",
            "main",
            PricingMaterial::Rubber,
            attribution_for(&main_wave, PricingMaterial::Rubber),
            0,
            1000,
            "scooter-001",
        ),
    );
    commit_attributions.insert(
        "commit:rubber-shock".to_owned(),
        pricing_commit_attribution(
            &world,
            "commit:rubber-shock",
            "snapshot:pricing-shock",
            "pricing-shock",
            PricingMaterial::Rubber,
            attribution_for(&main_wave, PricingMaterial::Rubber),
            speculative_rubber_cost - main_rubber_cost,
            4000,
            "scooter-001",
        ),
    );

    let interleaved_wave = world.advance_material_streams();
    let live_main_steel_cost = world.current_material_price_microunits(PricingMaterial::Steel);
    let interleaved_main_rubber_cost = world.current_material_price_microunits(PricingMaterial::Rubber);
    let live_main_snapshot_base = world.snapshot_fixture("snapshot:pricing-main-live");
    let interleaved_main_snapshot_base = world.snapshot_fixture("snapshot:pricing-main-interleaved");
    let fanout_second_snapshot_base = world.snapshot_fixture("snapshot:pricing-fanout-b");
    let fanout_second_steel_cost = live_main_steel_cost;
    commit_attributions.insert(
        "commit:steel-main-live".to_owned(),
        pricing_commit_attribution(
            &world,
            "commit:steel-main-live",
            "snapshot:pricing-main-live",
            "main",
            PricingMaterial::Steel,
            attribution_for(&interleaved_wave, PricingMaterial::Steel),
            0,
            1000,
            "bicycle-000",
        ),
    );
    commit_attributions.insert(
        "commit:rubber-main-interleaved".to_owned(),
        pricing_commit_attribution(
            &world,
            "commit:rubber-main-interleaved",
            "snapshot:pricing-main-interleaved",
            "main",
            PricingMaterial::Rubber,
            attribution_for(&interleaved_wave, PricingMaterial::Rubber),
            0,
            1000,
            "scooter-001",
        ),
    );
    commit_attributions.insert(
        "commit:steel-fanout-b".to_owned(),
        pricing_commit_attribution(
            &world,
            "commit:steel-fanout-b",
            "snapshot:pricing-fanout-b",
            "main",
            PricingMaterial::Steel,
            attribution_for(&interleaved_wave, PricingMaterial::Steel),
            0,
            1000,
            "bicycle-000",
        ),
    );
    let main_snapshot = snapshot_with_provenance(
        &main_snapshot_base,
        &[
            commit_attributions
                .get("commit:steel-main")
                .expect("steel main attribution should exist"),
            commit_attributions
                .get("commit:rubber-main")
                .expect("rubber main attribution should exist"),
        ],
    );
    let speculative_snapshot = snapshot_with_provenance(
        &speculative_snapshot_base,
        &[commit_attributions
            .get("commit:rubber-shock")
            .expect("rubber shock attribution should exist")],
    );
    let fanout_first_snapshot = snapshot_with_provenance(
        &fanout_first_snapshot_base,
        &[commit_attributions
            .get("commit:steel-main")
            .expect("steel main attribution should exist")],
    );
    let live_main_snapshot = snapshot_with_provenance(
        &live_main_snapshot_base,
        &[commit_attributions
            .get("commit:steel-main-live")
            .expect("steel live attribution should exist")],
    );
    let interleaved_main_snapshot = snapshot_with_provenance(
        &interleaved_main_snapshot_base,
        &[commit_attributions
            .get("commit:rubber-main-interleaved")
            .expect("rubber interleaved attribution should exist")],
    );
    let fanout_second_snapshot = snapshot_with_provenance(
        &fanout_second_snapshot_base,
        &[commit_attributions
            .get("commit:steel-fanout-b")
            .expect("steel fanout attribution should exist")],
    );

    GeneratedPricingScenario {
        main_snapshot,
        speculative_snapshot,
        live_main_snapshot,
        interleaved_main_snapshot,
        fanout_first_snapshot,
        fanout_second_snapshot,
        main_steel_cost,
        main_rubber_cost,
        speculative_rubber_cost,
        live_main_steel_cost,
        interleaved_main_rubber_cost,
        fanout_second_steel_cost,
        main_portfolio,
        speculative_portfolio,
        crisis_portfolio,
        main_material_prices,
        crisis_overrides,
        crisis_family_tariff_bps,
        commit_attributions,
    }
}

fn attribution_for(wave: &MaterialTickWave, material: PricingMaterial) -> MaterialPriceAttribution {
    wave.changed_materials
        .iter()
        .find(|tick| tick.material == material)
        .expect("requested material attribution should exist in generated wave")
        .attribution
        .clone()
}

fn pricing_commit_attribution(
    world: &PricingDomainWorld,
    commit_identity: &str,
    snapshot_identity: &str,
    branch_identity: &str,
    material: PricingMaterial,
    material_attribution: MaterialPriceAttribution,
    shock_delta_microunits: i64,
    shock_multiplier_per_mille: i64,
    representative_sku: &str,
) -> PricingCommitAttribution {
    PricingCommitAttribution {
        commit_identity: commit_identity.to_owned(),
        snapshot_identity: snapshot_identity.to_owned(),
        branch_identity: branch_identity.to_owned(),
        material,
        material_attribution,
        shock_delta_microunits,
        shock_multiplier_per_mille,
        representative_product: world.explain_product_price(representative_sku),
    }
}

impl TruthWritebackAuthority for RejectingPricingWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(TruthWritebackReceipt::new_with_failure_class(
            crate::facade::BridgeWritebackOutcomeClass::Rejected,
            Some(self.failure_class),
            format!("authoritative-rejection:{}", request.digest()),
            &request,
        ))
    }
}

fn pricing_mapping(
    component: &str,
    signal_target: impl Into<String>,
) -> BridgeMappingRegistration {
    let signal_target = signal_target.into();
    BridgeMappingRegistration::new(
        BridgeMappingId::new(format!(
            "pricing:{component}:{}",
            signal_target.replace(':', "-")
        )),
        TruthPatchScope::new(
            MappingSelector::exact(format!("component:{component}")),
            MappingSelector::exact("cost"),
            MappingSelector::exact("usd"),
        ),
        SignalInvalidationScope::new(signal_target),
        CoarseRoutingMode::Direct,
    )
}

fn pricing_patch(
    branch: &str,
    commit: &str,
    patch: &str,
    snapshot: &str,
    component: &str,
) -> RawCommittedPatchEnvelope {
    RawCommittedPatchEnvelope::new(
        TruthCommitIdentity::new(commit),
        TruthPatchIdentity::new(patch),
        TruthSnapshotIdentity::new(snapshot),
        TruthBranchIdentity::new(branch),
        vec![BridgeCommittedPatchItem::new(
            format!("component:{component}"),
            "cost",
            "usd",
        )],
    )
}

fn pricing_patch_items(
    branch: &str,
    commit: &str,
    patch: &str,
    snapshot: &str,
    items: Vec<BridgeCommittedPatchItem>,
) -> RawCommittedPatchEnvelope {
    RawCommittedPatchEnvelope::new(
        TruthCommitIdentity::new(commit),
        TruthPatchIdentity::new(patch),
        TruthSnapshotIdentity::new(snapshot),
        TruthBranchIdentity::new(branch),
        items,
    )
}

fn pricing_snapshot(snapshot: &str, steel_cost: &str, rubber_cost: &str) -> SnapshotFixture {
    SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot),
        vec![
            SnapshotReadRecord::new("component:steel:cost", steel_cost.as_bytes().to_vec()),
            SnapshotReadRecord::new("component:rubber:cost", rubber_cost.as_bytes().to_vec()),
        ],
    )
}

fn pricing_aspect_snapshot(snapshot: &str, steel_cost: &str, rubber_cost: &str) -> SnapshotFixture {
    SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot),
        vec![
            SnapshotReadRecord::new(
                "component:steel:cost:signal-field:usd",
                steel_cost.as_bytes().to_vec(),
            ),
            SnapshotReadRecord::new(
                "component:rubber:cost:signal-field:usd",
                rubber_cost.as_bytes().to_vec(),
            ),
        ],
    )
}

fn build_pricing_runtime(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
) -> RuntimeBridge {
    build_pricing_runtime_with_policy(source, sink, BridgeRuntimePolicy::development())
}

fn build_pricing_runtime_with_policy(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    policy: BridgeRuntimePolicy,
) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .register_mapping(pricing_mapping("steel", "price:bicycle"))
        .register_mapping(pricing_mapping("steel", "price:wheelbarrow"))
        .register_mapping(pricing_mapping("rubber", "price:scooter"))
        .build()
        .expect("pricing runtime should build")
}

fn build_pricing_runtime_with_policy_and_writeback_authority<A>(
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
        .register_mapping(pricing_mapping("steel", "price:bicycle"))
        .register_mapping(pricing_mapping("steel", "price:wheelbarrow"))
        .register_mapping(pricing_mapping("rubber", "price:scooter"))
        .build()
        .expect("pricing runtime with writeback authority should build")
}

fn build_pricing_runtime_with_aspects(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    policy: BridgeRuntimePolicy,
) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .register_mapping(pricing_mapping("steel", "price:bicycle"))
        .register_mapping(pricing_mapping("rubber", "price:scooter"))
        .register_aspect_mapping(pricing_field_aspect_registration("steel"))
        .register_aspect_mapping(pricing_field_aspect_registration("rubber"))
        .build()
        .expect("pricing runtime with aspects should build")
}

fn build_pricing_runtime_with_merge(
    source: InMemoryRelationalBridgeSource,
    sink: RecordingSignalBridgeSink,
    policy: BridgeRuntimePolicy,
) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_truth_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_compute_sink(sink)
        .with_policy(policy)
        .register_mapping(pricing_mapping("steel", "price:bicycle"))
        .register_mapping(pricing_mapping("rubber", "price:scooter"))
        .register_aspect_mapping(pricing_field_aspect_registration("steel"))
        .register_aspect_mapping(pricing_field_aspect_registration("rubber"))
        .register_merge(pricing_merge_declaration())
        .register_merge(pricing_topology_denial_merge_declaration())
        .build()
        .expect("pricing runtime with merge should build")
}

fn build_high_fanout_pricing_runtime(
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

fn build_high_fanout_pricing_runtime_with_policy(
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
        .register_mapping(pricing_mapping("steel", "price:product-000"));

    for product_idx in 1..steel_product_count {
        builder = builder.register_mapping(pricing_mapping(
            "steel",
            format!("price:product-{product_idx:03}"),
        ));
    }

    builder
        .register_mapping(pricing_mapping("rubber", "price:scooter"))
        .build()
        .expect("high-fanout pricing runtime should build")
}

fn pricing_preview_declaration() -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new("pricing:preview-declaration"),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new("pricing:binding"),
            TruthBranchIdentity::new("pricing-shock"),
            BridgeSignalBranchIdentity::new("signal:pricing-shock"),
        ),
        "truth-view:pricing-shock",
        "source-capability:pricing",
        "request-shape:pricing-shock",
        "artifact-schema:pricing-shock",
    )
}

fn pricing_merge_declaration() -> MergeHistoryDeclaration {
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
                TruthCommitIdentity::new("commit:rubber-main"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            ]),
        ),
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent)
}

fn pricing_topology_denial_merge_declaration() -> MergeHistoryDeclaration {
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
                TruthCommitIdentity::new("commit:rubber-main"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            ]),
        ),
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent)
}

fn pricing_writeback_declaration(
    declaration_identity: &str,
    request_kind: BridgeRequestKind,
    request_mode: BridgeWritebackRequestMode,
    strategy_descriptor_digest: &str,
) -> BridgeWritebackDeclaration {
    match request_mode {
        BridgeWritebackRequestMode::ReadOnly => BridgeWritebackDeclaration::read_only(
            BridgeWritebackDeclarationIdentity::new(declaration_identity),
            request_kind,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ),
        BridgeWritebackRequestMode::WritebackCapable => BridgeWritebackDeclaration::writeback_capable(
            BridgeWritebackDeclarationIdentity::new(declaration_identity),
            request_kind,
            BridgeWritebackFamilyKind::ProjectedStateDiff,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
            strategy_descriptor_digest,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ),
    }
}

fn pricing_lowered_policy(
    runtime: &RuntimeBridge,
) -> crate::facade::LoweredBridgeExecutionPolicy {
    let policy_contract = runtime
        .admit_policy_declaration(BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:pricing-writeback"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ))
        .expect("pricing writeback policy should admit");
    runtime.lower_admitted_policy(&policy_contract)
}

fn pricing_writeback_causality_basis(
    identity: &str,
    truth_trigger_digest: &str,
) -> BridgeWritebackCausalityBasis {
    BridgeWritebackCausalityBasis::new(
        BridgeWritebackCausalityIdentity::new(identity),
        truth_trigger_digest,
        "route:sha256:pricing",
        "evaluation:sha256:pricing",
        "truth-view:sha256:pricing",
    )
}

fn pricing_component_read_packet(component: &str) -> SnapshotReadPacket {
    SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
        format!("component:{component}"),
        "cost",
    )])
}

fn pricing_provenance_record_key(component: &str, field: &str) -> String {
    format!("component:{component}:provenance:{field}")
}

fn pricing_provenance_read_packet(component: &str) -> SnapshotReadPacket {
    SnapshotReadPacket::new(vec![
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            "provenance:regime",
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            "provenance:external-factor",
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            "provenance:factor-delta",
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            "provenance:trend-delta",
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            "provenance:jump-delta",
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            "provenance:shock-delta",
        ),
        SnapshotReadRequest::for_coarse(
            format!("component:{component}"),
            "provenance:shock-multiplier",
        ),
    ])
}

fn snapshot_with_provenance(
    snapshot: &SnapshotFixture,
    attributions: &[&PricingCommitAttribution],
) -> SnapshotFixture {
    let mut records = snapshot.records().to_vec();
    for attribution in attributions {
        let component = attribution.material.key();
        records.push(SnapshotReadRecord::new(
            pricing_provenance_record_key(component, "regime"),
            format!("{:?}", attribution.material_attribution.regime).into_bytes(),
        ));
        records.push(SnapshotReadRecord::new(
            pricing_provenance_record_key(component, "external-factor"),
            attribution
                .material_attribution
                .external_factor_microunits
                .to_string()
                .into_bytes(),
        ));
        records.push(SnapshotReadRecord::new(
            pricing_provenance_record_key(component, "factor-delta"),
            attribution
                .material_attribution
                .factor_delta_microunits
                .to_string()
                .into_bytes(),
        ));
        records.push(SnapshotReadRecord::new(
            pricing_provenance_record_key(component, "trend-delta"),
            attribution
                .material_attribution
                .trend_delta_microunits
                .to_string()
                .into_bytes(),
        ));
        records.push(SnapshotReadRecord::new(
            pricing_provenance_record_key(component, "jump-delta"),
            attribution
                .material_attribution
                .jump_delta_microunits
                .to_string()
                .into_bytes(),
        ));
        records.push(SnapshotReadRecord::new(
            pricing_provenance_record_key(component, "shock-delta"),
            attribution.shock_delta_microunits.to_string().into_bytes(),
        ));
        records.push(SnapshotReadRecord::new(
            pricing_provenance_record_key(component, "shock-multiplier"),
            attribution
                .shock_multiplier_per_mille
                .to_string()
                .into_bytes(),
        ));
    }
    SnapshotFixture::new(snapshot.identity().clone(), records)
        .with_read_result_identity(snapshot.read_result_identity().clone())
}

fn snapshot_with_corrupted_provenance_field(
    snapshot: &SnapshotFixture,
    component: &str,
    field: &str,
    payload: impl Into<Vec<u8>>,
) -> SnapshotFixture {
    let target_key = pricing_provenance_record_key(component, field);
    let replacement_payload = payload.into();
    let mut replaced = false;
    let mut records = Vec::with_capacity(snapshot.records().len());
    for record in snapshot.records() {
        if record.request_key() == target_key {
            records.push(SnapshotReadRecord::new(
                target_key.clone(),
                replacement_payload.clone(),
            ));
            replaced = true;
        } else {
            records.push(record.clone());
        }
    }
    assert!(
        replaced,
        "provenance record `{target_key}` should exist before corruption"
    );
    SnapshotFixture::new(snapshot.identity().clone(), records)
        .with_read_result_identity(snapshot.read_result_identity().clone())
}

fn read_single_payload(
    evaluation: &crate::facade::BridgeTruthViewEvaluation,
) -> String {
    let reads = evaluation
        .observation()
        .read_planned_packet()
        .expect("truth-view read packet should materialize");
    std::str::from_utf8(reads.records()[0].payload())
        .expect("pricing payload should be utf8")
        .to_owned()
}

fn read_single_money_cents(
    evaluation: &crate::facade::BridgeTruthViewEvaluation,
) -> i64 {
    read_single_payload(evaluation)
        .parse::<i64>()
        .expect("pricing payload should be parseable as integer cents")
}

fn read_packet_payloads(
    evaluation: &crate::facade::BridgeTruthViewEvaluation,
) -> Vec<String> {
    evaluation
        .observation()
        .read_planned_packet()
        .expect("truth-view read packet should materialize")
        .records()
        .iter()
        .map(|record| {
            std::str::from_utf8(record.payload())
                .expect("pricing payload should be utf8")
                .to_owned()
        })
        .collect()
}

fn pricing_field_aspect_registration(component: &str) -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new(format!("pricing-{component}-usd-field")),
        TruthPatchScope::new(
            MappingSelector::exact(format!("component:{component}")),
            MappingSelector::exact("cost"),
            MappingSelector::exact("usd"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::Disallow,
    )
}

fn pricing_reference_source() -> InMemoryRelationalBridgeSource {
    let scenario = generated_pricing_scenario();

    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-main",
        "patch:steel-main",
        "snapshot:pricing-main",
        "steel",
    ));
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:rubber-main",
        "patch:rubber-main",
        "snapshot:pricing-main",
        "rubber",
    ));
    source.insert_committed_patch(pricing_patch(
        "pricing-shock",
        "commit:rubber-shock",
        "patch:rubber-shock",
        "snapshot:pricing-shock",
        "rubber",
    ));
    source.insert_snapshot(scenario.main_snapshot);
    source.insert_snapshot(scenario.speculative_snapshot);
    source
}

fn pricing_reference_source_with_corrupted_shock_provenance(
    field: &str,
    payload: impl Into<Vec<u8>>,
) -> InMemoryRelationalBridgeSource {
    let scenario = generated_pricing_scenario();
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-main",
        "patch:steel-main",
        "snapshot:pricing-main",
        "steel",
    ));
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:rubber-main",
        "patch:rubber-main",
        "snapshot:pricing-main",
        "rubber",
    ));
    source.insert_committed_patch(pricing_patch(
        "pricing-shock",
        "commit:rubber-shock",
        "patch:rubber-shock",
        "snapshot:pricing-shock",
        "rubber",
    ));
    source.insert_snapshot(scenario.main_snapshot);
    source.insert_snapshot(snapshot_with_corrupted_provenance_field(
        &scenario.speculative_snapshot,
        "rubber",
        field,
        payload,
    ));
    source
}

fn pricing_reference_source_with_conflicting_shock_snapshot() -> InMemoryRelationalBridgeSource {
    let scenario = generated_pricing_scenario();
    let source = pricing_reference_source();
    source.insert_committed_patch(pricing_patch(
        "pricing-shock",
        "commit:rubber-shock",
        "patch:rubber-shock-conflicting",
        "snapshot:pricing-main",
        "rubber",
    ));
    source.insert_snapshot(scenario.main_snapshot);
    source.insert_snapshot(scenario.speculative_snapshot);
    source
}

fn pricing_reference_source_with_conflicting_commit_identity_for_route() -> InMemoryRelationalBridgeSource {
    let source = pricing_reference_source();
    source.insert_committed_patch(pricing_patch_items(
        "main",
        "commit:steel-main",
        "patch:steel-main-conflicting-meaning",
        "snapshot:pricing-main",
        vec![BridgeCommittedPatchItem::new(
            "component:rubber",
            "cost",
            "usd",
        )],
    ));
    source
}

fn pricing_reference_source_with_branch_head_pointing_to(
    branch: &str,
    commit: &str,
) -> InMemoryRelationalBridgeSource {
    let source = pricing_reference_source();
    source.set_branch_head(&TruthBranchIdentity::new(branch), &TruthCommitIdentity::new(commit));
    source
}

fn pricing_reference_source_with_missing_branch_head_snapshot(
    branch: &str,
    commit: &str,
    snapshot: &str,
    component: &str,
) -> InMemoryRelationalBridgeSource {
    let source = pricing_reference_source();
    source.insert_committed_patch(pricing_patch(branch, commit, "patch:missing-snapshot", snapshot, component));
    source.set_branch_head(&TruthBranchIdentity::new(branch), &TruthCommitIdentity::new(commit));
    source
}

fn pricing_merge_source() -> InMemoryRelationalBridgeSource {
    let scenario = generated_pricing_scenario();
    let source = pricing_reference_source();
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:pricing-merged",
        "patch:pricing-merged",
        "snapshot:pricing-merged",
        "rubber",
    ));
    source.insert_snapshot(pricing_snapshot(
        "snapshot:pricing-merged",
        &scenario.main_steel_cost.to_string(),
        &scenario.speculative_rubber_cost.to_string(),
    ));
    source.insert_snapshot(pricing_aspect_snapshot(
        "snapshot:pricing-merged-aspect",
        &scenario.main_steel_cost.to_string(),
        &scenario.speculative_rubber_cost.to_string(),
    ));
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:pricing-merged-aspect",
        "patch:pricing-merged-aspect",
        "snapshot:pricing-merged-aspect",
        "rubber",
    ));
    source
}

fn capture_pricing_reference_bundle(
    runtime: &RuntimeBridge,
    preview_session_identity: &str,
) -> PricingReferenceBundle {
    let route = runtime
        .route("commit:steel-main")
        .expect("pricing reference route should succeed");
    let route_record = runtime
        .diagnostics()
        .route_record_for_source_commit("commit:steel-main")
        .expect("pricing reference route should retain its route record");
    let main_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new("main"))
                .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("pricing main evaluation should succeed");
    let comparison = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new(preview_session_identity),
            pricing_preview_declaration(),
            4,
            2,
            2,
        ))
        .expect("pricing reference preview should activate")
        .compare_to_main();
    let speculative_eval = runtime
        .evaluate(
            comparison
                .speculative_evaluation_request()
                .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("pricing speculative evaluation should succeed");

    PricingReferenceBundle {
        source_branch: route_record.source_branch().as_str().to_owned(),
        source_commit: route_record.source_commit().as_str().to_owned(),
        route_snapshot: route.result().receipt().snapshot_identity().as_str().to_owned(),
        delivered_target_count: route.result().receipt().delivered_target_count(),
        route_entry_count: route_record.entries().len(),
        evaluation_record_identity: main_eval.record().record_identity().as_str().to_owned(),
        evaluation_selector_identity: main_eval
            .record()
            .decision_log()
            .selector_identity()
            .as_str()
            .to_owned(),
        main_snapshot: main_eval.snapshot_identity().as_str().to_owned(),
        main_rubber_cost_cents: read_single_money_cents(&main_eval),
        speculative_truth_branch: comparison.truth_branch_identity().as_str().to_owned(),
        speculative_signal_branch: comparison.signal_branch_identity().as_str().to_owned(),
        speculative_snapshot: speculative_eval.snapshot_identity().as_str().to_owned(),
        speculative_rubber_cost_cents: read_single_money_cents(&speculative_eval),
    }
}

fn capture_pricing_aspect_bundle(policy: BridgeRuntimePolicy) -> PricingAspectBundle {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-aspect",
        "patch:steel-aspect",
        "snapshot:pricing-aspect",
        "steel",
    ));
    source.insert_snapshot(pricing_aspect_snapshot("snapshot:pricing-aspect", "145", "40"));

    let runtime = build_pricing_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        policy,
    );
    runtime
        .route("commit:steel-aspect")
        .expect("aspect-aware pricing route should succeed");

    let route_record = runtime
        .diagnostics()
        .route_record_for_source_commit("commit:steel-aspect")
        .expect("aspect-aware pricing route should retain a route record");
    let explanation = runtime
        .diagnostics()
        .explain_route(route_record.route_identity().as_str())
        .expect("aspect-aware pricing route should be explainable");
    let entry = &explanation.route_entries()[0];

    PricingAspectBundle {
        route_identity: explanation.route_identity().as_str().to_owned(),
        snapshot: explanation.snapshot_identity().as_str().to_owned(),
        source_branch: route_record.source_branch().as_str().to_owned(),
        source_commit: route_record.source_commit().as_str().to_owned(),
        truth_surface_kind: format!("{:?}", entry.truth_surface_kind()),
        fine_grained_match_status: format!("{:?}", entry.fine_grained_match_status()),
        aspect_registration_id: entry
            .aspect_registration_id()
            .expect("aspect-aware route entry should retain the aspect registration id")
            .as_str()
            .to_owned(),
        subscription_slice_kind: format!(
            "{:?}",
            entry
                .subscription_slice_kind()
                .expect("aspect-aware route entry should retain the subscription slice kind")
        ),
        surface_label: entry.surface_label().to_owned(),
        invalidation_target: explanation.invalidation_targets()[0]
            .signal_scope()
            .to_owned(),
    }
}

fn capture_pricing_missing_snapshot_failure_bundle(
    runtime: &RuntimeBridge,
) -> PricingFailureBundle {
    let error = runtime
        .route("commit:steel-missing-snapshot")
        .expect_err("pricing route should fail when the source snapshot is absent");
    let error_kind = match error {
        BridgeStandardRouteError::Delivery(error) => error.kind(),
        BridgeStandardRouteError::Route(error) => {
            panic!("missing snapshot should fail at delivery, not route planning: {error}")
        }
    };

    let retained_failure = runtime
        .diagnostics()
        .last_failure_record()
        .expect("pricing failure should be retained in diagnostics");

    PricingFailureBundle {
        error_kind,
        failure_class: retained_failure.failure_class().clone(),
        source_commit: retained_failure.source_commit().as_str().to_owned(),
        source_snapshot: retained_failure.source_snapshot().as_str().to_owned(),
    }
}

fn capture_pricing_replay_bundle(runtime: &RuntimeBridge) -> PricingReplayBundle {
    runtime
        .route("commit:steel-main")
        .expect("pricing replay control route should succeed");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("pricing route should retain a canonical replay record");
    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("pricing route replay should preserve canonical main-branch truth");

    PricingReplayBundle {
        source_commit: replay.source_commit().as_str().to_owned(),
        source_snapshot: replay.source_snapshot().as_str().to_owned(),
        route_identity: replay.route_identity().as_str().to_owned(),
        invalidation_identity: replay.invalidation_identity().as_str().to_owned(),
    }
}

fn capture_pricing_certification_matrix(
    policy: BridgeRuntimePolicy,
    preview_session_identity: &str,
) -> PricingCertificationMatrix {
    let runtime = build_pricing_runtime_with_policy(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        policy,
    );

    PricingCertificationMatrix {
        reference: capture_pricing_reference_bundle(&runtime, preview_session_identity),
        replay: capture_pricing_replay_bundle(&runtime),
    }
}

fn capture_pricing_discard_bundle() -> PricingDiscardBundle {
    let scenario = generated_pricing_scenario();
    let source = pricing_reference_source();
    let runtime = build_pricing_runtime(source.clone(), RecordingSignalBridgeSink::default());
    let session = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("pricing:preview-discard-churn"),
            pricing_preview_declaration(),
            4,
            2,
            2,
        ))
        .expect("discard churn preview should activate");
    let comparison = session.compare_to_main();

    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-main-live",
        "patch:steel-main-live",
        "snapshot:pricing-main-live",
        "steel",
    ));
    source.insert_snapshot(scenario.live_main_snapshot);

    let live_main_route = runtime
        .route("commit:steel-main-live")
        .expect("main branch should keep routing during speculative churn");
    let speculative_eval = runtime
        .evaluate(
            comparison
                .speculative_evaluation_request()
                .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("speculative branch should still see shock pricing");

    let discarded = session
        .discard(vec![
            BridgePreviewResidueClass::PreviewExecutionRetained,
            BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
            BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
        ])
        .expect("discard should succeed with zero authoritative residue");

    let post_discard_main_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new("main"))
                .with_read_packet(pricing_component_read_packet("steel")),
        )
        .expect("main branch should still evaluate after discard");
    let replay_bundle = runtime
        .replay_preview_bundle("pricing:preview-discard-churn")
        .expect("discard replay bundle should be retained");

    PricingDiscardBundle {
        live_main_snapshot: live_main_route
            .result()
            .receipt()
            .snapshot_identity()
            .as_str()
            .to_owned(),
        speculative_rubber_cost_cents: read_single_money_cents(&speculative_eval),
        post_discard_main_snapshot: post_discard_main_eval.snapshot_identity().as_str().to_owned(),
        post_discard_main_steel_cost_cents: read_single_money_cents(&post_discard_main_eval),
        lifecycle_state: discarded.session().lifecycle_state_kind(),
        discard_record_count: runtime.diagnostics().preview_discard_records().len(),
        promotion_record_count: runtime.diagnostics().preview_promotion_records().len(),
        replay_outcome: replay_bundle.lifecycle_outcome(),
        has_discard_record: replay_bundle.preview_discard_record().is_some(),
        has_promotion_record: replay_bundle.preview_promotion_record().is_some(),
    }
}

fn capture_pricing_promotion_bundle() -> PricingPromotionBundle {
    let scenario = generated_pricing_scenario();
    let source = pricing_reference_source();
    let runtime = build_pricing_runtime(source.clone(), RecordingSignalBridgeSink::default());
    let session = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("pricing:preview-promote-churn"),
            pricing_preview_declaration(),
            4,
            2,
            2,
        ))
        .expect("promotion churn preview should activate");
    let comparison = session.compare_to_main();

    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:rubber-main-interleaved",
        "patch:rubber-main-interleaved",
        "snapshot:pricing-main-interleaved",
        "rubber",
    ));
    source.insert_snapshot(scenario.interleaved_main_snapshot);

    let main_eval = runtime
        .evaluate(
            comparison
                .main_evaluation_request(TruthBranchIdentity::new("main"))
                .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("interleaved main branch should remain independently readable");
    let speculative_eval = runtime
        .evaluate(
            comparison
                .speculative_evaluation_request()
                .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("speculative branch should keep its isolated shock view");

    let promoted = session
        .promote(BridgeSpeculativePromotionRequest::new(
            "commit-boundary:pricing-churn",
            "authoritative-artifact:pricing-shock",
        ))
        .expect("promotion should succeed after interleaved main churn");
    let replay_bundle = runtime
        .replay_preview_bundle("pricing:preview-promote-churn")
        .expect("promotion replay bundle should be retained");
    let promotion_record = replay_bundle
        .preview_promotion_record()
        .expect("promotion replay bundle should retain the promotion record");

    PricingPromotionBundle {
        main_snapshot: main_eval.snapshot_identity().as_str().to_owned(),
        speculative_snapshot: speculative_eval.snapshot_identity().as_str().to_owned(),
        main_rubber_cost_cents: read_single_money_cents(&main_eval),
        speculative_rubber_cost_cents: read_single_money_cents(&speculative_eval),
        lifecycle_state: promoted.session().lifecycle_state_kind(),
        promotion_session_identity: promotion_record.preview_session_identity().to_owned(),
        authoritative_commit_boundary_digest: promotion_record
            .authoritative_commit_boundary_digest()
            .to_owned(),
        authoritative_artifact_digest: promotion_record.authoritative_artifact_digest().to_owned(),
        replay_outcome: replay_bundle.lifecycle_outcome(),
        has_promotion_explanation: matches!(
            runtime.diagnostics().explain_session("pricing:preview-promote-churn"),
            Some(crate::facade::BridgeStandardSessionExplanation::PreviewPromotion(_))
        ),
    }
}

fn capture_pricing_fanout_bundle() -> PricingFanoutBundle {
    let scenario = generated_pricing_scenario();
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-fanout-a",
        "patch:steel-fanout-a",
        "snapshot:pricing-fanout-a",
        "steel",
    ));
    source.insert_snapshot(scenario.fanout_first_snapshot);

    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_high_fanout_pricing_runtime(source.clone(), sink.clone(), 100);

    let first_route = runtime
        .route("commit:steel-fanout-a")
        .expect("first steel fanout route should succeed");

    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-fanout-b",
        "patch:steel-fanout-b",
        "snapshot:pricing-fanout-b",
        "steel",
    ));
    source.insert_snapshot(scenario.fanout_second_snapshot);

    let second_route = runtime
        .route("commit:steel-fanout-b")
        .expect("second steel fanout route should succeed");
    let second_eval = runtime
        .evaluate_current(second_route.target())
        .expect("second steel fanout route should prepare evaluation");
    let branch_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new("main"))
                .with_read_packet(pricing_component_read_packet("steel")),
        )
        .expect("main branch should evaluate after repeated steel churn");

    let route_records = runtime.diagnostics().route_records();
    let last_record = route_records
        .last()
        .expect("repeated steel churn should retain the last route record");
    let last_targets = last_record
        .invalidation_targets()
        .iter()
        .map(|target| target.signal_scope().to_owned())
        .collect::<Vec<_>>();

    PricingFanoutBundle {
        total_deliveries: sink.deliveries().len(),
        first_delivery_target_count: first_route.result().receipt().delivered_target_count(),
        second_delivery_target_count: second_route.result().receipt().delivered_target_count(),
        second_source_commit: "commit:steel-fanout-b".to_owned(),
        second_snapshot: second_eval.snapshot().snapshot_identity().as_str().to_owned(),
        branch_snapshot: branch_eval.snapshot_identity().as_str().to_owned(),
        branch_steel_cost_cents: read_single_money_cents(&branch_eval),
        retained_target_count: last_targets.len(),
        first_target: last_targets.first().cloned().unwrap_or_default(),
        last_target: last_targets.last().cloned().unwrap_or_default(),
    }
}

fn capture_pricing_restart_replay_bundle(
    policy: BridgeRuntimePolicy,
) -> PricingRestartReplayBundle {
    let original_runtime = build_pricing_runtime_with_policy(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        policy.clone(),
    );
    original_runtime
        .route("commit:steel-main")
        .expect("pricing restart control route should succeed");
    let canonical_record = original_runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("pricing restart replay should retain a canonical route record");

    let restarted_runtime = build_pricing_runtime_with_policy(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        policy,
    );
    let replay = restarted_runtime
        .replay_canonical_record(&canonical_record)
        .expect("pricing restart replay should preserve canonical truth across rebuild");

    PricingRestartReplayBundle {
        source_commit: replay.source_commit().as_str().to_owned(),
        source_snapshot: replay.source_snapshot().as_str().to_owned(),
        route_identity: replay.route_identity().as_str().to_owned(),
        invalidation_identity: replay.invalidation_identity().as_str().to_owned(),
    }
}

fn capture_pricing_restart_failure_bundle() -> PricingRestartFailureBundle {
    let original_runtime = build_pricing_runtime(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
    );
    original_runtime
        .route("commit:steel-main")
        .expect("pricing restart mismatch control route should succeed");
    let canonical_record = original_runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("pricing restart mismatch should retain a canonical route record");

    let drifted_source = InMemoryRelationalBridgeSource::default();
    drifted_source.insert_committed_patch(pricing_patch_items(
        "main",
        "commit:steel-main",
        "patch:steel-main",
        "snapshot:pricing-main",
        vec![
            BridgeCommittedPatchItem::new("component:steel", "cost", "usd"),
            BridgeCommittedPatchItem::new("component:steel", "tariff", "usd"),
        ],
    ));
    drifted_source.insert_committed_patch(pricing_patch(
        "main",
        "commit:rubber-main",
        "patch:rubber-main",
        "snapshot:pricing-main",
        "rubber",
    ));
    drifted_source.insert_snapshot(pricing_snapshot("snapshot:pricing-main", "100", "40"));
    let restarted_runtime = build_pricing_runtime(
        drifted_source,
        RecordingSignalBridgeSink::default(),
    );

    let error = restarted_runtime
        .replay_canonical_record(&canonical_record)
        .expect_err("pricing restart replay should reject route drift after truth change");
    let failure_record = restarted_runtime
        .diagnostics()
        .last_failure_record()
        .expect("pricing restart replay mismatch should retain a failure record");

    PricingRestartFailureBundle {
        error_kind: error.kind(),
        replay_mismatch_count: failure_record.counters().route_replay_mismatch_count(),
    }
}

fn capture_pricing_replay_policy_failure_bundle() -> (String, String) {
    let runtime = build_pricing_runtime_with_policy(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        BridgeRuntimePolicy::operational().with_replay_artifacts(false),
    );
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:pricing-showcase-replay-conflict"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Minimal,
        true,
        false,
    );
    let rejection = runtime
        .admit_policy_declaration(declaration)
        .expect_err("replay requirement should fail when runtime policy disables replay");

    (
        format!("{:?}", rejection.kind()),
        format!("{:?}", rejection.field_kind()),
    )
}

fn capture_pricing_route_policy_conflict_bundle() -> String {
    let permissive = build_pricing_runtime_with_policy(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        BridgeRuntimePolicy::development(),
    );
    let restrictive = build_pricing_runtime_with_policy(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        BridgeRuntimePolicy::operational().with_replay_artifacts(false),
    );
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:pricing-route-policy-conflict"),
        BridgeRequestKind::Authoritative,
        BridgeExecutionPolicyClass::DeterministicCanonical,
        BridgeDiagnosticsTier::Standard,
        true,
        true,
    );
    let admitted = permissive
        .admit_policy_declaration(declaration)
        .expect("permissive runtime should admit replay-capable route policy");
    let lowered = permissive.lower_admitted_policy(&admitted);
    let error = restrictive
        .project_route_planning_policy(&lowered)
        .expect_err("restrictive runtime should reject incompatible route policy");

    format!("{:?}", error.kind())
}

fn capture_pricing_merge_denial_bundle() -> (String, String) {
    let runtime = build_pricing_runtime_with_merge(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        BridgeRuntimePolicy::development(),
    );
    let contract = runtime
        .admit_merge_history(pricing_topology_denial_merge_declaration())
        .expect("registered topology-denial merge should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("registered topology-denial merge should replay as a denied merge bundle");
    (
        format!("{:?}", bundle.lowered_packet_set().blocked_stage()),
        format!(
            "{:?}",
            bundle
                .lowered_packet_set()
                .denial_class()
                .expect("topology-denial merge should retain a typed denial class")
        ),
    )
}

fn capture_pricing_trust_attack_bundle() -> PricingTrustAttackBundle {
    let (replay_policy_error_kind, replay_policy_failure_class) =
        capture_pricing_replay_policy_failure_bundle();
    let route_policy_error_kind = capture_pricing_route_policy_conflict_bundle();
    let (merge_denial_blocked_stage, merge_denial_class) = capture_pricing_merge_denial_bundle();

    PricingTrustAttackBundle {
        replay_policy_error_kind,
        replay_policy_failure_class,
        route_policy_error_kind,
        merge_denial_blocked_stage,
        merge_denial_class,
    }
}

fn capture_pricing_historical_provenance_bundle(
    policy: BridgeRuntimePolicy,
) -> PricingHistoricalProvenanceBundle {
    let runtime = build_pricing_runtime_with_policy(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        policy,
    );
    let scenario = generated_pricing_scenario();
    let shock = scenario
        .commit_attributions
        .get("commit:rubber-shock")
        .expect("generated scenario should retain shock attribution");

    let main = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("main"),
                TruthCommitIdentity::new("commit:rubber-main"),
            )
            .with_read_packet(pricing_provenance_read_packet("rubber")),
        )
        .expect("historical main provenance should materialize");
    let shock_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_provenance_read_packet("rubber")),
        )
        .expect("historical shock provenance should materialize");
    let main_payloads = read_packet_payloads(&main);
    let shock_payloads = read_packet_payloads(&shock_eval);

    PricingHistoricalProvenanceBundle {
        main_commit: "commit:rubber-main".to_owned(),
        main_snapshot: main.snapshot_identity().as_str().to_owned(),
        main_regime: main_payloads[0].clone(),
        main_external_factor_microunits: main_payloads[1]
            .parse()
            .expect("main external factor should parse"),
        shock_commit: "commit:rubber-shock".to_owned(),
        shock_snapshot: shock_eval.snapshot_identity().as_str().to_owned(),
        shock_regime: shock_payloads[0].clone(),
        shock_external_factor_microunits: shock_payloads[1]
            .parse()
            .expect("shock external factor should parse"),
        shock_factor_delta_microunits: shock_payloads[2]
            .parse()
            .expect("shock factor delta should parse"),
        shock_trend_delta_microunits: shock_payloads[3]
            .parse()
            .expect("shock trend delta should parse"),
        shock_jump_delta_microunits: shock_payloads[4]
            .parse()
            .expect("shock jump delta should parse"),
        shock_delta_microunits: shock_payloads[5]
            .parse()
            .expect("shock delta should parse"),
        shock_multiplier_per_mille: shock_payloads[6]
            .parse()
            .expect("shock multiplier should parse"),
        representative_sku: shock.representative_product.sku.clone(),
        representative_retail_price_cents: shock.representative_product.retail_price_cents,
        representative_shipping_cost_cents: shock.representative_product.shipping_cost_cents,
        representative_fuel_shipping_component_cents: shock
            .representative_product
            .fuel_shipping_component_cents,
    }
}

fn capture_pricing_portfolio_blast_radius_bundle() -> PricingPortfolioBlastRadiusBundle {
    let scenario = generated_pricing_scenario();
    let product_count = scenario.main_portfolio.len();
    let main_repricing_count = scenario
        .main_portfolio
        .iter()
        .filter(|entry| entry.repricing_triggered)
        .count();
    let shock_repricing_count = scenario
        .speculative_portfolio
        .iter()
        .filter(|entry| entry.repricing_triggered)
        .count();
    let main_margin_floor_breach_count = scenario
        .main_portfolio
        .iter()
        .filter(|entry| entry.margin_floor_breached)
        .count();
    let shock_margin_floor_breach_count = scenario
        .speculative_portfolio
        .iter()
        .filter(|entry| entry.margin_floor_breached)
        .count();

    let mut positive_retail_delta_count = 0usize;
    let mut total_retail_delta_cents = 0i64;
    let mut max_retail_delta_sku = String::new();
    let mut max_retail_delta_cents = i64::MIN;
    let mut family_margin_erosion_cents = BTreeMap::<String, i64>::new();
    let mut family_shipping_delta_cents = BTreeMap::<String, i64>::new();
    let mut family_material_delta_cents = BTreeMap::<String, i64>::new();

    for (main_entry, shock_entry) in scenario
        .main_portfolio
        .iter()
        .zip(scenario.speculative_portfolio.iter())
    {
        let retail_delta_cents = shock_entry.retail_price_cents - main_entry.retail_price_cents;
        if retail_delta_cents > 0 {
            positive_retail_delta_count += 1;
        }
        total_retail_delta_cents += retail_delta_cents;
        if retail_delta_cents > max_retail_delta_cents {
            max_retail_delta_cents = retail_delta_cents;
            max_retail_delta_sku = shock_entry.sku.clone();
        }
        *family_margin_erosion_cents
            .entry(shock_entry.family.clone())
            .or_default() += shock_entry.margin_cents - main_entry.margin_cents;
        *family_shipping_delta_cents
            .entry(shock_entry.family.clone())
            .or_default() += shock_entry.shipping_cost_cents - main_entry.shipping_cost_cents;
        *family_material_delta_cents
            .entry(shock_entry.family.clone())
            .or_default() += shock_entry.material_cost_cents - main_entry.material_cost_cents;
    }

    let (top_margin_erosion_family, top_margin_erosion_cents) = family_margin_erosion_cents
        .into_iter()
        .min_by_key(|(_, erosion)| *erosion)
        .expect("family margin erosion should not be empty");
    let (most_shipping_sensitive_family, most_shipping_sensitive_delta_cents) =
        family_shipping_delta_cents
            .into_iter()
            .max_by_key(|(_, delta)| *delta)
            .expect("family shipping delta should not be empty");
    let (most_material_sensitive_family, most_material_sensitive_delta_cents) =
        family_material_delta_cents
            .into_iter()
            .max_by_key(|(_, delta)| *delta)
            .expect("family material delta should not be empty");

    PricingPortfolioBlastRadiusBundle {
        product_count,
        main_repricing_count,
        shock_repricing_count,
        main_margin_floor_breach_count,
        shock_margin_floor_breach_count,
        positive_retail_delta_count,
        total_retail_delta_cents,
        max_retail_delta_sku,
        max_retail_delta_cents,
        top_margin_erosion_family,
        top_margin_erosion_cents,
        most_shipping_sensitive_family,
        most_shipping_sensitive_delta_cents,
        most_material_sensitive_family,
        most_material_sensitive_delta_cents,
    }
}

fn capture_pricing_crisis_bundle() -> PricingCrisisBundle {
    let scenario = generated_pricing_scenario();
    let main_total_retail_cents = scenario
        .main_portfolio
        .iter()
        .map(|entry| entry.retail_price_cents)
        .sum::<i64>();
    let crisis_total_retail_cents = scenario
        .crisis_portfolio
        .iter()
        .map(|entry| entry.retail_price_cents)
        .sum::<i64>();
    let affected_product_count = scenario
        .main_portfolio
        .iter()
        .zip(scenario.crisis_portfolio.iter())
        .filter(|(main_entry, crisis_entry)| {
            crisis_entry.retail_price_cents > main_entry.retail_price_cents
        })
        .count();

    let mut family_deltas = BTreeMap::<String, i64>::new();
    for (main_entry, crisis_entry) in scenario
        .main_portfolio
        .iter()
        .zip(scenario.crisis_portfolio.iter())
    {
        let family = crisis_entry.family.clone();
        *family_deltas.entry(family).or_default() +=
            crisis_entry.retail_price_cents - main_entry.retail_price_cents;
    }
    let (top_impacted_family, top_impacted_family_delta_cents) = family_deltas
        .into_iter()
        .max_by_key(|(_, delta)| *delta)
        .expect("family deltas should not be empty");
    let (policy_pressure_family, policy_pressure_bps) = scenario
        .crisis_family_tariff_bps
        .iter()
        .max_by_key(|(_, bps)| **bps)
        .map(|(family, bps)| (family.clone(), *bps))
        .expect("crisis family tariff map should not be empty");
    let mut material_deltas = BTreeMap::<String, i64>::new();
    for (material, crisis_value) in &scenario.crisis_overrides {
        let main_value = scenario
            .main_material_prices
            .get(material)
            .copied()
            .expect("main material price should exist for crisis material");
        material_deltas.insert(material.key().to_owned(), crisis_value - main_value);
    }
    let (top_exposure_material, top_exposure_material_delta_cents) = material_deltas
        .into_iter()
        .max_by_key(|(_, delta)| *delta)
        .expect("material deltas should not be empty");

    PricingCrisisBundle {
        crisis_name: "energy-logistics-industrial-crunch".to_owned(),
        affected_product_count,
        main_total_retail_cents,
        crisis_total_retail_cents,
        total_retail_delta_cents: crisis_total_retail_cents - main_total_retail_cents,
        top_impacted_family,
        top_impacted_family_delta_cents,
        dominant_shock_material: "rubber".to_owned(),
        dominant_shock_multiplier_per_mille: 4_000,
        policy_pressure_family,
        policy_pressure_bps,
        top_exposure_material,
        top_exposure_material_delta_cents,
    }
}

fn capture_pricing_strategy_bundle() -> PricingStrategyBundle {
    let scenario = generated_pricing_scenario();
    let mut hold_unprofitable_count = 0usize;
    let mut partial_absorb_unprofitable_count = 0usize;
    let mut targeted_reprice_positive_delta_count = 0usize;
    let mut targeted_reprice_total_delta_cents = 0i64;
    let mut hold_total_margin_delta_cents = 0i64;
    let mut partial_absorb_total_margin_delta_cents = 0i64;
    let mut targeted_reprice_margin_recovery_cents = 0i64;

    for (main_entry, crisis_entry) in scenario
        .main_portfolio
        .iter()
        .zip(scenario.crisis_portfolio.iter())
    {
        let hold_margin_cents = main_entry.retail_price_cents - crisis_entry.landed_cost_cents;
        hold_total_margin_delta_cents += hold_margin_cents - main_entry.margin_cents;
        if hold_margin_cents < 0 {
            hold_unprofitable_count += 1;
        }

        let partial_absorb_retail_cents =
            main_entry.retail_price_cents + ((crisis_entry.retail_price_cents - main_entry.retail_price_cents) / 2);
        let partial_absorb_margin_cents = partial_absorb_retail_cents - crisis_entry.landed_cost_cents;
        partial_absorb_total_margin_delta_cents += partial_absorb_margin_cents - main_entry.margin_cents;
        if partial_absorb_margin_cents < 0 {
            partial_absorb_unprofitable_count += 1;
        }

        let retail_delta_cents = crisis_entry.retail_price_cents - main_entry.retail_price_cents;
        if retail_delta_cents > 0 {
            targeted_reprice_positive_delta_count += 1;
            targeted_reprice_total_delta_cents += retail_delta_cents;
            targeted_reprice_margin_recovery_cents += crisis_entry.margin_cents - hold_margin_cents;
        }
    }

    let (recommended_strategy, recommendation_reason) = if hold_unprofitable_count > 0 {
        (
            "targeted-reprice".to_owned(),
            "hold strategy leaves part of the portfolio underwater under the crisis cost basis"
                .to_owned(),
        )
    } else if partial_absorb_unprofitable_count > 0 {
        (
            "partial-absorb".to_owned(),
            "full hold remains too aggressive, but partial absorption protects more portfolio than broad stasis"
                .to_owned(),
        )
    } else {
        (
            "hold".to_owned(),
            "the portfolio remains profitable without emergency repricing under this crisis basis"
                .to_owned(),
        )
    };
    let promotion_strategy = if recommended_strategy == "hold" {
        "discard-speculative-strategy".to_owned()
    } else {
        "promote-speculative-strategy".to_owned()
    };

    PricingStrategyBundle {
        hold_unprofitable_count,
        partial_absorb_unprofitable_count,
        targeted_reprice_positive_delta_count,
        targeted_reprice_total_delta_cents,
        hold_total_margin_delta_cents,
        partial_absorb_total_margin_delta_cents,
        targeted_reprice_margin_recovery_cents,
        recommended_strategy,
        recommendation_reason,
        promotion_strategy,
    }
}

fn simulation_candidate_materials() -> &'static [PricingMaterial] {
    &[
        PricingMaterial::Steel,
        PricingMaterial::Aluminum,
        PricingMaterial::Copper,
        PricingMaterial::Rubber,
        PricingMaterial::PlasticResin,
        PricingMaterial::Electronics,
        PricingMaterial::Packaging,
        PricingMaterial::Labor,
        PricingMaterial::Fuel,
    ]
}

fn base_shock_multiplier_per_mille(material: PricingMaterial) -> i64 {
    match material {
        PricingMaterial::Rubber => 2_350,
        PricingMaterial::Fuel => 2_200,
        PricingMaterial::Steel => 1_650,
        PricingMaterial::Copper => 1_800,
        PricingMaterial::Electronics => 1_700,
        PricingMaterial::Aluminum => 1_500,
        PricingMaterial::PlasticResin => 1_450,
        PricingMaterial::Packaging => 1_180,
        PricingMaterial::Labor => 1_120,
    }
}

fn regime_pressure_per_mille(regime: FeedVolatilityRegime) -> i64 {
    match regime {
        FeedVolatilityRegime::Calm => 0,
        FeedVolatilityRegime::Normal => 120,
        FeedVolatilityRegime::Volatile => 320,
        FeedVolatilityRegime::Stressed => 720,
    }
}

fn event_pressure_per_mille(event_kind: FeedStreamEventKind) -> i64 {
    match event_kind {
        FeedStreamEventKind::Stable => 0,
        FeedStreamEventKind::Noise => 35,
        FeedStreamEventKind::Drift => 80,
        FeedStreamEventKind::MinorShift => 180,
        FeedStreamEventKind::MajorShift => 420,
        FeedStreamEventKind::RegimeShift => 760,
    }
}

fn natural_shock_multiplier_per_mille(
    material: PricingMaterial,
    attribution: &MaterialPriceAttribution,
    branch_index: usize,
    iteration_index: usize,
) -> i64 {
    let branch_variation = ((branch_index as i64 * 67) + (iteration_index as i64 * 29)) % 240;
    let jump_pressure = (attribution.jump_delta_microunits.abs() / 2_000).clamp(0, 500);
    let factor_pressure = (attribution.external_factor_microunits.abs() / 3_000).clamp(0, 260);
    (
        base_shock_multiplier_per_mille(material)
            + regime_pressure_per_mille(attribution.regime)
            + event_pressure_per_mille(attribution.event_kind)
            + branch_variation
            + jump_pressure
            + factor_pressure
    )
    .clamp(1_100, 4_500)
}

fn family_tariff_bps_for_material(
    material: PricingMaterial,
    branch_index: usize,
    iteration_index: usize,
) -> BTreeMap<String, i64> {
    let pulse = 40 + ((branch_index as i64 * 13 + iteration_index as i64 * 11) % 140);
    match material {
        PricingMaterial::Fuel => BTreeMap::from([
            ("washer".to_owned(), 320 + pulse),
            ("dryer".to_owned(), 300 + pulse),
            ("e-bike".to_owned(), 180 + (pulse / 2)),
        ]),
        PricingMaterial::Electronics => BTreeMap::from([
            ("e-bike".to_owned(), 360 + pulse),
            ("washer".to_owned(), 240 + (pulse / 2)),
            ("dryer".to_owned(), 220 + (pulse / 2)),
        ]),
        PricingMaterial::Steel | PricingMaterial::Copper => BTreeMap::from([
            ("washer".to_owned(), 220 + pulse),
            ("dryer".to_owned(), 210 + pulse),
        ]),
        PricingMaterial::Rubber => BTreeMap::from([
            ("bicycle".to_owned(), 120 + (pulse / 3)),
            ("scooter".to_owned(), 140 + (pulse / 3)),
            ("e-bike".to_owned(), 160 + (pulse / 3)),
        ]),
        _ => BTreeMap::new(),
    }
}

fn sum_retail_cents(entries: &[ProductPriceBreakdown]) -> i64 {
    entries.iter().map(|entry| entry.retail_price_cents).sum()
}

fn capture_pricing_simulation_suite() -> PricingShockSimulationSuite {
    const BRANCH_COUNT: usize = 10;
    const ITERATIONS_PER_BRANCH: usize = 10;

    let mut material_summaries = Vec::new();
    let mut iteration_traces = Vec::new();

    for &material in simulation_candidate_materials() {
        let mut branch_mean_deltas = Vec::<(String, i64)>::new();
        let mut material_total_delta = 0i64;
        let mut shipping_total_delta = 0i64;
        let mut material_cost_total_delta = 0i64;
        let mut breach_total = 0i64;
        let mut repricing_total = 0i64;

        for branch_index in 0..BRANCH_COUNT {
            let mut world = PricingDomainWorld::new(70_000 + (material as u64 * 1_000) + branch_index as u64);
            let mut branch_total_delta = 0i64;

            for iteration_index in 0..ITERATIONS_PER_BRANCH {
                let wave = world.advance_material_streams();
                let attribution = attribution_for(&wave, material);
                let baseline = world.price_matrix();
                let multiplier = natural_shock_multiplier_per_mille(
                    material,
                    &attribution,
                    branch_index,
                    iteration_index,
                );
                let override_map = BTreeMap::from([(
                    material,
                    world.shocked_material_price_microunits(material, multiplier),
                )]);
                let tariff_map =
                    family_tariff_bps_for_material(material, branch_index, iteration_index);
                let shocked = world.price_matrix_with_scenario(override_map, tariff_map);

                let baseline_total_retail_cents = sum_retail_cents(&baseline);
                let shocked_total_retail_cents = sum_retail_cents(&shocked);
                let total_retail_delta_cents =
                    shocked_total_retail_cents - baseline_total_retail_cents;
                let shipping_delta_cents = baseline
                    .iter()
                    .zip(shocked.iter())
                    .map(|(base, shock)| shock.shipping_cost_cents - base.shipping_cost_cents)
                    .sum::<i64>();
                let material_delta_cents = baseline
                    .iter()
                    .zip(shocked.iter())
                    .map(|(base, shock)| shock.material_cost_cents - base.material_cost_cents)
                    .sum::<i64>();
                let margin_floor_breach_count = shocked
                    .iter()
                    .filter(|entry| entry.margin_floor_breached)
                    .count();
                let repricing_count = shocked
                    .iter()
                    .filter(|entry| entry.repricing_triggered)
                    .count();

                branch_total_delta += total_retail_delta_cents;
                material_total_delta += total_retail_delta_cents;
                shipping_total_delta += shipping_delta_cents;
                material_cost_total_delta += material_delta_cents;
                breach_total += margin_floor_breach_count as i64;
                repricing_total += repricing_count as i64;

                iteration_traces.push(PricingShockSimulationIterationTrace {
                    material: material.key().to_owned(),
                    branch_identity: format!("sim:{}:branch-{branch_index:02}", material.key()),
                    iteration_index,
                    regime: format!("{:?}", attribution.regime),
                    event_kind: format!("{:?}", attribution.event_kind),
                    shock_multiplier_per_mille: multiplier,
                    baseline_total_retail_cents,
                    shocked_total_retail_cents,
                    total_retail_delta_cents,
                    shipping_delta_cents,
                    material_delta_cents,
                    margin_floor_breach_count,
                    repricing_count,
                });
            }

            branch_mean_deltas.push((
                format!("sim:{}:branch-{branch_index:02}", material.key()),
                branch_total_delta / ITERATIONS_PER_BRANCH as i64,
            ));
        }

        let (worst_branch_identity, worst_branch_mean_total_delta_cents) = branch_mean_deltas
            .iter()
            .max_by_key(|(_, delta)| *delta)
            .cloned()
            .expect("branch means should not be empty");
        let total_iterations = (BRANCH_COUNT * ITERATIONS_PER_BRANCH) as i64;
        let mean_total_retail_delta_cents = material_total_delta / total_iterations;
        let mean_shipping_delta_cents = shipping_total_delta / total_iterations;
        let mean_material_delta_cents = material_cost_total_delta / total_iterations;
        let mean_margin_floor_breach_count = breach_total / total_iterations;
        let mean_repricing_count = repricing_total / total_iterations;
        let damage_score = mean_total_retail_delta_cents
            + (mean_margin_floor_breach_count * 50)
            + mean_shipping_delta_cents.abs() / 10;

        material_summaries.push(PricingShockSimulationMaterialSummary {
            material: material.key().to_owned(),
            branch_count: BRANCH_COUNT,
            iterations_per_branch: ITERATIONS_PER_BRANCH,
            mean_total_retail_delta_cents,
            mean_shipping_delta_cents,
            mean_material_delta_cents,
            mean_margin_floor_breach_count,
            mean_repricing_count,
            worst_branch_identity,
            worst_branch_mean_total_delta_cents,
            damage_score,
        });
    }

    material_summaries.sort_by(|left, right| {
        right
            .damage_score
            .cmp(&left.damage_score)
            .then_with(|| right.mean_total_retail_delta_cents.cmp(&left.mean_total_retail_delta_cents))
    });
    let ranked_materials_by_damage = material_summaries
        .iter()
        .map(|summary| summary.material.clone())
        .collect::<Vec<_>>();

    PricingShockSimulationSuite {
        branch_count: BRANCH_COUNT,
        iterations_per_branch: ITERATIONS_PER_BRANCH,
        material_summaries,
        ranked_materials_by_damage,
        iteration_traces,
    }
}

fn capture_pricing_writeback_bundle(policy: BridgeRuntimePolicy) -> PricingWritebackBundle {
    let writeback_authority = RecordingTruthWritebackAuthority::default();
    let runtime = build_pricing_runtime_with_policy_and_writeback_authority(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        policy.clone(),
        writeback_authority.clone(),
    );
    let lowered_policy = pricing_lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            pricing_writeback_declaration(
                "writeback:pricing-authority",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:pricing-authority",
            ),
            &lowered_policy,
        )
        .expect("pricing writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &pricing_writeback_causality_basis(
            "causality:pricing-authority",
            "truth-trigger:pricing-steel-main",
        ),
        BridgeWritebackEffectIdentity::new("effect:pricing-authority"),
        "effect:sha256:pricing-authority",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:pricing-main",
        BridgeWritebackIdempotenceIdentity::new("idempotence:pricing-authority"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let (commit_outcome, commit_receipt) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("pricing writeback authority should commit the first time");
    let commit_record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("pricing writeback commit should retain an execution record");
    let commit_replay_bundle =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &commit_outcome);

    let (noop_outcome, noop_receipt) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("pricing writeback authority should classify repeated causality as canonical noop");
    let noop_record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("pricing writeback noop should retain an execution record");
    let noop_replay_bundle =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &noop_outcome);

    let rejecting_runtime = build_pricing_runtime_with_policy_and_writeback_authority(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
        policy,
        RejectingPricingWritebackAuthority {
            failure_class: BridgeWritebackFailureClass::MergeAuthorityRejected,
        },
    );
    let rejecting_lowered_policy = pricing_lowered_policy(&rejecting_runtime);
    let rejecting_contract = rejecting_runtime
        .admit_writeback_declaration(
            pricing_writeback_declaration(
                "writeback:pricing-rejection",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:pricing-rejection",
            ),
            &rejecting_lowered_policy,
        )
        .expect("pricing rejection declaration should admit");
    let rejecting_effect = rejecting_runtime.lower_writeback_effect(
        &rejecting_contract,
        &pricing_writeback_causality_basis(
            "causality:pricing-rejection",
            "truth-trigger:pricing-rubber-shock",
        ),
        BridgeWritebackEffectIdentity::new("effect:pricing-rejection"),
        "effect:sha256:pricing-rejection",
    );
    let rejecting_idempotence = rejecting_runtime.classify_writeback_idempotence(
        &rejecting_effect,
        &rejecting_lowered_policy,
        "truth-state:sha256:pricing-shock",
        BridgeWritebackIdempotenceIdentity::new("idempotence:pricing-rejection"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let rejection_error = rejecting_runtime
        .execute_writeback_authority(
            &rejecting_contract,
            &rejecting_effect,
            &rejecting_idempotence,
        )
        .expect_err("pricing writeback rejection should stay typed");
    let rejection_record = rejecting_runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("pricing writeback rejection should retain an execution record");

    PricingWritebackBundle {
        family_kind: format!("{:?}", effect.family_kind()),
        strategy_class: format!("{:?}", effect.strategy_class()),
        commit_outcome_class: commit_outcome.outcome_class(),
        noop_outcome_class: noop_outcome.outcome_class(),
        commit_replay_semantic_digest: commit_replay_bundle.semantic_digest().to_owned(),
        noop_replay_semantic_digest: noop_replay_bundle.semantic_digest().to_owned(),
        shared_authoritative_artifact: commit_receipt.authoritative_artifact_digest()
            == noop_receipt.authoritative_artifact_digest(),
        authority_commit_count: writeback_authority.committed_causality_count(),
        execution_request_count: noop_record.counters().writeback_request_count(),
        execution_commit_count: commit_record.counters().writeback_commit_count(),
        execution_noop_count: noop_record.counters().writeback_noop_count(),
        rejection_error_kind: rejection_error.kind(),
        rejection_failure_class: rejection_record
            .failure_class()
            .expect("pricing writeback rejection should carry a failure class"),
        rejection_request_emitted: rejection_record.request_digest().is_some(),
        rejection_receipt_emitted: rejection_record.receipt_digest().is_some(),
    }
}

fn capture_pricing_merge_bundle(policy: BridgeRuntimePolicy) -> PricingMergeBundle {
    let runtime = build_pricing_runtime_with_merge(
        pricing_merge_source(),
        RecordingSignalBridgeSink::default(),
        policy,
    );
    let contract = runtime
        .admit_merge_history(pricing_merge_declaration())
        .expect("pricing merge declaration should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("pricing merge bundle should replay");
    let canonical_record = runtime.canonicalize_merge_record(&bundle);
    let replayed = runtime
        .replay_canonical_merge_record(&canonical_record)
        .expect("pricing merge canonical replay should succeed");
    runtime
        .route("commit:pricing-merged-aspect")
        .expect("pricing merged aspect route should succeed");
    let merged_route_record = runtime
        .diagnostics()
        .route_record_for_source_commit("commit:pricing-merged-aspect")
        .expect("pricing merged aspect route should retain a route record");
    let merged_explanation = runtime
        .diagnostics()
        .explain_route(merged_route_record.route_identity().as_str())
        .expect("pricing merged aspect route should be explainable");
    let merged_entry = &merged_explanation.route_entries()[0];

    let main_premerge_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("main"),
                TruthCommitIdentity::new("commit:rubber-main"),
            )
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("main premerge evaluation should succeed");
    let speculative_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("speculative evaluation should succeed");
    let merged_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("main"),
                TruthCommitIdentity::new("commit:pricing-merged"),
            )
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("merged historical evaluation should succeed");

    PricingMergeBundle {
        bridge_class: format!(
            "{:?}",
            contract.validated_declaration().declaration().bridge_class()
        ),
        outcome_class: format!("{:?}", bundle.reduced_routing_artifact().outcome_class()),
        blocked_stage: bundle
            .lowered_packet_set()
            .blocked_stage()
            .map(|value| format!("{value:?}")),
        denial_class: bundle
            .lowered_packet_set()
            .denial_class()
            .map(|value| format!("{value:?}")),
        continuity_published: bundle.continuity_artifact().is_some(),
        remap_published: bundle.remap_artifact().is_some(),
        parent_order_digest: bundle
            .lowered_packet_set()
            .parent_order_digest_basis()
            .digest()
            .to_owned(),
        bundle_digest: bundle.digest().to_owned(),
        canonical_replay_digest: replayed.digest().to_owned(),
        replay_request_count: replayed
            .reduced_routing_artifact()
            .counters()
            .merge_replay_request_count(),
        main_premerge_snapshot: main_premerge_eval.snapshot_identity().as_str().to_owned(),
        main_premerge_rubber_cost_cents: read_single_money_cents(&main_premerge_eval),
        speculative_snapshot: speculative_eval.snapshot_identity().as_str().to_owned(),
        speculative_rubber_cost_cents: read_single_money_cents(&speculative_eval),
        merged_snapshot: merged_eval.snapshot_identity().as_str().to_owned(),
        merged_rubber_cost_cents: read_single_money_cents(&merged_eval),
        merged_aspect_registration_id: merged_entry
            .aspect_registration_id()
            .expect("merged pricing route should retain aspect registration id")
            .as_str()
            .to_owned(),
        merged_fine_grained_match_status: format!("{:?}", merged_entry.fine_grained_match_status()),
    }
}

fn capture_pricing_workload_certification_bundle(
    policy: BridgeRuntimePolicy,
    preview_session_identity: &str,
) -> PricingWorkloadCertificationBundle {
    let hostile_source = InMemoryRelationalBridgeSource::default();
    hostile_source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-missing-snapshot",
        "patch:steel-missing-snapshot",
        "snapshot:pricing-missing",
        "steel",
    ));
    let hostile_runtime = build_pricing_runtime_with_policy(
        hostile_source,
        RecordingSignalBridgeSink::default(),
        policy.clone(),
    );

    PricingWorkloadCertificationBundle {
        matrix: capture_pricing_certification_matrix(policy.clone(), preview_session_identity),
        aspect: capture_pricing_aspect_bundle(policy.clone()),
        discard: capture_pricing_discard_bundle(),
        promotion: capture_pricing_promotion_bundle(),
        fanout: capture_pricing_fanout_bundle(),
        restart_replay: capture_pricing_restart_replay_bundle(policy.clone()),
        restart_failure: capture_pricing_restart_failure_bundle(),
        writeback: capture_pricing_writeback_bundle(policy.clone()),
        merge: capture_pricing_merge_bundle(policy.clone()),
        provenance: capture_pricing_historical_provenance_bundle(policy),
        portfolio: capture_pricing_portfolio_blast_radius_bundle(),
        crisis: capture_pricing_crisis_bundle(),
        strategy: capture_pricing_strategy_bundle(),
        simulation: capture_pricing_simulation_suite(),
        trust_attacks: capture_pricing_trust_attack_bundle(),
        hostile_failure: capture_pricing_missing_snapshot_failure_bundle(&hostile_runtime),
    }
}

#[test]
fn pricing_shock_standard_path_routes_evaluates_and_keeps_speculation_local() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-main",
        "patch:steel-main",
        "snapshot:pricing-main",
        "steel",
    ));
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:rubber-main",
        "patch:rubber-main",
        "snapshot:pricing-main",
        "rubber",
    ));
    source.insert_snapshot(pricing_snapshot("snapshot:pricing-main", "100", "40"));

    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_pricing_runtime(source.clone(), sink.clone());

    let steel_route = runtime
        .route("commit:steel-main")
        .expect("steel pricing route should succeed");
    let steel_eval = runtime
        .evaluate_current(steel_route.target())
        .expect("steel route should prepare signal evaluation");
    let branch_eval = runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new("main"),
        ))
        .expect("main pricing branch-head evaluation should succeed");

    assert_eq!(
        steel_route.result().receipt().delivered_target_count(),
        2,
        "shared steel cost should fan out to multiple product price invalidations"
    );
    assert_eq!(
        steel_route.result().receipt().snapshot_identity().as_str(),
        "snapshot:pricing-main"
    );
    assert_eq!(
        steel_eval.snapshot().snapshot_identity().as_str(),
        "snapshot:pricing-main"
    );
    assert_eq!(branch_eval.snapshot_identity().as_str(), "snapshot:pricing-main");
    let diagnostics = runtime.diagnostics();
    let delivered_targets = diagnostics
        .route_records()
        .last()
        .expect("steel route should produce a diagnostics record")
        .invalidation_targets()
        .iter()
        .map(|target| target.signal_scope().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(delivered_targets, vec!["price:bicycle", "price:wheelbarrow"]);
    assert_eq!(sink.deliveries().len(), 1);

    let discarded = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("pricing:preview-discard"),
            pricing_preview_declaration(),
            4,
            2,
            2,
        ))
        .expect("pricing shock preview should activate")
        .discard(vec![
            BridgePreviewResidueClass::PreviewExecutionRetained,
            BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
            BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
        ])
        .expect("pricing shock discard should succeed");

    let promoted = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("pricing:preview-promote"),
            pricing_preview_declaration(),
            4,
            2,
            2,
        ))
        .expect("pricing shock promotion preview should activate")
        .promote(BridgeSpeculativePromotionRequest::new(
            "commit-boundary:pricing",
            "authoritative-artifact:pricing",
        ))
        .expect("pricing shock promotion should succeed");

    assert_eq!(
        runtime.diagnostics().preview_discard_records().len(),
        1,
        "discard should stay isolated and queryable"
    );
    assert_eq!(
        runtime.diagnostics().preview_promotion_records().len(),
        1,
        "promotion should stay isolated and queryable"
    );
    assert!(matches!(
        runtime.diagnostics().explain_session("pricing:preview-promote"),
        Some(crate::facade::BridgeStandardSessionExplanation::PreviewPromotion(_))
    ));
    assert_eq!(
        discarded.session().session_identity().as_str(),
        "pricing:preview-discard"
    );
    assert_eq!(
        promoted.session().session_identity().as_str(),
        "pricing:preview-promote"
    );
}

#[test]
fn pricing_shock_split_screen_keeps_main_and_speculative_truth_isolated() {
    let scenario = generated_pricing_scenario();
    let source = pricing_reference_source();
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_pricing_runtime(source, sink);
    let comparison = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("pricing:preview-compare"),
            pricing_preview_declaration(),
            4,
            2,
            2,
        ))
        .expect("pricing shock comparison preview should activate")
        .compare_to_main();

    let rubber_read = pricing_component_read_packet("rubber");
    let main_eval = runtime
        .evaluate(
            comparison
                .main_evaluation_request(TruthBranchIdentity::new("main"))
                .with_read_packet(rubber_read.clone()),
        )
        .expect("main branch should evaluate against its retained snapshot");
    let speculative_eval = runtime
        .evaluate(
            comparison
                .speculative_evaluation_request()
                .with_read_packet(rubber_read),
        )
        .expect("speculative branch should evaluate against its isolated snapshot");
    let live_main_route = runtime
        .route("commit:steel-main")
        .expect("main branch routing should remain live while speculation is open");

    let main_rubber_cost = read_single_payload(&main_eval);
    let speculative_rubber_cost = read_single_payload(&speculative_eval);

    assert_eq!(comparison.truth_branch_identity().as_str(), "pricing-shock");
    assert_eq!(comparison.signal_branch_identity().as_str(), "signal:pricing-shock");
    assert_eq!(main_eval.snapshot_identity().as_str(), "snapshot:pricing-main");
    assert_eq!(
        speculative_eval.snapshot_identity().as_str(),
        "snapshot:pricing-shock"
    );
    assert_eq!(main_rubber_cost, scenario.main_rubber_cost.to_string());
    assert_eq!(speculative_rubber_cost, scenario.speculative_rubber_cost.to_string());
    assert_eq!(
        live_main_route.result().receipt().snapshot_identity().as_str(),
        "snapshot:pricing-main"
    );
    assert_eq!(live_main_route.result().receipt().delivered_target_count(), 2);
}

#[test]
fn pricing_shock_generated_commit_attribution_exposes_stream_and_product_criteria() {
    let scenario = generated_pricing_scenario();
    let shock = scenario
        .commit_attributions
        .get("commit:rubber-shock")
        .expect("generated scenario should retain shock attribution");

    assert_eq!(shock.snapshot_identity, "snapshot:pricing-shock");
    assert_eq!(shock.branch_identity, "pricing-shock");
    assert_eq!(shock.material, PricingMaterial::Rubber);
    assert_eq!(shock.shock_multiplier_per_mille, 4000);
    assert_eq!(
        shock.shock_delta_microunits,
        scenario.speculative_rubber_cost - scenario.main_rubber_cost
    );
    assert_eq!(
        shock.material_attribution.current_value_microunits,
        scenario.main_rubber_cost
    );
    assert_eq!(shock.representative_product.sku, "scooter-001");
    assert!(shock.representative_product.material_cost_cents > 0);
    assert!(shock.representative_product.shipping_cost_cents > 0);
    assert!(
        shock.representative_product
            .material_contributions_cents
            .iter()
            .any(|(material, cents)| *material == PricingMaterial::Rubber && *cents > 0)
    );
    assert_ne!(shock.material_attribution.external_factor_microunits, 0);
    assert!(
        shock.material_attribution.factor_delta_microunits != 0
            || shock.material_attribution.trend_delta_microunits != 0
            || shock.material_attribution.idiosyncratic_noise_microunits != 0
            || shock.material_attribution.jump_delta_microunits != 0
    );
}

#[test]
fn pricing_shock_historical_commit_reads_bridge_visible_provenance_from_truth() {
    let scenario = generated_pricing_scenario();
    let runtime = build_pricing_runtime(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
    );

    let historical = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_provenance_read_packet("rubber")),
        )
        .expect("historical pricing shock provenance should materialize");
    let payloads = read_packet_payloads(&historical);
    let shock = scenario
        .commit_attributions
        .get("commit:rubber-shock")
        .expect("generated scenario should retain shock attribution");

    assert_eq!(historical.snapshot_identity().as_str(), "snapshot:pricing-shock");
    assert_eq!(payloads[0], format!("{:?}", shock.material_attribution.regime));
    assert_eq!(
        payloads[1],
        shock.material_attribution.external_factor_microunits.to_string()
    );
    assert_eq!(
        payloads[2],
        shock.material_attribution.factor_delta_microunits.to_string()
    );
    assert_eq!(
        payloads[3],
        shock.material_attribution.trend_delta_microunits.to_string()
    );
    assert_eq!(
        payloads[4],
        shock.material_attribution.jump_delta_microunits.to_string()
    );
    assert_eq!(payloads[5], shock.shock_delta_microunits.to_string());
    assert_eq!(payloads[6], shock.shock_multiplier_per_mille.to_string());
}

#[test]
fn pricing_shock_historical_provenance_corruption_is_detectable_against_independent_oracle() {
    let scenario = generated_pricing_scenario();
    let runtime = build_pricing_runtime(
        pricing_reference_source_with_corrupted_shock_provenance("shock-delta", b"999999".to_vec()),
        RecordingSignalBridgeSink::default(),
    );

    let provenance_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_provenance_read_packet("rubber")),
        )
        .expect("corrupted historical provenance should still materialize as truth");
    let cost_eval = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("historical component cost should still materialize");
    let payloads = read_packet_payloads(&provenance_eval);
    let shock = scenario
        .commit_attributions
        .get("commit:rubber-shock")
        .expect("generated scenario should retain shock attribution");

    assert_eq!(provenance_eval.snapshot_identity().as_str(), "snapshot:pricing-shock");
    assert_eq!(read_single_money_cents(&cost_eval), scenario.speculative_rubber_cost);
    assert_eq!(payloads[0], format!("{:?}", shock.material_attribution.regime));
    assert_eq!(
        payloads[1],
        shock.material_attribution.external_factor_microunits.to_string()
    );
    assert_ne!(payloads[5], shock.shock_delta_microunits.to_string());
    assert_eq!(payloads[5], "999999");
    assert_eq!(payloads[6], shock.shock_multiplier_per_mille.to_string());
}

#[test]
fn pricing_shock_provenance_mutation_sweep_is_detectable_against_independent_oracle() {
    let scenario = generated_pricing_scenario();
    let shock = scenario
        .commit_attributions
        .get("commit:rubber-shock")
        .expect("generated scenario should retain shock attribution");

    for (field, expected_payload, corrupted_payload) in [
        (
            "external-factor",
            shock.material_attribution.external_factor_microunits.to_string(),
            "444444".to_owned(),
        ),
        (
            "factor-delta",
            shock.material_attribution.factor_delta_microunits.to_string(),
            "555555".to_owned(),
        ),
        (
            "trend-delta",
            shock.material_attribution.trend_delta_microunits.to_string(),
            "666666".to_owned(),
        ),
        (
            "jump-delta",
            shock.material_attribution.jump_delta_microunits.to_string(),
            "777777".to_owned(),
        ),
        (
            "shock-delta",
            shock.shock_delta_microunits.to_string(),
            "888888".to_owned(),
        ),
        (
            "shock-multiplier",
            shock.shock_multiplier_per_mille.to_string(),
            "999999".to_owned(),
        ),
    ] {
        let runtime = build_pricing_runtime(
            pricing_reference_source_with_corrupted_shock_provenance(
                field,
                corrupted_payload.clone().into_bytes(),
            ),
            RecordingSignalBridgeSink::default(),
        );
        let historical = runtime
            .evaluate(
                BridgeTruthViewEvaluationRequest::for_historical_commit(
                    TruthBranchIdentity::new("pricing-shock"),
                    TruthCommitIdentity::new("commit:rubber-shock"),
                )
                .with_read_packet(pricing_provenance_read_packet("rubber")),
            )
            .expect("corrupted provenance field should still materialize");
        let payloads = read_packet_payloads(&historical);
        let field_index = match field {
            "external-factor" => 1,
            "factor-delta" => 2,
            "trend-delta" => 3,
            "jump-delta" => 4,
            "shock-delta" => 5,
            "shock-multiplier" => 6,
            _ => unreachable!("unexpected provenance field"),
        };

        assert_eq!(historical.snapshot_identity().as_str(), "snapshot:pricing-shock");
        assert_eq!(payloads[field_index], corrupted_payload);
        assert_ne!(payloads[field_index], expected_payload);
    }
}

#[test]
fn pricing_shock_conflicting_historical_basis_is_detectable_against_independent_oracle() {
    let scenario = generated_pricing_scenario();
    let runtime = build_pricing_runtime(
        pricing_reference_source_with_conflicting_shock_snapshot(),
        RecordingSignalBridgeSink::default(),
    );

    let historical_cost = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("conflicting historical basis should still materialize as retained truth");
    let historical_provenance = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_provenance_read_packet("rubber")),
        )
        .expect("conflicting historical basis should materialize provenance packet");
    let payloads = read_packet_payloads(&historical_provenance);

    assert_eq!(historical_cost.snapshot_identity().as_str(), "snapshot:pricing-main");
    assert_eq!(historical_provenance.snapshot_identity().as_str(), "snapshot:pricing-main");
    assert_eq!(read_single_money_cents(&historical_cost), scenario.main_rubber_cost);
    assert_ne!(read_single_money_cents(&historical_cost), scenario.speculative_rubber_cost);
    assert_ne!(
        payloads[5],
        scenario
            .commit_attributions
            .get("commit:rubber-shock")
            .expect("generated scenario should retain shock attribution")
            .shock_delta_microunits
            .to_string()
    );
}

#[test]
fn pricing_shock_branch_head_and_snapshot_basis_mutation_sweep_is_detectable() {
    for (label, source, branch) in [
        (
            "speculative-branch-head-points-at-main",
            pricing_reference_source_with_branch_head_pointing_to("pricing-shock", "commit:rubber-main"),
            "pricing-shock",
        ),
        (
            "main-branch-head-points-at-speculative",
            pricing_reference_source_with_branch_head_pointing_to("main", "commit:rubber-shock"),
            "main",
        ),
    ] {
        let runtime = build_pricing_runtime(source, RecordingSignalBridgeSink::default());
        let error = runtime
            .evaluate(
                BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new(branch))
                    .with_read_packet(pricing_component_read_packet("rubber")),
            )
            .err()
            .unwrap_or_else(|| panic!("{label} should fail closed under branch-head mutation"));
        assert!(!error.to_string().is_empty());
    }

    let missing_snapshot_runtime = build_pricing_runtime(
        pricing_reference_source_with_missing_branch_head_snapshot(
            "pricing-shock",
            "commit:rubber-shock-missing-snapshot",
            "snapshot:pricing-shock-missing",
            "rubber",
        ),
        RecordingSignalBridgeSink::default(),
    );
    let error = missing_snapshot_runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new(
                "pricing-shock",
            ))
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .err()
        .expect("missing branch-head snapshot basis should fail closed");

    assert!(!error.to_string().is_empty());
}

#[test]
fn pricing_shock_discard_stays_zero_residue_under_interleaved_main_churn() {
    let scenario = generated_pricing_scenario();
    let discard = capture_pricing_discard_bundle();

    assert_eq!(discard.live_main_snapshot, "snapshot:pricing-main-live");
    assert_eq!(discard.speculative_rubber_cost_cents, scenario.speculative_rubber_cost);
    assert_eq!(discard.post_discard_main_snapshot, "snapshot:pricing-main-live");
    assert_eq!(discard.post_discard_main_steel_cost_cents, scenario.live_main_steel_cost);
    assert_eq!(discard.lifecycle_state, BridgePreviewLifecycleStateKind::Discarded);
    assert_eq!(discard.discard_record_count, 1);
    assert_eq!(discard.promotion_record_count, 0);
    assert_eq!(discard.replay_outcome, BridgePreviewLifecycleStateKind::Discarded);
    assert!(discard.has_discard_record);
    assert!(!discard.has_promotion_record);
}

#[test]
fn pricing_shock_promotion_stays_distinct_from_interleaved_main_truth() {
    let scenario = generated_pricing_scenario();
    let promotion = capture_pricing_promotion_bundle();

    assert_eq!(promotion.main_snapshot, "snapshot:pricing-main-interleaved");
    assert_eq!(promotion.speculative_snapshot, "snapshot:pricing-shock");
    assert_eq!(promotion.main_rubber_cost_cents, scenario.interleaved_main_rubber_cost);
    assert_eq!(promotion.speculative_rubber_cost_cents, scenario.speculative_rubber_cost);
    assert_eq!(promotion.lifecycle_state, BridgePreviewLifecycleStateKind::Promoted);
    assert_eq!(
        promotion.promotion_session_identity,
        "pricing:preview-promote-churn"
    );
    assert_eq!(
        promotion.authoritative_commit_boundary_digest,
        "commit-boundary:pricing-churn"
    );
    assert_eq!(
        promotion.authoritative_artifact_digest,
        "authoritative-artifact:pricing-shock"
    );
    assert_eq!(promotion.replay_outcome, BridgePreviewLifecycleStateKind::Promoted);
    assert!(promotion.has_promotion_explanation);
}

#[test]
fn pricing_shock_live_graph_shared_input_fans_out_across_one_hundred_products() {
    let scenario = generated_pricing_scenario();
    let fanout = capture_pricing_fanout_bundle();

    assert_eq!(fanout.total_deliveries, 2);
    assert_eq!(fanout.first_delivery_target_count, 100);
    assert_eq!(fanout.second_delivery_target_count, 100);
    assert_eq!(fanout.second_source_commit, "commit:steel-fanout-b");
    assert_eq!(fanout.second_snapshot, "snapshot:pricing-fanout-b");
    assert_eq!(fanout.branch_snapshot, "snapshot:pricing-fanout-b");
    assert_eq!(fanout.branch_steel_cost_cents, scenario.fanout_second_steel_cost);
    assert_eq!(fanout.retained_target_count, 100);
    assert_eq!(fanout.first_target, "price:product-000");
    assert_eq!(fanout.last_target, "price:product-099");
}

#[test]
fn pricing_shock_writeback_lane_preserves_authority_boundary_and_noop_classification() {
    let writeback = capture_pricing_writeback_bundle(BridgeRuntimePolicy::development());

    assert_eq!(
        writeback.family_kind,
        format!("{:?}", BridgeWritebackFamilyKind::ProjectedStateDiff)
    );
    assert_eq!(
        writeback.strategy_class,
        format!("{:?}", BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation)
    );
    assert_eq!(
        writeback.commit_outcome_class,
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        writeback.noop_outcome_class,
        crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop
    );
    assert_ne!(
        writeback.commit_replay_semantic_digest,
        writeback.noop_replay_semantic_digest
    );
    assert!(writeback.shared_authoritative_artifact);
    assert_eq!(writeback.authority_commit_count, 1);
    assert_eq!(writeback.execution_request_count, 1);
    assert_eq!(writeback.execution_commit_count, 1);
    assert_eq!(writeback.execution_noop_count, 1);
    assert_eq!(
        writeback.rejection_error_kind,
        BridgeWritebackErrorKind::MergeAuthorityRejected
    );
    assert_eq!(
        writeback.rejection_failure_class,
        BridgeWritebackFailureClass::MergeAuthorityRejected
    );
    assert!(writeback.rejection_request_emitted);
    assert!(writeback.rejection_receipt_emitted);
}

#[test]
fn pricing_shock_merge_lane_preserves_aspect_reconciliation_history_and_revisitability() {
    let scenario = generated_pricing_scenario();
    let merge = capture_pricing_merge_bundle(BridgeRuntimePolicy::development());

    assert_eq!(
        merge.bridge_class,
        format!("{:?}", BridgeMergeConsumptionClass::AspectReconciliationMerge)
    );
    assert_eq!(merge.outcome_class, "ContinuityCandidate");
    assert_eq!(merge.blocked_stage, None);
    assert_eq!(merge.denial_class, None);
    assert!(merge.continuity_published);
    assert!(merge.remap_published);
    assert_eq!(merge.main_premerge_snapshot, "snapshot:pricing-main");
    assert_eq!(merge.main_premerge_rubber_cost_cents, scenario.main_rubber_cost);
    assert_eq!(merge.speculative_snapshot, "snapshot:pricing-shock");
    assert_eq!(merge.speculative_rubber_cost_cents, scenario.speculative_rubber_cost);
    assert_eq!(merge.merged_snapshot, "snapshot:pricing-merged");
    assert_eq!(merge.merged_rubber_cost_cents, scenario.speculative_rubber_cost);
    assert_eq!(merge.merged_aspect_registration_id, "pricing-rubber-usd-field");
    assert_eq!(
        merge.merged_fine_grained_match_status,
        format!("{:?}", FineGrainedMatchStatus::Matched)
    );
    assert_eq!(merge.bundle_digest, merge.canonical_replay_digest);
    assert_eq!(merge.replay_request_count, 1);
    assert!(!merge.parent_order_digest.is_empty());
}

#[test]
fn pricing_shock_reference_matrix_preserves_semantic_truth_across_diagnostics_profiles() {
    let baseline =
        capture_pricing_certification_matrix(BridgeRuntimePolicy::development(), "pricing:preview-baseline");
    let forensic =
        capture_pricing_certification_matrix(BridgeRuntimePolicy::forensic(), "pricing:preview-forensic");

    assert_eq!(baseline.reference, forensic.reference);
    assert_eq!(baseline.replay, forensic.replay);
}

#[test]
fn pricing_shock_route_replay_preserves_canonical_main_branch_truth() {
    let replay = capture_pricing_certification_matrix(
        BridgeRuntimePolicy::development(),
        "pricing:preview-replay-control",
    )
    .replay;

    assert_eq!(replay.source_commit, "commit:steel-main");
    assert_eq!(replay.source_snapshot, "snapshot:pricing-main");
    assert!(!replay.route_identity.is_empty());
    assert!(!replay.invalidation_identity.is_empty());
}

#[test]
fn pricing_shock_duplicate_commit_identity_with_conflicting_route_meaning_is_detectable() {
    let runtime = build_pricing_runtime(
        pricing_reference_source_with_conflicting_commit_identity_for_route(),
        RecordingSignalBridgeSink::default(),
    );

    let route = runtime
        .route("commit:steel-main")
        .expect("conflicting duplicate commit identity should still route as retained truth");

    assert_eq!(
        route.result().result_summary().source_commit().as_str(),
        "commit:steel-main"
    );
    assert_eq!(route.result().receipt().snapshot_identity().as_str(), "snapshot:pricing-main");
    assert_eq!(route.result().receipt().delivered_target_count(), 1);
    assert_eq!(
        route.result()
            .artifact()
            .invalidation_targets()
            .targets()[0]
            .signal_scope(),
        "price:scooter"
    );
}

#[test]
fn pricing_shock_non_commuting_route_history_attack_fails_closed_on_replay() {
    let original_runtime = build_pricing_runtime(
        pricing_reference_source(),
        RecordingSignalBridgeSink::default(),
    );
    original_runtime
        .route("commit:steel-main")
        .expect("original steel route should succeed before replay attack");
    let canonical_record = original_runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("original runtime should expose canonical route record");

    let restarted_runtime = build_pricing_runtime(
        pricing_reference_source_with_conflicting_commit_identity_for_route(),
        RecordingSignalBridgeSink::default(),
    );
    let error = restarted_runtime
        .replay_canonical_record(&canonical_record)
        .expect_err("replay should reject non-commuting route history drift");

    assert!(!error.to_string().is_empty());
    let failure_record = restarted_runtime
        .diagnostics()
        .last_failure_record()
        .expect("replay failure should retain diagnostics");
    assert_eq!(failure_record.counters().route_replay_mismatch_count(), 1);
}

#[test]
fn pricing_shock_certification_matrix_distinguishes_control_replay_and_hostile_lanes() {
    let scenario = generated_pricing_scenario();
    let control = capture_pricing_certification_matrix(
        BridgeRuntimePolicy::development(),
        "pricing:preview-certification-control",
    );

    let hostile_source = InMemoryRelationalBridgeSource::default();
    hostile_source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-missing-snapshot",
        "patch:steel-missing-snapshot",
        "snapshot:pricing-missing",
        "steel",
    ));
    let hostile_runtime = build_pricing_runtime(hostile_source, RecordingSignalBridgeSink::default());
    let hostile = capture_pricing_missing_snapshot_failure_bundle(&hostile_runtime);

    assert_eq!(control.reference.route_snapshot, "snapshot:pricing-main");
    assert_eq!(control.reference.source_branch, "main");
    assert_eq!(control.reference.source_commit, "commit:steel-main");
    assert_eq!(control.reference.route_entry_count, 2);
    assert_eq!(control.reference.main_rubber_cost_cents, scenario.main_rubber_cost);
    assert_eq!(
        control.reference.speculative_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert!(!control.reference.evaluation_record_identity.is_empty());
    assert!(!control.reference.evaluation_selector_identity.is_empty());
    assert_eq!(control.replay.source_snapshot, control.reference.route_snapshot);
    assert_eq!(control.replay.source_commit, "commit:steel-main");
    assert_eq!(
        hostile.failure_class,
        BridgeFailureClass::Delivery(BridgeDeliveryErrorKind::SnapshotAcquisitionFailure)
    );
    assert_eq!(hostile.source_commit, "commit:steel-missing-snapshot");
    assert_eq!(hostile.source_snapshot, "snapshot:pricing-missing");
}

#[test]
fn pricing_shock_aspect_lane_preserves_fine_grained_truth_and_history() {
    let aspect = capture_pricing_aspect_bundle(BridgeRuntimePolicy::development());

    assert_eq!(aspect.snapshot, "snapshot:pricing-aspect");
    assert_eq!(aspect.source_branch, "main");
    assert_eq!(aspect.source_commit, "commit:steel-aspect");
    assert_eq!(
        aspect.truth_surface_kind,
        format!("{:?}", TruthDeltaSurfaceKind::EntityField)
    );
    assert_eq!(
        aspect.fine_grained_match_status,
        format!("{:?}", FineGrainedMatchStatus::Matched)
    );
    assert_eq!(aspect.aspect_registration_id, "pricing-steel-usd-field");
    assert_eq!(
        aspect.subscription_slice_kind,
        format!("{:?}", SubscriptionSliceKind::SignalField)
    );
    assert_eq!(aspect.surface_label, "usd");
    assert_eq!(aspect.invalidation_target, "price:bicycle");
}

#[test]
fn pricing_shock_workload_certification_bundle_is_profile_invariant_for_semantic_truth() {
    let baseline = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        "pricing:preview-workload-baseline",
    );
    let forensic = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::forensic(),
        "pricing:preview-workload-forensic",
    );

    assert_eq!(baseline.matrix, forensic.matrix);
    assert_eq!(baseline.aspect, forensic.aspect);
    assert_eq!(baseline.discard, forensic.discard);
    assert_eq!(baseline.promotion, forensic.promotion);
    assert_eq!(baseline.fanout, forensic.fanout);
    assert_eq!(baseline.restart_replay, forensic.restart_replay);
    assert_eq!(baseline.restart_failure, forensic.restart_failure);
    assert_eq!(baseline.writeback, forensic.writeback);
    assert_eq!(baseline.merge, forensic.merge);
    assert_eq!(baseline.provenance, forensic.provenance);
    assert_eq!(baseline.hostile_failure, forensic.hostile_failure);
    assert_eq!(baseline.summary_json(), forensic.summary_json());
    assert_eq!(baseline.digest(), forensic.digest());

    let comparison = baseline.comparison_against(&forensic);
    assert_eq!(comparison["matrix_equal"], json!(true));
    assert_eq!(comparison["aspect_equal"], json!(true));
    assert_eq!(comparison["discard_equal"], json!(true));
    assert_eq!(comparison["promotion_equal"], json!(true));
    assert_eq!(comparison["fanout_equal"], json!(true));
    assert_eq!(comparison["restart_replay_equal"], json!(true));
    assert_eq!(comparison["restart_failure_equal"], json!(true));
    assert_eq!(comparison["writeback_equal"], json!(true));
    assert_eq!(comparison["merge_equal"], json!(true));
    assert_eq!(comparison["provenance_equal"], json!(true));
    assert_eq!(comparison["portfolio_equal"], json!(true));
    assert_eq!(comparison["crisis_equal"], json!(true));
    assert_eq!(comparison["strategy_equal"], json!(true));
    assert_eq!(comparison["simulation_equal"], json!(true));
    assert_eq!(comparison["trust_attacks_equal"], json!(true));
    assert_eq!(comparison["hostile_failure_equal"], json!(true));
    assert_eq!(comparison["suite_25_equal"], json!(true));
    assert_eq!(comparison["suite_26_equal"], json!(true));
    assert_eq!(comparison["suite_27_equal"], json!(true));
    assert_eq!(comparison["summary_equal"], json!(true));
    assert_eq!(comparison["digest_equal"], json!(true));
}

#[test]
fn pricing_shock_workload_certification_bundle_exposes_phase_3_truth_edges() {
    let scenario = generated_pricing_scenario();
    let bundle = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        "pricing:preview-workload-edges",
    );
    let summary = bundle.summary_json();
    let suite_25 = bundle.suite_25_artifact_json();
    let suite_26 = bundle.suite_26_artifact_json();
    let suite_27 = bundle.suite_27_artifact_json();
    let counters = bundle.counter_snapshot_json();

    assert_eq!(bundle.matrix.reference.route_snapshot, "snapshot:pricing-main");
    assert_eq!(bundle.matrix.reference.main_rubber_cost_cents, scenario.main_rubber_cost);
    assert_eq!(
        bundle.matrix.reference.speculative_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert_ne!(
        bundle.matrix.reference.main_snapshot,
        bundle.matrix.reference.speculative_snapshot
    );
    assert_ne!(
        bundle.matrix.reference.main_rubber_cost_cents,
        bundle.matrix.reference.speculative_rubber_cost_cents
    );
    assert_ne!(
        bundle.matrix.reference.speculative_truth_branch,
        bundle.matrix.reference.source_branch
    );
    assert_eq!(bundle.aspect.source_commit, "commit:steel-aspect");
    assert_eq!(bundle.aspect.aspect_registration_id, "pricing-steel-usd-field");
    assert_eq!(bundle.matrix.replay.source_snapshot, "snapshot:pricing-main");
    assert_eq!(bundle.discard.replay_outcome, BridgePreviewLifecycleStateKind::Discarded);
    assert_eq!(bundle.promotion.replay_outcome, BridgePreviewLifecycleStateKind::Promoted);
    assert_ne!(bundle.discard.lifecycle_state, bundle.promotion.lifecycle_state);
    assert!(!bundle.discard.has_promotion_record);
    assert!(bundle.promotion.has_promotion_explanation);
    assert_eq!(bundle.fanout.second_delivery_target_count, 100);
    assert_eq!(bundle.fanout.second_source_commit, "commit:steel-fanout-b");
    assert_eq!(bundle.restart_replay.source_commit, "commit:steel-main");
    assert_eq!(bundle.restart_failure.error_kind, BridgeReplayErrorKind::RouteMismatch);
    assert_eq!(
        bundle.writeback.commit_outcome_class,
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        bundle.writeback.noop_outcome_class,
        crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop
    );
    assert_eq!(bundle.writeback.authority_commit_count, 1);
    assert_eq!(
        bundle.writeback.rejection_error_kind,
        BridgeWritebackErrorKind::MergeAuthorityRejected
    );
    assert_eq!(
        bundle.merge.bridge_class,
        format!("{:?}", BridgeMergeConsumptionClass::AspectReconciliationMerge)
    );
    assert_eq!(bundle.merge.outcome_class, "ContinuityCandidate");
    assert_eq!(bundle.merge.main_premerge_rubber_cost_cents, scenario.main_rubber_cost);
    assert_eq!(
        bundle.merge.speculative_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert_eq!(
        bundle.merge.merged_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert_ne!(
        bundle.merge.main_premerge_rubber_cost_cents,
        bundle.merge.merged_rubber_cost_cents
    );
    assert_eq!(
        bundle.merge.speculative_rubber_cost_cents,
        bundle.merge.merged_rubber_cost_cents
    );
    assert_eq!(bundle.merge.merged_aspect_registration_id, "pricing-rubber-usd-field");
    assert_eq!(bundle.provenance.main_commit, "commit:rubber-main");
    assert_eq!(bundle.provenance.shock_commit, "commit:rubber-shock");
    assert_eq!(bundle.provenance.shock_snapshot, "snapshot:pricing-shock");
    assert_eq!(
        bundle.provenance.shock_delta_microunits,
        scenario.speculative_rubber_cost - scenario.main_rubber_cost
    );
    assert_eq!(bundle.provenance.shock_multiplier_per_mille, 4000);
    assert_eq!(bundle.provenance.representative_sku, "scooter-001");
    assert_eq!(
        bundle.hostile_failure.error_kind,
        BridgeDeliveryErrorKind::SnapshotAcquisitionFailure
    );
    assert_eq!(summary["ordinary_matrix"]["route_snapshot"], json!("snapshot:pricing-main"));
    assert_eq!(summary["fanout_lane"]["second_delivery_target_count"], json!(100));
    assert_eq!(summary["restart_failure"]["error_kind"], json!("RouteMismatch"));
    assert_eq!(
        summary["writeback_lane"]["commit_outcome_class"],
        json!("AuthoritativeCommit")
    );
    assert_eq!(
        summary["writeback_lane"]["noop_outcome_class"],
        json!("CanonicalNoop")
    );
    assert_eq!(summary["writeback_lane"]["authority_commit_count"], json!(1));
    assert_eq!(
        summary["writeback_lane"]["rejection_error_kind"],
        json!("MergeAuthorityRejected")
    );
    assert_eq!(
        summary["merge_lane"]["bridge_class"],
        json!("AspectReconciliationMerge")
    );
    assert_eq!(
        summary["merge_lane"]["outcome_class"],
        json!("ContinuityCandidate")
    );
    assert_eq!(
        summary["merge_lane"]["main_premerge_rubber_cost_cents"],
        json!(scenario.main_rubber_cost)
    );
    assert_eq!(
        summary["merge_lane"]["speculative_rubber_cost_cents"],
        json!(scenario.speculative_rubber_cost)
    );
    assert_eq!(
        summary["merge_lane"]["merged_rubber_cost_cents"],
        json!(scenario.speculative_rubber_cost)
    );
    assert_eq!(
        summary["merge_lane"]["merged_aspect_registration_id"],
        json!("pricing-rubber-usd-field")
    );
    assert_eq!(
        summary["merge_lane"]["merged_fine_grained_match_status"],
        json!("Matched")
    );
    assert_eq!(
        summary["hostile_failure"]["error_kind"],
        json!("SnapshotAcquisitionFailure")
    );
    assert_eq!(summary["aspect_lane"]["surface_label"], json!("usd"));
    assert_eq!(
        summary["historical_provenance"]["shock_commit"],
        json!("commit:rubber-shock")
    );
    assert_eq!(
        summary["historical_provenance"]["shock_snapshot"],
        json!("snapshot:pricing-shock")
    );
    assert_eq!(
        summary["historical_provenance"]["shock_delta_microunits"],
        json!(scenario.speculative_rubber_cost - scenario.main_rubber_cost)
    );
    assert_eq!(
        summary["historical_provenance"]["shock_multiplier_per_mille"],
        json!(4000)
    );
    assert_eq!(summary["portfolio_blast_radius"]["product_count"], json!(100));
    assert_eq!(
        summary["portfolio_blast_radius"]["positive_retail_delta_count"],
        json!(bundle.portfolio.positive_retail_delta_count)
    );
    assert_eq!(
        summary["portfolio_blast_radius"]["top_margin_erosion_family"],
        json!(bundle.portfolio.top_margin_erosion_family)
    );
    assert_eq!(
        summary["portfolio_blast_radius"]["most_shipping_sensitive_family"],
        json!(bundle.portfolio.most_shipping_sensitive_family)
    );
    assert_eq!(
        summary["portfolio_blast_radius"]["most_material_sensitive_family"],
        json!(bundle.portfolio.most_material_sensitive_family)
    );
    assert_eq!(
        summary["portfolio_blast_radius"]["max_retail_delta_sku"],
        json!(bundle.portfolio.max_retail_delta_sku)
    );
    assert_eq!(
        summary["crisis_lane"]["crisis_name"],
        json!("energy-logistics-industrial-crunch")
    );
    assert_eq!(
        summary["crisis_lane"]["policy_pressure_family"],
        json!(bundle.crisis.policy_pressure_family)
    );
    assert_eq!(
        summary["crisis_lane"]["policy_pressure_bps"],
        json!(bundle.crisis.policy_pressure_bps)
    );
    assert_eq!(
        summary["crisis_lane"]["top_exposure_material"],
        json!(bundle.crisis.top_exposure_material)
    );
    assert_eq!(
        summary["strategy_lane"]["recommended_strategy"],
        json!(bundle.strategy.recommended_strategy)
    );
    assert_eq!(
        summary["strategy_lane"]["promotion_strategy"],
        json!(bundle.strategy.promotion_strategy)
    );
    assert_eq!(bundle.simulation.branch_count, 10);
    assert_eq!(bundle.simulation.iterations_per_branch, 10);
    assert_eq!(bundle.simulation.material_summaries.len(), 9);
    assert_eq!(bundle.simulation.iteration_traces.len(), 900);
    assert!(!bundle.simulation.ranked_materials_by_damage.is_empty());
    assert_eq!(bundle.trust_attacks.replay_policy_error_kind, "ReplayPolicyConflict");
    assert_eq!(bundle.trust_attacks.replay_policy_failure_class, "ReplayArtifacts");
    assert_eq!(
        bundle.trust_attacks.route_policy_error_kind,
        "RoutePolicyMismatch"
    );
    assert_eq!(
        bundle.trust_attacks.merge_denial_blocked_stage,
        "Some(DeletionTopologyGate)"
    );
    assert_eq!(
        bundle.trust_attacks.merge_denial_class,
        "TopologyRewireGate"
    );
    assert_eq!(
        summary["simulation_lane"]["branch_count"],
        json!(bundle.simulation.branch_count)
    );
    assert_eq!(
        summary["simulation_lane"]["iterations_per_branch"],
        json!(bundle.simulation.iterations_per_branch)
    );
    assert_eq!(
        summary["simulation_lane"]["material_count"],
        json!(bundle.simulation.material_summaries.len())
    );
    assert_eq!(
        summary["simulation_lane"]["trace_count"],
        json!(bundle.simulation.iteration_traces.len())
    );
    assert_eq!(
        summary["simulation_lane"]["top_damage_material"],
        json!(bundle.simulation.ranked_materials_by_damage.first().cloned().unwrap_or_default())
    );
    assert_eq!(
        summary["trust_attack_lane"]["replay_policy_error_kind"],
        json!("ReplayPolicyConflict")
    );
    assert_eq!(
        summary["trust_attack_lane"]["replay_policy_failure_class"],
        json!("ReplayArtifacts")
    );
    assert_eq!(
        summary["ordinary_matrix"]["main_rubber_cost_cents"],
        json!(scenario.main_rubber_cost)
    );
    assert_eq!(
        summary["ordinary_matrix"]["speculative_rubber_cost_cents"],
        json!(scenario.speculative_rubber_cost)
    );
    assert_eq!(summary["suite_25"], suite_25);
    assert_eq!(summary["suite_26"], suite_26);
    assert_eq!(summary["suite_27"], suite_27);
    assert_eq!(summary["suite_27"]["counter_snapshot"], counters);
    assert_eq!(counters["causality_bundle_count"], json!(1));
    assert_eq!(counters["causality_bundle_replay_match_count"], json!(3));
    assert_eq!(counters["causality_bundle_replay_mismatch_count"], json!(1));
    assert_eq!(counters["failure_taxonomy_classification_count"], json!(3));
    assert_eq!(counters["failure_taxonomy_unclassified_count"], json!(0));
    assert_eq!(counters["diagnostics_entrypoint_request_count"], json!(9));
    assert_eq!(counters["showcase_entrypoint_request_count"], json!(1));
    assert_eq!(counters["simulation_trace_bundle_count"], json!(1));
    assert_eq!(counters["trust_attack_classification_count"], json!(8));
    assert_eq!(counters["diagnostics_entrypoint_reconstruction_count"], json!(1));
    assert_eq!(counters["speculative_branch_bundle_count"], json!(1));
    assert_eq!(counters["speculative_discard_residue_check_count"], json!(1));
    assert_eq!(counters["speculative_discard_residue_nonzero_count"], json!(0));
    assert_eq!(counters["branch_comparison_bundle_count"], json!(1));
    assert_eq!(counters["offline_bundle_diagnosis_count"], json!(1));
    assert_eq!(counters["offline_bundle_insufficiency_count"], json!(0));
    assert!(suite_25["causality_digest"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(suite_25["routing_digest"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(suite_25["explanation_digest"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(suite_25["replay_digest"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(suite_25["reference_workload_bundle_digest"]
        .as_str()
        .is_some_and(|value| !value.is_empty()));
    assert_ne!(suite_25["causality_digest"], suite_25["routing_digest"]);
    assert_ne!(suite_25["routing_digest"], suite_25["replay_digest"]);
    assert_eq!(
        suite_26["failure_localization_matrix"]["routing_failure"]["class"],
        json!("Delivery(SnapshotAcquisitionFailure)")
    );
    assert_eq!(
        suite_26["failure_localization_matrix"]["writeback_failure"]["error_kind"],
        json!("MergeAuthorityRejected")
    );
    assert_eq!(
        suite_26["failure_localization_matrix"]["replay_failure"]["error_kind"],
        json!("RouteMismatch")
    );
    assert_eq!(
        suite_26["failure_localization_matrix"]["residue_surface"]["nonzero_residue_detected"],
        json!(false)
    );
    assert_eq!(
        suite_27["bundle_completeness_report"]["offline_sufficient"],
        json!(true)
    );
    assert_eq!(
        suite_27["bundle_completeness_report"]["insufficiency_count"],
        json!(0)
    );
    assert_eq!(suite_27["diagnostics_entrypoint_matrix"]["routing"], json!(true));
    assert_eq!(suite_27["diagnostics_entrypoint_matrix"]["merge"], json!(true));
    assert_eq!(
        suite_27["diagnostics_entrypoint_matrix"]["historical_provenance"],
        json!(true)
    );
    assert_eq!(suite_27["diagnostics_entrypoint_matrix"]["portfolio"], json!(true));
    assert_eq!(suite_27["diagnostics_entrypoint_matrix"]["crisis"], json!(true));
    assert_eq!(suite_27["diagnostics_entrypoint_matrix"]["strategy"], json!(true));
    assert_eq!(suite_27["diagnostics_entrypoint_matrix"]["simulation"], json!(true));
    assert_eq!(suite_27["diagnostics_entrypoint_matrix"]["trust_attacks"], json!(true));
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["main_vs_speculative_snapshot_distinct"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["merged_vs_speculative_rubber_cost_equal"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["merged_vs_premerge_rubber_cost_distinct"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["historical_provenance_commit_matches_shock"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["portfolio_reports_positive_blast_radius"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["crisis_affects_portfolio_breadth"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["strategy_recommends_non_hold_response"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["promotion_strategy_prefers_authoritative_action"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["simulation_identifies_at_least_one_damaging_material"],
        json!(true)
    );
    assert_eq!(
        suite_27["reference_workload_bundle_comparison"]["trust_attack_matrix_is_typed"],
        json!(true)
    );
    assert!(!bundle.digest().is_empty());
}

#[test]
fn pricing_shock_repricing_signal_is_delta_driven_not_always_on() {
    let scenario = generated_pricing_scenario();
    let product_count = scenario.main_portfolio.len();
    let main_repricing_count = scenario
        .main_portfolio
        .iter()
        .filter(|entry| entry.repricing_triggered)
        .count();
    let shock_repricing_count = scenario
        .speculative_portfolio
        .iter()
        .filter(|entry| entry.repricing_triggered)
        .count();
    let increased_shock_pressure_count = scenario
        .main_portfolio
        .iter()
        .zip(scenario.speculative_portfolio.iter())
        .filter(|(main_entry, shock_entry)| {
            shock_entry.landed_cost_delta_cents > main_entry.landed_cost_delta_cents
        })
        .count();

    assert!(main_repricing_count < product_count);
    assert!(shock_repricing_count <= product_count);
    assert!(increased_shock_pressure_count > 0);
    assert!(scenario
        .main_portfolio
        .iter()
        .all(|entry| entry.repricing_threshold_cents > 0));
    assert!(scenario
        .speculative_portfolio
        .iter()
        .all(|entry| entry.repricing_threshold_cents > 0));
    assert!(scenario
        .main_portfolio
        .iter()
        .any(|entry| !entry.repricing_triggered));
    assert!(scenario.main_portfolio.iter().all(|entry| {
        entry.repricing_triggered
            == ((entry.repricing_threshold_cents > 0
                && entry.landed_cost_delta_cents >= entry.repricing_threshold_cents)
                || entry.margin_floor_breached)
    }));
    assert!(scenario.speculative_portfolio.iter().all(|entry| {
        entry.repricing_triggered
            == ((entry.repricing_threshold_cents > 0
                && entry.landed_cost_delta_cents >= entry.repricing_threshold_cents)
                || entry.margin_floor_breached)
    }));
}

#[test]
fn pricing_shock_suites_25_through_27_emit_canonical_machine_checkable_artifacts() {
    let bundle = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        "pricing:preview-workload-suites",
    );
    let suite_25 = bundle.suite_25_artifact_json();
    let suite_26 = bundle.suite_26_artifact_json();
    let suite_27 = bundle.suite_27_artifact_json();

    assert_ne!(
        suite_25["reference_workload_bundle_digest"],
        suite_26["reference_workload_failure_bundle_digest"]
    );
    assert_ne!(suite_26["failure_digest"], suite_26["replay_failure_digest"]);
    assert_eq!(
        suite_27["counter_snapshot"]["offline_bundle_insufficiency_count"],
        json!(0)
    );
    assert_eq!(
        suite_27["diagnostics_entrypoint_matrix"],
        json!({
            "routing": true,
            "branch_isolation": true,
            "policy": true,
            "source": true,
            "preview": true,
            "merge": true,
            "writeback": true,
            "residue": true,
            "historical_provenance": true,
            "portfolio": true,
            "crisis": true,
            "strategy": true,
            "simulation": true,
            "trust_attacks": true,
        })
    );
    assert_eq!(
        suite_27["bundle_completeness_report"],
        json!({
            "has_routing_artifact": true,
            "has_branch_comparison_artifact": true,
            "has_policy_artifact": true,
            "has_source_artifact": true,
            "has_preview_artifact": true,
            "has_merge_artifact": true,
            "has_writeback_artifact": true,
            "has_residue_artifact": true,
            "has_historical_provenance_artifact": true,
            "has_portfolio_artifact": true,
            "has_crisis_artifact": true,
            "has_strategy_artifact": true,
            "has_simulation_artifact": true,
            "has_trust_attack_artifact": true,
            "offline_sufficient": true,
            "insufficiency_count": 0,
        })
    );
}

#[test]
fn pricing_shock_showcase_artifact_explains_retained_commit_without_hidden_memory() {
    let scenario = generated_pricing_scenario();
    let bundle = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        "pricing:preview-showcase-artifact",
    );
    let artifact = bundle.showcase_artifact_json();
    let shock_commit = bundle
        .showcase_commit_explorer_json("commit:rubber-shock")
        .expect("shock commit should be explorable from the showcase artifact");
    let markdown = bundle.showcase_markdown_report();

    assert_eq!(
        artifact["executive_summary"]["shock_commit"],
        json!("commit:rubber-shock")
    );
    assert_eq!(
        artifact["executive_summary"]["shock_multiplier_per_mille"],
        json!(4000)
    );
    assert_eq!(
        artifact["executive_summary"]["fanout_target_count"],
        json!(100)
    );
    assert_eq!(
        artifact["executive_summary"]["main_vs_speculative_rubber_delta_cents"],
        json!(scenario.speculative_rubber_cost - scenario.main_rubber_cost)
    );
    assert_eq!(
        artifact["branch_comparison"]["main_and_speculative_distinct"],
        json!(true)
    );
    assert_eq!(
        artifact["branch_comparison"]["merged_matches_speculative_rubber_cost"],
        json!(true)
    );
    assert_eq!(
        artifact["retained_commit_explorer"]["commit:rubber-shock"]["shock_delta_microunits"],
        json!(scenario.speculative_rubber_cost - scenario.main_rubber_cost)
    );
    assert_eq!(
        artifact["retained_commit_explorer"]["commit:rubber-shock"]["shock_multiplier_per_mille"],
        json!(4000)
    );
    assert_eq!(
        artifact["retained_commit_explorer"]["commit:rubber-shock"]["representative_sku"],
        json!("scooter-001")
    );
    assert_eq!(artifact["portfolio_blast_radius"]["product_count"], json!(100));
    assert_eq!(
        artifact["portfolio_blast_radius"]["positive_retail_delta_count"],
        json!(bundle.portfolio.positive_retail_delta_count)
    );
    assert_eq!(
        artifact["portfolio_blast_radius"]["top_margin_erosion_family"],
        json!(bundle.portfolio.top_margin_erosion_family)
    );
    assert_eq!(
        artifact["multi_factor_crisis"]["crisis_name"],
        json!("energy-logistics-industrial-crunch")
    );
    assert_eq!(
        artifact["multi_factor_crisis"]["policy_pressure_family"],
        json!(bundle.crisis.policy_pressure_family)
    );
    assert_eq!(
        artifact["strategy_comparison"]["recommended_strategy"],
        json!(bundle.strategy.recommended_strategy)
    );
    assert_eq!(
        artifact["strategy_comparison"]["promotion_strategy"],
        json!(bundle.strategy.promotion_strategy)
    );
    assert_eq!(
        artifact["shock_simulation"]["branch_count"],
        json!(bundle.simulation.branch_count)
    );
    assert_eq!(
        artifact["shock_simulation"]["iterations_per_branch"],
        json!(bundle.simulation.iterations_per_branch)
    );
    assert_eq!(
        artifact["shock_simulation"]["ranked_materials_by_damage"][0],
        json!(bundle.simulation.ranked_materials_by_damage[0].clone())
    );
    let trust_attacks = artifact["trust_attack_matrix"]
        .as_array()
        .expect("trust attack matrix should be an array");
    assert_eq!(trust_attacks.len(), 8);
    assert!(trust_attacks.iter().any(|entry| {
        entry["attack"] == json!("replay_policy_mismatch")
            && entry["classification"] == json!("ReplayPolicyConflict")
    }));
    assert!(trust_attacks.iter().any(|entry| {
        entry["attack"] == json!("simulation_damaging_material_ranked")
    }));
    assert_eq!(
        artifact["trust_proof"]["suite_27"]["bundle_completeness_report"]["offline_sufficient"],
        json!(true)
    );
    assert_eq!(
        artifact["trust_attack_matrix"][0]["attack"],
        json!("missing_snapshot_basis")
    );
    assert_eq!(
        artifact["trust_attack_matrix"][1]["classification"],
        json!("RouteMismatch")
    );
    let trust_attacks = artifact["trust_attack_matrix"]
        .as_array()
        .expect("trust attack matrix should be an array");
    assert_eq!(trust_attacks.len(), 8);
    assert!(trust_attacks.iter().any(|entry| {
        entry["attack"] == json!("replay_policy_mismatch")
            && entry["classification"] == json!("ReplayPolicyConflict")
    }));
    assert!(trust_attacks.iter().any(|entry| {
        entry["attack"] == json!("simulation_damaging_material_ranked")
    }));
    assert_eq!(
        artifact["demo_flow"][3],
        json!("measure portfolio blast radius")
    );
    assert_eq!(
        artifact["demo_artifact_family"]["showcase_digest"],
        json!(bundle.digest())
    );
    assert_eq!(
        artifact["timeline"][2]["commit"],
        json!("commit:rubber-shock")
    );
    assert_eq!(
        artifact["timeline"][4]["snapshot"],
        json!(bundle.merge.merged_snapshot)
    );
    assert_eq!(
        shock_commit["snapshot"],
        json!("snapshot:pricing-shock")
    );
    assert_eq!(
        shock_commit["representative_retail_price_cents"],
        json!(bundle.provenance.representative_retail_price_cents)
    );
    assert!(markdown.contains("# Pricing Shock Showcase Report"));
    assert!(markdown.contains("commit:rubber-shock"));
    assert!(markdown.contains("scooter-001"));
    assert!(markdown.contains("Suite 27"));
    assert!(markdown.contains("Trust Attacks"));
    assert!(markdown.contains("Demo Flow"));
    assert!(markdown.contains("energy-logistics-industrial-crunch"));
    assert!(markdown.contains(&bundle.strategy.recommended_strategy));
    assert!(markdown.contains(&bundle.simulation.ranked_materials_by_damage[0]));
}

#[test]
fn pricing_shock_ml_pipeline_export_contains_full_traceable_simulation_artifacts() {
    let bundle = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        "pricing:preview-ml-export",
    );
    let export = bundle.ml_pipeline_export_json();

    assert_eq!(
        export["schema"],
        json!("forge-runtime-bridge.pricing-showcase.ml-pipeline.v1")
    );
    assert_eq!(export["bundle_digest"], json!(bundle.digest()));
    assert_eq!(
        export["simulation"]["branch_count"],
        json!(bundle.simulation.branch_count)
    );
    assert_eq!(
        export["simulation"]["iterations_per_branch"],
        json!(bundle.simulation.iterations_per_branch)
    );
    assert_eq!(
        export["simulation"]["material_summaries"]
            .as_array()
            .map(|array| array.len())
            .unwrap_or_default(),
        bundle.simulation.material_summaries.len()
    );
    assert_eq!(
        export["simulation"]["iteration_traces"]
            .as_array()
            .map(|array| array.len())
            .unwrap_or_default(),
        bundle.simulation.iteration_traces.len()
    );
    assert_eq!(
        export["simulation"]["ranked_materials_by_damage"][0],
        json!(bundle.simulation.ranked_materials_by_damage[0].clone())
    );
    assert_eq!(
        export["showcase_artifact"]["executive_summary"]["shock_commit"],
        json!("commit:rubber-shock")
    );
    assert_eq!(
        export["lineage_provenance"]["reference_lineage"]["source_commit"],
        json!(bundle.matrix.reference.source_commit)
    );
    assert_eq!(
        export["lineage_provenance"]["reference_lineage"]["speculative_truth_branch"],
        json!(bundle.matrix.reference.speculative_truth_branch)
    );
    assert_eq!(
        export["lineage_provenance"]["route_and_aspect_lineage"]["aspect_registration_id"],
        json!(bundle.aspect.aspect_registration_id)
    );
    assert_eq!(
        export["lineage_provenance"]["speculation_lifecycle_lineage"]["promotion_session_identity"],
        json!(bundle.promotion.promotion_session_identity)
    );
    assert_eq!(
        export["lineage_provenance"]["writeback_and_merge_lineage"]["merge_bundle_digest"],
        json!(bundle.merge.bundle_digest)
    );
    assert_eq!(
        export["lineage_provenance"]["historical_provenance"]["shock_commit"],
        json!(bundle.provenance.shock_commit)
    );
    assert_eq!(
        export["lineage_provenance"]["historical_provenance"]["shock_delta_microunits"],
        json!(bundle.provenance.shock_delta_microunits)
    );
    assert_eq!(
        export["lineage_provenance"]["hostile_and_trust_lineage"]["hostile_source_commit"],
        json!(bundle.hostile_failure.source_commit)
    );
    assert_eq!(
        export["lineage_provenance"]["hostile_and_trust_lineage"]["replay_policy_error_kind"],
        json!(bundle.trust_attacks.replay_policy_error_kind)
    );
    assert_eq!(
        export["lineage_provenance"]["causality"]["bundle_digest"],
        json!(bundle.digest())
    );
    assert_eq!(
        export["lineage_provenance"]["causality"]["suite_25_causality_digest"],
        bundle.suite_25_artifact_json()["causality_digest"]
    );
    assert_eq!(
        export["suite_27"]["reference_workload_bundle_comparison"]["simulation_identifies_at_least_one_damaging_material"],
        json!(true)
    );
    assert_eq!(
        export["suite_27"]["reference_workload_bundle_comparison"]["trust_attack_matrix_is_typed"],
        json!(true)
    );
}

#[test]
fn pricing_shock_can_emit_ml_pipeline_export_file_when_requested() {
    let Some(path) = std::env::var_os("FORGE_PRICING_SHOWCASE_EXPORT_PATH") else {
        return;
    };
    let bundle = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        "pricing:preview-ml-export-file",
    );
    let export = serde_json::to_string_pretty(&bundle.ml_pipeline_export_json())
        .expect("ml pipeline export should serialize");
    std::fs::write(&path, export).expect("ml pipeline export file should write");
}

#[test]
fn pricing_shock_restart_replay_preserves_canonical_truth_across_rebuild() {
    let restart = capture_pricing_restart_replay_bundle(BridgeRuntimePolicy::development());
    let replay = capture_pricing_certification_matrix(
        BridgeRuntimePolicy::development(),
        "pricing:preview-restart-parity",
    )
    .replay;

    assert_eq!(restart.source_commit, replay.source_commit);
    assert_eq!(restart.source_snapshot, replay.source_snapshot);
    assert_eq!(restart.route_identity, replay.route_identity);
    assert_eq!(restart.invalidation_identity, replay.invalidation_identity);
}

#[test]
fn pricing_shock_restart_replay_rejects_route_drift_after_truth_change() {
    let restart_failure = capture_pricing_restart_failure_bundle();

    assert_eq!(
        restart_failure.error_kind,
        BridgeReplayErrorKind::RouteMismatch
    );
    assert_eq!(restart_failure.replay_mismatch_count, 1);
}

#[test]
fn pricing_shock_missing_snapshot_fails_with_typed_delivery_record() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(pricing_patch(
        "main",
        "commit:steel-missing-snapshot",
        "patch:steel-missing-snapshot",
        "snapshot:pricing-missing",
        "steel",
    ));

    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_pricing_runtime(source, sink);
    let failure = capture_pricing_missing_snapshot_failure_bundle(&runtime);

    assert_eq!(failure.error_kind, BridgeDeliveryErrorKind::SnapshotAcquisitionFailure);
    assert_eq!(
        failure.failure_class,
        BridgeFailureClass::Delivery(BridgeDeliveryErrorKind::SnapshotAcquisitionFailure)
    );
    assert_eq!(failure.source_commit, "commit:steel-missing-snapshot");
    assert_eq!(failure.source_snapshot, "snapshot:pricing-missing");
}
