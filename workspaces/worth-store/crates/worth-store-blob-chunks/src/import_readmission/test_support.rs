#![cfg_attr(not(test), allow(dead_code, unused_imports))]

use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_operations_vocabulary::{
    BackupExportCustodyDeclaration, BackupExportCustodyMode, BackupExportCustodyReadiness,
};
use worth_store_security::{
    store_custody_domain_boundary_fact, store_key_scope_generation_boundary_fact,
    store_offline_transfer_boundary_fact, store_tenant_scope_authority_boundary_fact,
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyDomainBoundaryEvidence, StoreCustodyDomainBoundaryFact,
    StoreKeyScopeGenerationBoundaryEvidence, StoreKeyScopeGenerationBoundaryFact,
    StoreKeyVersionPosture, StoreOfflineExportImportBoundaryEvidence,
    StoreOfflineExportImportBoundaryFact, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreTenantScope,
    StoreTenantScopeAuthorityBoundaryEvidence, StoreTenantScopeAuthorityBoundaryFact,
    StoreTrustBoundaryCrossing, StoreTrustBoundaryReadmissionTrigger,
};

use crate::placement::admission::test_support::admit_inline_placement;
use crate::reachability::BlobChunkReachabilityRegistry;
use crate::test_support::{admitted_multichunk_sequence_for_scope, blob_scope, current_authority};
use crate::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobChunkByteWindow, BlobChunkProofLeaf,
    BlobChunkRootPublication, BlobExportAuthority, BlobExportIntent, BlobExportPublishedBundle,
    BlobGeneration, BlobGenerationRegistry, BlobGenerationRegistryAdmission,
    BlobImportReadmissionAuthority, BlobImportedChunkEvidence, BlobLifecycleAdmission,
    BlobLifecycleDeclaration, BlobLifecycleReadinessAuthority, BlobLifecycleReplayInput,
    BlobLifecycleStoreAuthority, BlobObjectId, ScopedBlobChunk,
};

pub(crate) struct ImportLane<'a> {
    pub(crate) bundle: BlobExportPublishedBundle,
    pub(crate) ordered_leaves: Vec<BlobChunkProofLeaf>,
    pub(crate) bytes: &'a [u8],
    pub(crate) placement: crate::AdmittedBlobPlacement,
    pub(crate) reachability: crate::BlobChunkReachabilityProofSet,
}

pub(crate) fn import_lane(
    case: &str,
    bytes: &'static [u8],
    chunk_size: u64,
) -> ImportLane<'static> {
    let export_authority =
        BlobExportAuthority::from_current_store_authority(current_authority(case, "export"));
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    let sequence = admitted_multichunk_sequence_for_scope(scope, bytes, chunk_size);
    let publication = BlobChunkRootPublication::publish(sequence.clone()).expect("publication");
    let ordered_leaves = sequence.proof_frontier().ordered_leaves().to_vec();
    let declaration = BlobLifecycleDeclaration::new(
        BlobObjectId::from_declared_digest(
            crate::lifecycle::generation_registry_test_support::digest(&format!(
                "sha256:{case}.object"
            )),
        ),
        BlobGeneration::published(1),
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        ordered_leaves[0].security_metadata(),
        ordered_leaves[0].stored_digest().clone(),
        AuthenticatedFrameDigest::from_declared_digest(
            crate::lifecycle::generation_registry_test_support::digest(&format!(
                "sha256:{case}.frame"
            )),
        ),
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let object_id = declaration.object_id().clone();
    let generation = declaration.generation();
    let mut reachability_registry = BlobChunkReachabilityRegistry::new_store_owned();
    let mut exported = Vec::new();
    for leaf in &ordered_leaves {
        let start = leaf.byte_range().start() as usize;
        let end = leaf.byte_range().end() as usize;
        let chunk_bytes = &bytes[start..end];
        let proof = crate::test_support::integrity_proof_for_scope(
            blob_scope(case, StoreTenantScope::TenantPhysicalBoundary),
            chunk_bytes,
        );
        reachability_registry
            .admit_lifecycle_primary_reference(
                &declaration,
                ScopedBlobChunk::from_integrity_proof(proof),
            )
            .expect("reachability should admit");
        exported.push(
            export_authority
                .collect_exported_chunk_bytes(
                    leaf,
                    BlobChunkByteWindow::borrowed(leaf.byte_range().start(), chunk_bytes)
                        .expect("window"),
                )
                .expect("export input should admit"),
        );
    }
    let reachability = reachability_registry
        .prove_reachable_chunks()
        .expect("proof");
    let placement = admit_inline_placement(&reachability);
    let store_authority = BlobLifecycleStoreAuthority::from_current_store_authority(
        crate::lifecycle::generation_registry_test_support::current_authority(case, "lifecycle"),
    );
    let lowering = store_authority.lowering_capability();
    let readiness = BlobLifecycleReadinessAuthority::from_admitted_placement(placement.clone());
    let lifecycle = BlobLifecycleAdmission::start(declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(reachability.clone())
        .success("reachability should admit")
        .admit_placement(placement.clone())
        .success("placement should admit")
        .ready_for_execution(readiness)
        .success("readiness should admit")
        .execute_lifecycle_replay(BlobLifecycleReplayInput::from_stored_chunk_digest(
            reachability.stored_digest().clone(),
        ))
        .success("lifecycle should execute")
        .into_lifecycle_receipt();
    let mut registry = BlobGenerationRegistry::new();
    let classification =
        crate::BlobObjectClassificationAdmission::from_executed_lifecycle(&lifecycle);
    BlobGenerationRegistryAdmission::from_executed_lifecycle(
        publication.clone(),
        lifecycle,
        classification,
    )
    .publish(
        &mut registry,
        crate::lifecycle::generation_registry_test_support::registry_authority(case),
    )
    .expect("registry publication should admit");
    let observation = registry
        .observe_registered_generation(&object_id, generation)
        .expect("registered generation");
    let bundle = export_authority
        .publish_export_bundle(
            BlobExportIntent::for_current_lifecycle(
                observation.lifecycle_receipt(),
                &publication,
                &reachability,
                &placement,
                &export_readiness(case),
            )
            .with_export_name("tenant/blob/gen")
            .with_exported_chunks(exported),
        )
        .expect("bundle should publish");
    ImportLane {
        bundle,
        ordered_leaves,
        bytes,
        placement,
        reachability,
    }
}

pub(crate) fn collect_current_chunks<'a>(
    authority: &BlobImportReadmissionAuthority,
    lane: &'a ImportLane<'a>,
) -> Vec<BlobImportedChunkEvidence<'a>> {
    lane.ordered_leaves
        .iter()
        .map(|leaf| {
            let start = leaf.byte_range().start() as usize;
            let end = leaf.byte_range().end() as usize;
            authority
                .collect_current_chunk_evidence(
                    leaf,
                    BlobChunkByteWindow::borrowed(
                        leaf.byte_range().start(),
                        &lane.bytes[start..end],
                    )
                    .expect("window"),
                )
                .expect("current chunk evidence")
        })
        .collect()
}

