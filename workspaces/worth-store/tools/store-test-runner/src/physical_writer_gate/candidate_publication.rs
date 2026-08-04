use std::path::Path;

pub(super) fn inspect_current_sources(record_serving_root: &Path) -> Result<(), String> {
    let frame_ports = record_serving_root.join("residency/candidate_frame_residency.rs");
    let frame_ports = std::fs::read_to_string(frame_ports)
        .map_err(|error| format!("cannot read C.6 frame port seam: {error}"))?;
    inspect_publication_port(&frame_ports)?;

    let write_evidence =
        record_serving_root.join("residency/candidate_frame_residency/write_evidence.rs");
    let write_evidence = std::fs::read_to_string(write_evidence)
        .map_err(|error| format!("cannot read candidate write evidence: {error}"))?;
    inspect_write_evidence(&write_evidence)?;

    let write_progression =
        record_serving_root.join("residency/candidate_frame_residency/write_progression.rs");
    let write_progression = std::fs::read_to_string(write_progression)
        .map_err(|error| format!("cannot read candidate write progression: {error}"))?;
    inspect_write_progression(&write_progression)
}

fn inspect_publication_port(source: &str) -> Result<(), String> {
    let start = source
        .find("trait CandidateFramePublicationPort")
        .ok_or_else(|| "C.6 candidate publication port is missing".to_owned())?;
    let candidate_tail = &source[start..];
    if candidate_tail.contains("QualifiedFilesystemMedia") {
        return Err(
            "C.5/C.6 boundary: CandidateFramePublicationPort acquired physical media authority"
                .to_owned(),
        );
    }
    let contract = trait_contract(source, "CandidateFramePublicationPort")?;
    for (fragment, authority) in [
        ("publication_progression", "publication progression"),
        ("RecordArtifactFile", "artifact naming"),
        ("MediaCounterSnapshot", "media-effect evidence"),
        ("ArtifactTreeFailure", "backend failure"),
        ("FnMut", "Store-owned write callback"),
        ("replace_catalog", "catalog replacement"),
    ] {
        if contract.contains(fragment) {
            return Err(format!(
                "C.5/C.6 boundary: CandidateFramePublicationPort acquired {authority} authority"
            ));
        }
    }
    if !contract.contains("fn begin<'allocation>(")
        || !contract.contains(
            "allocation: &'allocation worth_store_buffer_pool::ForegroundWriteAllocationGrant",
        )
        || !contract.contains("CandidateFrameResidencySession + 'allocation")
        || contract.contains("fn submit(")
    {
        return Err(
            "C.5/C.6 boundary: candidate port must borrow exact allocation proof for the full residency session without submitting publication".to_owned(),
        );
    }
    let residency = trait_contract(source, "CandidateFrameResidencySession")?;
    if !residency.contains("fn retain(")
        || !residency.contains("Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial>")
        || residency.contains("FnMut")
        || residency.contains("ArtifactTreeFailure")
    {
        return Err(
            "C.5/C.6 boundary: residency must own each frame for the duration of Store's physical write"
                .to_owned(),
        );
    }
    let resident = trait_contract(source, "ResidentCandidateFrame")?;
    if !resident.contains("fn role(&self) -> CandidateFrameRole;")
        || !resident.contains("fn coordinate(&self) -> CandidateFrameCoordinate;")
        || !resident.contains("fn bytes(&self) -> &[u8];")
        || !resident.contains("fn publish_clean(")
        || !resident.contains("Result<CandidateFrameWriteCompletion, RecordAppendDenial>")
        || resident.contains("FnMut")
        || resident.contains("ArtifactTreeFailure")
    {
        return Err(
            "C.5/C.6 boundary: the resident guard must expose bytes to Store and release ownership without acquiring write authority"
                .to_owned(),
        );
    }
    Ok(())
}

fn inspect_write_evidence(source: &str) -> Result<(), String> {
    for forbidden in [
        "Option<CompletedArtifactNewWrite>",
        "Option<crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement>",
        "for_contract_test",
    ] {
        if source.contains(forbidden) {
            return Err(format!(
                "C.6 candidate write evidence contains forbidden optional or test-only proof state: {forbidden}"
            ));
        }
    }
    let physical_write = struct_contract(source, "CandidateFramePhysicalWrite")?;
    for required in [
        "receipt: CompletedArtifactNewWrite,",
        "settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,",
    ] {
        if !physical_write.contains(required) {
            return Err(format!(
                "C.6 candidate write evidence does not require complete receipt and settlement proof: missing {required}"
            ));
        }
    }
    let receipt = physical_write
        .find("receipt: CompletedArtifactNewWrite,")
        .expect("required receipt was checked");
    let settlement = physical_write
        .find("settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,")
        .expect("required settlement was checked");
    if receipt >= settlement {
        return Err(
            "C.6 candidate write evidence must bind physical receipt before Store settlement"
                .to_owned(),
        );
    }
    Ok(())
}

