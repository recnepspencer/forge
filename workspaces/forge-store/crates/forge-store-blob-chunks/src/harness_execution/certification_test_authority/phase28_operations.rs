use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_security::{
    store_offline_transfer_boundary_fact, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreOfflineExportImportBoundaryEvidence,
    StoreOfflineExportImportBoundaryFact, StoreSecurityScopeAdmissionExpectation, StoreTenantScope,
    StoreTrustBoundaryReadmissionTrigger,
};

use crate::capsule_readiness::{
    BlobCapsuleMaterializationAuthority, BlobCapsuleSliceDeclaration, BlobCapsuleSliceSelection,
};
use crate::handoffs::{
    BlobHarnessChunkTopology, BlobHarnessPlacementClass, BlobHarnessSecurityScopeClass,
};
use crate::harness_execution::backend::{current_authority, physical_payload_for_bytes};
use crate::harness_execution::chunk_sequence::{build_chunk_sequence, chunk_window_for_ordinal};
use crate::harness_execution::export_publication::publish_export_bundle;
use crate::harness_execution::lifecycle_execution::execute_lifecycle;
use crate::harness_execution::placement_admission::admit_placement;
use crate::harness_execution::scope_admission::blob_scope;
use crate::{
    bridge_canonical_export_trust_boundary, BlobChunkByteWindow, BlobGeneration,
    BlobGenerationRegistry, BlobGenerationRegistryAdmission, BlobImportReadmissionAuthority,
    BlobObjectClassificationAdmission, BlobStreamingReadObservation,
    BlobStreamingReadObservedChunk, BlobStreamingReadWindow, BlobStreamingVerifiedRead,
};

pub struct Phase28OperationsWitnesses {
    export_bundle: crate::BlobExportPublishedBundle,
    readmitted_import: crate::ReadmittedBlobImport<'static>,
    capsule_readiness: crate::BlobCapsuleReadinessWitness,
}

impl Phase28OperationsWitnesses {
    pub fn export_bundle(&self) -> &crate::BlobExportPublishedBundle {
        &self.export_bundle
    }

    pub const fn readmitted_import(&self) -> &crate::ReadmittedBlobImport<'static> {
        &self.readmitted_import
    }

    pub const fn capsule_readiness(&self) -> &crate::BlobCapsuleReadinessWitness {
        &self.capsule_readiness
    }
}