pub(crate) fn readmission_trigger(
    crossing: StoreTrustBoundaryCrossing,
    declaration: StoreRawSecurityScopeDeclaration,
    case: &str,
) -> StoreTrustBoundaryReadmissionTrigger {
    let authority = current_authority(case, "import");
    let expectation = blob_import_expectation();
    match crossing {
        StoreTrustBoundaryCrossing::OfflineExportImport => {
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
        StoreTrustBoundaryCrossing::KeyScopeGenerationChanged => {
            StoreTrustBoundaryReadmissionTrigger::key_scope_generation_changed(
                StoreKeyScopeGenerationBoundaryFact::from_readmission_candidate(
                    StoreKeyScopeGenerationBoundaryEvidence::from_category_facts(
                        store_key_scope_generation_boundary_fact(boundary_fact(
                            "store.trust_boundary.key_scope_generation",
                            "exported",
                        ))
                        .expect("exported"),
                        store_key_scope_generation_boundary_fact(boundary_fact(
                            "store.trust_boundary.key_scope_generation",
                            "current",
                        ))
                        .expect("current"),
                    )
                    .expect("key category"),
                    declaration,
                    &authority,
                    expectation,
                )
                .expect("key trigger"),
            )
        }
        StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged => {
            StoreTrustBoundaryReadmissionTrigger::tenant_scope_authority_changed(
                StoreTenantScopeAuthorityBoundaryFact::from_readmission_candidate(
                    StoreTenantScopeAuthorityBoundaryEvidence::from_category_facts(
                        store_tenant_scope_authority_boundary_fact(boundary_fact(
                            "store.trust_boundary.tenant_scope_authority",
                            "exported",
                        ))
                        .expect("exported"),
                        store_tenant_scope_authority_boundary_fact(boundary_fact(
                            "store.trust_boundary.tenant_scope_authority",
                            "current",
                        ))
                        .expect("current"),
                    )
                    .expect("tenant category"),
                    declaration,
                    &authority,
                    expectation,
                )
                .expect("tenant trigger"),
            )
        }
        StoreTrustBoundaryCrossing::CustodyDomainChanged => {
            StoreTrustBoundaryReadmissionTrigger::custody_domain_changed(
                StoreCustodyDomainBoundaryFact::from_readmission_candidate(
                    StoreCustodyDomainBoundaryEvidence::from_category_facts(
                        store_custody_domain_boundary_fact(boundary_fact(
                            "store.trust_boundary.custody_domain",
                            "exported",
                        ))
                        .expect("exported"),
                        store_custody_domain_boundary_fact(boundary_fact(
                            "store.trust_boundary.custody_domain",
                            "current",
                        ))
                        .expect("current"),
                    )
                    .expect("custody category"),
                    declaration,
                    &authority,
                    expectation,
                )
                .expect("custody trigger"),
            )
        }
        other => panic!("unsupported crossing for tests: {other:?}"),
    }
}

pub(super) fn blob_import_expectation() -> StoreSecurityScopeAdmissionExpectation {
    StoreSecurityScopeAdmissionExpectation::new(
        worth_store_security::StoreKeyScope::BlobChunkEnvelope,
        worth_store_security::StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        worth_store_security::StoreCustodyPosture::Readmitted,
    )
}

pub(crate) fn export_readiness(case: &str) -> BackupExportCustodyReadiness {
    let authority = current_authority(case, "export");
    let admission = BackupExportCustodyDeclaration::native(
        &authority,
        BackupExportCustodyMode::Export,
        StoreKeyVersionPosture::Current,
    )
    .expect("custody declaration should admit")
    .admit_with_current_authority(&authority)
    .expect("custody admission should succeed");
    BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("export readiness should build")
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
            worth_store_aspect_native::StorePhysicalBoundaryWitness::from_physical_authority(
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
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

trait TestTransitionSuccess<S> {
    fn success(self, message: &str) -> S;
}

impl<S, D, De, St, R, F> TestTransitionSuccess<S> for TransitionOutcome<S, D, De, St, R, F>
where
    S: core::fmt::Debug,
    D: core::fmt::Debug,
    De: core::fmt::Debug,
    St: core::fmt::Debug,
    R: core::fmt::Debug,
    F: core::fmt::Debug,
{
    fn success(self, message: &str) -> S {
        match self {
            TransitionOutcome::Success(value) => value,
            outcome => panic!("{message}: {outcome:?}"),
        }
    }
}