fn inspect_write_progression(source: &str) -> Result<(), String> {
    if source.contains("for_contract_test")
        || source.contains("complete_candidate_publication")
        || source.contains("publish_clean(&physical)")
    {
        return Err(
            "C.6 candidate write progression contains forged or lower-authority cleaning"
                .to_owned(),
        );
    }
    let store_write = source
        .find("store_write(resident.bytes())")
        .ok_or_else(|| "C.6 candidate write progression omitted the Store effect".to_owned())?;
    let settlement = source
        .find(".settle_residency(")
        .ok_or_else(|| "C.6 candidate write progression skipped exact settlement".to_owned())?;
    let publish_clean = source
        .find(".publish_clean(settlement)")
        .ok_or_else(|| "C.6 candidate write progression skipped settled cleaning".to_owned())?;
    if !(store_write < settlement && settlement < publish_clean) {
        return Err(
            "C.6 candidate write progression must order Store effect, settlement, then cleaning"
                .to_owned(),
        );
    }
    Ok(())
}

fn trait_contract<'source>(source: &'source str, name: &str) -> Result<&'source str, String> {
    let marker = format!("trait {name}");
    let start = source
        .find(&marker)
        .ok_or_else(|| format!("C.6 `{name}` contract is missing"))?;
    let tail = &source[start..];
    let end = tail
        .find("\n}")
        .ok_or_else(|| format!("C.6 `{name}` contract is malformed"))?
        + 2;
    Ok(&tail[..end])
}

fn struct_contract<'source>(source: &'source str, name: &str) -> Result<&'source str, String> {
    let marker = format!("struct {name}");
    let start = source
        .find(&marker)
        .ok_or_else(|| format!("C.6 `{name}` contract is missing"))?;
    let tail = &source[start..];
    let end = tail
        .find('}')
        .ok_or_else(|| format!("C.6 `{name}` contract is malformed"))?
        + 1;
    Ok(&tail[..end])
}

#[test]
fn candidate_port_cannot_acquire_current_truth_authority() {
    let mutant = r#"
        pub(super) trait CandidateFramePublicationPort {
            fn submit(
                &self,
                media: &QualifiedFilesystemMedia,
                candidate: CandidateFrameSet,
            ) -> Result<PublishedRecordBatch, RecordAppendError> {
                publication_progression::execute(media, candidate.into_plan())
            }
        }
    "#;
    let denial = inspect_publication_port(mutant)
        .expect_err("a C.6 port with media and publication authority must be rejected");
    assert!(denial.contains("physical media"));
}

#[test]
fn write_evidence_rejects_optional_or_test_forged_proof() {
    for mutant in [
        "receipt: Option<CompletedArtifactNewWrite>,",
        "settlement: Option<CanonicalRecordMutationSettlement>,",
        "fn for_contract_test() -> Self { todo!() }",
        "fn completed(receipt: CompletedArtifactNewWrite) -> Self { todo!() }",
    ] {
        inspect_write_evidence(mutant)
            .expect_err("incomplete or test-forged physical proof must be rejected");
    }
}

#[test]
fn write_evidence_accepts_semantic_fields_independent_of_indentation() {
    let source = "
        struct CandidateFramePhysicalWrite {
          receipt: CompletedArtifactNewWrite,
            settlement: crate::physical_runtime::record_serving::CanonicalRecordMutationSettlement,
        }
    ";
    inspect_write_evidence(source).unwrap();
}

#[test]
fn progression_rejects_skipped_or_reordered_settlement() {
    let valid = "
        let physical = store_write(resident.bytes());
        let settlement = physical.settle_residency();
        resident.publish_clean(settlement);
    ";
    inspect_write_progression(valid).unwrap();

    for mutant in [
        "
            let physical = store_write(resident.bytes());
            resident.publish_clean(settlement);
        ",
        "
            resident.publish_clean(settlement);
            let physical = store_write(resident.bytes());
            let settlement = physical.settle_residency();
        ",
        "
            let physical = CandidateFramePhysicalWrite::for_contract_test();
            let settlement = physical.settle_residency();
            resident.publish_clean(settlement);
        ",
    ] {
        inspect_write_progression(mutant)
            .expect_err("skipped, reordered, or forged candidate settlement must fail");
    }
}