pub fn phase28_operations_witnesses(
    case: &str,
    logical_bytes: &'static [u8],
    chunk_size: u64,
) -> Phase28OperationsWitnesses {
    let topology = BlobHarnessChunkTopology::from_executed_projection(
        logical_bytes.len().div_ceil(chunk_size as usize) as u64,
        logical_bytes.len() as u64,
        chunk_size,
    )
    .expect("topology");
    let generated = build_chunk_sequence(
        case,
        BlobHarnessSecurityScopeClass::ScopePreserving,
        topology,
        None,
    );
    let publication =
        crate::BlobChunkRootPublication::publish(generated.sequence.clone()).expect("publication");
    let lane = execute_lifecycle(
        case,
        BlobHarnessSecurityScopeClass::ScopePreserving,
        &publication,
        &generated,
    );
    let placed_lane = crate::harness_execution::lifecycle_execution::ExecutedBlobLane {
        placement: admit_placement(
            case,
            &lane.reachability,
            BlobHarnessPlacementClass::StoreLocal,
        ),
        ..lane
    };
    let export_bundle = publish_export_bundle(case, &placed_lane, &publication, &generated);

    let import_authority = BlobImportReadmissionAuthority::from_current_store_authority(
        current_authority(case, "import"),
    );
    let current_chunks = generated
        .sequence
        .proof_frontier()
        .ordered_leaves()
        .iter()
        .map(|leaf| {
            let (offset, bytes) = chunk_window_for_ordinal(&generated, leaf.ordinal().get());
            let bytes = Box::leak(bytes.into_boxed_slice());
            import_authority
                .collect_current_chunk_evidence(
                    leaf,
                    BlobChunkByteWindow::borrowed(offset, bytes).expect("window"),
                )
                .expect("current chunk evidence")
        })
        .collect::<Vec<_>>();
    let bridged = Box::leak(Box::new(bridge_canonical_export_trust_boundary(
        &export_bundle,
    )));
    let readmitted_import = import_authority
        .readmit_import_declaration_after_boundary(
            bridged,
            offline_export_import_trigger(case, bridged.declaration().chunk_scope()),
            &current_chunks,
        )
        .expect("readmission should admit");

    let ordered_leaves = Box::leak(
        generated
            .sequence
            .proof_frontier()
            .ordered_leaves()
            .to_vec()
            .into_boxed_slice(),
    );
    let capsule_placement = placed_lane.placement.clone();
    let capsule_reachability = placed_lane.reachability.clone();
    let mut registry = BlobGenerationRegistry::new();
    let classification =
        BlobObjectClassificationAdmission::from_executed_lifecycle(&placed_lane.lifecycle);
    BlobGenerationRegistryAdmission::from_executed_lifecycle(
        publication.clone(),
        placed_lane.lifecycle,
        classification,
    )
    .publish(&mut registry, registry_authority(case))
    .expect("registry publication");
    let registry = Box::leak(Box::new(registry));
    let observation = registry
        .observe_registered_generation(export_bundle.object_id(), BlobGeneration::published(1))
        .expect("registered generation");
    let capsule_authority = BlobCapsuleMaterializationAuthority::from_generation_observation(
        &observation,
        ordered_leaves,
    )
    .expect("capsule authority");
    let scope = blob_scope(case, BlobHarnessSecurityScopeClass::ScopePreserving);
    let declaration = BlobCapsuleSliceDeclaration::for_generation(observation.generation())
        .select(
            BlobCapsuleSliceSelection::chunk_ordinals(
                ordered_leaves.iter().map(|leaf| leaf.ordinal().get()),
            )
            .expect("selection"),
        )
        .require_parent_root_basis();
    let planned = capsule_authority.plan_slice(declaration).expect("planned");
    let classified = capsule_authority
        .classify_slice_for_materialization(planned, &scope, &capsule_placement, &[])
        .expect("classified");
    let observations = ordered_leaves
        .iter()
        .map(|leaf| {
            let (offset, bytes) = chunk_window_for_ordinal(&generated, leaf.ordinal().get());
            BlobStreamingReadObservation::from_chunk(
                BlobStreamingReadObservedChunk::from_store_payload(
                    leaf.ordinal(),
                    offset,
                    physical_payload_for_bytes(&bytes),
                    BlobStreamingReadWindow::bounded(8).expect("window"),
                )
                .expect("observed chunk"),
            )
        })
        .collect::<Vec<_>>();
    let verified_read = BlobStreamingVerifiedRead::for_movement_certification_test(
        export_bundle.object_id().clone(),
        export_bundle.generation(),
        export_bundle.chunk_tree_root().clone(),
        export_bundle
            .digest_evidence()
            .logical_content_digest()
            .clone(),
        topology.logical_bytes(),
    );
    let prepared = capsule_authority
        .admit_materialized_capsule_read(&classified, verified_read, observations)
        .expect("prepared");
    let materialized = capsule_authority
        .materialize_capsule_bundle(classified, &capsule_reachability, prepared)
        .expect("materialized");
    let capsule_readiness = capsule_authority
        .publish_capsule_readiness(materialized)
        .expect("readiness");

    Phase28OperationsWitnesses {
        export_bundle,
        readmitted_import,
        capsule_readiness,
    }
}

fn offline_export_import_trigger(
    case: &str,
    declaration: forge_store_security::StoreRawSecurityScopeDeclaration,
) -> StoreTrustBoundaryReadmissionTrigger {
    let authority = current_authority(case, "import");
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        forge_store_security::StoreKeyScope::BlobChunkEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        forge_store_security::StoreCustodyPosture::Readmitted,
    );
    StoreTrustBoundaryReadmissionTrigger::offline_export_import(
        StoreOfflineExportImportBoundaryFact::from_readmission_candidate(
            StoreOfflineExportImportBoundaryEvidence::from_category_facts(
                store_offline_transfer_boundary_fact(boundary_fact(
                    "store.trust_boundary.offline_transfer",
                    "exported",
                ))
                .expect("exported"),
                store_offline_transfer_boundary_fact(boundary_fact(
                    "store.trust_boundary.offline_transfer",
                    "current",
                ))
                .expect("current"),
            )
            .expect("offline category"),
            declaration,
            &authority,
            expectation,
        )
        .expect("offline trigger"),
    )
}

fn registry_authority(case: &str) -> crate::BlobGenerationRegistryAuthority {
    crate::BlobGenerationRegistryAuthority::from_current_store_authority(current_authority(
        &format!("{case}.registry"),
        "registry",
    ))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(
            admitted_state,
            forge_store_aspect_native::StorePhysicalBoundaryWitness::from_physical_authority(
                StorePhysicalAuthorityWitness::for_aspect_native_boundary(
                    ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
                )
                .expect("physical authority"),
            )
            .expect("physical boundary"),
        ),
    )
    .expect("boundary fact")
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> forge_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}
