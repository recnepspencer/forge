use super::*;

pub fn plan_write_compatibility(
    batch: &mut CompatibilityAdmissionBatch,
    manifest_index: &CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    writer_capabilities: &WriterCapabilitySet,
    intent: &CompatibilityWriteIntent,
    artifact: &QuarantinedDecodedArtifact,
) -> Result<WriteCompatibilityReceipt, CompatibilityRejection> {
    plan_write_compatibility_for_path(
        batch,
        manifest_index,
        edge_registry,
        writer_capabilities,
        intent,
        artifact,
        CompatibilityAdmissionPath::HotRead,
    )
}
pub(super) fn plan_write_compatibility_for_path(
    batch: &mut CompatibilityAdmissionBatch,
    manifest_index: &CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    writer_capabilities: &WriterCapabilitySet,
    intent: &CompatibilityWriteIntent,
    artifact: &QuarantinedDecodedArtifact,
    path: CompatibilityAdmissionPath,
) -> Result<WriteCompatibilityReceipt, CompatibilityRejection> {
    if artifact.family_id() != intent.family_id()
        || artifact.family_id() != writer_capabilities.family_id()
    {
        batch.counters.receipt_reuse_rejection_count += 1;
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::FamilyMismatch,
            artifact.family_id().clone(),
            "write compatibility family mismatch",
        ));
    }
    let key = ReceiptKey::write(manifest_index, artifact, writer_capabilities, intent, path);
    if let Some(receipt) = batch.write_receipts.get(&key) {
        batch.counters.receipt_reuse_hit_count += 1;
        batch.counters.accepted_count += 1;
        return Ok(receipt.clone());
    }
    if has_stale_receipt_basis(batch.write_receipts.keys(), &key) {
        batch.counters.receipt_basis_mismatch_count += 1;
        batch.counters.receipt_reuse_rejection_count += 1;
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::ReceiptBasisMismatch,
            artifact.family_id().clone(),
            "write receipt reuse basis does not match registry or manifest frontier",
        ));
    }
    if let Err(rejection) = manifest_index.lookup(artifact, &mut batch.counters) {
        batch.counters.rejected_count += 1;
        return Err(rejection);
    }
    let relation = match resolve_relation(
        &mut batch.counters,
        edge_registry,
        artifact.family_id(),
        artifact.semantic_version(),
        intent.target_semantic_version(),
        path,
    ) {
        Ok(relation) => relation,
        Err(rejection) => {
            batch.counters.rejected_count += 1;
            return Err(rejection);
        }
    };
    if !writer_capabilities.admits_semantic_version(intent.target_semantic_version()) {
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::WriterCapabilityUnsupported,
            artifact.family_id().clone(),
            "writer capability does not admit target semantic version",
        ));
    }
    let receipt = WriteCompatibilityReceipt::new(CompatibilityAdmissionReceipt::new(
        artifact.family_id().clone(),
        artifact.manifest_digest().clone(),
        manifest_index.registry_snapshot_identity(),
        manifest_index.manifest_frontier_identity(),
        artifact.semantic_version(),
        intent.target_semantic_version(),
        path,
        relation,
    ));
    batch.write_receipts.insert(key, receipt.clone());
    batch.counters.accepted_count += 1;
    batch.counters.record_admitted_relation(relation);
    Ok(receipt)
}
