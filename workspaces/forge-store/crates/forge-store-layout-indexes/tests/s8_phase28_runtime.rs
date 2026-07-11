use forge_store_blob_chunks::certification_test_authority::phase28_operations_witnesses;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase28_offline_verifier_rule, OfflineVerifierAccessShape, OfflineVerifierAuthorityPosture,
    OfflineVerifierEvidenceKind, Phase28OfflineVerifierLayoutExt,
};
use forge_store_offline_verifier::{
    inspect_offline_export_bundle, OfflineExportChunkDeclaration, OfflineExportDigestEvidence,
};
use forge_store_operations::{
    CapsuleOperationLayoutReport, ExportLayoutEvidenceReport, ImportLayoutEvidenceReport,
};

#[test]
fn phase28_offline_layout_reports_consume_the_admitted_grammar_rule() {
    let declarations = vec![OfflineExportChunkDeclaration {
        ordinal: 0,
        chunk_identity: "s7:chunk:a".to_owned(),
        stored_digest: "s7:stored:a".to_owned(),
        checksum_digest: "fnv64:abc".to_owned(),
        bytes: 4,
    }];
    let digest = OfflineExportDigestEvidence {
        logical_content_digest: "s7:logical:a".to_owned(),
        export_bundle_digest: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_owned(),
        declaration_digest: declaration_digest(&declarations),
        declared_chunk_count: 1,
        declared_total_bytes: 4,
    };
    let rule = phase28_offline_verifier_rule().expect("phase-28 offline rule");
    let export = inspect_offline_export_bundle(declarations, digest)
        .expect("offline observation")
        .admit_offline_verifier_layout(rule);
    assert_eq!(
        export.family_id(),
        DurableArtifactFamilyId::OfflineVerificationRecord
    );
    assert_eq!(
        export.authority_posture(),
        OfflineVerifierAuthorityPosture::TerminalOnly
    );
    assert_eq!(
        export.declared_access_shape(),
        OfflineVerifierAccessShape::FullDeclaredScan
    );
    assert_eq!(
        export.evidence_kind(),
        OfflineVerifierEvidenceKind::ExportBundle
    );
    assert!(export.cannot_be_foreground_authority());
}

#[test]
fn phase28_operations_layout_reports_follow_real_blob_witness_paths_and_closeout() {
    let witnesses = phase28_operations_witnesses("phase28.runtime", b"abcdefghijklmno", 4);
    let export = ExportLayoutEvidenceReport::from_blob_export_bundle(witnesses.export_bundle());
    let import =
        ImportLayoutEvidenceReport::from_readmitted_blob_import(witnesses.readmitted_import());
    let capsule =
        CapsuleOperationLayoutReport::from_blob_capsule_readiness(witnesses.capsule_readiness());

    assert_eq!(export.family_id(), DurableArtifactFamilyId::ExportBundle);
    assert!(export.cannot_be_foreground_authority());
    assert_eq!(export.declared_chunks(), 4);

    assert_eq!(import.family_id(), DurableArtifactFamilyId::ImportBundle);
    assert!(import.cannot_be_foreground_authority());
    assert_eq!(import.declared_chunks(), 4);
    assert_eq!(import.local_chunks(), 4);

    assert_eq!(
        capsule.family_id(),
        DurableArtifactFamilyId::CapsuleArtifact
    );
    assert!(capsule.cannot_be_foreground_authority());
    assert_eq!(capsule.declared_bytes(), 15);
}

fn declaration_digest(declarations: &[OfflineExportChunkDeclaration]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = stable_hash_bytes(hash, b"phase19.export.declarations");
    let total_bytes: u64 = declarations.iter().map(|chunk| chunk.bytes).sum();
    for declaration in declarations {
        hash = stable_hash_bytes(hash, &declaration.ordinal.to_le_bytes());
        hash = stable_hash_bytes(hash, declaration.chunk_identity.as_bytes());
        hash = stable_hash_bytes(hash, declaration.stored_digest.as_bytes());
        hash = stable_hash_bytes(hash, declaration.checksum_digest.as_bytes());
        hash = stable_hash_bytes(hash, &declaration.bytes.to_le_bytes());
    }
    hash = stable_hash_bytes(hash, &total_bytes.to_le_bytes());
    hash = stable_hash_bytes(hash, &(declarations.len() as u64).to_le_bytes());
    format!("s7:export-declarations:{hash:016x}")
}

fn stable_hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
