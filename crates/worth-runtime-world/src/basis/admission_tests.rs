use std::sync::Arc;

use worth_foundational::facade::{
    AspectBinding, AspectKey, AspectMask, AuthoritativeAspectChangeKind, FieldKey, ProjectionMask,
    ScalarAspectType,
};
use worth_relational::facade::{
    bridge::RuntimeBridgeRelationalSource,
    runtime::{RelationalRuntime, RelationalRuntimeApi},
};
use worth_runtime_bridge::facade::{
    AdmittedRuntimeWorldCorrespondenceBasis, BridgeAspectRegistration, BridgeAspectRegistrationId,
    BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    BridgeSemanticCorrespondenceRegistration, BridgeSemanticDependencyCandidate,
    BridgeSemanticDependencyCandidateParts, BridgeSemanticLocality,
    BridgeSignalAspectTargetDeclaration, CoarseRoutingMode, InvalidationSink, MappingSelector,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SliceWideningPolicy,
    SnapshotReadContract, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
    TruthPatchTargetSelector,
};
use worth_signal::facade::{SignalGraph, SignalRuntime};

use super::{admit_current, CompositeBasisAdmissionDenial};
use crate::basis::compare_exact;
use crate::identity::RuntimeWorldIdentityIssuer;

#[derive(Clone)]
struct AdmissionSink;

impl InvalidationSink for AdmissionSink {
    fn deliver_invalidation(
        &self,
        delivery: worth_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

struct ComponentFixture {
    relational: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
    signal: worth_signal::facade::branch::AdmittedSignalBranchBasis,
    signal_port: worth_signal::facade::branch::SignalBranchBasisPort<(), (), ()>,
    correspondence: AdmittedRuntimeWorldCorrespondenceBasis,
    _signal_runtime: SignalRuntime<(), (), (), (), ()>,
}

fn component_fixture() -> ComponentFixture {
    let relational_runtime = Arc::new(RelationalRuntimeApi::builder().build());
    let relational_identity = relational_runtime.main_branch_identity();
    let (_, relational) = relational_runtime
        .observe_branch(&relational_identity)
        .expect("real Relational owner admits its main basis");

    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let worth_proof::TransitionOutcome::Success(node) = graph.admit_installed_node(node) else {
        panic!("real Signal owner admits its installed node");
    };
    let aspect_key = AspectKey::new("profile").expect("valid aspect key");
    let field_key = FieldKey::new("name").expect("valid field key");
    let snapshot_contract =
        SnapshotReadContract::scalar(aspect_key.clone(), ScalarAspectType::String);
    let dependency_contract = snapshot_contract.aspect_contract().clone();
    let mapping = BridgeMappingRegistration::new(
        BridgeMappingId::from_stable_name("profile-name"),
        TruthPatchScope::for_target(
            MappingSelector::exact("relational-record:entity:0:1:1"),
            aspect_key.clone(),
            TruthPatchTargetSelector::authoritative_aspect(),
        ),
        snapshot_contract.clone(),
        SignalInvalidationScope::from_stable_name("signal.profile.name"),
        CoarseRoutingMode::Direct,
    );
    let aspect_registration = BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::from_stable_name("profile-name"),
        mapping.truth_scope().clone(),
        snapshot_contract,
        TruthDeltaSurfaceKind::AuthoritativeAspect,
        SubscriptionSliceKind::SignalField,
        SliceWideningPolicy::Disallow,
    );
    let target = BridgeSignalAspectTargetDeclaration::allocate(
        BridgeAspectRegistrationId::from_stable_name("profile-name"),
        worth_signal::facade::PartitionToken::new("bridge-main"),
        node,
    );
    let source = RuntimeBridgeRelationalSource::for_graph_role(
        Arc::clone(&relational_runtime),
        "runtime-world-test",
    )
    .expect("real Relational owner provides the Bridge source");
    let source_profile = source.authoritative_source_profile();
    let dependency =
        BridgeSemanticDependencyCandidate::admit(BridgeSemanticDependencyCandidateParts {
            source_installation_identity: Arc::from("relational-installation:1"),
            source_basis: Arc::from("relational-main"),
            source_runtime_authority: source_profile.runtime_instance_id(),
            source_installation_generation: 1,
            source_authority_binding_identity: Arc::from("relational-owner-binding:1"),
            source_stage_identity: None,
            source_node_identity: Arc::from("runtime-world-test"),
            dependency_ordinal: 0,
            declared_graph_role: Arc::from("runtime-world-test"),
            graph_participation_identity: Arc::from("runtime-world-graph"),
            graph_adapter_identity: Arc::from("relational-bridge-adapter"),
            source_record_identity: Some(
                worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 1, 1),
            ),
            observation_record_identity: Some(
                worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 1, 1),
            ),
            contract: dependency_contract,
            projection_mask: AspectMask::<ProjectionMask>::whole_aspect(),
            binding: AspectBinding::EntityField { field: field_key },
            locality: BridgeSemanticLocality::SourceRecord,
            relevant_changes: vec![AuthoritativeAspectChangeKind::FieldSet],
        })
        .expect("real Bridge owner admits its semantic dependency");
    let registration =
        BridgeSemanticCorrespondenceRegistration::new(dependency.clone(), vec![target])
            .expect("real Bridge registration is coherent");
    let bridge = RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(AdmissionSink)
        .register_mapping(mapping)
        .register_aspect_mapping(aspect_registration)
        .register_semantic_correspondence(registration)
        .build()
        .expect("real Bridge owner builds the installed correspondence");
    let installed = {
        let mut binding = bridge
            .bind_signal_graph(&mut graph)
            .expect("Bridge binds the real Signal graph");
        match binding.install_semantic_correspondence(dependency) {
            worth_proof::TransitionOutcome::Success(installed) => installed,
            other => {
                panic!("real Bridge owner admits its registered correspondence: {other:?}")
            }
        }
    };
    let correspondence = bridge
        .runtime_world_correspondence_port()
        .admit_installed_basis(&installed)
        .expect("Bridge Runtime World port admits its installed basis");

    let mut signal_runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let signal_port = signal_runtime
        .owner_component_services()
        .expect("real Signal owner issues its basis service")
        .basis_port();
    let signal = signal_runtime
        .observe_signal_branch_basis(signal_runtime.current_branch())
        .expect("real Signal owner admits its current basis");

    ComponentFixture {
        relational,
        signal,
        signal_port,
        correspondence,
        _signal_runtime: signal_runtime,
    }
}

#[test]
fn foreign_owner_equal_descriptor_cannot_substitute_during_composite_admission() {
    let fixture = component_fixture();
    let mut foreign_signal_runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let foreign_signal_port = foreign_signal_runtime
        .owner_component_services()
        .expect("foreign real Signal owner issues its basis service")
        .basis_port();
    let transported_signal = fixture.signal.clone();
    assert_eq!(
        fixture.signal.descriptor(),
        transported_signal.descriptor(),
        "the transported artifact has an exactly equal serializable descriptor"
    );

    let (mut identities, owner) = RuntimeWorldIdentityIssuer::new().expect("World owner");
    let (mut foreign_identities, foreign_owner) =
        RuntimeWorldIdentityIssuer::new().expect("foreign World owner");
    assert_ne!(owner, foreign_owner);

    let denial = admit_current(
        &mut identities,
        &foreign_signal_port,
        fixture.relational.clone(),
        transported_signal,
        fixture.correspondence.clone(),
    )
    .expect_err("a foreign Signal owner must reject the equal-looking basis");
    assert!(matches!(
        denial,
        CompositeBasisAdmissionDenial::Signal(
            worth_signal::facade::branch::SignalBranchBasisReadmissionDenial::OwnerMismatch { .. }
        )
    ));

    let admitted = admit_current(
        &mut identities,
        &fixture.signal_port,
        fixture.relational.clone(),
        fixture.signal.clone(),
        fixture.correspondence.clone(),
    )
    .expect("the Signal owner port admits the live basis");
    let foreign_world_admission = admit_current(
        &mut foreign_identities,
        &fixture.signal_port,
        fixture.relational.clone(),
        fixture.signal.clone(),
        fixture.correspondence.clone(),
    )
    .expect("the second World owner admits through the real Signal owner port");
    let repeated_admission = admit_current(
        &mut identities,
        &fixture.signal_port,
        fixture.relational,
        fixture.signal,
        fixture.correspondence,
    )
    .expect("a second owner admission remains a valid World operation");

    assert_eq!(admitted.owner_identity(), owner);
    assert_eq!(admitted, admitted.clone());
    assert_eq!(
        admitted.signal_basis().descriptor(),
        foreign_world_admission.signal_basis().descriptor(),
        "foreign World admissions carry equal Signal descriptors"
    );
    assert_ne!(
        admitted, foreign_world_admission,
        "World owner identity, not a Signal descriptor, defines composite equivalence"
    );
    assert!(compare_exact(&admitted, &foreign_world_admission).is_err());
    assert_ne!(
        admitted, repeated_admission,
        "distinct World owner-issued admissions do not collapse by descriptor"
    );
    assert!(compare_exact(&admitted, &admitted.clone()).is_ok());
    assert!(compare_exact(&admitted, &repeated_admission).is_err());
}
